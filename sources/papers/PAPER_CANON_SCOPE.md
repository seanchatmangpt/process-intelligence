# Paper Canon Classification Scope

**Authority:** wasm4pm-compat PAPERS_METHODOLOGY.md
**Corpus:** 81 papers tracked in PAPER_COVERAGE_LEDGER.md
**Date:** 2026-05-31

---

## Classification Framework (from PAPERS_METHODOLOGY.md)

Five mutually exclusive statuses govern every paper in the corpus.

### COVERED_BY_TYPE
The paper introduces formal objects (types, structures, laws) directly encoded as zero-cost Rust types in wasm4pm-compat.
**Test:** point to a `src/*.rs` file with types named after the paper's formal objects.
**Criteria for assignment:**
- Paper introduces a named formal object (event log, Petri net, process tree, constraint, metric)
- That object exists in `src/` as a Rust type with the same name or semantically equivalent name
- The type enforces the paper's structural law at compile time (via `PhantomData`, const-generics, or sealed traits)

### COVERED_BY_GRADUATION_BOUNDARY
The paper's primary contribution is an algorithm (discovery, conformance checking, replay, alignment, prediction, query evaluation). Output shapes may be typed here; the algorithm itself graduates to wasm4pm.
**Test:** does `docs/GRADUATION_BOUNDARIES.md` name this paper's algorithm as graduating?
**Criteria for assignment:**
- Primary contribution is a procedure, not a structural definition
- The procedure's inputs and outputs have shape types in `src/`
- The computation itself (A* search, inductive cuts, token replay) is not present and should not be

### PARTIAL_WITH_REASON
Some formal objects are typed; others are absent or misclassified.
**Test:** named types in `src/` for SOME but not ALL formal objects.
**Criteria for assignment:**
- At least one key formal object from the paper exists as a Rust type
- At least one key formal object is absent (not a deliberate graduation boundary)
- The reason for absence must be named — not "TODO" but a specific type that is missing

### DUPLICATE_OR_BACKGROUND
The paper repeats or summarises content from another ledgered paper, or provides background context without new formal objects.
**Test:** does another ledger row cover the same formal objects with a more primary citation?
**Criteria for assignment:**
- Another paper in the ledger is the canonical primary citation for the same formal objects
- OR the paper is a survey/textbook that introduces no novel formal object

### OUT_OF_SCOPE_WITH_REASON
The paper does not introduce process-mining formal objects.
**Test:** does the paper introduce any formal object that could be a zero-cost Rust type? If yes, cannot be OUT_OF_SCOPE.
**Criteria for assignment:**
- Domain is entirely outside BPM/process mining (HPC, HRI, DevOps, etc.)
- OR the paper is a user manual with no formal object definitions
- The reason must be specific — not "not relevant" but a named domain difference

---

## Current Distribution (81 papers)

| Status | Count | Description |
|---|---|---|
| `COVERED_BY_TYPE` | 18 | Core formal objects reified as Rust types with type-law receipts |
| `COVERED_BY_GRADUATION_BOUNDARY` | 39 | Structural shapes in compat; algorithm execution graduates to wasm4pm |
| `PARTIAL_WITH_REASON` | 10 | Partially covered; named type gaps remain as active obligations |
| `DUPLICATE_OR_BACKGROUND` | 4 | Redundant with primary citation; no new type-law burden |
| `OUT_OF_SCOPE_WITH_REASON` | 10 | No process-mining type-law relevance; named domain reasons |

---

## Primary Law Sources (COVERED_BY_TYPE — 18 papers)

These papers are the canonical sources for type law in wasm4pm-compat. Each has a direct Rust type surface.

| Paper | Formal Law | Rust Module |
|---|---|---|
| OCEL 2.0 Specification (van der Aalst et al. 2023) | OcelLog/Event/Object/EventObjectLink/ObjectObjectLink | `src/ocel.rs` |
| XES IEEE 1849-2023 | XesLog/Trace/Event/CaseCentricMarker/XesExtension | `src/xes.rs` |
| YAWL: Yet Another Workflow Language (2004) | WfNetConst soundness, workflow patterns | `src/petri.rs` |
| The Application of Petri Nets to Workflow Management (1998) | WF-net soundness criterion (3 conditions), WfNetSoundnessWitness | `src/petri.rs`, `src/witness.rs` |
| Petri Nets: Properties, Analysis and Applications (1989) | Bipartite arc law, incidence matrix W-/W+, enabling condition | `src/petri.rs`, `src/nightly_foundry.rs` |
| Object-Centric Behavioral Constraints (2019) | OC-Petri net typed arcs, soundness | `src/petri.rs` |
| POWL (Kourani & van der Aalst) | TreeProjectable, ChoiceGraph, WfNet2PowlWitness | `src/powl.rs` |
| Hierarchical Decomposition of Separable Workflow-Nets (2026) | SeparableWfNet, WfNet2PowlWitness conversion | `src/powl.rs`, `src/petri.rs` |
| Declare: Full Support for Loosely-Structured Processes (2006) | DeclareConstraint, DeclareTemplate, DeclareWitness | `src/declare.rs` |
| Real-Life BPMN 4th edition (2019) | BPMN gateway/pool/lane/subprocess structure | `src/bpmn.rs` |
| BPMN 2.0 OMG Specification | BpmnElement/GatewayKind/EventKind metamodel | `src/bpmn.rs` |
| Workflow Patterns: The Definitive Guide (2016) | WorkflowPattern as ConstParamTy (17 of 20 patterns) | `src/law.rs` |
| Event Logs and Their Metadata (van der Aalst) | EventLog metadata shapes | `src/eventlog.rs` |
| Modeling Business Processes (van der Aalst & Stahl 2011) | WF-net soundness + BPMN vocabulary | `src/petri.rs`, `src/bpmn.rs` |
| Process Mining 2nd ed. (van der Aalst 2016) | All structural shapes across canon modules | Multiple `src/*.rs` |
| BPMN Miner (Conforti et al. 2015) | BPMN pool/lane/subprocess structural shapes | `src/bpmn.rs` |
| OC-PM precursor (van der Aalst 2013) | Subsumed by OCEL 1.0 and 2.0 | `src/ocel.rs` |
| Process Cubes (van der Aalst 2013) | Multi-dimensional event log slicing | `src/eventlog.rs` |

---

## Algorithmic Sources (COVERED_BY_GRADUATION_BOUNDARY — key 39 papers)

These papers drive the wasm4pm execution engine. Their output shapes are typed in wasm4pm-compat; their computation logic must graduate.

**Discovery algorithms:** Alpha Miner, Inductive Miner, IMf, Heuristics Miner, OC-DFG, BPMN Miner, Scalable IM
**Conformance:** Token Replay, Alignments, ETC Precision, Conformance Checking Book, Burattin 2017
**Querying:** OCPQ execution, Approximate Semantic Process Querying
**Prediction:** Predictive PM survey, LSTM (Tax 2017), Polato 2018, Di Francescomarino 2017
**Temporal:** MagTempMiner, Temporal Profile Conformance
**Other:** PM4Py, PMAx, YAWL Technical Manual, No AI Without PI!, Log Skeleton derivation

---

## Active Partial Obligations (PARTIAL_WITH_REASON — 10 papers)

These papers have acknowledged type-law gaps. Each gap is a named missing type, not a vague TODO.

| Paper | Missing Types |
|---|---|
| Compliance-Aware PPM (#1) | `ComplianceConstraintWitness<W>`, `Between01` compliance score |
| Separable WF-nets (#3) | Compile-fail fixture for forged non-separable conversion |
| XES→OCED (#5) | `XesToOcedProjection` with explicit `LossPolicy` and `LossReport` |
| OCPQ (#6) | `ObjectTypeSet`/`EventTypeSet` const params; `ConstraintViolation<ObjType, EvType>` |
| Heuristics Miner (#45) | `CausalNet`, `CausalBinding`, `DependencyMeasure as Between01` |
| Multi-Perspective PM (#48) | `ResourcePerspective`, `DataPerspective` as PhantomData markers |
| OC-PM Divergence/Convergence (#49) | `DivergenceWitness`, `ConvergenceWitness` in `src/witness.rs` |
| Process Querying Methods (#51) | `ProcessQueryWitness`, full `TemporalPredicate` axiom coverage |
| Stochastic Conformance (#57) | `StochasticArcWeight<NUM,DEN>`, `ImmediateTransition`, `TimedTransition` |
| Mannhardt Multi-Perspective 2016 (#71) | `ResourcePerspective`, `DataPerspective` typed namespaces |

---

## Out-of-Scope Papers (10 papers)

| Paper | Reason |
|---|---|
| Hands-On Python for DevOps | Python DevOps book — no process-mining objects |
| How Anthropic Teams Use Claude Code | Internal tool report — no process-mining objects |
| Process mining for healthcare | Domain application — all objects already in canon |
| sAirflow serverless | HPC/serverless infrastructure — different workflow paradigm |
| Why Automate This? | HRI/sociology — no formal process objects |
| Workflows Community Summit 2024 | Scientific/HPC workflows — different domain |
| YAWL User Manual 5.1 | End-user manual — no formal definitions |
| M2M Transformation of Workflow Specs | Toolchain paper — BPMN target already in canon |
| Procedure Model for Building Knowledge Graphs | KG construction — no process-mining objects |
| RDFGraphGen | SHACL-based RDF generation — no process-mining relevance |
