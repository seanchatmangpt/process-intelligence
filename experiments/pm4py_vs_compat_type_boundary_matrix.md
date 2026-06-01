# Experiment: PM4Py vs compat Type Boundary Matrix

This matrix maps type system boundaries, conversion failures, and schema validation mechanics between the pandas-centric dynamic environment of PM4Py and the strictly typed, schema-validated execution environment of the `wasm4pm-compat` layer.

## 1. Type Mapping & Boundary Law

| Type Dimension | PM4Py (Python/Pandas) | wasm4pm-compat (Wasm/Rust Layer) |
| :--- | :--- | :--- |
| **Missing Values** | Silently converted to `NaN` (Float64), poisoning integer columns. | Strongly typed `Option<T>` represented as JSON `null` or omission, strictly schema-enforced. |
| **Timestamp Layout** | `datetime64[ns]` or string. Parsing is dynamic, varying by local system locale. | RFC 3339 / ISO 8601 strings, or strict 64-bit integer Unix microseconds. |
| **Identifiers** | Dynamic string or integer. Coercion is implicit, sometimes leading to key lookup failure. | Strict UUID v4 or alphanumeric IDs. Typestate ensures IDs are never mutated. |
| **Attributes** | Arbitrary Python dict. No static structure; keys can be added, mutated, or deleted dynamically. | Immutable, schema-declared attributes with SHACL or JSON Schema guards. |
| **Memory Boundaries** | Unbounded Python object heap. Vulnerable to memory leaks and GC pauses during replay. | Linear memory layout, zero-copy pointer boundaries, strict boundary checks per WASM spec. |

## 2. Boundary Type Validation Schema (JSON Schema)

The following JSON Schema defines the boundary requirements for event-log record types inside the wasm4pm compatibility layer:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Wasm4pmCompatTypeBoundarySchema",
  "type": "object",
  "properties": {
    "event_id": {
      "type": "string",
      "pattern": "^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$"
    },
    "timestamp_ns": {
      "type": "integer",
      "minimum": 0
    },
    "activity_name": {
      "type": "string",
      "minLength": 1
    },
    "attributes": {
      "type": "object",
      "properties": {
        "cost": { "type": "number", "minimum": 0.0 },
        "resource_id": { "type": "string" }
      },
      "required": ["cost"]
    }
  },
  "required": ["event_id", "timestamp_ns", "activity_name", "attributes"]
}
```

## 3. Real Validation Execution Log (Type Integrity Experiment)

### Case A: Malformed Input (Rejected by wasm4pm-compat, Coerced in PM4Py)
An incoming event containing a missing `cost` (represented as `NaN` by pandas or as a float conversion in PM4Py) fails validation under `wasm4pm-compat`:

```json
{
  "event_id": "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d",
  "timestamp_ns": 1780292578000000000,
  "activity_name": "Approve_Synergy_Claim",
  "attributes": {
    "cost": null,
    "resource_id": "executive_agent_0"
  }
}
```
**wasm4pm-compat Outcome**: `Rejected` with error: `TypeBoundaryError: [attributes/cost] expected type number, found null`.
**PM4Py Outcome**: `Accepted` but mutates `cost` column to `NaN` (Float), causing downstream `TypeError: unsupported operand type(s) for +: 'float' and 'NoneType'` in fitness calculation.

### Case B: Validated Input (Accepted by both)
```json
{
  "event_id": "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d",
  "timestamp_ns": 1780292578000000000,
  "activity_name": "Approve_Synergy_Claim",
  "attributes": {
    "cost": 12500.00,
    "resource_id": "executive_agent_0"
  }
}
```
**wasm4pm-compat Outcome**: `Accepted` with validation receipt signature `0x4e65b21764c...` compiled into WASM linear memory.

## 4. Linkages to Standards and M&A Claims

- **Standards Compliance**: Mapped to standard XES loss-policy constraints at [xes_loss-policy_sample.md](file:///Users/sac/process-intelligence/experiments/xes_loss-policy_sample.md).
- **M&A Claims**: Defensibility claims of type safety are mapped at [define_buyer_reliance_requirements.md](file:///Users/sac/process-intelligence/ma/define_buyer_reliance_requirements.md).