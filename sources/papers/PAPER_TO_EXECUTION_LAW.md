# Paper to Execution Law Obligations

**Date:** 2026-05-31
**Source:** wasm4pm-compat PAPER_COVERAGE_LEDGER.md (81 papers)
**Purpose:** For each paper: what algorithms does it introduce? These graduate to wasm4pm.
Format: algorithm name | paper | input types (in wasm4pm-compat) | output types (in wasm4pm-compat) | wasm4pm coverage YES/NO/PARTIAL

---

## Execution Law Table

All algorithms listed here must execute in wasm4pm, not in wasm4pm-compat. The input and output TYPE SHAPES are defined in wasm4pm-compat. The computation is absent from wasm4pm-compat by design.

| Algorithm | Paper | Citation | Input Types (compat) | Output Types (compat) | wasm4pm Coverage |
|---|---|---|---|---|---|
| Alpha Miner | Workflow Mining: Discovering Process Models from Event Logs | van der Aalst, Weijters, Maruster 2004 | `EventLog`, `XesTrace` | `WfNetConst<SOUNDNESS>`, `PlaceToTransitionArc`, `TransitionToPlaceArc` | NO |
| Inductive Miner (IM) | Inductive Miner: Discovering Block-Structured Process Models | Leemans, Fahland, van der Aalst 2013 | `EventLog`, directly-follows relation | `ProcessTree`, `TypedLoopNode<2>`, `ProcessOperator` | NO |
| Inductive Miner — Infrequent (IMf) | IMf / Infrequent IM | Leemans et al. | `EventLog`, frequency threshold param | `ProcessTree` (filtered) | NO |
| Scalable IM | Scalable Process Discovery | Leemans 2022 | `EventLog` (large-scale) | `ProcessTree` | NO |
| Heuristics Miner | Heuristics Miner (Weijters & Ribeiro 2011) | Weijters, Ribeiro 2011 | `EventLog` | `CausalNet` (MISSING type), `DependencyMeasure` (MISSING type) | NO |
| Token-Based Replay | Token Replay conformance | Rozinat & van der Aalst | `EventLog`, `WfNetConst<SOUNDNESS>` | `FitnessConst<NUM, DEN>`, `Metric<Fitness, NUM, DEN>` | NO |
| Alignment-Based Conformance | Aligning Observed and Modeled Behavior | Adriansyah et al. 2011 | `EventLog`, `WfNetConst<SOUNDNESS>` | `FitnessConst`, `PrecisionConst`, `F1Const` (all `Between01`-bounded) | NO |
| ETC Precision | Measuring Precision of Event Logs | Muñoz-Gama & Carmona | `EventLog`, `WfNetConst<SOUNDNESS>` | `PrecisionConst<NUM, DEN>` | NO |
| OC-DFG Discovery | OC-DFG (van der Aalst 2019) | van der Aalst 2019 | `OcelLog` | `Dfg` | NO |
| OC-Petri Net Discovery | OC-Petri Net Discovery | van der Aalst et al. | `OcelLog` | `WfNetConst<SOUNDNESS>` with OC arc inscriptions | NO |
| Log Skeleton Derivation | Log Skeleton: An Accurate Conformance Checker | Verbeek 2019 | `EventLog` | `DeclareConstraint` set (always-after, always-before, never-together, equivalence, directly-follows) | NO |
| Conformance Checking (log skeleton) | Log Skeleton | Verbeek 2019 | `EventLog`, `DeclareConstraint` set | `Metric<Fitness, NUM, DEN>` | NO |
| Concept Drift Detection | Concept Drift Detection (Ostovar et al.) | Ostovar et al. | `EventLog` | `DriftKind` (MISSING type: Sudden/Gradual/Recurring/Incremental) | NO |
| Prefix-Based Prediction | Predictive Business Process Monitoring — LSTM | Tax et al. 2017 | `EventLog`, `PrefixLength` (MISSING type) | `PredictionTarget` | NO |
| Predictive PM (general) | Predictive PM Methods Survey | Di Francescomarino et al. 2017 | `EventLog`, `PrefixLength` (MISSING type) | `PredictionTarget` | NO |
| Time-Aware Prediction | Time-Aware Predictive Monitoring | Polato et al. 2018 | `EventLog`, `PredictionHorizon` (MISSING type) | `PredictionTarget` | NO |
| Temporal Profile Conformance | Temporal Profile Conformance | Stertz et al. | `EventLog`, `TemporalProfile<ActivityPair>` | `ZScore` (MISSING type), `TimeDelta` (MISSING type) | NO |
| MagTempMiner | MagTempMiner | Maggi, Bose, van der Aalst | `EventLog` | `TimedResponse`, `TimedPrecedence`, `TimedChainResponse` variants in `DeclareTemplate` (MISSING variants) | NO |
| WF-net Reduction / Soundness Check | The Application of Petri Nets to Workflow Management | van der Aalst 1998 | `WfNetConst<false>` | `WfNetConst<true>` via `WfNetSoundnessWitness` | NO |
| Petri Net Unfolding | Petri Net Unfoldings | McMillan / Esparza | `WfNetConst<SOUNDNESS>` | `BranchingProcess` (MISSING type), `UnfoldingPrefix` (MISSING type) | NO |
| Anti-Alignment Discovery | Anti-Alignments | de Weerdt et al. | `EventLog`, `WfNetConst<SOUNDNESS>` | anti-alignment trace shapes | NO |
| Mining Roles | Mining Roles | van der Aalst et al. | `EventLog` with resource attributes | `RoleAttribute` (MISSING type), `ResourceGrouping` (MISSING type) | NO |
| Process Performance Analysis | Process Performance Analysis | van der Aalst et al. | `EventLog`, `WfNetConst<SOUNDNESS>` | `PerformanceMetric<KIND, UNIT>` (MISSING type) | NO |
| OCEL Extraction / Materialization | OCEL Extraction and Materialization | Ghahfarokhi et al. | Relational database / ERP | `OcelLog` via `ExtractionProjection<POLICY>` (MISSING type) | NO |
| Event Abstraction | Event Abstraction | Mannhardt et al. | `EventLog` (low-level) | `EventLog` with `AbstractionLevel` (MISSING type) | NO |
| Approximate Semantic Process Querying | Approximate Semantic Process Querying | Polyvyanyy et al. | `OcpqQuery`, process model | `SimilarityScore<NUM, DEN>` (MISSING type, Between01-bounded) | NO |
| OCPQ Constraint Evaluation | OCPQ: Object-Centric Process Querying | Küsters & van der Aalst 2025 | `OcpqQuery<ObjectTypes, EventTypes>`, `OcelLog` | `OcpqResult` with `ConstraintViolation<ObjType, EvType>` (MISSING full typing) | NO |
| YAWL Worklet Dispatch | YAWL Technical Manual v5 | YAWL Foundation 2023 | YAWL specification (XSD), task decomposition | worklet service response shapes | NO |
| PMAx Engineer/Analyst Execution | PMAx: An Agentic Framework | Antonov et al. 2026 | schema abstraction over `EventLog`/`OcelLog` | PM4Py artifact shapes, discovery results | NO |
| OCED Discovery | No AI Without PI! | van der Aalst 2025 | `OcelLog` | discovered OC-Petri net shapes | NO |

---

## GraduationReason Variants Mapped to Papers

The following `GraduationReason` variants (defined in `src/graduation.rs`) map to specific algorithm families:

| GraduationReason Variant | Algorithm Family | Papers |
|---|---|---|
| `NeedsDiscovery` | All process discovery algorithms | Alpha Miner, IM, IMf, Scalable IM, Heuristics Miner, OC-DFG, OC-Petri Net Discovery, OCED Discovery |
| `NeedsConformanceExecution` | All conformance checking algorithms | Token Replay, Alignments, ETC Precision, Log Skeleton conformance, YAWL Interface B, PMAx conformance |
| `NeedsObjectCentricQueryExecution` | OCPQ evaluation, OCED performance | OCPQ, OC-DFG frequency/time aggregation, Process Cubes slicing |
| `NeedsReceipts` | Provenance chain generation | YAWL custom service, PMAx artifact grounding, No AI Without PI! AI output grounding |
| `RebuildingProcessMiningLocally` | Full PM pipeline reconstruction | PMAx Engineer agent, YAWL engine, OCPM compliance pipeline |

---

## Input/Output Type Coverage Assessment

### Types defined in wasm4pm-compat that are algorithm inputs/outputs (ready for wasm4pm)

- `EventLog`, `XesLog`, `XesTrace`, `XesEvent` — event log inputs
- `OcelLog`, `OcelEvent`, `OcelObject`, `EventObjectLink`, `ObjectObjectLink` — OCEL inputs
- `WfNetConst<SOUNDNESS>`, `PlaceToTransitionArc`, `TransitionToPlaceArc` — Petri net I/O
- `ProcessTree`, `TypedLoopNode<2>`, `ProcessOperator` — process tree outputs
- `DeclareConstraint`, `DeclareTemplate` — constraint model outputs
- `FitnessConst<N,D>`, `PrecisionConst<N,D>`, `F1Const<N,D>` — conformance metric outputs
- `Dfg` — directly-follows graph output
- `OcpqQuery`, `OcpqResult` — query I/O shapes

### Types MISSING from wasm4pm-compat that block algorithm graduation

- `CausalNet`, `DependencyMeasure` — Heuristics Miner output
- `DriftKind` — Concept Drift Detection output
- `PrefixLength`, `PredictionHorizon` — prediction algorithm params
- `TimeDelta`, `ZScore` — Temporal Profile output
- `BranchingProcess`, `UnfoldingPrefix` — Petri Net Unfolding output
- `RoleAttribute`, `ResourceGrouping` — Mining Roles output
- `PerformanceMetric<KIND, UNIT>` — Process Performance output
- `ExtractionProjection<POLICY>` — OCEL Extraction output
- `AbstractionLevel` — Event Abstraction marker
- `SimilarityScore<NUM, DEN>` — Approximate Querying output
- `StochasticArcWeight<NUM, DEN>`, `ImmediateTransition`, `TimedTransition` — Stochastic net params

**Total missing output types: 14 named types.** Until these exist in wasm4pm-compat, the corresponding algorithms cannot be fully typed at the wasm4pm graduation boundary.
