# OpenTelemetry Telemetry Schemas and Version Translation

In OpenTelemetry, semantic conventions change over time. Attributes are renamed, metric names are standardized, and obsolete keys are removed. To prevent this natural evolution from breaking downstream consumers, OpenTelemetry introduces the concept of **Telemetry Schemas** and **Schema URLs**.

This document details the telemetry schema architecture, the syntax of version translation files, and how Weaver leverages these schemas to align historical data.

---

## 1. The Schema URL: An Observability Contract

A **Schema URL** is a unique identifier (typically an HTTP URL) representing a immutable version of OpenTelemetry semantic conventions (e.g. `https://opentelemetry.io/schemas/1.20.0`). 

When an SDK emits metrics, logs, or traces, it attaches the `schema_url` of the conventions it used. Downstream consumers (backends, dashboards, process mining algorithms) inspect this URL to determine how to parse the incoming fields.

---

## 2. Telemetry Feedstock and the Court of Process Consequence

From a process intelligence perspective:
*   **Telemetry is Feedstock**: Raw log events are streamed from runtime systems.
*   **Process Consequence is Court**: Downstream compliance models evaluate trace logs to verify service-level agreements (SLAs), transaction safety, and structural business rules.

If an attribute representing case ID is renamed from `trace.case_id` to `process.case.id` in a newer SDK version, a downstream process engine evaluating the event stream will fail to correlate events across these versions. This discrepancy breaks the auditability of the feedstock.

By defining a **Telemetry Schema file**, we establish translation rules. The schema translation file allows the ingest gateway (e.g. OTel Collector or Weaver) to rewrite old feedstock fields to match the current schema url before the data is submitted to the process court.

---

## 3. Schema File Format Specification

A Telemetry Schema file is a YAML document containing translation instructions mapping historical schema versions.

### 3.1. Structure of the Telemetry Schema File

```yaml
file_format: "1.0.0"
schema_url: "https://opentelemetry.io/schemas/1.25.0"
versions:
  1.25.0:
    # Set of changes introduced in this version relative to 1.24.0
    changes:
      - rename_attributes:
          # Structure: <old_name>: <new_name>
          http.status_code: http.response.status_code
          http.method: http.request.method
      - rename_metrics:
          - from: http.client.duration
            to: http.client.request.duration
      - rename_spans:
          - from: HTTP GET
            to: http.client.request

  1.24.0:
    changes:
      - rename_attributes:
          db.system.name: db.system
          net.peer.name: server.address
          net.peer.port: server.port
      - rename_metrics:
          - from: db.client.duration
            to: db.client.operation.duration
```

### 3.2. Mapping Fields:
*   **`file_format`**: The version of the telemetry schema file syntax itself (currently must be `1.0.0`).
*   **`schema_url`**: The canonical URL identifying the target/latest version of the schema defined in this file.
*   **`versions`**: A key-value map where each key is a semantic version string. Under each version, you declare the `changes` list that describes how to transform telemetry conforming to the *previous* version to make it conform to *this* version.
    *   **`rename_attributes`**: Renames span, metric, log, or resource attributes.
    *   **`rename_metrics`**: Renames metric instrument identifiers.
    *   **`rename_spans`**: Renames trace spans.
    *   **`resources`**: Updates to resource-level attributes.

---

## 4. How Weaver Uses Schema Mappings

Weaver uses these translation mappings in the following commands:
1.  **`weaver registry diff`**: Resolves two versions of the registry and determines if they conform to the schema translation file (verifying that all breaking changes have corresponding rename declarations).
2.  **`weaver registry live-check`**: When evaluating telemetry feedstock containing older `schema_url` headers, Weaver applies the rename rules dynamically to convert attributes to their latest equivalents before validating the policies.
3.  **Code Generation**: When generating SDK code, Weaver can generate helper classes or translation layers using these version history maps, allowing developers to support historical payloads seamlessly.
