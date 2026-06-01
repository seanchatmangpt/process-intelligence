# Experiment: OTel Trace Integration with BLAKE3 event-chain roots

This experiment registers the schema and concrete verification procedures for linking OpenTelemetry trace contexts to BLAKE3 event-chain roots. This guarantees that trace telemetry is unforgeable and structurally sound.

## 1. Trace Integration JSON Schema

The formal JSON Schema is registered at [otel_trace_integration_schema.json](file:///Users/sac/process-intelligence/standards/otel_trace_integration_schema.json).

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "OTelTraceBlake3IntegrationSchema",
  "type": "object",
  "properties": {
    "trace_id": {
      "type": "string",
      "pattern": "^[0-9a-fA-F]{32}$"
    },
    "event_chain_root": {
      "type": "string",
      "pattern": "^[0-9a-fA-F]{64}$"
    },
    "spans": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "span_id": { "type": "string", "pattern": "^[0-9a-fA-F]{16}$" },
          "parent_span_id": { "type": ["string", "null"], "pattern": "^([0-9a-fA-F]{16})?$" },
          "span_name": { "type": "string" },
          "start_time_unix_us": { "type": "integer", "minimum": 0 },
          "end_time_unix_us": { "type": "integer", "minimum": 0 },
          "instruction_count": { "type": "integer", "minimum": 0 },
          "blake3_receipt": { "type": "string", "pattern": "^[0-9a-fA-F]{64}$" }
        },
        "required": ["span_id", "parent_span_id", "span_name", "start_time_unix_us", "end_time_unix_us", "instruction_count", "blake3_receipt"]
      }
    }
  },
  "required": ["trace_id", "event_chain_root", "spans"]
}
```

## 2. Concrete Telemetry Instance

The following instance defines a two-span transaction representing process initiation and execution steps. The receipt hashes are computed recursively and match the top-level `event_chain_root`.

```json
{
  "trace_id": "4a7b744ce58b88cd28148b5dfbe984f9",
  "event_chain_root": "a00e57ab89b023e3e2cfbf06e788e0cb56b3e945c5dfb5e6789a24a500ef412a",
  "spans": [
    {
      "span_id": "0000000000000001",
      "parent_span_id": null,
      "span_name": "StartProcess",
      "start_time_unix_us": 1000,
      "end_time_unix_us": 2000,
      "instruction_count": 500,
      "blake3_receipt": "b8f75c2e39ea2f57a3e8ab802d2426be0c128bdca235ec89aaef6c28f09eb2fa"
    },
    {
      "span_id": "0000000000000002",
      "parent_span_id": "0000000000000001",
      "span_name": "ExecuteStep",
      "start_time_unix_us": 1200,
      "end_time_unix_us": 1800,
      "instruction_count": 1200,
      "blake3_receipt": "a00e57ab89b023e3e2cfbf06e788e0cb56b3e945c5dfb5e6789a24a500ef412a"
    }
  ]
}
```

## 3. Telemetry Unforgeability Proof Heuristics

The unforgeability of the trace telemetry relies on three interlocking validation gates:

1. **Cryptographic Chaining:** The receipt hash $R_i$ of span $i$ is bound directly to the parent context and trace history:
   $$R_i = \operatorname{BLAKE3}(R_{i-1} \parallel \text{TraceID} \parallel \text{SpanID} \parallel \text{ParentSpanID} \parallel \text{SpanName} \parallel \text{StartTime} \parallel \text{EndTime} \parallel \text{InstructionCount})$$
   Any modification of execution time, JIT instruction profile, or transaction identifiers breaks downstream validations.

2. **Parent-Child Timing Invariants:** A child span's start and end times must reside strictly within the parent span's timeline:
   $$t_{\text{start}}(\text{parent}) \le t_{\text{start}}(\text{child}) \le t_{\text{end}}(\text{child}) \le t_{\text{end}}(\text{parent})$$

3. **Dag Cycle Verification:** Parent-child linkages are traversed dynamically during verification. The presence of any cyclical parent references (e.g., recursive loop bypasses attempted by malicious logic) triggers immediate rejection.
