# Telemetry Schema Migration and Version Upgrades

As software platforms evolve, telemetry schemas must undergo updates. Attributes are renamed for clarity, metric definitions are aligned with UCUM standards, and stability lifecycle states transition. This document outlines migration strategies, differences between Weaver V1 and V2 schema formats, and upgrading from legacy generation scripts.

---

## 1. Migration Philosophy: Schema Evolution vs. Process Drift

From our architectural guidelines, we must strictly separate the static schema contract level from the dynamic process runtime execution level:

*   **Telemetry Schema Migrations**: Represent static adjustments to the syntax and structures of our telemetry contracts. They are structural upgrades.
*   **Process Drift**: Represents changes in actual runtime behavior (e.g., execution path changes, sequence delays, or activity omissions) compared to process models.
*   **Feedstock and Court Connection**: Upgrading raw telemetry feedstock schemas must be done using version translation files. This maintains historical continuity so that process mining engines (court) can evaluate trace logs spanning multiple years without correlation failures.

---

## 2. Upgrading Registries from V1 to V2 Schema Specifications

Weaver introduces support for the **V2 Schema Specification** (`--v2` flag). Understanding the structure changes between V1 and V2 is vital for maintaining valid registries.

### 2.1. Structural Differences
The V2 specification changes how metrics, attributes, and entities are linked:

| Feature / Concept | V1 Specification | V2 Specification |
| :--- | :--- | :--- |
| **Attribute Binding** | Attributes are defined inline in groups or referenced using flat lists. | Introduction of `entity` types and structured associations (`entity_associations`) to define bindings. |
| **Metric Definition** | Metrics define attributes directly in the metric group. | Metrics bind attributes via explicit attribute groups, promoting reuse and modularity. |
| **Stability Enforcement** | Basic stability fields. | Granular stability lifecycle gates (`alpha`, `beta`, `release_candidate`, `stable`) with strict policy checking. |

### 2.2. Migrating YAML files to V2
When upgrading a group YAML to V2, you modify the `file_format` header and structure your metric and attribute links using references:

#### Legacy V1 Format:
```yaml
file_format: "definition/1"
groups:
  - id: app.request
    type: metric
    metric_name: app.request.duration
    instrument: histogram
    unit: "s"
    brief: "Duration of requests"
    attributes:
      - id: method
        type: string
        brief: "HTTP method"
```

#### Modern V2 Format (aligned with Weaver V2 resolving engine):
```yaml
file_format: "definition/2"
groups:
  - id: app.request.duration
    type: metric
    metric_name: app.request.duration
    instrument: histogram
    unit: "s"
    brief: "Duration of requests"
    attributes:
      - ref: app.request.method
        requirement_level: required

  - id: app.request
    type: attribute_group
    brief: "Shared HTTP request parameters"
    prefix: app.request
    attributes:
      - id: method
        type: string
        brief: "HTTP method"
        stability: stable
```

To validate your registry with the V2 resolving engine, execute:
```bash
weaver registry check --registry ./my-registry --v2
```

---

## 3. Migrating from Legacy Python `semconvgen` to Weaver

Many OpenTelemetry repositories historically used a legacy Python script (`semconvgen` or the generic semantic conventions generator) to build code and markdown tables.

### 3.1. Why Migrate to Weaver?
*   **Performance**: Weaver is written in Rust, generating artifacts up to 100x faster than legacy scripts.
*   **OPA Rego Policies**: Weaver contains a built-in policy validation step. Python scripts could not validate naming rules or requirements using declarative policy languages.
*   **Dependency Management**: Weaver allows importing remote git registries inside `manifest.yaml`, avoiding manual copying of upstream semantic convention files.

### 3.2. Step-by-Step Tool Migration

1.  **Replace Parameter Snippets**:
    In legacy scripts, parameters were passed via command line flags (e.g. `--output` or `--prefix`). In Weaver, create a `weaver.yaml` template configuration file inside your template folder. Define mapping types under `text_maps` and variables under `params`.

2.  **Translate Jinja2 Templates**:
    Weaver uses Jinja2 templates, which are compatible with most legacy Python templates. However, Weaver introduces custom filters (like `comment` and `unique`) and passes a fully resolved JSON structure. Update references from legacy python objects to Weaver's resolved model (e.g., using `group.prefix` instead of Python class variables).

3.  **Replace Invocation Commands in CI**:
    Replace legacy invocations in your shell scripts or Makefile:
    *   *Old Command*:
        ```bash
        python -m semconvgen -t templates/go -o output/go
        ```
    *   *New Command*:
        ```bash
        weaver registry generate -r ./semconv -t templates/go go ./output/go
        ```

---

## 4. Resolving Breaking Changes in Upstream Semantic Conventions

When upgrading the `dependencies` section in `manifest.yaml` to a newer official OpenTelemetry version (e.g., moving from `v1.20.0` to `v1.25.0`), attributes will inevitably break or change names.

### 4.1. Step 1: Detect Mismatches
Run the `diff` command to generate an audit report of all changed or deleted attributes:
```bash
weaver registry diff \
  --registry ./my-registry \
  --baseline-registry https://github.com/open-telemetry/semantic-conventions.git@v1.20.0[model]
```

### 4.2. Step 2: Write translation schema rules
Create a telemetry schema translation file (`schema.yaml`) to map old attribute keys to new names. Add a version entry detailing the renames (see [Telemetry Schema URL Version Model](file:///Users/sac/process-intelligence/otel-weaver/intel/schema-url-version-model.yaml)).

### 4.3. Step 3: Run live-check verification
Deploy the schema translation file to your live checks to verify that microservices emitting old attributes are seamlessly mapped without causing policy failures or ingestion halts.
```bash
weaver registry live-check \
  --registry ./my-registry \
  --advice-policies ./advice \
  --input-source otlp
```
This ensures upstream dependency upgrades do not break the down-stream feedstock validation pipeline.
