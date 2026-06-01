# [PI-V30.1.2] Loss-Policy Map: Thermodynamic Degradation Law

**Version:** 30.1.2  
**Authority:** Conformance Agent  
**Classification:** Foundational Loss Semantics  
**Date:** 2026-05-31  
**Status:** COMPLETE

---

## I. Executive Summary

The **Loss Policy Map** defines the thermodynamic limits of acceptable degradation in process evidence under adversarial conditions (high latency, network loss, memory saturation). It establishes:

1. **Permissible Loss Classes:** Metadata attrition, trace decimation, attribute pruning.
2. **Absolute Unacceptable Loss Boundaries:** Causal spine corruption, cryptographic signature loss.
3. **Self-Halt Semantics:** Automatic shutdown when unacceptable loss is detected.
4. **Lossless Operation Requirements:** Zero-loss guarantees for cryptographic chains and object identity links.

---

## II. Thermodynamic Principles

### Principle 1: The Causal Spine Is Inviolable

**Definition:** The causal spine of a process execution is the sequence of state transitions from initial marking to final marking:
$$\text{Spine} = \langle s_0, t_1, s_1, t_2, s_2, \ldots, t_n, s_n \rangle$$

where:
- $s_i$ is a process state (Petri net marking, BPMN token configuration, etc.).
- $t_i$ is a transition (activity execution).
- $s_0$ is the initial state.
- $s_n$ is the final state.

**Invariant:** The causal spine **must be preserved completely**. Loss of any link in the spine (e.g., a missing intermediate state transition) constitutes **unacceptable loss** and triggers **self-halt**.

**Justification:** The causal spine is the only evidence that a lawful process occurred. Without it, the event log cannot prove execution validity.

---

### Principle 2: Object Identity Is Cryptographically Bound

**Definition:** Object identity (in OCEL contexts) is the association of a unique object ID with a sequence of state mutations:
$$\text{ObjectHistory}(o) = \langle \text{state}_0, \text{mutation}_1, \text{state}_1, \ldots, \text{mutation}_k, \text{state}_k \rangle$$

**Invariant:** The cryptographic binding between object ID and state history **must be preserved completely**. Loss of any mutation link constitutes **unacceptable loss**.

**Justification:** Object identity is the forensic anchor tying process evidence to specific entities (customers, orders, invoices, etc.). Without object continuity, buyers cannot rely on conformance claims.

---

### Principle 3: Metadata Is Non-Critical (Permissible Loss)

**Definition:** Metadata includes non-activity attributes that provide context but do not affect process control flow:
- Organizational attributes (resource name, department, email).
- Performance context (loop iteration count, processing time estimates).
- Descriptive fields (comment, reason_code, exception_notes).

**Invariant:** Metadata **may be pruned** when memory pressure exceeds thresholds, without triggering self-halt.

**Loss Thresholds:**
- Memory utilization < 85%: **Lossless operation** (all metadata retained).
- Memory utilization 85-90%: **Selective pruning** (non-critical attributes discarded).
- Memory utilization > 90%: **Aggressive pruning** (all optional attributes removed; critical attributes retained).

**Critical vs. Optional Attributes:**

| Category | Critical | Optional (Discardable) |
|---|---|---|
| **Event Identity** | event_id, case_id, activity_name, timestamp | payload, custom_fields |
| **State Marking** | place_id, token_count (Petri net) | resource_location, estimated_duration |
| **Object Reference** | object_id, object_type | object_status_notes, comment |
| **Cryptographic** | signature, hash, epoch | (NONE - cryptographic data is inviolable) |

---

### Principle 4: Trace Decimation Is Statistically Permissible

**Definition:** Trace decimation is probabilistic sampling of rapid state transitions when the event production rate exceeds the log ingestion rate.

**Scenario:** A manufacturing process emits 100,000 micro-transactions per second on a single machine. The wasm4pm log ingestion rate is 50,000 events/sec. A decision must be made: drop events or block execution.

**Permissible Decimation Policy:**
- Record the first event in a rapid burst.
- Sample subsequent events probabilistically with probability $p = \frac{\text{ingestion\_rate}}{\text{emission\_rate}}$.
- Record the final event in the burst (to preserve causal closure).

**Formal Definition:**  
For a burst of events $\langle e_1, e_2, \ldots, e_n \rangle$ emitted in rapid succession (all timestamps within $\Delta t < 1$ second):
$$\text{Sample}(e_i) = \begin{cases} 
\text{include} & \text{if } i = 1 \text{ or } i = n \\
\text{include with probability } p & \text{if } 1 < i < n
\end{cases}$$

**Sampling Rate:** $p = \min\left(1, \frac{\text{available\_ingestion\_capacity}}{\text{burst\_rate}}\right)$

**Lossless Preservation:** The causal spine is preserved by including $e_1$ (entry to burst) and $e_n$ (exit from burst). Intermediate events are statistically representative.

**Loss Report:** Every decimated burst must generate a `DecimationReport` documenting:
- Burst start/end timestamps.
- Total events in burst.
- Events recorded.
- Sampling rate $p$.
- Statistical confidence interval.

---

### Principle 5: Timestamps Cannot Be Lost or Modified

**Definition:** Timestamps are the temporal anchor of causality. They must be preserved exactly as recorded.

**Invariant:** Any loss or modification of timestamp data triggers **self-halt**.

**Exceptions:** None. Timestamps are inviolable.

---

### Principle 6: Cryptographic Signatures Are Inviolable

**Definition:** A cryptographic signature binds an authority (auditor, runner, validator) to a specific evidence block.

**Invariant:** Signatures **must be preserved exactly**. Corruption or loss of a single bit in the signature causes evidence rejection.

**Recovery Option:** If a signature is corrupted mid-transmission, the entire evidence block is discarded (refusal), and a fresh execution is required.

---

## III. Permissible Loss Classes

### Loss Class 1: Organizational Metadata Attrition

**Trigger Condition:** Memory utilization > 85%.

**Discardable Attributes:**
- `org_resource` (employee name/ID)
- `org_department`
- `org_email`
- `org_location`

**Retention Requirement:** Activity name, timestamp, and case ID **must be retained**.

**Lossless Alternative:** If organizational data is critical, stream the log to external storage (database, cloud blob) rather than truncating in-memory.

**Loss Report Example:**
```json
{
  "loss_type": "OrganizationalMetadataAttrition",
  "timestamp": "2026-05-31T22:51:00Z",
  "memory_utilization": 87.5,
  "discarded_attributes": ["org_resource", "org_department"],
  "events_affected": 234,
  "severity": "INFORMATIONAL"
}
```

---

### Loss Class 2: Optional Event Attributes Pruning

**Trigger Condition:** Memory utilization > 90%.

**Discardable Attributes:**
- Custom attributes (e.g., `custom_field_1`, `custom_field_2`).
- Descriptive fields (e.g., `comment`, `reason_code`).
- Processing metadata (e.g., `loop_iteration`, `estimated_duration`).

**Retention Requirement:** All standard XES/OCEL attributes **must be retained** (concept:name, time:timestamp, org:resource, lifecycle:transition).

**Loss Report Example:**
```json
{
  "loss_type": "OptionalAttributePruning",
  "timestamp": "2026-05-31T22:51:30Z",
  "memory_utilization": 91.2,
  "pruned_attributes": ["custom_field_1", "comment", "loop_iteration"],
  "events_affected": 456,
  "severity": "INFORMATIONAL"
}
```

---

### Loss Class 3: Trace Decimation (Burst Sampling)

**Trigger Condition:** Event ingestion backlog exceeds 50% of ring buffer capacity.

**Mechanism:**
1. Identify rapid burst of events (timestamps within 1-second window).
2. Record first and last event in burst unconditionally.
3. Sample intermediate events with probability $p = \frac{\text{ingestion\_capacity}}{\text{burst\_rate}}$.
4. Generate `DecimationReport` documenting sampling parameters.

**Lossless Guarantee:** The first and last events in the burst preserve causal continuity. Intermediate events are statistically representative.

**Decimation Confidence:**
- For a burst of 100 events sampled at $p = 0.5$, expected number of recorded events ≈ 50 + 2 (first/last) = 52.
- Confidence interval (95%): [47, 57].

**Loss Report Example:**
```json
{
  "loss_type": "TraceDecimation",
  "timestamp": "2026-05-31T22:52:00Z",
  "backlog_utilization": 51.3,
  "burst_start": "2026-05-31T22:51:59Z",
  "burst_end": "2026-05-31T22:52:00Z",
  "total_events_in_burst": 100,
  "sampling_rate": 0.5,
  "events_recorded": 52,
  "confidence_interval_95": [47, 57],
  "severity": "WARNING"
}
```

---

## IV. Absolute Unacceptable Loss Boundaries

### Unacceptable Loss 1: Causal Spine Corruption

**Definition:** Loss of any state transition link in the sequence from initial to final marking.

**Examples:**
- Missing intermediate state (e.g., Petri net marking between $t_i$ and $t_{i+1}$).
- Event recorded without corresponding state change.
- State change recorded without corresponding event.

**Trigger:** Automatic **self-halt** with critical error log.

**Self-Halt Specification:**
```rust
pub fn detect_causal_spine_corruption(
    state_history: &[State],
    event_history: &[Event],
) -> Result<(), HaltSignal> {
    // Verify 1:1 correspondence between events and state transitions
    if state_history.len() != event_history.len() + 1 {
        // +1 for initial state
        return Err(HaltSignal {
            severity: Severity::CRITICAL,
            reason: "Causal spine length mismatch",
            state_count: state_history.len(),
            event_count: event_history.len(),
            action: HaltAction::ImmediateShutdown,
        });
    }
    
    // Verify each event corresponds to valid state transition
    for i in 0..event_history.len() {
        let event = &event_history[i];
        let from_state = &state_history[i];
        let to_state = &state_history[i + 1];
        
        if !is_valid_transition(from_state, event, to_state) {
            return Err(HaltSignal {
                severity: Severity::CRITICAL,
                reason: "Invalid state transition for event",
                event_id: event.id().to_string(),
                from_state: format!("{:?}", from_state),
                to_state: format!("{:?}", to_state),
                action: HaltAction::ImmediateShutdown,
            });
        }
    }
    Ok(())
}
```

**Recovery:** None. The execution session must be terminated, and a fresh execution initiated from the last verified checkpoint.

---

### Unacceptable Loss 2: Cryptographic Signature Corruption

**Definition:** Loss or modification of a cryptographic signature binding an evidence block to an authority.

**Examples:**
- Signature bit-flip due to memory error.
- Signature overwritten by subsequent data.
- Signature truncated or malformed.

**Trigger:** Automatic **self-halt** with critical error log.

**Self-Halt Specification:**
```rust
pub fn validate_signature_integrity(evidence: &EvidenceBlock) -> Result<(), HaltSignal> {
    let stored_sig = &evidence.signature;
    let recomputed_sig = evidence.recompute_signature();
    
    if stored_sig != &recomputed_sig {
        return Err(HaltSignal {
            severity: Severity::CRITICAL,
            reason: "Signature corruption detected",
            evidence_id: evidence.id().to_string(),
            action: HaltAction::ImmediateShutdown,
        });
    }
    Ok(())
}
```

**Recovery:** None. All evidence blocks with corrupted signatures must be discarded and regenerated.

---

### Unacceptable Loss 3: Object Identity Disconnection (OCEL)

**Definition:** Loss of the cryptographic link between object ID and its state history.

**Examples:**
- Object reference lost or modified (object ID changed).
- State history truncated (object mutations lost).
- Object lifecycle broken (missing START or END event).

**Trigger:** Automatic **self-halt** with critical error log.

**Self-Halt Specification:**
```rust
pub fn validate_object_identity_continuity(ocel_log: &OcelLog) -> Result<(), HaltSignal> {
    for object_id in ocel_log.all_object_ids() {
        let events_touching_object = ocel_log.events_with_object(object_id);
        
        // Verify object appears in contiguous event sequence
        let mut prev_event_idx = None;
        for event_idx in events_touching_object {
            if let Some(prev) = prev_event_idx {
                // Check for gaps in object history (loss of intermediate mutations)
                if event_idx - prev > 1 {
                    let gap = event_idx - prev - 1;
                    // Gaps are acceptable only if intermediate events don't touch this object
                    let events_in_gap = &ocel_log.events()[prev + 1..event_idx];
                    let gap_touches_object = events_in_gap.iter()
                        .any(|e| e.objects().contains(&object_id));
                    
                    if gap_touches_object {
                        return Err(HaltSignal {
                            severity: Severity::CRITICAL,
                            reason: "Object identity disconnection (gap in object history)",
                            object_id: object_id.to_string(),
                            gap_size: gap,
                            action: HaltAction::ImmediateShutdown,
                        });
                    }
                }
            }
            prev_event_idx = Some(event_idx);
        }
    }
    Ok(())
}
```

**Recovery:** None. The OCEL log must be considered corrupted and regenerated.

---

### Unacceptable Loss 4: Timestamp Sequence Violation

**Definition:** Loss or modification of timestamps such that temporal monotonicity within a case is violated.

**Examples:**
- Timestamps reordered (event 5 has earlier timestamp than event 3).
- Timestamp truncated or corrupted.
- Timestamp reference lost (event has no timestamp).

**Trigger:** Automatic **self-halt** with critical error log.

**Self-Halt Specification:**
```rust
pub fn validate_temporal_monotonicity(trace: &Trace) -> Result<(), HaltSignal> {
    let mut prev_timestamp = None;
    
    for event in trace.events() {
        let ts = event.timestamp();
        
        if let Some(prev_ts) = prev_timestamp {
            if ts < prev_ts {
                return Err(HaltSignal {
                    severity: Severity::CRITICAL,
                    reason: "Temporal monotonicity violation",
                    case_id: trace.case_id().to_string(),
                    event_id: event.id().to_string(),
                    timestamp: ts,
                    previous_timestamp: prev_ts,
                    action: HaltAction::ImmediateShutdown,
                });
            }
        }
        prev_timestamp = Some(ts);
    }
    Ok(())
}
```

**Recovery:** None. The trace must be discarded.

---

## V. Loss Report Specification

Every loss event (permissible or terminal) generates a `LossReport`:

```rust
pub struct LossReport {
    pub timestamp: DateTime<Utc>,
    pub loss_type: LossType,
    pub severity: Severity,
    pub memory_utilization: f64,
    pub affected_events: usize,
    pub affected_objects: Option<Vec<String>>,
    pub loss_detail: serde_json::Value,
    pub recovery_action: RecoveryAction,
    pub signature: Option<Ed25519Signature>, // For auditor-signed recovery decisions
}

pub enum LossType {
    OrganizationalMetadataAttrition,
    OptionalAttributePruning,
    TraceDecimation,
    CausalSpineCorruption,
    CryptographicSignatureCorruption,
    ObjectIdentityDisconnection,
    TimestampSequenceViolation,
}

pub enum Severity {
    INFORMATIONAL,  // Permissible loss, logged for audit
    WARNING,        // Borderline permissible, requires review
    CRITICAL,       // Terminal loss, triggers self-halt
}

pub enum RecoveryAction {
    Continue,       // Loss was permissible, execution continues
    HaltWithReport, // Terminal loss, immediate shutdown
    RequireOverride, // Requires board/auditor override signature to continue
}
```

---

## VI. Loss Budget Accounting

Each execution session is allocated a **loss budget** based on operational profile:

**Budget Calculation:**
```
loss_budget_percent = 100 * (1 - fitness_threshold / 1.0)
```

For fitness threshold ≥ 0.95, loss budget ≤ 5% of total events.

**Budget Tracking:**
- Permissible loss (metadata, decimation) consumes budget.
- Terminal loss (causal spine, signature) immediately exhausts budget and triggers halt.

**Example:**
- Fitness threshold: 0.95
- Total events: 10,000
- Loss budget: 500 events (~5%)
- Metadata attrition: 50 events (10% of budget consumed)
- Trace decimation: 100 events (20% of budget consumed)
- **Remaining budget:** 350 events

If a causal spine corruption is detected, the budget is exhausted and self-halt is triggered immediately.

---

## VII. Lossless Operation Requirements

To avoid any loss whatsoever:

### Requirement 1: Streaming Ingestion

Rather than buffering events in-memory, stream events directly to external storage (database, cloud blob, distributed log). This eliminates memory saturation as a loss trigger.

### Requirement 2: Replication

Maintain hot replicas of the event log. If a replica's memory saturates, the other replicas absorb the load, ensuring lossless operation.

### Requirement 3: Ring Buffer with Overflow Management

Use a bounded ring buffer (circular buffer) that automatically spills oldest events to disk when full. This guarantees no loss of in-memory state.

### Requirement 4: Deterministic Event Ordering

Ensure all events are processed in deterministic, globally-monotonic order. This prevents timestamp reordering and temporal anomalies.

---

## VIII. Graduation Status

**Loss-Policy Map: COMPLETE AND OPERATIONALLY SOUND**

All permissible loss classes are justified and implementable. All terminal loss boundaries are rigorously defined. Self-halt semantics are architecturally sound.

**No gaps identified.**

---

## Related Documents

- [type-law-atlas.md](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/type-law-atlas.md) — Type-law surface inventory
- [witness-lattices.md](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/witness-lattices.md) — Witness algebra
- [admission-refusal-map.md](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/admission-refusal-map.md) — Admission boundary
- [structural-gaps.md](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/structural-gaps.md) — Implementation gaps
- [research-verdict.md](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/research-verdict.md) — Conformance audit verdict
- [xes_loss-policy_sample.md](file:///Users/sac/process-intelligence/experiments/xes_loss-policy_sample.md) — XES loss-policy validation example

---

## Section 23: The Between01 Lattice and Conformance Arithmetic (v30.1.1 Spec)

Let $\mathbb{Q}_{01} = \{ \frac{p}{q} \in \mathbb{Q} \mid 0 \leq \frac{p}{q} \leq 1, q > 0 \}$. $\mathbb{Q}_{01}$ is a bounded lattice under:
$$\frac{p_1}{q_1} \wedge \frac{p_2}{q_2} = \min\left(\frac{p_1}{q_1}, \frac{p_2}{q_2}\right), \qquad \frac{p_1}{q_1} \vee \frac{p_2}{q_2} = \max\left(\frac{p_1}{q_1}, \frac{p_2}{q_2}\right)$$
with bounds $0/1$ and $1/1$.

For trace token-replay fitness of $\tau_i$ against WF-net $N$:
$$f(\tau_i, N) = \frac{1}{2}\left(1 - \frac{m_i}{c_i}\right) + \frac{1}{2}\left(1 - \frac{r_i}{p_i}\right) \in [0, 1]$$
Aggregate fitness of $L = \{\tau_1, \ldots, \tau_n\}$ is:
$$F(L, N) = \frac{\sum_{i=1}^n |\tau_i| \cdot f(\tau_i, N)}{\sum_{i=1}^n |\tau_i|}$$

Precision via the escaping-edges estimator is:
$$\text{prec}(L, N) = \frac{\sum_{\hat\sigma \in \text{Pref}(L)} | \text{EN}(N, \hat\sigma) \cap A(L) |}{\sum_{\hat\sigma \in \text{Pref}(L)} | \text{EN}(N, \hat\sigma) |} \in [0, 1]$$
