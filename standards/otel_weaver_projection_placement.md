# OTEL Weaver Projection Standard Ledger Placement

The **OpenTelemetry (OTEL)** standard defines a vendor-neutral framework for collecting distributed tracing data. The **Weaver Projection** is the formal translation engine within the Process Intelligence Research Foundry that projects distributed trace spans into object-centric event logs (OCEL) or trace logs (XES). This document establishes how OTEL metrics, trace spans, and projections are registered and verified on the ledger.

---

## 1. Trace Span Mapping to the Ledger

OTEL distributed traces consist of nested spans representing execution intervals. The Weaver Projection maps these spans to event stream concepts:

| OTEL Span Element | Process Mining equivalent | Ledger Representation | Description |
| :--- | :--- | :--- | :--- |
| `TraceID` | Case ID / Object ID | `CaseCorrelationID` | Identifies the execution flow across distributed systems. |
| `SpanID` | Event ID | `EventTransactionID` | Unique identifier for a specific execution step. |
| `ParentSpanID` | Preceding Event / Edge | `ParentChildRelation` | Defines the hierarchical execution context. |
| `SpanName` | Activity Label | `ActivityLabel` | Name of the process activity. |
| `StartTime` / `EndTime` | Event Timestamps | `StartEndTimestamp` | Marks the task lifecycle transitions. |
| `Attributes` | Event Attributes | `DynamicAttributes` | Payload parameters (e.g., payload size, server host). |

The ledger registers each projection run:

```json
{
  "projection_id": "otel-880e8400-e29b-41d4-a716-446655442222",
  "source_trace_count": 45000,
  "projected_ocel_hash": "b4c3d2...",
  "weaver_version": "weaver-otel-1.0.3",
  "witness_signature": "SIG_ED25519_..."
}
```

---

## 2. Type Laws and Timing Constraints

The Weaver Projection enforces strict timing constraints during translation:

1.  **Parent-Child Timing Constraint**: A child span's start time must be equal to or greater than its parent span's start time, and its end time must be equal to or less than the parent's end time:
    $$t_{\text{start}}(\text{parent}) \le t_{\text{start}}(\text{child}) \le t_{\text{end}}(\text{child}) \le t_{\text{end}}(\text{parent})$$
2.  **No Cyclic Dependencies**: Parent-child relationship graphs must be directed trees or DAGs; circular parent references are rejected.
3.  **Cryptographic Context Chaining**: Trace contexts propagated via HTTP headers (W3C Trace Context) are checked against log records to ensure no span injection occurs.

---

## 3. Academic Foundations and Conformance

*   OTEL logs are projected into event streams to perform conformance checking against baseline models.
*   For experimental projections, see the [POWL Projection Sample](file:///Users/sac/process-intelligence/experiments/powl_projection_sample.md).
*   For the autonomic adaptions of logs, see [Autonomic Knowledge Actuation](file:///Users/sac/process-intelligence/doctrine/autonomic-knowledge-actuation.md).

---

## 4. M&A Slide-to-Receipt Bridge

To verify continuous compliance assertions:
1.  Any performance or latency assertions based on live cloud trace data must map to a Weaver Projection receipt on the ledger.
2.  The resulting log hash is linked under the [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md).
3.  Due diligence advisors verify the projection mapping to guarantee that performance metrics are mathematically derived from real-time OTEL telemetry, as required by the [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).