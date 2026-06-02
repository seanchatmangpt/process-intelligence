# wasm4pm + wasm4pm-compat / Market Physics Evidence Contract

**Status:** CONTRACT_UPDATED  
**Date:** 2026-06-01  
**Authority:** Process Intelligence Research Foundry / Agent 5  
**Source Verification:** c8-market/src/lib.rs, c8-time/src/lib.rs, c8-receipts/src/lib.rs, c8-adversary/src/lib.rs

---

## Mapping: Market Physics -> Process Evidence
| Market Physics Concept | Process Evidence Equivalent | Owner |
|---|---|---|
| MarketPlanckCell | OCEL object event | c8-market / wasm4pm boundary |
| Construct8Delta | bounded graph mutation event | c8-graph (not wasm4pm) |
| RepresentationGap | conformance deviation | wasm4pm conformance engine |
| C8Receipt | process evidence receipt | c8-receipts |
| VectorClock8 | causal ordering in OCEL | c8-time |

---

## Grounding in Source Types

### MarketPlanckCell -> OCEL Object Event
`MarketPlanckCell` (c8-market/src/lib.rs:131) is the atomic, indivisible unit of market
observation. It carries: `instrument_id`, `venue_id`, `relation_kind`, `causal_time`
(VectorClock8), `monotonic_time` (MonotonicStamp), `pre_state_hash`, `post_state_hint`,
`delta_mask`, `confidence_bucket`, `actuation_class`.

In OCEL terms this maps to a single object-centric event: the instrument and venue are
the OCEL objects, the relation_kind is the activity, and causal_time + monotonic_time
provide the dual timestamp required by the OCEL 2.0 tuple (E, O, EA, OA, E2O, O2O).

### Construct8Delta -> Bounded Graph Mutation Event
`Construct8Delta` (c8-market/src/lib.rs:106) is extracted from a MarketPlanckCell via
`to_construct8_delta()`. It captures the pre_state_hash, post_state_hint, delta_mask, and
confidence. This is a bounded graph mutation — the change described by an edge in a causal
graph, not a mining artefact. Its owner is the graph-layer (c8-graph concept), not wasm4pm.

### RepresentationGap -> Conformance Deviation
`RepresentationGap` (c8-adversary/src/lib.rs:177) quantifies the difference between logic
and graph representations of the same market event. It exposes `gap_magnitude` (0.0 = identical,
1.0 = completely different) and `bias` (which representation is more complete). In process
mining terms this is a conformance deviation — the delta between the declared process model
and what the log reveals. The wasm4pm conformance engine is the authorised consumer.

### C8Receipt -> Process Evidence Receipt
`C8Receipt` (c8-receipts crate) is a single state transition proof: (pre, delta, post,
causal_time, hash). Receipt chains provide forward-chaining verification. In process evidence
terms this is the immutable proof that a lawful state transition occurred. The chain is the
object lifecycle history that OCEL replay uses to confirm conformance.

### VectorClock8 -> Causal Ordering in OCEL
`VectorClock8` (c8-time/src/lib.rs:24) is an 8-lane logical timestamp capturing partial
ordering relationships across independent causal axes (instrument, venue, actor). In OCEL
this maps to the causal ordering requirement: events must carry their original causal time,
never re-stamped at boundary crossing.

---

## What belongs in wasm4pm-compat
- Type-law compatibility layers (Blue River Dam Level 2 and 3)
- Witness lattice definitions
- Admitted evidence structures (`AdmittedEvidence<T>`)
- Format crossing under named projection + LossPolicy + LossReport (formats feature)
- Graduation bridge types (wasm4pm feature, graduation.rs)
- NOT: market instruments, not hot-path market types
- NOT: mining algorithms, conformance engines, or replay infrastructure

## What belongs ONLY in full wasm4pm
- Process discovery algorithms (all 60 registered algorithms including dfg, inductive, etc.)
- Conformance checking engine (fitness, precision, generalization, simplicity)
- OCEL replay infrastructure
- Token replay
- Prediction problems

---

## MarketPlanckCell -> OCEL Transformation Path

```
MarketPlanckCell (c8-market)
    |
    to_construct8_delta()     [COMPAT LAYER]
    |
Construct8Delta
    |
    -> OcelEvent               [COMPAT LAYER]
    -> OcelObject              [COMPAT LAYER]
    -> EventObjectLink (E2O)   [COMPAT LAYER]
    |
OcelLog (Admitted)
    |
    [GRADUATION BOUNDARY]
    |
wasm4pm discovery/conformance [EXECUTION]
```

---

## Hard Gate
wasm4pm-compat must NOT gain mining/conformance/replay engine behavior.
The PRD/ARD doctrine is explicit: "The doorway must not become the throne room."
Violation = BLOCKED status for this adapter.

---

## Format Laundering Test
A MarketPlanckCell emitted as OCEL must carry its original causal time (VectorClock8
lanes), not re-stamp. The monotonic_time (MonotonicStamp) is the wall-clock anchor.
The causal_time (VectorClock8) is the logical order anchor. Both must survive boundary
crossing intact. Dropping or re-stamping causal_time at the wasm4pm-compat admission
boundary is a format laundering violation and must be refused by the LossPolicy layer.

---

## Banned Translations (from CONSTRUCT8_PROJECT_CONTRACTS.md)
- wasm4pm-compat must never be called a "lite engine"
- RepresentationGap must never be called "just a threshold check"
- VectorClock8 causal ordering must never be flattened to wall-clock-only ordering
- C8Receipt chains must never be described as "hashes" or "logs"

---

## References
- `~/wasm4pm-compat/README.md` — Structure-only design philosophy
- `~/wasm4pm/README.md` — Execution engine CLI and algorithms
- `~/wasm4pm-compat/c8-market/src/lib.rs` — MarketPlanckCell, Construct8Delta source types
- `~/wasm4pm-compat/c8-time/src/lib.rs` — VectorClock8 source type
- `~/wasm4pm-compat/c8-receipts/src/lib.rs` — C8Receipt source type
- `~/wasm4pm-compat/c8-adversary/src/lib.rs` — RepresentationGap source type
- `~/wasm4pm-compat/WASM4PM-COMPAT-PRD-ARD.md` — Blue River Dam five-level model
- `~/wasm4pm-compat/CONSTRUCT8_PROJECT_CONTRACTS.md` — Doctrine naming enforcement
- `~/.claude/rules/process-mining-chicago-tdd.md` — Van der Aalst Constitution
