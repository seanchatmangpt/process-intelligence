# GAP ANALYSIS — wasm4pm Execution Engine

**Agent:** E3 — wasm4pm Execution Authority
**Date:** 2026-05-31
**Source of Truth:** Live source inspection of ~/wasm4pm/

---

## GAP_001 — Cross-Compat: Zero Consumption of wasm4pm-compat Type Law

**Severity:** Critical

The graduation bridge at `/Users/sac/wasm4pm-compat/src/graduation.rs` defines `GraduateToWasm4pm` and `GraduationCandidate` as the typed seam between structure (compat) and execution (wasm4pm). The covenant: compat carries the evidence, wasm4pm adjudicates it.

**Finding:** wasm4pm has no dependency on wasm4pm-compat in any `Cargo.toml` or `*.rs` file. The seam is declared on the compat side but entirely absent on the engine side.

**Specific missing integrations:**
- `Evidence<T, State, W>` (`wasm4pm-compat/src/evidence.rs`) — the universal carrier type. wasm4pm algorithms take raw `&EventLog`, not admitted evidence.
- `Admission<T, W>` / `Refusal<R, W>` — typed refusals with named structural laws. wasm4pm uses `Error::ValidationError(String)`.
- `LossReport<From, To, Items>` — loss accounting on projections. No equivalent exists in wasm4pm.
- `Metric<KIND, NUM, DEN>` with `Between01` bounds — typed conformance metrics. wasm4pm `ConformanceResult` stores raw `f64`.

---

## GAP_002 — String-Typed Error Surface

**Severity:** High

All wasm4pm errors carry `String` payloads:

```rust
Error::ValidationError(String)
Error::ParseError(String)
Error::ExecutionError(String)
```

wasm4pm-compat mandates: "Every refusal must carry a **specific named law** as the reason type. `InvalidInput` or string-typed catch-alls are defects."

wasm4pm's entire error surface is catch-all strings. There are no named structural law types like `DanglingEventObjectLink`, `MissingFinalMarking`, or `InvalidDiscriminant` as first-class error variants.

---

## GAP_003 — Missing Inductive Miner

**Severity:** High

The `streaming.rs` module carries the comment "Inductive Miner - discovers structured process models recursively" but implements `discover_streaming_dfg` — a DFG, not a ProcessTree. The Inductive Miner algorithm (Leemans et al. 2013 ICATPN) is absent.

**What exists:**
- `pm-core/src/process_tree.rs` — `ProcessTree` and `ProcessOperator` types (no_std, no algorithms)
- `wasm4pm-types/src/powl8_op.rs` — `Powl8Op` enum with all operators including `ChoiceGraph`
- `wasm4pm-types/src/choice_graph.rs` — `ChoiceGraph` spec-compliant type

**What is missing:**
- An Inductive Miner implementation that takes `&EventLog` and returns `ProcessTree`
- Recursive base cases: base case empty, base case single activity, base case skip
- Cut detection: sequence, exclusive choice, parallel, loop cuts
- POWL Discovery (Partially Ordered Workflow Language) returning a POWL model

---

## GAP_004 — Missing POWL Discovery

**Severity:** High

`Powl8Op` enum in `wasm4pm-types` defines all POWL operators including `ChoiceGraph` (Kourani et al. arXiv:2505.07052). The `ChoiceGraph` type is spec-compliant. But no algorithm in `wasm4pm-algos` takes an `&EventLog` and returns a POWL model. The discovery surface is DFG-only plus Alpha+ Petri net.

---

## GAP_005 — Missing Object-Centric DFG (OC-DFG)

**Severity:** High

wasm4pm-algos has no Object-Centric DFG discovery. All DFG algorithms operate on flat `EventLog` traces. wasm4pm-types re-exports `OCEL` from ocel-core with full `e2o` and `o2o` relations, but no algorithm consumes OCEL to produce per-object-type DFGs.

**PM4Py comparison:** pm4py has `pm4py.algo.discovery.ocel.ocdfg` with full OC-DFG construction. wasm4pm has zero equivalent.

---

## GAP_006 — Missing Log Skeleton Discovery and Conformance

**Severity:** Medium

`pm-core/src/log_skeleton.rs` defines `LogSkeleton` with all five constraint sets (always_before, always_after, equivalence, never_together, activity_count). No algorithm in wasm4pm-algos:
- discovers a LogSkeleton from a log
- checks conformance of a log against a LogSkeleton

**PM4Py comparison:** pm4py has `pm4py.algo.discovery.log_skeleton` and `pm4py.algo.conformance.log_skeleton`.

---

## GAP_007 — Missing Alignment-Based Conformance

**Severity:** High

`pm-core/src/alignment.rs` defines `AlignmentMove`, `Alignment`, `AlignmentCost` grounded in Adriansyah 2014. No algorithm in wasm4pm-algos implements:
- A* shortest path in the synchronous product of a trace automaton and Petri net
- Alignment-based fitness computation
- Optimal/approximate alignment

The only conformance algorithm is token replay on DFG. Alignment is the gold standard for Petri net conformance and is entirely absent.

**PM4Py comparison:** pm4py has `pm4py.algo.conformance.alignments.petri_net.algorithm`.

---

## GAP_008 — OCPQ Execution Isolated from Algos

**Severity:** Medium

`crates/ocpq/src/lib.rs` implements the OCPQ runtime (Küsters & van der Aalst arXiv:2506.11541) with `Binding`, `BindingBox`, `QueryTree`, `Constraint`, `evaluate_constraint`. This is a significant capability. However:
- OCPQ is a separate crate not integrated into `wasm4pm-algos`
- No algorithm pipeline connects admitted OCEL evidence to OCPQ evaluation
- wasm4pm-compat `GraduationReason::NeedsObjectCentricQueryExecution` names this gap explicitly but the connection is missing

---

## GAP_009 — No Provenance on Algorithm Outputs

**Severity:** High

`ProvenanceChain` in `wasm4pm-types/src/provenance.rs` has all fields for a BLAKE3 receipt (`input_hash`, `config_hash`, `plan_hash`, `output_hash`, `combined_hash`). But:
- `discover_dfg` returns `Result<DFG>` — no `ProvenanceChain`
- `check_conformance_token_replay` returns `Result<ConformanceResult>` — no `ProvenanceChain`
- No algorithm returns a receipted output

`truex/verify.rs` verifies receipt envelopes but nothing in the discovery/conformance surface generates them. Provenance is a structure that exists but is disconnected from algorithm execution.

---

## GAP_010 — No ETConformance Precision Implementation

**Severity:** Medium

`pm-core/src/precision.rs` defines `EtcPrecisionResult` and `PrecisionScore` grounded in Munoz-Gama & Carmona 2010. `ConformanceResult` in wasm4pm-types has `precision: Option<f64>` but the `with_precision()` builder must be populated by callers manually. No algorithm computes ETConformance precision from escaping edges.

---

## Summary Table

| Gap | Description | Severity |
|---|---|---|
| GAP_001 | Zero consumption of wasm4pm-compat type law | Critical |
| GAP_002 | String-typed error surface | High |
| GAP_003 | Missing Inductive Miner | High |
| GAP_004 | Missing POWL Discovery | High |
| GAP_005 | Missing Object-Centric DFG | High |
| GAP_006 | Missing Log Skeleton discovery/conformance | Medium |
| GAP_007 | Missing alignment-based conformance | High |
| GAP_008 | OCPQ isolated from algorithm pipeline | Medium |
| GAP_009 | No provenance receipts on algorithm outputs | High |
| GAP_010 | No ETConformance precision implementation | Medium |
