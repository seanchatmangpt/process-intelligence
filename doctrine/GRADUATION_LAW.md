# Graduation Law: Between Compat and Execution Authority

---

## The Constitutional Boundary

> Structure belongs in wasm4pm-compat. Execution belongs in wasm4pm.

This is not a preference. It is the architectural constitution of the Blue River Dam system.
The boundary is enforced by `#![forbid(unsafe_code)]` at the crate level and by the absence
of any data-traversal algorithm in wasm4pm-compat's source.

**The doorway must not become the throne room.** (Blue River Dam doctrine)

---

## The Law

> **No algorithm that traverses data belongs in wasm4pm-compat.**

An algorithm traverses data if it:
- Iterates over the events in an `EventLog` or `OcelLog`
- Fires transitions in a Petri net
- Computes counts, sums, ratios, or frequencies over event or object attributes
- Applies a decision tree or ML model
- Searches a state space (A*)
- Produces aggregate statistics from event sequences

Structure defines **what** can exist and **what laws it must obey**. Execution decides
**what actually happened** and **what the conformance verdict is**.

The dividing question: **Does this code produce a new value by reading existing values?**
If yes, it is execution. It belongs in wasm4pm.

---

## What This Means in Practice

### Belongs in compat:
- Type definitions for all formal process objects (L1–L4 taxonomy)
- `PhantomData` witness markers that certify structural laws
- Const-generic bounds (`Between01<NUM, DEN>`, `Require<{ ARITY == 2 }>: IsTrue`)
- Builder APIs for constructing evidence shapes
- `Admit::admit()` — validates structure, not data values
- `LossPolicy` and `LossReport` types — declare intent, do not compute
- `GraduationCandidate` — the typed bridge artifact carrying `GraduationReason`

### Belongs in wasm4pm:
- Token replay (fire transitions, count tokens)
- Alignment computation (A* cost search)
- DFG computation from event log (count directly-follows pairs)
- Inductive Miner, Alpha Miner, Heuristics Miner (traverse log to discover model)
- OC-DFG computation from OCEL (aggregate per-object-type DFG)
- Conformance metric computation (divide missing by produced)
- Prediction inference (ML model execution)
- Receipt emission (compute proof hash over execution record)

---

## The `GraduationCandidate` Type

Located in `src/graduation.rs` (behind the `wasm4pm` feature flag).

`GraduationCandidate` is the typed bridge artifact. It carries:
- The structural type being graduated (the `T` in `GraduationCandidate<T>`)
- A `GraduationReason` naming why graduation is required:
  - `NeedsDiscovery` — the type is an output shape for a discovery algorithm
  - `NeedsConformanceExecution` — the type is an input/output shape for conformance checking
  - `NeedsObjectCentricQueryExecution` — the type is an input/output for OCPQ evaluation
  - `NeedsReplay` — the type is a replay target
  - `NeedsReceipts` — the type requires receipt emission from wasm4pm
  - `RebuildingProcessMiningLocally` — the type wraps a PM algorithm being rebuilt inside wasm4pm
- Whether the candidate `is_grounded()` — i.e., whether the structural law is fully typed in compat

`GraduationCandidate::is_grounded()` returns true when:
1. The structural type `T` is fully typed in compat
2. All required compile-pass and compile-fail fixtures pass
3. The `GraduationReason` is specific (not a catch-all)

`GraduationCandidate::is_grounded()` returns false when:
1. The structural type `T` has gaps (e.g., `CausalNet` missing, `OcDfgEdge` missing)
2. Fixtures are incomplete
3. The reason is under-specified

---

## The Gap: The Bridge Is One-Sided

**Current state:** wasm4pm has no intake function for `GraduationCandidate`.

The bridge is fully defined on the compat side:
- `GraduationCandidate<T>` struct exists
- `GraduationReason` enum is complete
- `GraduateToWasm4pm` trait is defined
- `graduation` example demonstrates the API

The bridge is **not yet defined on the wasm4pm side:**
- wasm4pm has no `fn ingest_candidate<T: GraduateToWasm4pm>(candidate: GraduationCandidate<T>)`
- wasm4pm has no `fn execute_discovery(candidate: GraduationCandidate<EventLog>)`
- wasm4pm has no receipt-bearing entry point that consumes a `GraduationCandidate`

**This is the highest-priority structural gap** in the entire system. Without the intake
function in wasm4pm, the graduation bridge is a declaration without a destination. It is
Level 4 maturity by structure but not by function.

---

## What Closing the Gap Requires

### On the wasm4pm side:
1. Define `pub fn ingest_graduation<T>(c: GraduationCandidate<T>) -> GraduationReceipt<T>`
   as the canonical entry point for all graduated types.
2. Implement per-`GraduationReason` dispatch:
   - `NeedsDiscovery` → route to discovery engine
   - `NeedsConformanceExecution` → route to token replay or alignment engine
   - `NeedsReplay` → route to replay surface
   - `NeedsReceipts` → emit receipt on completion
3. Return a `GraduationReceipt<T>` that carries the execution result and a `Receipt`.

### On the compat side (remaining gaps):
1. `CausalNet` structural type — needed before `GraduationCandidate<CausalNet>` is grounded
2. `OcDfgEdge<ObjectType>` with `PhantomData` marker — needed before OC-DFG candidates are grounded
3. `DivergenceWitness` and `ConvergenceWitness` in `src/witness.rs` — needed before OC-PM
   candidates citing van der Aalst & Berti (2020) are fully grounded

---

## The Permanent Prohibition

The following must **never** cross from wasm4pm into compat:
- Any `impl` block that iterates over event data
- Any `fn` that counts tokens, fires transitions, or computes alignment costs
- Any dependency on a PM algorithm library (pm4py, petri_net_simpy, etc.)
- Any `#[cfg]`-gated algorithm behind a "computation" feature flag

If such code appears in wasm4pm-compat, it violates the graduation law. It is not a
technical debt item — it is a constitutional violation that must be reverted.

**The doorway does not contain the engine. The doorway leads to the engine.**

---

## Graduation Law as Board Claim

> "Our architecture maintains a hard boundary between process evidence structure
> (wasm4pm-compat) and process truth execution (wasm4pm). No algorithm that traverses
> data lives in the structure layer. Every execution result is receipted before being
> surfaced as a claim. The graduation bridge is typed, named, and auditable."

This claim becomes fully board-admissible when the wasm4pm intake function for
`GraduationCandidate` is implemented and receipted.
