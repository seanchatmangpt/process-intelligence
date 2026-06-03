# Agent 5: wasm4pm + wasm4pm-compat Process Evidence Boundary

**Date:** 2026-06-01  
**Agent:** Agent 5 — wasm4pm + wasm4pm-compat Process Evidence Boundary  
**Status:** COMPLETE

---

## Mission

Define how market physics evidence maps to process evidence boundaries between
c8-market (wasm4pm-compat) and the full wasm4pm execution engine.

---

## Inspection Summary

### wasm4pm (Full Execution Engine)
- Location: `/Users/sac/wasm4pm`
- 60 discovery and analysis algorithms (dfg, inductive, heuristic, genetic, etc.)
- Nine Old-AI cognition breeds
- Native OCEL 2.0 support
- CLI: `wpm run <log>`, `wpm algorithms`
- Key crates: `ocel-core`, `pm-core`, `wasm4pm-algos`, `wasm4pm-cognition`, `wasm4pm-types`

### wasm4pm-compat (Structure-Only Boundary Layer)
- Location: `/Users/sac/wasm4pm-compat`
- Nightly Rust only — no stable build target
- Blue River Dam Level 2 (base) and Level 3 (strict feature)
- Key subcrates: `c8-market`, `c8-time`, `c8-receipts`, `c8-adversary`, `c8-instruments`
- Three Cargo features: `formats`, `strict`, `wasm4pm` (graduation bridge)

---

## Top 5 Evidence Mappings (Source-Verified)

### 1. MarketPlanckCell -> OCEL Object Event
Source: `c8-market/src/lib.rs:131`

`MarketPlanckCell` is the atomic unit of market event quantization. It holds instrument_id,
venue_id, relation_kind, causal_time (VectorClock8), monotonic_time (MonotonicStamp),
pre_state_hash, post_state_hint, delta_mask, confidence_bucket, and actuation_class.

This maps directly to an OCEL 2.0 object-centric event. The instrument+venue pair forms
the OCEL object; the relation_kind is the activity; causal_time + monotonic_time satisfy
the OCEL 2.0 dual-timestamp requirement in tuple (E, O, EA, OA, E2O, O2O).

### 2. VectorClock8 -> Causal Ordering in OCEL
Source: `c8-time/src/lib.rs:24`

`VectorClock8` is an 8-lane logical timestamp for capturing partial ordering relationships
across independent causal axes (instrument, venue, actor). It maps to the causal ordering
field in OCEL events. Critical constraint: causal_time must not be re-stamped when a
MarketPlanckCell is admitted through the compat boundary. Re-stamping is format laundering
and must be refused by the LossPolicy layer.

### 3. RepresentationGap -> Conformance Deviation
Source: `c8-adversary/src/lib.rs:177`

`RepresentationGap` carries `gap_magnitude` (0.0 = identical, 1.0 = completely different)
and `bias` (which representation — graph or logic — is more complete). In process mining
terms this is a conformance deviation: the measurable delta between the declared process
model and what the event log reveals. The wasm4pm conformance engine is the sole authorised
consumer of this type.

### 4. C8Receipt -> Process Evidence Receipt
Source: `c8-receipts/src/lib.rs`

`C8Receipt` is a single state transition proof (pre, delta, post, causal_time, hash).
`ReceiptChain` provides forward-chaining verification. In process evidence terms this is
the immutable record that a lawful state transition occurred — the object lifecycle
history used by OCEL replay. Structure is in compat; minting (cryptographic operations
+ timestamp binding) is in wasm4pm.

### 5. Construct8Delta -> Bounded Graph Mutation Event
Source: `c8-market/src/lib.rs:106`

`Construct8Delta` is extracted from a MarketPlanckCell via `to_construct8_delta()`.
It carries pre_state_hash, post_state_hint, delta_mask, and confidence_bucket.
This represents a bounded graph mutation — the edge description in a causal graph.
It is NOT a mining artefact. Its owner is the graph-layer concept (c8-graph), not wasm4pm.
wasm4pm consumes it only after it has been admitted and shaped by compat.

---

## Hard Gates Identified

### Gate 1: Doorway Must Not Become Throne Room
wasm4pm-compat (PRD/ARD doctrine): "The doorway must not become the throne room."
Any feature that requires running an algorithm, measuring runtime, minting receipts
cryptographically, or evaluating a query must NOT be added to compat.
Status: ENFORCED by Blue River Dam Level 2/3 scope constraint.

### Gate 2: Format Laundering is a Violation
A MarketPlanckCell admitted as OCEL must retain its original VectorClock8 causal_time.
Re-stamping at the compat boundary is format laundering. The LossPolicy layer
(formats feature) must refuse this transformation or name it explicitly with a LossReport.
Status: ENFORCED by LossPolicy type law in compat.

### Gate 3: Banned Translations (from CONSTRUCT8_PROJECT_CONTRACTS.md)
- wasm4pm-compat must never be called a "lite engine"
- RepresentationGap must never be called "just a threshold check"
- VectorClock8 causal ordering must never be flattened to wall-clock-only ordering
- C8Receipt chains must never be described as "hashes" or "logs"
Status: ENFORCED by doctrine audit (Agent 2 verification complete as of 2026-06-01).

---

## Evidence Flow

```
External Format (XES, OCEL, c8-market data)
    |
    +-> Compat: Parse -> Raw
    |
    +-> Compat: Admit -> Admitted (refused if laws violated)
    |
    +-> Compat: Shape checks
    |   - Temporal consistency (monotonic_time never regresses)
    |   - Causal consistency (VectorClock8 happens-before preserved)
    |   - EventObjectLink correctness
    |   - No dangling references
    |
    +-> Compat: MarketPlanckCell -> OcelLog (via Construct8Delta)
    |
    +-> Compat: Declare GraduationReason (if computation needed)
    |
    +-> [GRADUATION BOUNDARY]
        |
        +-> wasm4pm: Discovery / Conformance / Replay / Receipts / OCPQ
            |
            +-> Evidence<T, Executed, Receipt>
                (Final proof of lawful process execution)
```

---

## Artifact Produced

- `/Users/sac/process-intelligence/cross-project-coordinate-alpha/adapters/wasm4pm_evidence_contract.md`

---

## References

- `/Users/sac/wasm4pm-compat/c8-market/src/lib.rs` — MarketPlanckCell, Construct8Delta
- `/Users/sac/wasm4pm-compat/c8-time/src/lib.rs` — VectorClock8
- `/Users/sac/wasm4pm-compat/c8-receipts/src/lib.rs` — C8Receipt, ReceiptChain
- `/Users/sac/wasm4pm-compat/c8-adversary/src/lib.rs` — RepresentationGap
- `/Users/sac/wasm4pm-compat/WASM4PM-COMPAT-PRD-ARD.md` — Blue River Dam five-level model
- `/Users/sac/wasm4pm-compat/CONSTRUCT8_PROJECT_CONTRACTS.md` — Naming enforcement audit
- `/Users/sac/wasm4pm/README.md` — 60-algorithm execution engine
