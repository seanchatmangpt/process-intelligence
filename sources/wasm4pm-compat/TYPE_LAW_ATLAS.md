# TYPE LAW ATLAS — wasm4pm-compat (37 modules)

**Source:** /Users/sac/wasm4pm-compat/src/
**Gate:** 398 compile-fail fixtures + 406 compile-pass fixtures (ALIVE gate)
**Invariant:** Structure-only. No engine logic. Everything graduates to wasm4pm.

---

## Layer 1: The Type Law Kernel

### `law.rs`
The compile-time law kernel. All other modules import from here.

Formal objects defined:
- `Assert<const OK: bool>` / `IsTrue` trait / `Require<OK>` alias — the "must be true" compile-time gate. Any `where Require<{ EXPR }>: IsTrue` becomes a compile error when EXPR is false. Requires `generic_const_exprs`.
- `ConditionCell<BITS>` — Blue River Dam "Need9 means split" law. At most 8 primary condition bits. `ConditionCell<9>` does not compile.
- `Between01<NUM, DEN>` — rational metric provably in [0,1] at the type level. `Between01<2,1>` does not compile.
- `EvidenceMode` (ConstParamTy enum) — const-generic lifecycle stage (Raw, Parsed, Admitted, Refused, Projected, Exportable, Witnessed, Receipted).
- `ProjectionLaw` (ConstParamTy enum) — Lossless | NamedProjection | LossReportRequired | RefuseLoss.
- `AdmissionLaw` (ConstParamTy enum) — Unchecked | ParsedOnly | WitnessRequired | RefusalRequired | LossPolicyRequired | ReceiptRequired.
- `FormatKindConst` (ConstParamTy enum) — OcelJson | OcelXml | OcelSqlite | OcelNdjson | XesXml | BpmnXml | Pnml | PowlJson | CompatNative.
- `EndpointKind` (ConstParamTy enum) — Place | Transition. Enforces bipartite arc law in const-generic position.
- `ArcDirectionConst` (ConstParamTy enum) — PlaceToTransition | TransitionToPlace.
- `SoundnessState` (ConstParamTy enum) — Unknown | Claimed | Witnessed. Non-forgeable; `WfNetConst<{Witnessed}>` requires a `SoundnessProof`.
- `PowlProjectionState` (ConstParamTy enum) — Unknown | ProcessTreeProjectable | ExceedsProcessTree | RefusedProjection.
- `QualityMetricKind` (ConstParamTy enum) — Fitness | Precision | F1 | Generalization | Simplicity.

Paper anchoring: Blue River Dam covenant (Need9 law), general PM quality metric vocabulary.

---

## Layer 2: Lifecycle Machinery

### `state.rs`
Seven typestate tokens as uninhabited empty enums, all sealed by `EvidenceState` trait.

Formal objects:
- `Raw` — untrusted external input, freely constructible.
- `Parsed` — structurally well-formed, not yet admitted.
- `Admitted` — crossed a named boundary law via `Admit::admit()`. Constructor is `pub(crate)`.
- `Refused` — terminal. Carries a specific named law reason. No free conversion to any other state.
- `Projected` — produced by a named, accounted lossy projection.
- `Exportable` — cleared to leave the crate boundary.
- `Receipted` — sealed in a provenance-bearing receipt envelope.

The sealed `EvidenceState` trait prevents downstream crates from inventing arbitrary lifecycle positions.

### `evidence.rs`
The universal carrier. `Evidence<T, State, W>` bundles: a value T, a lifecycle State tag (PhantomData), a Witness W tag (PhantomData).

Formal objects:
- `Evidence<T, State, W>` — zero-cost carrier. `Evidence<T, Raw, W>` and `Evidence<T, Admitted, W>` are different types.
- `RawOcelEvidence<T>` / `AdmittedOcelEvidence<T>` / `RawXesEvidence<T>` / `AdmittedXesEvidence<T>` / `ReceiptedEvidence<T, W>` — convenience aliases.
- `Evidence::raw(v)` — the only freely-constructible entry point.
- Transition chain: `into_parsed`, `into_admitted` (via Admit), `into_projected`, `into_exportable`, `into_receipted`.

The `Admitted` constructor is `pub(crate)` — the only public path is through `Admit::admit()`.

### `admission.rs`
The first-class boundary verdict surface. The only sanctioned `Raw → Admitted` path.

Formal objects: See ADMISSION_REFUSAL_MAP.md for full detail.

### `witness.rs`
Witness markers — 40+ empty enum types naming papers, standards, and laws. See WITNESS_LATTICE.md.

---

## Layer 3: Process Model Shapes

### `petri.rs`
Petri net, WF-net, and OC-Petri-net structural shapes. Two parallel WfNet representations (see STRUCTURAL_GAPS.md).

Formal objects:
- `PlaceNodeMarker` / `TransitionNodeMarker` — sealed zero-sized node kind markers.
- `IsPlaceNode` / `IsTransitionNode` — sealed traits.
- `PlaceToTransitionArc<P, T, Weight>` / `TransitionToPlaceArc<T, P, Weight>` — typed bipartite arcs. No `PlaceToPlaceArc` type exists; unconstructible.
- `BipartiteArcConst<const DIR: ArcDirectionConst, Weight>` — single type parameterised over direction. Distinct types for each direction constant.
- `WfNetConst<const SOUNDNESS: SoundnessState>` — const-generic, non-forgeable. `WfNetConst<{Witnessed}>` only constructible via `witness_soundness(SoundnessProof)`.
- `SoundnessProof` / `WfNetSoundnessProofOf<N>` — proof tokens. `SoundnessProof` constructible only inside the petri module or wasm4pm graduation bridge.
- `WfNetQuery` trait — `soundness_state()` query for generic contexts.
- `WfNet<S>` (S defaults to `SoundnessUnknown`) — older typestate-token design using `SoundnessUnknown` / `SoundnessClaimed` / `SoundnessWitnessed` empty enums. `attest_witnessed()` is forgeable (no proof required).
- `Place` / `Transition` / `Arc` / `Marking` / `PetriNet` — structural shapes.
- `ObjectCentricPetriNet` — OC-Petri-net with object-type-typed arcs, variable arcs.
- `PetriRefusal` enum — `MissingInitialMarking` | `MissingFinalMarking` | named structural refusals.

Paper anchoring: Murata (1989) §2 bipartite F ⊆ (P×T) ∪ (T×P); van der Aalst (1998) WF-net soundness; van der Aalst & Berti (2020) OC-Petri-nets.

### `process_tree.rs`
Typed process tree with ARITY law.

Formal objects:
- `TypedLoopNode<ARITY>` with `Require<{ ARITY == 2 }>: IsTrue` — a loop node must have exactly 2 children (do-body + redo-body). Other arities do not compile.
- `ProcessTree` / `TreeNode` / `TreeOperator` (→, ×, ∧, ↺) — structural shapes.
- `ProcessTreeRefusal` — named refusal reasons.

Paper anchoring: van der Aalst process tree canon; Inductive Miner output shape.

### `powl.rs`
Partially Ordered Workflow Language shapes.

Formal objects:
- `TreeProjectable` — sealed trait for POWL fragments that can project losslessly to a process tree.
- `assert_tree_projectable` — compile-time assertion.
- `PowlModel` / `PowlFragment` / `PowlOperator` (Sequence, Choice, Parallel, PartialOrder, Loop) — structural shapes.
- `PowlProjectionRefusal` — named refusal reasons.

Paper anchoring: Kourani & van Zelst (2023) POWL; Kourani, Park & van der Aalst (2026) separable WF-net → POWL 2.0 transformation.

### `bpmn.rs`
BPMN 2.0 structural shapes. No execution semantics.

Formal objects: `BpmnProcess`, `FlowNode`, `SequenceFlow`, `Gateway` (Exclusive/Parallel/Inclusive/Event-based), `Pool`, `Lane`, `BpmnTask`, `BpmnEvent`, `BpmnEdge`. All typed so Pool ≠ Lane and Gateway ≠ Event at the type level.

Paper anchoring: BPMN 2.0 OMG specification.

### `causal_net.rs`
Causal net (C-net) — Heuristics Miner output shape.

Formal objects: `CausalNet`, `CausalTask`, `DependencyArc` (with `DependencyMeasure` in [0,1]), `InputBinding`, `OutputBinding`. No mining, no score computation. Degree of causal strength is structural metadata only.

Paper anchoring: Weijters & Ribeiro (2011) Heuristics Miner.

### `declare.rs`
Declarative process modeling — Declare constraint shapes.

Formal objects: `DeclareModel`, `DeclareConstraint`, `ConstraintTemplate` (Response, Precedence, ChainSuccession, Absence, Coexistence, etc.), `DeclareActivity`. All structure; no LTL evaluation.

Paper anchoring: `DeclareFamily` witness (2007); `DeclareConstraints` witness / Pesic & van der Aalst (2006).

### `dfg.rs`
Directly-Follows Graph shapes.

Formal objects: `DirectlyFollowsGraph`, `DfgArc` (with frequency count), `DfgActivity`. Structure-only; no discovery.

### `workflow.rs`
Generic workflow shapes.

Formal objects: `Workflow`, `WorkflowStep`, `WorkflowTransition`. General-purpose structural surface.

---

## Layer 4: Object-Centric Structures

### `ocel.rs`
OCEL 2.0 object-centric event log shapes.

Formal objects: `OcelLog`, `OcelEvent`, `OcelObject`, `EventObjectLink`, `ObjectObjectLink`, `ObjectChange`. Builder chains on all link/change types. Structural `validate()` checks for dangling links.

Paper anchoring: OCEL 2.0 standard (2023). Witness: `Ocel20`.

### `xes.rs`
XES (IEEE 1849-2016) event log shapes.

Formal objects: `XesLog`, `XesTrace`, `XesEvent`, `XesAttribute`, `XesExtension`. No parsing engine.

Paper anchoring: IEEE 1849-2016. Witness: `Xes1849`.

### `eventlog.rs`
Flat (non-object-centric) event log shapes.

Formal objects: `EventLog`, `Trace`, `Event`. Builder chain: `Event::new().at_ns().by().with_lifecycle()`. `EventLog::from_traces()`, `Trace::new(id, [events])`. `EventStream` append-only buffer.

### `ocpq.rs`
Object-Centric Process Querying shapes.

Formal objects: `OcpqQuery`, `OcpqResult`, `OcpqPredicate`. No query execution engine.

Paper anchoring: `OcpqPaper` witness (2024).

### `object_lifecycle.rs`
Object lifecycle shapes — transitions in an object's lifecycle across events.

Formal objects: `ObjectLifecycle`, `LifecycleStage`, `LifecycleTransition`.

### `correlation.rs`
Cross-log correlation shapes — linking events from distinct logs.

Formal objects: `CorrelationSchema`, `CorrelationKey`, `MergedLog`. No join execution.

### `causality.rs`
Causal ordering structures — cross-object causal links.

Formal objects: `CausalLink`, `CausalChain`, `CausalOrder`. No cycle detection engine.

### `multiperspective.rs`
Multi-perspective conformance shapes (Mannhardt et al., 2016).

Formal objects: `MultiPerspectiveVerdict`, `PerspectiveWeight`, `PerspectiveCost`. Four perspective witnesses: `ControlFlowPerspectiveWitness`, `DataPerspectiveWitness`, `ResourcePerspectiveWitness`, `TimePerspectiveWitness`.

### `process_cube.rs`
Process Cube shapes (van der Aalst, 2013).

Formal objects: `ProcessCube`, `CubeDimension`, `CubeSlice`, `CubeCell`, `CubeProjectionWitness`, `CellComparison`. No sub-log extraction. Witnesses: `ProcessCubePaper`, `OperationalView`, `AnalyticalView`, `AggregationView`.

### `temporal.rs`
Temporal profile shapes — statistical time distance between activity pairs.

Formal objects: `TemporalProfile`, `ActivityPairStats` (AVG/STD). No derivation engine.

Paper anchoring: Stertz, Rinderle-Ma & Rinderle (2020) Temporal Profile Conformance Checking.

### `streaming.rs`
Streaming evidence shapes — online/windowed collection context.

Formal objects: `StreamingBuffer`, `WindowedLog`, `StreamingEventRef`. No ingestion engine.

### `prediction.rs`
Predictive process monitoring shapes.

Formal objects: `PredictionTarget`, `PredictionOutcome`, `OutcomeLabel`. No prediction model.

Paper anchoring: `PredictiveMonitoringFamily` witness (2018).

---

## Layer 5: Format and Boundary Surfaces

### `formats.rs`
Import/export contracts, round-trip claims, loss surfaces. Active when `formats` feature is enabled (default: yes).

Formal objects:
- `LossyFormatExport` trait — requires a non-optional loss report. No silent structure loss.
- `FormatEnvelope<const KIND: FormatKindConst, W>` — typed format envelope.
- `RoundTripClaim` — a structural claim that export-then-import yields the same shape.
- `XesExportRefusal` / `OcelExportRefusal` — named export refusal reasons.

### `strict.rs`
Opt-in boundary judgment. Active when `strict` feature is enabled.

Formal objects:
- `ExportBoundaryConst<HAS_WITNESS, HAS_ROUND_TRIP>` — const-generic type encoding presence/absence of witness and round-trip claims. `ExportBoundaryConst<false, false>` is a different type from `ExportBoundaryConst<true, true>`.
- `ProcessBoundary` — declares a named process boundary with kind and name. `ProcessBoundary::fully_attested(kind, name)` convenience constructor.
- `StrictCheck` — runs strict admission/refusal checks.
- `StrictViolation` enum — `MissingLossPolicy` | `MissingRefusalPath` | `HiddenProcessMiningGrowth` | named violations. `StrictViolation::law()` returns `&'static str` human-readable law name.

### `interop.rs`
Interoperability shapes — cross-format structural compatibility.

Formal objects: `InteropSurface`, `InteropClaim`. No conversion engine.

---

## Layer 6: Evidence Management

### `loss.rs`
Loss accounting — the only sanctioned lossy transformation path.

Formal objects:
- `Project` trait — the only sanctioned lossy transformation.
- `ProjectionName` — `&'static str` newtype implementing `Display`.
- `LossPolicy` enum — `RefuseLoss` | `AllowNamedProjection` | `AllowLossWithReport`. Guard helpers: `is_refusing()`, `is_named()`, `is_reporting()`.
- `LossReport<From, To, Items>` — required on every non-refusing lossy path.

No external format → external format conversion. Only: `external → admitted compat → external | wasm4pm`.

### `receipt.rs`
Receipt-shaped, provenance-bearing evidence.

Formal objects: `Receipt<T, W>`, `ReceiptEnvelope`, `ReceiptChain`. Structure-only; no minting engine.

### `diagnostic.rs`
`CompatDiagnostic` enum — named diagnostics explaining law violations in human-readable form.

Notable variants: `RawEvidenceExportedAsAdmitted`, `MissingLossPolicy`, `ForbiddenDirectFormatConversion`, `UnsoundWfNetClaimed`.

### `ids.rs`
Zero-cost identifier newtypes.

Formal objects: `CaseId`, `ActivityId`, `TraceId`, `EventId`, `ObjectId`, `ObjectTypeId`. All `#[repr(transparent)]` with `From<&str>`, `From<String>`, and `Display`. `CaseId` and `ActivityId` are distinct types — a compile-fail fixture (`case_id_as_activity_id.rs`) proves this.

---

## Layer 7: Nightly Staging and Graduation

### `nightly_foundry.rs`
Zero-cost type-law surfaces derived directly from papers. Always compiled; no cfg gate. An experimental staging area.

Four sub-surfaces:

| Sub-module | Nightly feature | Paper |
|---|---|---|
| `petri_law` | `generic_const_exprs` | Murata (1989) §2 — PreMatrix<P,T> / PostMatrix<P,T> / Marking<P> arc matrices |
| `powl_law` | `adt_const_params` | Kourani (2505.07052) §3 — POWL fragment kinds as const-generic enum |
| `evidence_law` | `min_specialization` | Blue River Dam — admitted vs raw label specialization |
| `token_law` | `portable_simd` | Murata §2 enabling condition ∀p: M[p] ≥ W⁻[p][t] |

Zero-cost guarantee: every type is `#[repr(transparent)]` over a fixed-size array or `u32`, or is zero-sized.

### `graduation.rs`
The graduation bridge toward wasm4pm. See GRADUATION_BOUNDARY_MAP.md.

### `conformance.rs`
Conformance verdict shapes. `Metric<KIND, NUM, DEN>` with `Between01` bounds. See law.rs for `Between01`.

Formal objects: `Fitness`, `Precision`, `F1`, `Generalization`, `Simplicity` newtypes; `Deviation`; `SyncMove` / `LogOnlyMove` / `ModelOnlyMove`; `ConformanceVerdict`; `ConformanceRefusal`.

---

## Layer 8: Support Modules

### `lib.rs`
Crate root. Declares all nightly features unconditionally: `generic_const_exprs`, `adt_const_params`, `const_trait_impl`, `min_specialization`, `portable_simd`, `allow(incomplete_features)`. `#![forbid(unsafe_code)]`.

### `prelude.rs`
Re-exports the most commonly used types. Convenience only.

### `test_utils.rs`
Test-only utilities. Not part of the public API.

---

## Module Count

38 `.rs` files total (including `lib.rs`). 37 content modules. All base-profile modules are always compiled; no canon knowledge is hidden behind a cfg gate.
