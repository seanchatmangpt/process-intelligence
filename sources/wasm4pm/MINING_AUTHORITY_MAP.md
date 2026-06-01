# MINING AUTHORITY MAP — wasm4pm Process Mining Surfaces

**Agent:** E3 — wasm4pm Execution Authority
**Date:** 2026-05-31

---

## Mandate

wasm4pm must own: process discovery, conformance checking, replay, alignment, prediction, query evaluation. No other crate in the ecosystem may implement these. wasm4pm-compat is structure-only; the moment it touches an algorithm, that is `GraduationReason::RebuildingProcessMiningLocally` — a hard graduation signal.

---

## 1. Process Discovery

### 1.1 DFG Discovery — PRESENT

| | |
|---|---|
| **File** | `wasm4pm-algos/src/dfg.rs` |
| **Function** | `discover_dfg(log: &EventLog, activity_key: &str) -> Result<DFG>` |
| **Algorithm** | Single-pass columnar encoding, O(n) time, O(k+e) space |
| **Paper** | van der Aalst 2016 §3 |
| **PM4Py equiv** | `pm4py.algo.discovery.dfg.algorithm` |
| **Status** | Present, tested |
| **Gap** | Does not return provenance; takes raw EventLog not admitted evidence |

### 1.2 Alpha+ Miner — PRESENT (partial)

| | |
|---|---|
| **File** | `wasm4pm-algos/src/alpha.rs` |
| **Function** | `discover_alpha(log: &EventLog, activity_key: &str) -> Result<PetriNet>` |
| **Algorithm** | Alpha+ with causality and parallel relations |
| **Paper** | van der Aalst et al. 2004 |
| **PM4Py equiv** | `pm4py.algo.discovery.alpha.algorithm` |
| **Status** | Present, partial — implicit place handling approximated |
| **Gap** | Not true Alpha+ (no loop handling, no implicit place discovery) |

### 1.3 Heuristic Miner — PRESENT (DFG only)

| | |
|---|---|
| **File** | `wasm4pm-algos/src/heuristic.rs` |
| **Function** | `discover_heuristic(log: &EventLog, activity_key: &str) -> Result<DFG>` |
| **Algorithm** | DFG with frequency thresholds |
| **Paper** | Weijters & van der Aalst 2003 |
| **PM4Py equiv** | `pm4py.algo.discovery.heuristics.algorithm` |
| **Status** | Present — returns DFG, not HeuristicsNet |
| **Gap** | Should return `HeuristicsNet` (defined in pm-core); dependency graph construction missing |

### 1.4 Inductive Miner — MISSING

| | |
|---|---|
| **File** | None |
| **Paper** | Leemans, Fahland & van der Aalst 2013 ICATPN LNCS 7927 |
| **PM4Py equiv** | `pm4py.algo.discovery.inductive.algorithm` |
| **Status** | MISSING — streaming.rs is falsely labeled "Inductive Miner" but implements DFG |
| **Types ready** | `pm-core/src/process_tree.rs` has `ProcessTree` and all operators |
| **Gap** | No recursive cut detection, no base cases, no ProcessTree output |

### 1.5 POWL Discovery — MISSING

| | |
|---|---|
| **File** | None |
| **Paper** | Kourani & van der Aalst 2023 (POWL); Kourani et al. arXiv:2505.07052 (ChoiceGraph) |
| **PM4Py equiv** | `pm4py.algo.discovery.powl.inductive.algorithm` |
| **Status** | MISSING |
| **Types ready** | `wasm4pm-types/src/powl8_op.rs` (`Powl8Op` with 9 operators including `ChoiceGraph`), `wasm4pm-types/src/choice_graph.rs` |
| **Gap** | No algorithm that discovers a POWL model from a log |

### 1.6 Object-Centric DFG — MISSING

| | |
|---|---|
| **File** | None |
| **Paper** | Berti & van der Aalst 2021 |
| **PM4Py equiv** | `pm4py.algo.discovery.ocel.ocdfg` |
| **Status** | MISSING |
| **Types ready** | `wasm4pm-types/src/ocel.rs` re-exports OCEL with full e2o/o2o relations |
| **Gap** | No per-object-type DFG construction from OCEL |

### 1.7 Log Skeleton Discovery — MISSING

| | |
|---|---|
| **File** | None |
| **Paper** | Verbeek 2021 STTT |
| **PM4Py equiv** | `pm4py.algo.discovery.log_skeleton` |
| **Status** | MISSING |
| **Types ready** | `pm-core/src/log_skeleton.rs` has `LogSkeleton` with all five constraint sets |
| **Gap** | No algorithm that extracts constraint sets from a log |

---

## 2. Conformance Checking

### 2.1 Token Replay on DFG — PRESENT

| | |
|---|---|
| **File** | `wasm4pm-algos/src/conformance.rs` |
| **Function** | `check_conformance_token_replay(log: &EventLog, model: &DFG, activity_key: &str) -> Result<ConformanceResult>` |
| **Algorithm** | Token multiset replay; Rozinat & van der Aalst fitness formula |
| **Paper** | Rozinat & van der Aalst 2008 |
| **PM4Py equiv** | `pm4py.algo.conformance.tokenreplay` |
| **Status** | Present, tested |
| **Gap** | DFG only — no Petri net replay; no provenance; raw EventLog not admitted evidence |

### 2.2 Prefix Conformance (Online) — PRESENT

| | |
|---|---|
| **File** | `wasm4pm-algos/src/prefix_conformance/` |
| **Key types** | `PrefixOracle`, `PrefixVerdict` (ALIVE/DEAD/TERMINAL), `PrefixRefusal` (10 named codes) |
| **Status** | Present — streaming DFA-based conformance against `OrderingLaw` |
| **Strength** | Named refusal codes (not strings): `ReceiptBeforeGate`, `RepairWithoutRoute`, etc. |
| **Gap** | Domain-specific (RevOps-shaped law), not a general-purpose Petri net conformance oracle |

### 2.3 TrueX Receipt Verification — PRESENT

| | |
|---|---|
| **File** | `wasm4pm-algos/src/truex/verify.rs` |
| **Function** | `verify_receipt(envelope: &Value) -> (VerificationResult, String, String)` |
| **Status** | Present — BLAKE3 batch hash + admission signature verification |
| **Gap** | Verifies existing receipts; does not generate receipts from algorithm runs |

### 2.4 Alignment-Based Conformance — MISSING

| | |
|---|---|
| **Paper** | Adriansyah 2014 PhD TU/e — A* in synchronous product |
| **PM4Py equiv** | `pm4py.algo.conformance.alignments.petri_net.algorithm` |
| **Status** | MISSING |
| **Types ready** | `pm-core/src/alignment.rs` — `AlignmentMove`, `Alignment`, `AlignmentCost` |
| **Gap** | No synchronous product construction, no A*, no optimal alignment |

### 2.5 ETConformance Precision — MISSING (type exists, algo absent)

| | |
|---|---|
| **Paper** | Munoz-Gama & Carmona 2010 |
| **PM4Py equiv** | `pm4py.algo.conformance.alignments.decomposed` |
| **Status** | MISSING |
| **Types ready** | `pm-core/src/precision.rs` — `EtcPrecisionResult`, `PrecisionScore` |
| **Gap** | `ConformanceResult::precision` is `Option<f64>` set manually; no escaping-edges algorithm |

### 2.6 Log Skeleton Conformance — MISSING

| | |
|---|---|
| **Paper** | Verbeek 2021 STTT |
| **PM4Py equiv** | `pm4py.algo.conformance.log_skeleton` |
| **Status** | MISSING |

---

## 3. Replay

### 3.1 Token Replay on DFG — Same as 2.1

### 3.2 Petri Net Token Replay — MISSING

| | |
|---|---|
| **Status** | MISSING — only DFG replay exists |
| **Types ready** | `PetriNet`, `FlatIncidenceMatrix`, `DenseIndex` all present |
| **Gap** | No state space traversal over Petri net marking |

---

## 4. Prediction

### 4.1 Remaining Time / Next Activity — MISSING

| | |
|---|---|
| **PM4Py equiv** | `pm4py.algo.prediction` |
| **Status** | MISSING — no prediction surface in wasm4pm-algos |

---

## 5. Query Evaluation

### 5.1 OCPQ — PRESENT (isolated)

| | |
|---|---|
| **File** | `crates/ocpq/src/lib.rs` |
| **Paper** | Küsters & van der Aalst arXiv:2506.11541v1 2025 |
| **Status** | Present — full runtime with `Binding`, `BindingBox`, `QueryTree`, `Constraint`, `evaluate_constraint` |
| **Gap** | Isolated crate — no integration with wasm4pm-algos pipeline; not connected to OCEL admission layer |

---

## PM4Py Coverage Comparison

| Capability | PM4Py | wasm4pm | Gap |
|---|---|---|---|
| DFG Discovery | Yes | Yes | Minor — no provenance |
| Inductive Miner | Yes | No | MISSING |
| POWL Discovery | Yes | No | MISSING |
| Alpha Miner | Yes | Partial | Partial |
| Heuristic Miner (net) | Yes | Partial (DFG only) | Partial |
| Log Skeleton | Yes | No | MISSING |
| OC-DFG | Yes | No | MISSING |
| Token Replay (DFG) | Yes | Yes | Minor |
| Token Replay (Petri) | Yes | No | MISSING |
| Alignment Conformance | Yes | No | MISSING |
| ETConformance Precision | Yes | No | MISSING |
| Log Skeleton Conformance | Yes | No | MISSING |
| OCPQ | No | Yes (isolated) | Integration gap |
| Provenance Receipts | No | Struct only | Generation gap |
| Typed Evidence | No | Compat only | Cross-crate gap |
