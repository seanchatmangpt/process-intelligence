# REFACTOR ROADMAP — wasm4pm Post-Research Program

**Agent:** E3 — wasm4pm Execution Authority
**Date:** 2026-05-31
**Status:** RESEARCH AUTHORITY — no code changes until ALIVE gate is sealed

---

## CRITICAL CONSTRAINT

This roadmap is **RESEARCH AUTHORITY ONLY**. No changes to wasm4pm source code are authorized until:

1. PAPERLAW_ALIVE_003 gate is sealed on wasm4pm-compat
2. All trybuild fixtures in `tests/ui/` pass for the named type laws
3. The graduation bridge `GraduateToWasm4pm` trait is formally ratified

The purpose of this document is to capture what the research program has revealed about the correct refactor sequence, not to authorize implementation.

---

## P1 — Import wasm4pm-compat, Consume Evidence Types

**Priority:** Blocking all other work
**Gaps closed:** GAP_001

Add to `wasm4pm-types/Cargo.toml`:

```toml
[dependencies]
wasm4pm-compat = { path = "../../wasm4pm-compat" }
```

Then implement `GraduateToWasm4pm` for the wasm4pm engine types. The trait surface in `wasm4pm-compat/src/graduation.rs` provides the typed seam. Algorithm functions should accept `Evidence<EventLog, Admitted, W>` rather than raw `&EventLog`.

The `Admitted` constructor on `Evidence` is `pub(crate)` in compat — the only public path is `Admit::admit()`. This means wasm4pm must pass evidence through the admission layer, which enforces that logs are structurally sound before any algorithm touches them.

**Expected outcome:** Every algorithm invocation carries a witness type `W` that names the specific paper/standard the input was admitted against (e.g., `Ocel20`, `Xes1849`). Admission failures return `Refusal<R, W>` with a named structural law, not `ValidationError(String)`.

---

## P2 — Replace ValidationError(String) with Named Law Refusals

**Priority:** High — enables all downstream typed error handling
**Gaps closed:** GAP_002

Replace the string-typed error enum with a set of named structural law types:

```rust
// BEFORE
Error::ValidationError(String)

// AFTER
Error::EmptyEventLog
Error::MissingActivityAttribute { key: &'static str }
Error::DanglingEventObjectLink { event_id: String, object_id: String }
Error::MissingFinalMarking
Error::UnsoundWorkflowNet { reason: NetSoundnessViolation }
```

The law type must be a type, not a string. This mirrors wasm4pm-compat's mandate: "bare `InvalidInput` is forbidden."

---

## P3 — Implement Inductive Miner Returning ProcessTree

**Priority:** High — closes the most visible algorithm gap
**Gaps closed:** GAP_003, GAP_004

The Inductive Miner implementation must:
- Take `Evidence<EventLog, Admitted, W>` as input
- Return `Evidence<ProcessTree, Projected, W>` as output
- Implement the four cuts (sequence, exclusive choice, parallel, loop) from Leemans et al. 2013
- Handle base cases (empty log, single activity, single trace)
- Handle infrequent behavior via the IMf variant
- Return a POWL model variant using `Powl8Op` from `wasm4pm-types` for the POWL Discovery surface

`pm-core/src/process_tree.rs` already has `ProcessOperator` with all four operators plus `Or`. `wasm4pm-types/src/powl8_op.rs` has `Powl8Op` including `ChoiceGraph`. The type infrastructure exists; only the algorithm is missing.

---

## P4 — Implement Alignment-Based Conformance

**Priority:** High — required for gold-standard conformance
**Gaps closed:** GAP_007

The alignment implementation must:
- Take `Evidence<(EventLog, PetriNet), Admitted, W>` as input
- Implement A* shortest path in the synchronous product of trace automaton × Petri net reachability graph
- Return `Evidence<Vec<Alignment>, Projected, W>` where each alignment is a sequence of `AlignmentMove`
- Compute fitness as `1 − cost(γ) / max_cost(γ)` per Adriansyah 2014 Def. 3
- Return a `ConformanceResult` with fitness, precision, generalization, simplicity

`pm-core/src/alignment.rs` defines `AlignmentMove`, `Alignment`, `AlignmentCost` already. `wasm4pm-types/src/models.rs` has `FlatIncidenceMatrix` and `DenseIndex` for efficient net traversal.

---

## P5 — Add OCPQ Query Evaluation to Algorithm Pipeline

**Priority:** Medium
**Gaps closed:** GAP_008

The `ocpq` crate has a complete OCPQ runtime. The missing piece is an algorithm entry point in `wasm4pm-algos` that:
- Accepts `Evidence<OCEL, Admitted, Ocel20>` and a `QueryTree`
- Evaluates the query using `evaluate_constraint`
- Returns a typed result with `LossReport` if the query required projection

`wasm4pm-compat/src/graduation.rs` `GraduationReason::NeedsObjectCentricQueryExecution` is the signal that a compat user needs this. The pipeline: compat admits OCEL → graduates via `GraduateToWasm4pm` → wasm4pm evaluates OCPQ query.

---

## P6 — Add Provenance Receipts on All Algorithm Outputs

**Priority:** High — the covenant between compat and engine requires receipts
**Gaps closed:** GAP_009

Every algorithm output must carry a `ProvenanceChain`. The chain structure already exists in `wasm4pm-types/src/provenance.rs`. The required change: wrap every algorithm return type in a receipted evidence carrier:

```rust
// BEFORE
pub fn discover_dfg(log: &EventLog, activity_key: &str) -> Result<DFG>

// AFTER
pub fn discover_dfg(
    log: Evidence<EventLog, Admitted, W>,
    activity_key: &str,
) -> Result<Evidence<(DFG, ProvenanceChain), Receipted, W>>
```

`truex/verify.rs` already implements receipt verification via BLAKE3. The generation side needs to be wired to algorithm execution using `ProvenanceChain::builder()`.

---

## P7 — Implement Object-Centric DFG Discovery

**Priority:** Medium
**Gaps closed:** GAP_005

Add `discover_oc_dfg(log: &OCEL) -> Result<HashMap<ObjectType, DFG>>` to `wasm4pm-algos`. This must:
- Iterate objects by type
- For each object type, extract the directly-follows relations over events touching that object type
- Return a per-type DFG map
- Reference: Berti & van der Aalst 2021 (OC-DFG paper)

---

## P8 — Implement ETConformance Precision

**Priority:** Medium
**Gaps closed:** GAP_010

`pm-core/src/precision.rs` defines `EtcPrecisionResult`. Implement the escaping-edges algorithm in `wasm4pm-algos` that populates it from a log and a Petri net replay. Wire the result into `ConformanceResult::with_precision()`.

---

## Dependency Order

```
P1 (compat import)
  └── P2 (named law errors)
        ├── P3 (Inductive Miner)
        │     └── P4 (alignment conformance)
        ├── P6 (provenance receipts)
        │     └── P5 (OCPQ pipeline)
        └── P7 (OC-DFG)
              └── P8 (ETConformance)
```

P1 and P2 are the pre-conditions for everything else. Nothing else should be attempted until the compat seam is connected and errors are named.
