# Algorithm Crosswalk: PM4Py vs. wasm4pm

**Source PM4Py:** ~/chatmangpt/pm4py/pm4py/ — Process Intelligence Solutions GmbH, 2026
**WASM bridge:** pm4wasm.d.ts
**wasm4pm-compat source:** ~/wasm4pm-compat/paper/

Coverage codes:
- COVERED — wasm4pm-compat has a type surface directly corresponding to this algorithm's output
- PARTIAL — wasm4pm-compat has related structure but the algorithm itself is not present (engine logic graduates to wasm4pm)
- MISSING — neither type surface nor algorithm is present in wasm4pm-compat

---

## Process Discovery Algorithms

| Algorithm | PM4Py Function | WASM Bridge | wasm4pm-compat Coverage | Notes |
|---|---|---|---|---|
| Alpha Miner | `discover_petri_net_alpha()` | `discover_petri_net_alpha(log_json)` | PARTIAL | `src/petri.rs` defines `WfNetConst<SOUNDNESS>` type surface; no alpha miner implementation (engine logic) |
| Inductive Miner (Petri) | `discover_petri_net_inductive()` | `discover_petri_net_inductive(log_json)` | PARTIAL | Petri net type surface covered; inductive algorithm engine not in compat crate |
| Inductive Miner (Process Tree) | `discover_process_tree_inductive()` | `discover_process_tree_inductive(log_json)` | PARTIAL | `src/process_tree.rs` defines `TypedLoopNode<ARITY>` and tree structure types; no algorithm |
| Inductive Miner (BPMN) | `discover_bpmn_inductive()` | `discover_bpmn_inductive(log_json)` | PARTIAL | `src/bpmn.rs` defines BPMN structure; no inductive algorithm |
| Heuristics Miner | `discover_heuristics_miner()` | `discover_heuristics_miner(log_json, dep_threshold)` | MISSING | No HeuristicsNet type in wasm4pm-compat |
| Genetic Miner | `discover_petri_net_genetic()` | `discover_petri_net_genetic(log_json, config_json)` | MISSING | No genetic algorithm surface; evolutionary algorithms do not produce typed receipts |
| POWL Discovery (Inductive variants) | `discover_powl()` (MAXIMAL, DYNAMIC_CLUSTERING, etc.) | no direct WASM mirror | PARTIAL | `src/powl.rs` defines `TreeProjectable` sealed trait, `assert_tree_projectable`; POWL type structure present, discovery not |
| POWL 2.0 ChoiceGraph / mineDG | `InductiveMinerChoiceGraph`, `mine_dg()` | NO — not in pm4wasm.d.ts | MISSING | `src/powl.rs` has no `DecisionGraph` type; this is the key gap (see POWL_CHOICEGRAPH_ANALYSIS.md) |
| DFG Discovery | `discover_dfg()` | `discover_dfg(log_json)` | PARTIAL | `src/dfg.rs` defines DFG structure; no discovery operator |
| Performance DFG | `discover_performance_dfg()` | `discover_performance_dfg(log_json)` | MISSING | No duration-annotated DFG type surface |
| Eventually-Follows Graph | `discover_eventually_follows_graph()` | `discover_eventually_follows_graph(log_json)` | MISSING | No EFG type |
| Correlation Mining | `discover_correlation()` | `discover_correlation(log_json, threshold)` | MISSING | No case-less DFG discovery type |
| DECLARE Discovery | `discover_declare()` | `discover_declare(log_json)` | PARTIAL | `src/declare.rs` defines DECLARE constraint types; no discovery algorithm |
| Log Skeleton | `discover_log_skeleton()` | `discover_log_skeleton(log_json)` | MISSING | No log skeleton type surface |
| Temporal Profile | `discover_temporal_profile()` | `discover_temporal_profile(log_json)` | MISSING | No temporal profile type |
| Batch Detection | `discover_batches()` | `discover_batches(log_json)` | MISSING | No batch pattern type |
| OC-DFG | `discover_ocdfg()` | NO WASM bridge | PARTIAL | `src/ocel.rs` defines OCEL structure; OC-DFG type not present |
| OCPQ | `pm4py.ocel.*` OCPQ queries | NO WASM bridge | PARTIAL | `src/ocpq.rs` in module list but OCPQ query types need verification |

---

## Conformance Checking Algorithms

| Algorithm | PM4Py Function | WASM Bridge | wasm4pm-compat Coverage | Notes |
|---|---|---|---|---|
| Token-based Replay | `conformance_diagnostics_token_based_replay()` | `token_replay_fitness(pn_json, log_json)` | PARTIAL | `src/conformance.rs` defines `Metric<KIND, NUM, DEN>` with `Between01` bounds; no replay algorithm |
| A* Alignment | `conformance_diagnostics_alignments()` | `align_log(pn_json, log_json)` | PARTIAL | Alignment result type implied by `Metric<FITNESS, ...>`; A* algorithm engine not in compat crate |
| ETConformance Precision | `precision_etconformance()` | `precision_etconformance(pn_json, log_json)` | PARTIAL | `Metric<PRECISION, NUM, DEN>` type surface covers this |
| Footprints Conformance | `conformance_diagnostics_footprints()` | `conformance_footprints(log_json, model_str)` | MISSING | No footprints type surface |
| Temporal Conformance | `conformance_temporal_profile()` | `check_temporal_conformance(log_json, profile_json, zeta)` | MISSING | No temporal conformance type |
| DECLARE Conformance | `conformance_declare()` | NO WASM bridge | PARTIAL | `src/declare.rs` has constraint types; conformance checking type missing |
| Log Skeleton Conformance | `conformance_log_skeleton()` | NO WASM bridge | MISSING | |
| Generalization Metric | `generalization()` | `generalization(pn_json, log_json)` | PARTIAL | `Metric<GENERALIZATION, NUM, DEN>` implied by `src/conformance.rs` design |
| POWL Structural Validation | `validate_partial_orders()` | `validate_partial_orders(model)` | COVERED | `src/petri.rs` — `WfNetConst<SOUNDNESS>` provides non-forgeable witness; `src/powl.rs` structural invariants |
| Petri Net Soundness | `check_soundness()` | `check_soundness(pn_json)` | COVERED | `WfNetConst<SOUNDNESS>` with const generic soundness flag |
| Streaming Conformance | `pm4py.streaming.*` | `streaming_create/push_trace/snapshot` | MISSING | No streaming conformance type surface in wasm4pm-compat |

---

## Variant Analysis

| Algorithm | PM4Py Function | WASM Bridge | wasm4pm-compat Coverage | Notes |
|---|---|---|---|---|
| Variant Discovery | `get_variants()` | `get_variants(log_json)` | MISSING | No variant set type; `src/eventlog.rs` has trace structure but no variant abstraction |
| Top-K Variant Filtering | `filter_variants_top_k()` | `filter_variants_top_k(log_json, k)` | MISSING | |
| Variant Coverage Filtering | `filter_variants_reaching()` | `filter_variants_coverage(log_json, min)` | MISSING | |
| Prefix Analysis | `get_prefixes_from_log()` | `get_prefixes_from_log(log_json)` | MISSING | |

---

## Object-Centric Discovery (OC)

| Algorithm | PM4Py Function | WASM Bridge | wasm4pm-compat Coverage | Notes |
|---|---|---|---|---|
| OC-DFG Discovery | `discover_ocdfg()` | NO | PARTIAL | OCEL types present in `src/ocel.rs`; OC-DFG discovery type missing |
| OC-DFG Conformance | `conformance_ocdfg()` | NO | MISSING | |
| OCPQ Queries | `pm4py.ocel.*` | NO | PARTIAL | `src/ocpq.rs` present per module list |
| Object Interaction Graph | `pm4py.ocel.*` | NO | MISSING | |

---

## Summary Table

| Category | Total Algorithms | COVERED | PARTIAL | MISSING |
|---|---|---|---|---|
| Process Discovery | 18 | 0 | 8 | 10 |
| Conformance Checking | 11 | 2 | 6 | 3 |
| Variant Analysis | 4 | 0 | 0 | 4 |
| Object-Centric | 4 | 0 | 2 | 2 |
| **Total** | **37** | **2** | **16** | **19** |

**COVERED count is intentionally low.** wasm4pm-compat is a type-law and structure crate — it provides the type surfaces that engine algorithms produce results into. The algorithms themselves (alpha, inductive, alignment A*, token replay) are engine logic that belongs in `wasm4pm`, not here. PARTIAL means the output type surface exists; MISSING means even the output type is absent.

---

## Critical Gaps for wasm4pm-compat

Ranked by impact on the ALIVE gate and paper conformance:

1. **POWL 2.0 DecisionGraph type** — `mine_dg` / ChoiceGraph outputs a `DecisionGraph` object that `src/powl.rs` does not currently model. Required for Kourani-Park-van-der-Aalst 2025 paper conformance.
2. **Streaming conformance type surface** — `streaming_create/push_trace/snapshot` has no Rust type equivalent in wasm4pm-compat.
3. **HeuristicsNet type** — `src/` has no heuristics net type surface; WASM bridge exposes full heuristics miner.
4. **Temporal Profile / Temporal Conformance** — both discovery and conformance surfaces absent.
5. **Log Skeleton** — 6-constraint-type structure with no wasm4pm-compat equivalent.
6. **OC-DFG** — object-centric DFG type missing despite OCEL admission types being present.
7. **Variant types** — `VariantSet`, `VariantTuples`, prefix/suffix abstractions absent.
8. **Performance Spectrum** — per-activity performance type missing.
