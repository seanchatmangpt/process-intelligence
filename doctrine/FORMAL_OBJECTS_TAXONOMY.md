# Formal Objects Taxonomy by Lifecycle Layer

This taxonomy enumerates all formal process objects by layer and assigns ownership to either
`wasm4pm-compat` (structure) or `wasm4pm` (execution). The boundary is the one-way door:
structure is admitted here; execution graduates there.

---

## L1: Event Structures

These are the raw and admitted forms of evidence about what happened in a process.
**All L1 structures are owned by wasm4pm-compat.**

| Object | Rust Location | Paper Grounding | Notes |
|---|---|---|---|
| `Event` | `src/eventlog.rs` | XES 1849-2023; van der Aalst 2011 | Core event: activity name + timestamp + lifecycle + resource |
| `Trace` | `src/eventlog.rs` | XES 1849-2023 | Ordered sequence of events for one case |
| `EventLog` | `src/eventlog.rs` | XES 1849-2023; van der Aalst 2011 | Collection of traces; base substrate for classic PM |
| `XesLog` | `src/xes.rs` | IEEE 1849-2023 | IEEE-standard flat event log with extension declarations |
| `XesTrace` | `src/xes.rs` | IEEE 1849-2023 | Case-centric trace; `CaseCentricMarker` seals flat vs. OC |
| `XesEvent` | `src/xes.rs` | IEEE 1849-2023; multi-perspective (van der Aalst 2011) | Event with attribute map, timestamp, lifecycle |
| `OcelEvent` | `src/ocel.rs` | OCEL 2.0 (2023); OCEL 1.0 (2020) | Object-centric event; no case-id; carries E2O links |
| `OcelObject` | `src/ocel.rs` | OCEL 2.0 (2023) | Named object instance with type, attributes, changes |
| `OcelLog` | `src/ocel.rs` | OCEL 2.0 (2023) | Root container: events + objects + links |
| `EventObjectLink` | `src/ocel.rs` | OCEL 2.0 (2023); OC-PM divergence/convergence (2020) | E2O link; resolves divergence and convergence structurally |
| `ObjectObjectLink` | `src/ocel.rs` | OCEL 2.0 (2023) | O2O link; enables object genealogy and dependency chains |
| `ObjectChange` | `src/ocel.rs` | OCEL 2.0 (2023) | Attribute value change on an object at a timestamped event |
| `ObjectInstance` | `src/ids.rs` | OCEL 2.0 (2023) | Zero-cost `#[repr(transparent)]` newtype; typed object identity |

**Graduation boundary for L1:** None. All L1 objects are structure-only and remain in compat.
Execution that reads L1 objects (replay, discovery, mining) graduates to wasm4pm.

---

## L2: Model Structures

These are the formal process models that describe lawful or discovered behavior.
**All L2 structures are owned by wasm4pm-compat (shapes). Discovery execution is owned by wasm4pm.**

| Object | Rust Location | Paper Grounding | Notes |
|---|---|---|---|
| `WfNetConst<SOUNDNESS>` | `src/petri.rs` | van der Aalst 1998; Murata 1989 | Workflow net with non-forgeable soundness witness |
| `PlaceToTransitionArc` | `src/petri.rs` | Murata 1989; Alpha Miner 2004; OC-Petri 2019 | Typed arc enforcing bipartite law at compile time |
| `TransitionToPlaceArc` | `src/petri.rs` | Murata 1989; Alpha Miner 2004; OC-Petri 2019 | Typed arc enforcing bipartite law at compile time |
| `IncidenceMatrix` W-/W+ | `src/petri.rs` | Murata 1989 §2 | Structure-only incidence matrix; no firing execution |
| `SeparableWfNet<SOUNDNESS>` | `src/petri.rs` | POWL 2.0 / WF-net decomposition (2026) | Separable WF-net subclass; WF-net→POWL conversion witness |
| `BpmnElement` | `src/bpmn.rs` | BPMN 2.0 OMG (2011); Weske (2012); Real-Life BPMN (2019) | BPMN 2.0 metamodel element |
| `GatewayKind` | `src/bpmn.rs` | BPMN 2.0 OMG (2011) | XOR/AND/OR as structural law, not runtime choice |
| `BpmnSubprocess` | `src/bpmn.rs` | BPMN 2.0 OMG (2011); Real-Life BPMN (2019) | Subprocess scope; boundary events |
| `EventKind` | `src/bpmn.rs` | BPMN 2.0 OMG (2011) | Start/Intermediate/End as distinct types |
| `ProcessTree` | `src/process_tree.rs` | Inductive Miner (2013); IMf (2014) | Block-structured tree; output shape of Inductive Miner family |
| `TypedLoopNode<ARITY>` | `src/process_tree.rs` | Inductive Miner (2013) | `Require<{ ARITY == 2 }>: IsTrue` enforces arity law |
| `ProcessOperator` | `src/process_tree.rs` | Inductive Miner (2013) | Sequence/ExclusiveChoice/Parallel/Loop as `ConstParamTy` |
| `PowlNodeKind` | `src/powl.rs` | POWL (2023); POWL 2.0 (2026) | StrictPartialOrder/OperatorNode/Transition/SilentTransition |
| `ChoiceGraphEdge` | `src/powl.rs` | POWL (2023); POWL 2.0 (2026) | Choice graph edge; distinct type from `OrderEdge` |
| `OrderEdge` | `src/powl.rs` | POWL (2023) | Partial-order edge; confusing with ChoiceGraphEdge is a compile error |
| `DeclareConstraint` | `src/declare.rs` | Declare (2006); Log Skeleton (2018) | Named template + activation/correlation condition |
| `DeclareTemplate` | `src/declare.rs` | Declare (2006) | `ConstParamTy` variant; each template is a distinct type |
| `Dfg` | `src/dfg.rs` | DFG Mining & Filtering (2019); OC-DFG (2020) | Directly-Follows Graph; structure only |
| `DfgNode` | `src/dfg.rs` | DFG Mining & Filtering (2019) | Activity node in DFG |
| `DfgEdge` | `src/dfg.rs` | DFG Mining & Filtering (2019) | Directly-follows arc; flat |
| `DfgWeight` | `src/dfg.rs` | DFG Mining & Filtering (2019) | Frequency or performance weight |
| `OcpqQuery` | `src/ocpq.rs` | OCPQ (2025); Process Querying Methods (2017) | Typed query over OCED structure |
| `OcpqResult` | `src/ocpq.rs` | OCPQ (2025) | Typed query result with violation sets |
| `WorkflowPattern` | `src/law.rs` | Workflow Patterns (2016); YAWL (2004) | 17 of 20 canonical WCP variants as `ConstParamTy` |

**Graduation boundary for L2:** Discovery algorithms (Alpha, Inductive Miner, Heuristics Miner,
POWL/ChoiceGraph mining, OC-DFG computation) all graduate to wasm4pm. The model shapes remain.

---

## L3: Analysis Structures

These carry the results of process analysis: conformance verdicts, performance metrics,
prediction targets, violations.
**Shapes owned by wasm4pm-compat. Computation owned by wasm4pm.**

| Object | Rust Location | Paper Grounding | Notes |
|---|---|---|---|
| `Metric<KIND, NUM, DEN>` | `src/conformance.rs` | Conformance Checking (2018); Alignments (2011) | `Between01<NUM,DEN>` bound enforced at compile time |
| `FitnessConst` | `src/conformance.rs` | Token Replay (2019); Alignments (2011) | Token-replay fitness formula shape |
| `PrecisionConst` | `src/conformance.rs` | ETC Precision (2010); Alignments (2011) | ETC precision / anti-alignment precision shape |
| `F1Const` | `src/conformance.rs` | Conformance Checking (2018) | Harmonic mean of fitness and precision |
| `AlignmentResult` | `src/conformance.rs` | Alignments (2011); Conformance Checking (2018) | Shape of alignment output; execution graduates |
| `ConformanceViolation` | `src/conformance.rs` | Conformance Checking (2018); Chicago TDD doctrine | Named structural violation record |
| `PredictionTarget` | `src/prediction.rs` | Predictive PM Survey (2019); Compliance-Aware PPM (2026) | What is being predicted (outcome/remaining-time/next-activity) |
| `DiagnosticRecord` | `src/diagnostic.rs` | van der Aalst 2011 | Structured diagnostic output |
| `OcpqRefusal` | `src/ocpq.rs` | OCPQ (2025) | Named refusal for invalid OCPQ constraint |
| `StrictViolation` | `src/strict.rs` | Blue River Dam doctrine | `law()` returns human-readable law name |

**Graduation boundary for L3:** Token replay execution, alignment computation (A* cost search),
ETC precision computation, prediction model inference all graduate to wasm4pm.
The metric shapes and violation record shapes remain in compat.

---

## L4: Lifecycle Structures

These govern the admission, refusal, loss accounting, and receipting of evidence throughout the
one-way door lifecycle.
**All L4 structures are owned by wasm4pm-compat.**

| Object | Rust Location | Purpose | Notes |
|---|---|---|---|
| `Evidence<T, State, W>` | `src/evidence.rs` | Universal typed carrier; `State` and `W` are `PhantomData` | State tags (`Raw`, `Admitted`, etc.) are different types |
| `Admission<T, W>` | `src/admission.rs` | The only public output of `Admit::admit()` | `Admitted` constructor is `pub(crate)` |
| `Refusal<R, W>` | `src/admission.rs` | Carries a specific named law as reason type `R` | Bare `InvalidInput` is a defect |
| `LossReport<From, To, Items>` | `src/loss.rs` | Every lossy projection emits this | Silent structure loss is a defect |
| `ProjectionName` | `src/loss.rs` | `&'static str` newtype; names the projection covenant | Required by `LossPolicy::AllowNamedProjection` |
| `LossPolicy` | `src/loss.rs` | `RefuseLoss \| AllowNamedProjection \| AllowLossWithReport` | Decided before loss occurs |
| `Receipt` | `src/receipt.rs` | Proof record emitted at closure | Replaces "log"; a log is not a receipt |
| `GraduationCandidate` | `src/graduation.rs` | Structure ready to cross the bridge to wasm4pm | Carries `GraduationReason`; currently one-sided bridge |
| `GraduationReason` | `src/graduation.rs` | `NeedsDiscovery \| NeedsConformanceExecution \| …` | Named reasons for graduation |
| `ProcessBoundary` | `src/strict.rs` | Declares boundary for strict admission/refusal audit | `fully_attested(kind, name)` constructor |

**Graduation boundary for L4:** `GraduationCandidate` is the bridge type. The bridge is
currently one-sided: wasm4pm has no intake function for `GraduationCandidate`. Closing this
gap is the highest-priority structural work outstanding.

---

## Ownership Summary

| Layer | Owned by compat | Owned by wasm4pm |
|---|---|---|
| L1 Event structures | All shapes | All execution over L1 shapes |
| L2 Model structures | All shapes | All discovery and mining algorithms |
| L3 Analysis structures | All result shapes and metric bounds | All conformance execution and prediction inference |
| L4 Lifecycle structures | All lifecycle types including `GraduationCandidate` | Intake of `GraduationCandidate` (gap) |

The product is CodeManufactory; RevOps is merely proof that CodeManufactory works.
