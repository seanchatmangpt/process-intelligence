# Integration Point 1: ZOEapp ↔ Blue River Dam

**Authority:** Process Intelligence Research Foundry  
**Status:** Integration Specification  
**Date:** 2026-06-01

---

## Purpose

ZOEapp serves as the **Proof Cell** — the real-world process execution engine that generates process evidence (event logs, OTel traces). Blue River Dam serves as the **Autonomic Governor** — the MAPE-K feedback controller that monitors ZOEapp execution and enforces governance constraints.

This integration establishes the **closed-loop autonomic actuation** backbone: real-world evidence → governance analysis → refusal/proceed decision → compliance enforcement.

---

## Interface Specification

### ZOEapp as Event Producer

ZOEapp must expose the following interface for Blue River to consume process evidence:

```typescript
// zoeapp/src/interfaces/process-event-stream.ts

interface ProcessEvent {
  process_id: string;              // Unique process instance ID
  process_type: string;            // E.g., "OrderToInvoice", "ClaimProcessing"
  timestamp: ISO8601;              // Event occurrence time
  stage_name: string;              // Current stage name from declared model
  previous_stage: string | null;   // Previous stage (for transition validation)
  event_type: ProcessEventType;    // "Started" | "Completed" | "Failed" | "Skipped"
  actor_id: string;                // User/service that triggered event
  
  // Telemetry linkage
  otel_trace_id: string;           // OpenTelemetry trace ID
  otel_span_id: string;            // OpenTelemetry span ID
  
  // Evidence payload
  context: {
    [key: string]: any;            // Stage-specific attributes
  };
  
  // Cryptographic integrity
  timestamp_epoch: number;         // UNIX timestamp (ms)
  received_at_epoch: number;       // When ZOEapp received the event
  signature?: string;              // Ed25519 signature (optional, for audit)
}

enum ProcessEventType {
  Started = "Started",
  StageCompleted = "Completed",
  StageFailed = "Failed",
  StageSkipped = "Skipped",
  ProcessTerminated = "Terminated",
  ProcessRolledBack = "RolledBack",
  ErrorDetected = "ErrorDetected",
  ManualInterventionRequired = "ManualInterventionRequired"
}

interface EventStream {
  subscribe(callback: (event: ProcessEvent) => void): Unsubscriber;
  unsubscribe(id: string): void;
  replay(start_time: ISO8601, end_time: ISO8601): Promise<ProcessEvent[]>;
}
```

### Blue River API Endpoints (Consumer)

Blue River provides the following REST endpoints for ZOEapp integration:

```bash
# Health check
GET /blue-river/health
Response: { status: "healthy", uptime_seconds: 12345, version: "1.0.0" }

# Subscribe to lifecycle events
POST /blue-river/subscribe
Body: {
  "subscriber_id": "zoeapp_instance_1",
  "event_types": ["ProcessStarted", "StageCompleted", "ErrorDetected"]
}
Response: { subscription_id: "sub_123", endpoint: "ws://blue-river:5000/ws/sub_123" }

# Emit process event to Blue River for analysis
POST /blue-river/ingest
Headers: { "Content-Type": "application/json", "X-Trace-ID": "0x4bf92f3577..." }
Body: { ProcessEvent JSON }
Response: {
  "event_id": "evt_456",
  "accepted": true,
  "receipt": "0xblake3hash...",
  "governance_decision": "proceed" | "refuse"
}

# Refuse a process transition (Blue River → ZOEapp)
POST /blue-river/refuse
Body: {
  "process_id": "proc_123",
  "reason": "stage_not_in_declared_model",
  "evidence": { "declared_model_stages": [...] },
  "signature": "ed25519_sig..."
}
Response: { "refused": true, "process_id": "proc_123" }

# Retrieve Blue River governance state
GET /blue-river/state
Response: {
  "lifecycle_state": "Monitor",
  "active_policies": 5,
  "conformance_fitness": 0.97,
  "last_correction_time": "2026-06-01T14:30:00Z"
}
```

---

## Process Event Flow

### Scenario 1: Normal Execution (No Drift)

```
┌─────────────────────────────────────────────────────────────────────┐
│                         ZOEapp Execution                            │
└─────────────────────────────────────────────────────────────────────┘

Time 14:30:00 - Start OrderFulfillment process (proc_123)
  │
  ├─ Emit OTel Span: process.start
  │   - trace_id: 0x4bf92f3577b34da6a3ce929d0e0e4736
  │   - attributes: {process_id: "proc_123", type: "OrderToInvoice"}
  │
  ├─ Emit ProcessEvent to Blue River:
  │   {
  │     "process_id": "proc_123",
  │     "process_type": "OrderToInvoice",
  │     "event_type": "Started",
  │     "stage_name": "OrderReceived",
  │     "timestamp": "2026-06-01T14:30:00Z",
  │     "otel_trace_id": "0x4bf92f3577b34da6a3ce929d0e0e4736",
  │     "actor_id": "user_alice"
  │   }
  │
  └─→ POST /blue-river/ingest
        ↓
┌─────────────────────────────────────────────────────────────────────┐
│                  Blue River Monitor (MAPE-K Cycle)                  │
└─────────────────────────────────────────────────────────────────────┘

Monitor Phase:
  ├─ Receive ProcessEvent
  ├─ Extract: process_id, stage_name, timestamp, otel_trace_id
  ├─ Create Evidence<ProcessEvent, State, Witness>
  │   - T = ProcessEvent
  │   - State = { declared_stage: "OrderReceived", exec_stage: "OrderReceived" }
  │   - Witness = { otel_trace_id, signature }
  │
  └─ Store in event ledger (append-only, BLAKE3 chained)

Analyze Phase:
  ├─ Load declared process model (PNML)
  ├─ Query: Is "OrderReceived" in declared model? YES
  ├─ Query: Is prev_stage → current_stage a valid transition? YES (start → OrderReceived)
  ├─ Compute conformance fitness: 1.0 (perfect alignment)
  ├─ Check against governance policies:
  │   ├─ Is stage_name in allowed_list? YES
  │   ├─ Is actor_id authorized for this stage? YES
  │   └─ Rate limit check (events/min): PASS
  │
  └─ Confidence: 1.0, Conformance_score: 1.0

Plan Phase:
  ├─ Decision: No corrections needed
  ├─ Plan: { action: "proceed", next_check_at: 14:30:30 }
  │
  └─ Generate Plan artifact with BLAKE3 receipt

Execute Phase:
  ├─ Response to ZOEapp: { governance_decision: "proceed" }
  │
  └─ No refusal action needed

Knowledge Phase:
  ├─ Store execution outcome: { process_id, stage_name, decision: "proceed", timestamp }
  ├─ Update knowledge registry
  └─ Emit OTel Span: mape_k.cycle_complete
      - attributes: {result: "proceed", conformance_score: 1.0}

        ↓
┌─────────────────────────────────────────────────────────────────────┐
│                   ZOEapp Receives Governance Decision               │
└─────────────────────────────────────────────────────────────────────┘

Receive response: { governance_decision: "proceed" }
  ├─ Decision matches expected? YES
  ├─ Timestamp valid (not stale)? YES
  ├─ Verify signature (if present)? YES
  │
  └─ Continue execution to next stage (PaymentProcessing)

Time 14:30:15 - Complete OrderFulfillment stage
  │
  ├─ Emit OTel Span: stage.complete
  │   - duration: 15s
  │
  ├─ Emit ProcessEvent:
  │   {
  │     "process_id": "proc_123",
  │     "event_type": "Completed",
  │     "stage_name": "OrderReceived",
  │     "previous_stage": null,
  │     "timestamp": "2026-06-01T14:30:15Z",
  │     "otel_trace_id": "0x4bf92f3577b34da6a3ce929d0e0e4736"
  │   }
  │
  └─→ POST /blue-river/ingest
        [Repeat MAPE-K cycle]
```

### Scenario 2: Drift Detection & Refusal

```
┌─────────────────────────────────────────────────────────────────────┐
│                    ZOEapp Execution (Drift Scenario)                │
└─────────────────────────────────────────────────────────────────────┘

Time 14:35:00 - ZOEapp Encounters Unexpected Condition
  │
  ├─ Declared model: OrderReceived → PaymentProcessing → Fulfillment
  ├─ Actual execution: OrderReceived → [DETECTED SKIP] → Fulfillment
  │   (Missing PaymentProcessing stage)
  │
  ├─ Emit OTel Span: stage.transition_unexpected
  │   - attributes: {from: "OrderReceived", to: "Fulfillment", reason: "manual_skip"}
  │
  ├─ Emit ProcessEvent:
  │   {
  │     "process_id": "proc_123",
  │     "event_type": "StageSkipped",
  │     "stage_name": "PaymentProcessing",
  │     "previous_stage": "OrderReceived",
  │     "context": { "reason": "manual_skip_by_user_bob" },
  │     "timestamp": "2026-06-01T14:35:00Z",
  │     "actor_id": "user_bob"
  │   }
  │
  └─→ POST /blue-river/ingest
        ↓
┌─────────────────────────────────────────────────────────────────────┐
│                     Blue River Analysis (Drift)                     │
└─────────────────────────────────────────────────────────────────────┘

Monitor Phase:
  ├─ Event type: StageSkipped
  ├─ Create Evidence with flag: stage_not_in_model

Analyze Phase:
  ├─ Query: Is PaymentProcessing skipped in declared model? NO
  ├─ Query: Is OrderReceived → Fulfillment a valid transition? NO
  │   (Declared model requires PaymentProcessing in between)
  ├─ Conformance violation detected
  ├─ Fitness impact: (1 missing move) / (total events) → fitness drops to 0.93
  ├─ Risk assessment: "skipped_payment_stage_may_indicate_fraud"
  │
  └─ Confidence: 0.98, Conformance_score: 0.93

Plan Phase:
  ├─ Decision: REFUSE (compliance-critical, not elastic)
  ├─ Reason: "declared_model_requires_payment_processing_before_fulfillment"
  ├─ Action: Send refusal to ZOEapp (blocking stage transition)
  ├─ Evidence:
  │   {
  │     "declared_model_transitions": [
  │       "OrderReceived → PaymentProcessing → Fulfillment"
  │     ],
  │     "observed_transitions": [
  │       "OrderReceived → Fulfillment"
  │     ]
  │   }
  │
  └─ Generate Plan artifact

Execute Phase:
  ├─ POST /blue-river/refuse:
  │   {
  │     "process_id": "proc_123",
  │     "reason": "stage_skip_not_in_declared_model",
  │     "blocked_transition": {
  │       "from": "OrderReceived",
  │       "to": "Fulfillment",
  │       "required_intermediate": "PaymentProcessing"
  │     },
  │     "evidence": {...},
  │     "signature": "ed25519_sig_by_governor"
  │   }
  │
  ├─ Log: "Refusal sent to ZOEapp for process proc_123"
  └─ Emit OTel Span: governance.refusal
      - attributes: {process_id: "proc_123", reason: "stage_skip"}

Knowledge Phase:
  ├─ Store refusal decision in registry
  ├─ Mark process as "REFUSED_TRANSITION_PENDING_MANUAL_INTERVENTION"
  └─ Trigger escalation: operator_notification() + security_alert()

        ↓
┌─────────────────────────────────────────────────────────────────────┐
│              ZOEapp Receives Refusal (Blocking Response)            │
└─────────────────────────────────────────────────────────────────────┘

Receive response: { refused: true, process_id: "proc_123", reason: "..." }
  │
  ├─ Verify signature against blue_river_governor_key? YES
  ├─ Process is refusal? YES
  ├─ Block execution to Fulfillment stage
  │
  ├─ Emit OTel Span: process.blocked_by_governance
  │   - attributes: {reason: "refusal_from_blue_river"}
  │
  ├─ Log event: "Process proc_123 blocked at Fulfillment stage"
  │
  └─ Wait for manual operator intervention
      (No automatic retry; escalation required)

Manual Remediation:
  ├─ Operator reviews refusal in dashboard
  ├─ Operator inspects governance policy
  │   - Policy: "Payment stage is mandatory"
  │   - Evidence: "PaymentProcessing stage not executed"
  │
  ├─ Operator chooses:
  │   Option A: Revert to OrderReceived, execute PaymentProcessing manually
  │   Option B: Request exception override (requires audit log)
  │   Option C: Escalate to compliance team
  │
  └─ If override approved:
      ZOEapp advances process with exception flag
      Blue River logs override + operator ID + justification
      Emit OTel Span: process.manual_override
```

---

## Data Contract: ProcessEvent Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "ProcessEvent",
  "description": "Event emitted by ZOEapp for Blue River analysis",
  "type": "object",
  "required": [
    "process_id",
    "process_type",
    "event_type",
    "stage_name",
    "timestamp",
    "otel_trace_id"
  ],
  "properties": {
    "process_id": {
      "type": "string",
      "pattern": "^proc_[a-zA-Z0-9]{16}$",
      "description": "Unique process instance identifier"
    },
    "process_type": {
      "type": "string",
      "enum": ["OrderToInvoice", "ClaimProcessing", "SupplyChainTiming"],
      "description": "Declared process type from TTL ontology"
    },
    "event_type": {
      "type": "string",
      "enum": [
        "Started",
        "Completed",
        "Failed",
        "Skipped",
        "Terminated",
        "RolledBack",
        "ErrorDetected",
        "ManualInterventionRequired"
      ]
    },
    "stage_name": {
      "type": "string",
      "description": "Current or target stage name"
    },
    "previous_stage": {
      "type": ["string", "null"],
      "description": "Previous stage (null if this is process start)"
    },
    "timestamp": {
      "type": "string",
      "format": "date-time",
      "description": "ISO 8601 event timestamp (UTC)"
    },
    "timestamp_epoch": {
      "type": "integer",
      "description": "UNIX timestamp in milliseconds"
    },
    "received_at_epoch": {
      "type": "integer",
      "description": "When ZOEapp received this event"
    },
    "otel_trace_id": {
      "type": "string",
      "pattern": "^0x[a-f0-9]{32}$",
      "description": "OpenTelemetry W3C Trace ID"
    },
    "otel_span_id": {
      "type": "string",
      "pattern": "^0x[a-f0-9]{16}$",
      "description": "OpenTelemetry span ID"
    },
    "actor_id": {
      "type": "string",
      "description": "User/service that triggered the event"
    },
    "context": {
      "type": "object",
      "additionalProperties": true,
      "description": "Stage-specific attributes"
    },
    "signature": {
      "type": ["string", "null"],
      "description": "Optional Ed25519 signature for non-repudiation"
    }
  },
  "additionalProperties": false
}
```

---

## Error Handling

### Network Failures

| Scenario | Detection | Recovery |
|----------|-----------|----------|
| Blue River unreachable (timeout > 5s) | ZOEapp HTTP timeout | Buffer event locally, retry async, fall back to "unmonitored" mode |
| Blue River returns HTTP 5xx | Service error response | Exponential backoff (2s, 4s, 8s, max 60s), log incident |
| Blue River closes connection | WebSocket EOF | Reconnect with new subscription, replay buffered events |
| Malformed refusal response | JSON schema validation fails | Log error, escalate to operator, refuse to proceed cautiously |

### Signature Failures

| Scenario | Detection | Recovery |
|----------|-----------|----------|
| Refusal signature invalid | Ed25519 verification fails | Log security alert, ignore refusal, escalate to security team |
| Refusal signature missing | Expected signature not found | Log incident, escalate (depends on policy) |
| Signature from unknown key | Key not in governance registry | Reject refusal, log, escalate |

### Data Consistency Failures

| Scenario | Detection | Recovery |
|----------|-----------|----------|
| Duplicate event received | Event ID already in ledger | Ignore duplicate, log warning |
| Out-of-order events | Timestamp inconsistent with ledger | Reorder using timestamp, re-analyze if necessary |
| Event for unknown process | process_id not in ZOEapp registry | Log error, escalate, do not create process |
| Stage name not in model | stage_name not in declared PNML | Treat as drift, trigger refusal path |

---

## Acceptance Criteria

- [ ] ZOEapp can emit ProcessEvent JSON and transmit to Blue River
- [ ] Blue River can receive and parse ProcessEvent with full fidelity
- [ ] Blue River MAPE-K cycle completes within 100ms for normal events
- [ ] Blue River can send refusal responses with valid Ed25519 signatures
- [ ] ZOEapp correctly handles refusal responses (blocks execution)
- [ ] Network failures are retried with exponential backoff
- [ ] Signature verification failures are escalated to operator
- [ ] ProcessEvent JSON schema validation passes for all test scenarios
- [ ] OTel trace IDs correctly link events to telemetry

---

**Authority:** Process Intelligence Research Foundry  
**Status:** Integration Specification (Ready for Implementation)
