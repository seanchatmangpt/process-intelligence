# Defining a Custom Telemetry Registry with Weaver

A **Custom Telemetry Registry** is the central repository where an organization defines its telemetry contracts (metrics, logs, traces). Rather than relying solely on global, generic semantic conventions, a custom registry allows you to define enterprise-specific namespaces, establish attribute requirement levels, and import/extend official OpenTelemetry schemas.

---

## 1. Registry Architecture and Feedstock Alignment

In a mature process intelligence environment, telemetry is the **feedstock** that feeds the **court** of process consequence analysis. If the feedstock is inconsistent, downstream calculations (such as throughput time, activity transitions, and compliance rules) will produce invalid assertions. 

The custom registry serves as the schema contract for this feedstock. By defining it clearly, you ensure that every microservice and system component emits data structured exactly as required for compliance audits.

---

## 2. Directory Layout of a Custom Registry

A standard registry repository consists of a metadata manifest file at the root, and one or more subdirectories containing YAML group definitions:

```text
my-telemetry-registry/
├── manifest.yaml
└── groups/
    ├── application.yaml
    ├── database.yaml
    └── transaction.yaml
```

---

## 3. Creating `manifest.yaml`

The `manifest.yaml` file defines the registry identity, schema URL version, and upstream dependencies.

Create `manifest.yaml` in the root of your registry folder:

```yaml
name: "acme-corp-telemetry"
description: "Acme Corporation Core Telemetry Semantic Conventions Registry"
schema_url: "https://opentelemetry.io/schemas/1.25.0"
dependencies:
  - name: "semconv"
    registry_path: "https://github.com/open-telemetry/semantic-conventions.git@v1.25.0[model]"
```

### Manifest Fields:
*   **`name`**: The unique identifier for your organization's registry.
*   **`schema_url`**: The base URL mapping to the specific OpenTelemetry schema version. This must match the schema version of your dependencies.
*   **`dependencies`**: Integrates upstream semantic conventions. Here we import the official OpenTelemetry semantic conventions registry (`semconv`) at tag `v1.25.0`, specifying the `[model]` directory where semantic conventions reside.

---

## 4. Writing Group Definitions

Inside the `groups/` directory, write YAML files to define your attributes, metrics, spans, and events.

### 4.1. Defining Attribute Groups
Attributes must be defined in groups. An `attribute_group` is an auxiliary bucket used to define attributes so they can be referenced elsewhere.

Create `groups/application.yaml`:

```yaml
file_format: "definition/2"
groups:
  - id: acme.app
    type: attribute_group
    brief: "Attributes describing Acme application runtime environments."
    prefix: acme.app
    attributes:
      - id: environment
        type: string
        brief: "The target execution environment (e.g. production, staging, development)."
        stability: stable
        requirement_level: required
        examples: ["production", "staging"]
      - id: tenant_id
        type: string
        brief: "Unique identifier for the SaaS tenant accessing the system."
        stability: stable
        requirement_level: required
        examples: ["tenant-992a", "tenant-883c"]
```

### 4.2. Defining Span Conventions
Spans represent execution steps. You can define a group of type `span` and reference attributes defined in your own registry or dependencies.

Create `groups/transaction.yaml`:

```yaml
file_format: "definition/2"
groups:
  - id: acme.transaction
    type: span
    brief: "Conventions for financial checkout operations."
    attributes:
      # Referencing custom attributes defined above:
      - ref: acme.app.tenant_id
        requirement_level: required
      - ref: acme.app.environment
        requirement_level: recommended
      # Referencing standard OTel attributes imported via dependencies:
      - ref: service.name
        requirement_level: required
      - ref: http.request.method
        requirement_level: recommended
      # Defining inline transaction-specific attributes:
      - id: acme.transaction.amount
        type: double
        brief: "The monetary value of the checkout transaction."
        requirement_level: required
        stability: stable
        examples: [99.95, 1250.00]
```

### 4.3. Defining Metrics
Metrics are numeric instruments. Define a group of type `metric`.

Create `groups/database.yaml`:

```yaml
file_format: "definition/2"
groups:
  - id: acme.db.pool.active_connections
    type: metric
    metric_name: acme.db.pool.active_connections
    instrument: gauge
    unit: "1"
    brief: "The current count of active database connections in the connection pool."
    stability: stable
    attributes:
      - ref: acme.app.environment
        requirement_level: required
```

---

## 5. Validating Your Custom Registry

Once the registry files are written, run the `weaver registry check` command to verify that:
1.  All YAML files parse successfully.
2.  All `ref` statements resolve cleanly against your attributes and the imported `semconv` dependency.
3.  The schema follows OTel registry rules.

```bash
weaver registry check --registry ./my-telemetry-registry --future
```

If you have custom policy rules in a directory called `./policies/`:
```bash
weaver registry check \
  --registry ./my-telemetry-registry \
  --policy ./policies
```
This validation guarantees that the custom registry files represent a sound, correct telemetry schema contract before code generation or live checks are executed.
