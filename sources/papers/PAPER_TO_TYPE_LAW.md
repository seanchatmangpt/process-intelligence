# Paper to Type Law Obligations

**Date:** 2026-05-31
**Source:** wasm4pm-compat PAPER_COVERAGE_LEDGER.md (81 papers)
**Purpose:** For each paper in the workflow corpus: what formal objects does it introduce that SHOULD exist in wasm4pm-compat? What is already covered? What is MISSING?

---

## Fully Covered Papers (COVERED_BY_TYPE)

### OCEL 2.0 Specification — van der Aalst et al. (2023)
**Formal objects introduced:** OcelLog, OcelEvent, OcelObject, EventObjectLink, ObjectObjectLink, OcelDims, OcelAttribute
**Rust surface:** `src/ocel.rs` — all 7 formal objects reified
**Type law enforced:** event-to-object and object-to-object links are first-class structural elements, not optional annotations
**Missing:** None. Full coverage with compile-pass and compile-fail fixtures.

### XES IEEE 1849-2023
**Formal objects introduced:** XesLog, XesTrace, XesEvent, CaseCentricMarker, XesExtension, UndeclaredExtensionPrefix refusal
**Rust surface:** `src/xes.rs`
**Type law enforced:** case-centric structure is a named structural law; XES is not object-centric (compile-fail seal)
**Missing:** None. Full fixture coverage.

### WF-net Soundness (van der Aalst 1998)
**Formal objects introduced:** WF-net (source/sink place, initial/final marking), soundness criterion (option completeness, proper completion, no dead transitions), WfNetSoundnessWitness
**Rust surface:** `src/petri.rs` `WfNetConst<SOUNDNESS>`, `src/witness.rs` `WfNetSoundnessPaper`
**Type law enforced:** `WfNetConst<true>` vs `WfNetConst<false>` const-generic; `WfNetSoundnessWitness` non-forgeable (pub(crate) constructor)
**Missing:** None. Compile-fail `wfnet_forged_soundness` verifies non-forgeability.

### Petri Nets (Murata 1989)
**Formal objects introduced:** Place, Transition, PlaceToTransitionArc, TransitionToPlaceArc, incidence matrix W-/W+, enabling condition, firing rule
**Rust surface:** `src/petri.rs`, `src/nightly_foundry.rs` petri_law and token_law
**Type law enforced:** bipartite arc law — no P→P or T→T arcs; compile-fail seals both violations
**Missing:** None.

### OC-Petri Nets (van der Aalst 2019)
**Formal objects introduced:** Object-type arc inscription, variable arc, binding element; extends PlaceToTransitionArc/TransitionToPlaceArc with object-centric semantics
**Rust surface:** `src/petri.rs` (typed arc newtypes)
**Type law enforced:** same bipartite law; object-type inscription as PhantomData markers
**Missing:** Object-type arc inscription PhantomData markers; variable arc distinction from regular arc (partial gap — low priority)

### POWL (Kourani & van der Aalst)
**Formal objects introduced:** TreeProjectable sealed trait, ChoiceGraph, PowlNodeKind, WfNet2PowlWitness
**Rust surface:** `src/powl.rs`
**Type law enforced:** Sealed TreeProjectable trait prevents non-tree projection; ChoiceGraph as named arc type
**Missing:** None for core POWL; see POWL 2.0 below.

### Separable WF-nets / POWL 2.0 (Kourani, Park, van der Aalst 2026)
**Formal objects introduced:** SeparableWfNet marker, ChoiceGraph (POWL 2.0), WfNet2PowlWitness for language-preserving conversion
**Rust surface:** `src/petri.rs` SeparableWfNet, `src/powl.rs` ChoiceGraph and WfNet2PowlWitness
**Type law enforced:** separability is a structural law; WF-net→POWL 2.0 conversion preserves language
**Missing:** Compile-fail fixture for forged non-separable conversion.

### Declare (Pesic & van der Aalst 2006)
**Formal objects introduced:** DeclareConstraint, DeclareTemplate (named constraint types), DeclareWitness, binary arity law
**Rust surface:** `src/declare.rs`
**Type law enforced:** Each template is a named ConstParamTy variant; unary constraint rejected by compile-fail
**Missing:** None.

### BPMN / Workflow Patterns — multiple papers
**Formal objects introduced:** BpmnElement, GatewayKind, EventKind, pool, lane, subprocess; 17 of 20 workflow control-flow patterns as ConstParamTy
**Rust surface:** `src/bpmn.rs`, `src/law.rs`
**Type law enforced:** WorkflowPattern as ConstParamTy; gateway type semantics named
**Missing:** WCP-14, WCP-15, WCP-18 pattern variants not yet named.

---

## Partially Covered Papers — Active Type-Law Obligations

### Compliance-Aware Predictive PM (#1) — De Santis et al. 2026
**Formal objects introduced:** LTN knowledge base, compliance constraint, prefix-based prediction, compliance score as [0,1] metric
**Currently covered:** `src/prediction.rs` has PredictionTarget structure
**MISSING:**
- `ComplianceConstraintWitness<W>` — binds prediction surface to a named constraint law (W = LTN, Declare, LTL)
- Compliance score as `Between01<NUM, DEN>` metric in `src/conformance.rs`

### OCPQ (#6) — Küsters & van der Aalst 2025
**Formal objects introduced:** OCED tuple (E, O, eaval, oaval), event/object universes, nested query, typed constraint violation set
**Currently covered:** `src/ocpq.rs` has OcpqQuery, OcpqResult, predicate witness markers, OcpqRefusal
**MISSING:**
- `ObjectTypeSet` and `EventTypeSet` as ConstParamTy const params on OcpqQuery
- `ConstraintViolation<ObjType, EvType>` typed result (not free-string violation)
- Compile-fail for mixing object types across constraint scopes

### XES→OCED Projection (#5) — Latif et al. 2025
**Formal objects introduced:** XES→OCED conversion with qualified object-to-object relations absent in XES
**Currently covered:** `src/xes.rs` and `src/ocel.rs` define structures; `src/interop.rs` bridges
**MISSING:**
- `XesToOcedProjection` with explicit `LossPolicy::AllowLossWithReport`
- `LossReport<Xes, Oced, Items>` naming which XES fields have no OCED equivalent

### Heuristics Miner / Causal Nets (#45, #56) — Weijters et al. / van der Aalst
**Formal objects introduced:** CausalNet, CausalBinding, InputBinding, OutputBinding, dependency measure in [0,1]
**Currently covered:** Output shapes only (discovery graduation boundary for computation)
**MISSING:**
- `CausalNet` struct in `src/causal_net.rs`
- `CausalBinding<Source, Target>` typed arc
- `InputBinding` / `OutputBinding` as distinct newtypes
- `DependencyMeasure` as `Between01<NUM, DEN>` const-generic fraction

### Multi-Perspective Process Mining (#48) — van der Aalst 2011
**Formal objects introduced:** Resource perspective, data perspective as typed extension namespaces on XES events
**Currently covered:** `src/xes.rs` has basic XesEvent; no perspective typing
**MISSING:**
- `ResourcePerspective` as PhantomData extension marker on XesEvent
- `DataPerspective` as PhantomData extension marker on XesEvent
- Perspective-scoped attribute typed surface

### OC-PM Divergence/Convergence (#49) — van der Aalst & Berti 2020
**Formal objects introduced:** Divergence witness (one case → multiple objects), Convergence witness (multiple cases → one object)
**Currently covered:** OCEL structures exist; divergence/convergence not named
**MISSING:**
- `DivergenceWitness` unit-struct in `src/witness.rs` — certifies structure assessed for divergence
- `ConvergenceWitness` unit-struct in `src/witness.rs` — certifies structure assessed for convergence

### Process Querying Methods (#51) — Polyvyanyy et al. 2017
**Formal objects introduced:** Process query framework, temporal predicate axioms, behavioral profile
**Currently covered:** `src/ocpq.rs` has partial predicate surface
**MISSING:**
- `ProcessQueryWitness` named witness linking OcpqQuery to Polyvyanyy et al. 2017
- Full temporal ordering axiom coverage in `TemporalPredicate`

### Stochastic Conformance (#57)
**Formal objects introduced:** StochasticArcWeight in [0,1], ImmediateTransition, TimedTransition (GSPN arc types)
**Currently covered:** Basic Petri net arc types; no stochastic annotation
**MISSING:**
- `StochasticArcWeight<NUM, DEN>` newtype bounded by `Between01`
- `ImmediateTransition` unit-struct marker in `src/petri.rs`
- `TimedTransition` unit-struct marker in `src/petri.rs`

### Mannhardt Multi-Perspective Checking (#71) — Mannhardt et al. 2016
**Formal objects introduced:** Resource and data perspective as weighted conformance dimensions; perspective weight in [0,1]
**Currently covered:** Conformance metric shapes exist; no perspective weighting
**MISSING:**
- `ResourcePerspective` and `DataPerspective` typed namespaces (same gap as #48)
- Perspective weight as `Between01<NUM, DEN>` const-generic in `src/conformance.rs`

---

## Summary: Type-Law Obligation Count

| Status | Papers | Named Missing Types |
|---|---|---|
| Fully covered | 18 | 0 |
| Partially covered (active gaps) | 10 | 22 distinct named types |
| Graduation boundary (no type gap) | 39 | 0 (deliberate) |
| Duplicate/Background | 4 | 0 |
| Out of scope | 10 | 0 |

The 22 named missing types constitute the complete type-law obligation backlog for wasm4pm-compat.
