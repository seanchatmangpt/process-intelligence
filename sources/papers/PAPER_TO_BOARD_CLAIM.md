# Paper to Board-Admissible Claims

**Date:** 2026-05-31
**Source:** wasm4pm-compat PAPER_COVERAGE_LEDGER.md (81 papers)
**Purpose:** What board-admissible claims can the research program make based on the paper corpus?
**Format:** Claim text | Paper citation(s) | Evidence path

A board-admissible claim is a claim that:
1. Is grounded in a specific paper with a specific formal result
2. Has a verifiable evidence path in wasm4pm-compat (a named Rust type, a compile-fail fixture, or a graduation boundary)
3. Cannot be falsified by pointing to a missing type that contradicts the claim

---

## Tier 1: Structural Completeness Claims

### Claim 1: The crate encodes WF-net soundness as a non-forgeable compile-time property
**Claim text:** "wasm4pm-compat enforces WF-net soundness (option completeness, proper completion, no dead transitions) at the Rust type level. A `WfNetConst<true>` cannot be constructed without a `WfNetSoundnessWitness`, and that witness constructor is `pub(crate)` — forgery is a compile error, not a runtime check."
**Paper citation(s):**
- van der Aalst (1998) "The Application of Petri Nets to Workflow Management" — defines the 3-condition soundness criterion
- van der Aalst & ter Hofstede (2004) "YAWL: Yet Another Workflow Language" — validates soundness via WOFLAN reduction
**Evidence path:**
- `src/petri.rs`: `WfNetConst<SOUNDNESS>`, `WfNetSoundnessWitness` (pub(crate) constructor)
- `src/witness.rs`: `WfNetSoundnessPaper` witness marker
- `tests/ui/compile_fail/wfnet_forged_soundness.rs` + `.stderr` — rejects forged soundness at compile time
- `tests/ui/compile_pass/wfnet_with_soundness_witness.rs` — confirms lawful path is open

### Claim 2: The crate enforces Petri net bipartite arc law at the type level
**Claim text:** "Place-to-place and transition-to-transition arcs are structurally impossible in this crate. The arc types `PlaceToTransitionArc` and `TransitionToPlaceArc` are distinct newtypes; there exist no `PlaceToPlaceArc` or `TransitionToTransitionArc` types. Every compile-fail fixture that attempts to create such an arc is rejected."
**Paper citation(s):**
- Murata (1989) "Petri Nets: Properties, Analysis and Applications" §2 — defines bipartite arc law
- van der Aalst (1998) — applies bipartite law to WF-nets
**Evidence path:**
- `src/petri.rs`: `PlaceToTransitionArc`, `TransitionToPlaceArc` newtypes
- `tests/ui/compile_fail/petri_place_to_place_arc.rs` + `.stderr`
- `tests/ui/compile_fail/petri_transition_to_transition_arc.rs` + `.stderr`
- `src/nightly_foundry.rs`: `petri_law` surface citing Murata (1989) §2

### Claim 3: The crate encodes the OCEL 2.0 formal data model without runtime overhead
**Claim text:** "Every formal object in the OCEL 2.0 specification (OcelLog, OcelEvent, OcelObject, EventObjectLink, ObjectObjectLink, OcelDims, OcelAttribute) is a zero-cost Rust type. Event-to-object and object-to-object links are first-class structural elements — they cannot be omitted at the type level."
**Paper citation(s):**
- van der Aalst, Berti, Ghahfarokhi, Klijn, Park, Pourbafrani (2023) "OCEL 2.0 Specification"
**Evidence path:**
- `src/ocel.rs`: 7 named types matching spec formal objects
- `tests/ui/compile_pass/ocel_event_object_relation.rs`, `ocel_object_object_relation.rs`
- `tests/ui/compile_fail/ocel_e2o_missing_link.rs`, `ocel_o2o_missing_link.rs` + `.stderr` files

### Claim 4: The crate enforces the XES case-centric vs. object-centric distinction at the type level
**Claim text:** "A `XesLog` carries a `CaseCentricMarker`. There is no path from XES to object-centric analysis that does not pass through an explicit projection with a named `LossPolicy`. The compile-fail fixture `xes_not_object_centric` rejects any attempt to treat a XES log as object-centric."
**Paper citation(s):**
- IEEE 1849-2023 "XES Standard" — defines case-centric structure as the formal schema
- Latif et al. (2025) — demonstrates that XES→OCED conversion has structural loss
**Evidence path:**
- `src/xes.rs`: `XesLog`, `CaseCentricMarker`, `XesExtension`
- `tests/ui/compile_fail/xes_not_object_centric.rs` + `.stderr`
- `src/loss.rs`: `LossPolicy` required for any projection

### Claim 5: The crate reifies 17 of 20 canonical workflow control-flow patterns as named types
**Claim text:** "The 20 workflow patterns from the Russell, van der Aalst, ter Hofstede (2016) taxonomy are structural laws. 17 of these patterns are encoded as `WorkflowPattern` enum variants (ConstParamTy), making `PatternNet<{ WorkflowPattern::ParallelSplit }>` a distinct compile-time type from `PatternNet<{ WorkflowPattern::ExclusiveChoice }>`."
**Paper citation(s):**
- Russell, van der Aalst, ter Hofstede (2016) "Workflow Patterns: The Definitive Guide"
**Evidence path:**
- `src/law.rs`: `WorkflowPattern` as `ConstParamTy` enum (17 variants)
- `tests/ui/compile_pass/workflow_pattern_const_param.rs`
- `tests/ui/compile_fail/workflow_pattern_wrong_kind.rs` + `.stderr`

---

## Tier 2: Boundary Enforcement Claims

### Claim 6: Every conformance metric is bounded in [0,1] by the type system
**Claim text:** "The types `FitnessConst<NUM, DEN>`, `PrecisionConst<NUM, DEN>`, and `F1Const<NUM, DEN>` are bounded by `Between01<NUM, DEN>`. A metric where NUM > DEN is a compile error — the metric cannot escape the unit interval by construction."
**Paper citation(s):**
- Adriansyah, van Dongen et al. (2011) "Aligning Observed and Modeled Behavior" — defines fitness and precision as [0,1] values
- Rozinat & van der Aalst (2008) — conformance by monitoring real behavior
**Evidence path:**
- `src/conformance.rs`: `Metric<KIND, NUM, DEN>` with `Between01<NUM, DEN>` bound
- `src/law.rs`: `Between01<NUM, DEN>`, `Assert`, `IsTrue` const-generic machinery
- `tests/ui/compile_fail/metric_out_of_bounds.rs` + `.stderr`

### Claim 7: Lossy projections are explicit, named, and structurally enforced
**Claim text:** "There is no path from one external format to another in this crate that does not go through: `external → Admitted → external | wasm4pm`. Every lossy transformation requires a `ProjectionName`, a `LossPolicy` declared before loss occurs, and a `LossReport` on every non-refusing path. Silent structure loss is a compile error."
**Paper citation(s):**
- Van der Aalst et al. (OCEL 2.0 spec) — defines the distinction between lossless and lossy format conversion
- Latif et al. (2025) — demonstrates structural loss in XES→OCED projection
**Evidence path:**
- `src/loss.rs`: `ProjectionName`, `LossPolicy`, `LossReport<From, To, Items>`
- `src/formats.rs`: `LossyFormatExport` requiring a non-optional loss report
- `examples/ocel_to_xes_projection.rs` — demonstrates covenant in action

### Claim 8: The POWL 2.0 language-preservation conversion is a non-forgeable typed witness
**Claim text:** "The `WfNet2PowlWitness` certifies that a WF-net has been transformed into a POWL 2.0 structure via a language-preserving conversion. The witness is non-forgeable: `SeparableWfNet<SOUNDNESS>` is the only valid input, and `WfNet2PowlWitness` cannot be constructed for a non-separable net."
**Paper citation(s):**
- Kourani, Park, van der Aalst (2026) "Hierarchical Decomposition of Separable Workflow-Nets"
**Evidence path:**
- `src/petri.rs`: `SeparableWfNet<SOUNDNESS>` marker
- `src/powl.rs`: `WfNet2PowlWitness`, `ChoiceGraph`
- `tests/ui/compile_pass/separable_wfnet_marker.rs`, `wfnet2powl_witness.rs`

---

## Tier 3: Research Program Claims

### Claim 9: The wasm4pm-compat crate provides a zero-cost type-law foundation for 81 canonical process mining papers
**Claim text:** "81 papers from the process mining canon have been assessed against the wasm4pm-compat type surface. 18 papers have all key formal objects reified as zero-cost Rust types. 39 papers have their output shapes typed while their computational algorithms are delegated to wasm4pm. 10 papers have named, specific type-law gaps constituting the active obligation backlog."
**Paper citation(s):** All 81 papers in the PAPER_COVERAGE_LEDGER.md
**Evidence path:**
- `docs/PAPER_COVERAGE_LEDGER.md`: 81-row inventory with per-paper verdict and evidence
- `src/`: 23 canon modules covering all assessed paper formal objects
- `tests/ui/compile_pass/`: pass fixtures for each COVERED_BY_TYPE paper
- `tests/ui/compile_fail/`: fail fixtures for each core type-law

### Claim 10: The Declare constraint model is encoded such that each named template is a distinct compile-time type
**Claim text:** "The 12 Declare templates (existence, absence, init, responded_existence, response, precedence, chain_response, chain_precedence, not_coexistence, not_succession, not_chain_succession, exclusive_choice) are `ConstParamTy` variants of `DeclareTemplate`. A constraint with the wrong template is a compile-time type mismatch. Unary constraint arity is rejected by `declare_binary_arity_rejected`."
**Paper citation(s):**
- Pesic & van der Aalst (2006) "Declare: Full Support for Loosely-Structured Processes"
**Evidence path:**
- `src/declare.rs`: `DeclareConstraint`, `DeclareTemplate` (ConstParamTy), `DeclareWitness`
- `tests/ui/compile_pass/declare_constraint_shape.rs`
- `tests/ui/compile_fail/declare_binary_arity_rejected.rs` + `.stderr`

### Claim 11: The graduation boundary between wasm4pm-compat (structure) and wasm4pm (execution) is formally defined
**Claim text:** "Every algorithm that computes over process structures — discovery, conformance checking, alignment, querying, prediction — is named as a `GraduationReason` variant. The boundary is not informal: a `GraduationCandidate::is_grounded()` check certifies whether a type has a lawful graduation path to wasm4pm."
**Paper citation(s):**
- All 39 papers classified as COVERED_BY_GRADUATION_BOUNDARY
**Evidence path:**
- `src/graduation.rs`: `GraduationReason`, `GraduationCandidate`, `is_grounded()`
- `docs/GRADUATION_BOUNDARIES.md`: all 39 papers' graduation targets named
- `examples/graduation_candidate.rs` — demonstrates grounded vs. ungrounded candidate

---

## Claims That Cannot Yet Be Made (Pending Type-Law Work)

The following claims are NOT yet board-admissible because the required types are missing:

1. "CausalNet is a zero-cost Rust type with dependency measures bounded in [0,1]" — `CausalNet` not yet in `src/causal_net.rs`
2. "Stochastic Petri net arc weights are bounded in [0,1] by the type system" — `StochasticArcWeight` not yet typed
3. "Resource and data perspectives are distinct named types on XES events" — `ResourcePerspective`/`DataPerspective` not yet in `src/xes.rs`
4. "The XES→OCED projection has a named, typed loss surface" — `XesToOcedProjection` not yet in `src/interop.rs`
5. "Divergence and convergence are non-forgeable certificates in the witness registry" — `DivergenceWitness`/`ConvergenceWitness` not yet in `src/witness.rs`

These become board-admissible claims once the 10 PARTIAL papers' type obligations are fulfilled.
