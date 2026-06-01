# WITNESS LATTICE — wasm4pm-compat

**Source:** /Users/sac/wasm4pm-compat/src/witness.rs
**Count:** 40 witness markers defined via `witness_marker!` macro
**Families:** Standard (6) | Paper (28) | ApiGrammar (2) | RustLaw (1) | InternalBridge (1) | Paper (no-year) (2+)

---

## What Witnesses Are

A witness is an uninhabited empty enum (`pub enum Ocel20 {}`) used purely as a `PhantomData<W>` type parameter in:
- `Evidence<T, State, W>` — the carrier
- `Admission<T, W>` — the admitted verdict
- `Refusal<R, W>` — the refused verdict

Because `W` is a type parameter, `Admission<T, Ocel20>` and `Admission<T, Xes1849>` are **different types**. The compiler rejects any attempt to pass one where the other is expected — no runtime check needed.

## What Witnesses Are Not

Witnesses never validate. They name the authority; the checking belongs to the wasm4pm engine. This crate is structure-only: a value tagged with a witness must graduate to wasm4pm when verification against that authority is actually required.

## The `Witness` Trait

```rust
pub trait Witness {
    const KEY: &'static str;   // stable machine key, e.g. "ocel-2.0"
    const FAMILY: WitnessFamily;
    const TITLE: &'static str; // human-facing title
    const YEAR: Option<u16>;   // publication year if dated
}
```

## `WitnessFamily` Taxonomy

| Family | Meaning |
|---|---|
| `Standard` | Published interchange/data standard |
| `Paper` | Academic paper defining a model or algorithm |
| `ApiGrammar` | API grammar a consumer must speak to interoperate |
| `RustLaw` | Rust-language law this crate enforces structurally |
| `InternalBridge` | Bridge toward graduation to wasm4pm |

---

## Full Witness Inventory

### Standards (WitnessFamily::Standard)

| Type | KEY | Year | What it names | Confusion prevented |
|---|---|---|---|---|
| `Ocel20` | `ocel-2.0` | 2023 | OCEL 2.0 object-centric event log standard | Cannot be confused with XES at type level |
| `Xes1849` | `xes-1849-2016` | 2016 | IEEE 1849-2016 XES interchange standard | Cannot be confused with OCEL |
| `XesLifecycleExt` | `xes-lifecycle-extension` | 2016 | XES lifecycle:transition alphabet authority | Distinct from full XES standard — sub-authority |
| `XesConceptExt` | `xes-concept-extension` | 2016 | XES concept:name authority | Distinct from XES standard and lifecycle ext |
| `OcelObjectType` | `ocel-object-type` | 2023 | OCEL 2.0 individual object-type namespace | Sub-authority of Ocel20; not the whole standard |
| `OcelEventType` | `ocel-event-type` | 2023 | OCEL 2.0 event-type (activity) namespace | Sub-authority of Ocel20; not the whole standard |
| `OcelAttributeType` | `ocel-attribute-type` | 2023 | OCEL 2.0 attribute-domain namespace | Sub-authority of Ocel20 |

### Papers (WitnessFamily::Paper) — Dated

| Type | KEY | Year | What it names | Confusion prevented |
|---|---|---|---|---|
| `PowlPaper` | `powl-paper` | 2023 | POWL: Partially Ordered Workflow Language (Kourani & van Zelst) | Distinct from WF-net soundness, YAWL |
| `ObjectCentricPetriNetPaper` | `oc-petri-net-paper` | 2020 | Discovering OC-Petri-nets (van der Aalst & Berti) — the *discovery* algorithm | Distinct from OcPetriNets (notation authority) |
| `WfNetSoundnessPaper` | `wfnet-soundness-paper` | 1998 | The Application of Petri Nets to Workflow Management (van der Aalst soundness) | Distinct from AlignmentPaper, OcpqPaper |
| `OcpqPaper` | `ocpq-paper` | 2024 | Object-Centric Process Querying | Distinct from WF-net soundness, conformance alignment |
| `DeclareFamily` | `declare-family` | 2007 | Declare constraint model family | Distinct from DeclareConstraints (individual constraint surface) |
| `PredictiveMonitoringFamily` | `predictive-monitoring-family` | 2018 | Predictive process monitoring family | Distinct from conformance and discovery witnesses |
| `YawlPaper` | `yawl-paper` | 2004 | YAWL: Yet Another Workflow Language (van der Aalst & ter Hofstede) — routing constructs, cancellation, MI tasks | Distinct from WfNetSoundnessPaper |
| `SeparableWfNetPaper` | `separable-wfnet-paper` | 2026 | Hierarchical Decomposition of Separable WF-nets (Kourani et al.) — separability subclass and WF-net→POWL 2.0 theorem | Distinct from WfNet2Powl (conversion authority) and PowlPaper (language authority) |
| `WorkflowPatternsPaper` | `workflow-patterns-paper` | 2016 | Workflow Patterns: The Definitive Guide (Russell, van der Aalst & ter Hofstede) — WP-1 through WP-43+ | Distinct from YAWL, BPMN, Declare |
| `InductiveMiner` | `inductive-miner` | 2013 | Inductive Miner (Leemans, Fahland & van der Aalst) — discovery algorithm family | Distinct from AlphaMiner (different algorithm, different guarantees) |
| `DeclareConstraints` | `declare-constraints` | 2006 | Declare constraint-template language (Pesic & van der Aalst) — individual constraint templates | Distinct from DeclareFamily (whole model) |
| `AlignmentPaper` | `alignment-paper` | 2008 | Alignment-Based Conformance Checking (van Dongen et al.) | Distinct from WfNetSoundnessPaper (structural soundness) and OcpqPaper (querying) |
| `OcPetriNets` | `oc-petri-nets` | 2020 | OC-Petri-net *notation* authority (model structure) | Distinct from ObjectCentricPetriNetPaper (discovery algorithm output) |
| `LogSkeleton` | `log-skeleton` | 2018 | Log Skeleton (Verbeek & Leemans) — six relations mined from log | Distinct from DeclareConstraints (different relation vocabulary) |
| `AlphaMiner` | `alpha-miner` | 2004 | Alpha Algorithm (van der Aalst, Weijters & Maruster) — causal-matrix WF-net discovery | Distinct from InductiveMiner |
| `ProcessCubePaper` | `process-cube-paper` | 2013 | Process Cubes (van der Aalst, APBC 2013) | Distinct from OCEL/XES |
| `OperationalView` | `process-cube-operational-view` | 2013 | Process Cube operational (execution traces) projection | Distinct from AnalyticalView and AggregationView |
| `AnalyticalView` | `process-cube-analytical-view` | 2013 | Process Cube analytical (discovered model) projection | Distinct from OperationalView and AggregationView |
| `AggregationView` | `process-cube-aggregation-view` | 2013 | Process Cube aggregation (statistical summary) projection | Distinct from OperationalView and AnalyticalView |
| `WfNet2Powl` | `wfnet-to-powl` | 2026 | WF-net to POWL 2.0 conversion authority (Kourani, Park & van der Aalst) | Distinct from SeparableWfNetPaper (separability) and PowlPaper (language) |
| `TimeAwareWitness` | `time-aware-witness` | 2020 | Temporal ordering relations established | Distinct from TemporalProfileWitness (full statistical profile) |
| `TemporalProfileWitness` | `temporal-profile-witness` | 2020 | Full temporal profile (AVG/STD per activity pair — Stertz et al.) | Distinct from TimeAwareWitness |
| `ControlFlowPerspectiveWitness` | `cf-perspective` | 2016 | Control-flow perspective (Mannhardt et al.) | Distinct from Data/Resource/Time perspectives |
| `DataPerspectiveWitness` | `data-perspective` | 2016 | Data perspective (Mannhardt et al.) | Distinct from CF/Resource/Time perspectives |
| `ResourcePerspectiveWitness` | `resource-perspective` | 2016 | Resource perspective (Mannhardt et al.) | Distinct from CF/Data/Time perspectives |
| `TimePerspectiveWitness` | `time-perspective` | 2016 | Time perspective (Mannhardt et al.) | Distinct from CF/Data/Resource perspectives |

### Papers (WitnessFamily::Paper) — Undated

| Type | KEY | Year | What it names |
|---|---|---|---|
| `ReceiptFamily` | `receipt-family` | None | Receipt-shaped provenance-bearing evidence |
| `DivergenceWitness` | `oc-pm-divergence` | None | OC-PM divergence detection authority (paper #49) |
| `ConvergenceWitness` | `oc-pm-convergence` | None | OC-PM convergence detection authority (paper #49) |
| `StreamingEvidenceWitness` | `streaming-evidence` | None | Streaming (online) collection context — partial/windowed/out-of-order |
| `CausalConsistencyWitness` | `causal-consistency` | None | Cross-object causal ordering verified — no cycles, no contradictions |
| `CrossLogCorrelationWitness` | `cross-log-correlation` | None | Multi-log provenance — merged result from correlating two source logs |

### API Grammars (WitnessFamily::ApiGrammar)

| Type | KEY | Year | What it names |
|---|---|---|---|
| `Pm4pyApiGrammar` | `pm4py-api-grammar` | None | pm4py API call grammar a consumer must speak to interoperate |
| `PmaxConsumerGrammar` | `pmax-consumer-grammar` | None | pmax-style consumer grammar a downstream caller must satisfy |

### Rust Laws (WitnessFamily::RustLaw)

| Type | KEY | Year | What it names |
|---|---|---|---|
| `RustTypestateLaw` | `rust-typestate-law` | None | States tracked at type level; illegal transitions unrepresentable |

### Internal Bridge (WitnessFamily::InternalBridge)

| Type | KEY | Year | What it names |
|---|---|---|---|
| `Wasm4pmBridge` | `wasm4pm-bridge` | None | Bridge toward the wasm4pm execution engine (graduation) |

---

## Confusion Prevention — Selected Type-Level Enforced Distinctions

The following pairs are proven incompatible at the type level via compile-fail fixtures:

| Fixture | Confusion prevented |
|---|---|
| `evidence_wrong_witness_ocel_as_xes.rs` | `Evidence<T, _, Ocel20>` ≠ `Evidence<T, _, Xes1849>` |
| `evidence_wrong_witness_xes_as_ocel.rs` | `Evidence<T, _, Xes1849>` ≠ `Evidence<T, _, Ocel20>` |
| `witness_xes_as_wfnet.rs` | XES witness ≠ WF-net soundness witness |
| `witness_ocel_as_powl.rs` | OCEL witness ≠ POWL witness |
| `witness_pm4py_as_pmax.rs` | pm4py API grammar ≠ pmax consumer grammar |
| `witness_declare_as_ocpq.rs` | Declare family ≠ OCPQ paper |
| `witness_yawl_as_inductive_miner.rs` | YAWL ≠ Inductive Miner |
| `witness_receipt_as_wasm4pm_bridge.rs` | Receipt family ≠ graduation bridge |
| `formats_envelope_wrong_witness.rs` | Format envelope witness type enforced |
| `receipt_wrong_witness_marker.rs` | Receipt witness type enforced |
| `compliance_witness_wrong_target.rs` | Compliance witness target enforced |

---

## Paper Coverage by Family

- **9 standards bodies/grammars** covered: OCEL 2.0, XES (IEEE 1849), two OCEL sub-namespaces, two XES extensions, pm4py API, pmax API.
- **22+ papers** covered spanning 1998–2026: Murata, van der Aalst (WF-net soundness, OC-Petri-nets, Process Cubes, Alpha Miner), Weijters (Heuristics Miner), Pesic & van der Aalst (Declare), Kourani (POWL, Separable WF-nets), Leemans (Inductive Miner), van Dongen (Alignment), Russell et al. (Workflow Patterns), Verbeek (Log Skeleton), Mannhardt (Multi-perspective), Stertz (Temporal Profile), OC-PM divergence/convergence (#49).
- **1 Rust law** (typestate invariant).
- **1 internal bridge** (graduation to wasm4pm).

The gap: No witness covers the 2505.07052 POWL paper directly — it is anchored through `PowlPaper` (Kourani & van Zelst 2023) and `SeparableWfNetPaper` (2026) as separate authorities.
