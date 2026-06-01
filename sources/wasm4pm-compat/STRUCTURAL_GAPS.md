# STRUCTURAL GAPS AND QUALITY DEFECTS — wasm4pm-compat

**Source:** /Users/sac/wasm4pm-compat/src/petri.rs, tests/ui/compile_fail/
**ALIVE gate:** 398 compile-fail fixtures | 406 compile-pass fixtures
**Dominant error code:** E0308 (type mismatch) — 262 fixtures. E0277 (trait bound not satisfied) — 20 fixtures.

---

## Defect 1: WfNet Split-Brain

**Severity:** High structural defect. Two incompatible representations for the same concept.

### The two representations

`petri.rs` contains two distinct WfNet types encoding WF-net soundness:

**`WfNet<S>` (older design, typestate tokens as empty enums):**
```rust
pub struct WfNet<S = SoundnessUnknown> {
    net: PetriNet,
    final_marking: Option<Marking>,
    _soundness: PhantomData<S>,
}
```
Soundness states: `SoundnessUnknown`, `SoundnessClaimed`, `SoundnessWitnessed` (empty enums).

The `attest_witnessed()` method (line 1177) transitions directly from `WfNet<SoundnessClaimed>` to `WfNet<SoundnessWitnessed>` **without requiring any proof token**. This is forgeable — a caller can call `attest_witnessed()` on any claimed WF-net without any engine-supplied evidence.

**`WfNetConst<const SOUNDNESS: SoundnessState>` (newer design, const-generic):**
```rust
pub struct WfNetConst<const SOUNDNESS: SoundnessState> {
    _seal: wfnet_seal::WfNetSeal, // private module, unconstructible outside petri
}
```
Soundness states: `SoundnessState::Unknown`, `SoundnessState::Claimed`, `SoundnessState::Witnessed` (ConstParamTy enum from law.rs).

The only path to `WfNetConst<{Witnessed}>` is `witness_soundness(SoundnessProof)`, where `SoundnessProof` is only constructible inside the petri module or via the wasm4pm graduation bridge. This is **non-forgeable** — the compile-fail fixture `wfnet_forged_soundness.rs` proves it (E0451: field `_seal` is private).

### The defect

The two designs are parallel and provide different guarantees. `WfNet<SoundnessWitnessed>` can be reached without a proof; `WfNetConst<{SoundnessState::Witnessed}>` cannot. Any code that accepts `WfNet<SoundnessWitnessed>` as a soundness guarantee is relying on the weaker (forgeable) surface.

There are also compile-fail fixtures covering `WfNetConst` (`wfnet_claimed_as_witnessed.rs`, `wfnet_forged_soundness.rs`, `wfnet_unknown_as_claimed.rs`) but no compile-fail fixture covering the `WfNet<S>` forgeable path. The `attest_witnessed()` loophole on `WfNet<S>` is undocumented in the ALIVE gate.

### What wasm4pm must address

The refactor should consolidate to `WfNetConst` and deprecate/remove `WfNet<S>`, or explicitly document `WfNet<S>` as a legacy compatibility surface with a clear caveat that it does not enforce soundness non-forgeability.

---

## Defect 2: Zero Cross-Witness Compile-Fail Fixtures Covering Multi-Witness Confusion

**Severity:** Medium gap in ALIVE gate coverage.

### The observation

The compile-fail fixtures cover witness confusion at the `Evidence` and format envelope level:
- `evidence_wrong_witness_ocel_as_xes.rs` — proves `Evidence<T, _, Ocel20>` ≠ `Evidence<T, _, Xes1849>`
- `evidence_wrong_witness_xes_as_ocel.rs` — the reverse

However, there are no fixtures proving that a **multi-witness admission pipeline** enforces witness consistency end-to-end. For example:
- An `Admission<T, Ocel20>` converted to `Evidence<T, Admitted, Ocel20>` cannot be accidentally passed to a function requiring `Evidence<T, Admitted, Xes1849>` — this is proven by the existing fixtures.
- But: there is no fixture showing that a *refusal* from an OCEL admission cannot be mistakenly used as a refusal from an XES admission in a chained pipeline. The `Refusal<R, W>` type does carry `W`, so this confusion would be caught at compile time — but no fixture documents this guarantee explicitly.

### The defect

The ALIVE gate has no "witness consistency in a multi-step pipeline" fixture. The type system does enforce it, but the receipt has not been minted. Under the Chicago TDD doctrine (if the event log cannot prove a lawful process happened, then it did not happen), the absence of a receipt means the guarantee is structural but unwitnessed.

---

## Defect 3: CausalNet Misclassification Risk

**Severity:** Low — documentation gap, not a type-safety defect.

### The observation

`causal_net.rs` is in the base-profile canon alongside `petri.rs`, `process_tree.rs`, and `powl.rs`. The module header correctly identifies the C-net as a Heuristics Miner output shape (Weijters & Ribeiro 2011). However, `DependencyMeasure` in a C-net is a floating-point dependency score — it is not a process-flow arc weight in the Petri net sense.

A C-net is not a Petri net. Its arcs carry statistical inference results (dependency measures), not incidence matrix weights. There is no `#[repr]` enforcement or newtype distinction between `DependencyMeasure` (C-net arc weight, causal strength in [0,1]) and a plain weight in a `BipartiteArcConst`. A caller could write code that treats both as equivalent numeric weights without a compile error.

### The defect

No compile-fail fixture enforces that `DependencyMeasure` from `causal_net` cannot be passed as a Petri-net arc weight. The gap is in documentation coverage and ALIVE gate minting, not in the type design.

---

## Defect 4: StochasticPetriNet Absent from Canon

**Severity:** Low — scope gap.

### The observation

The canon modules include `petri.rs` (WF-nets, OC-Petri-nets), `process_tree.rs`, `powl.rs`, `bpmn.rs`, and `dfg.rs`. Stochastic Petri nets (GSPNs) — where transitions carry firing rates or weights — are absent. This is not necessarily a defect (they are out of scope), but the graduation boundary documentation does not explicitly state this. A host working with stochastic models would have no named graduation path; `NeedsDiscovery` is the closest match but does not name the stochastic extension.

---

## Defect 5: E0308 Dominance — Possible Fixture Pattern Fragility

**Severity:** Informational.

### The observation

262 of 299 fixtures with named error codes produce E0308 (type mismatch). E0277 (trait bound not satisfied) accounts for 20 more. The `Between01` and `ConditionCell` law gates produce E0277. The typestate/witness confusion fixtures produce E0308.

No fixtures produce E0425 (unresolved name). This means the ALIVE gate does not include "absence of a forbidden symbol" fixtures — i.e., no fixture proves that a forbidden function or type does not exist in the public API. This is a valid design choice (presence-of-type-mismatch is a stronger receipt than absence-of-name), but it means the gate does not cover cases where a previously private API was accidentally made public.

---

## Defect 6: `WfNet::attest_witnessed()` Has No Compile-Fail Coverage

**Severity:** Medium.

**Evidence:**
```bash
grep -l "attest_witnessed" /Users/sac/wasm4pm-compat/tests/ui/compile_fail/*.rs
# (no output)
```

The `attest_witnessed()` method on `WfNet<SoundnessClaimed>` is callable without proof. No compile-fail fixture attempts to call it and expect a failure. No compile-pass fixture demonstrates this is the expected (weaker) API. The method exists and is callable — the ALIVE gate says nothing about it.

This is the practical consequence of the split-brain: the const-generic path is fully receipted; the typestate-token path is unreceipted.

---

## Summary Table

| Defect | Type | Severity | Fixtures covering it |
|---|---|---|---|
| WfNet split-brain: `WfNet<S>.attest_witnessed()` forgeable | Structural | High | 0 (undocumented in ALIVE gate) |
| WfNetConst sealed correctly | Structural | — | wfnet_forged_soundness.rs, wfnet_claimed_as_witnessed.rs, wfnet_unknown_as_claimed.rs |
| Zero multi-step pipeline witness-consistency fixtures | ALIVE gate gap | Medium | 0 |
| CausalNet DependencyMeasure ≠ Petri arc weight (undocumented) | Documentation | Low | 0 |
| StochasticPetriNet absent from canon graduation path | Scope | Low | N/A |
| No E0425 absence-proof fixtures | Gate coverage | Informational | 0 |
| attest_witnessed() uncovered by any fixture | ALIVE gate gap | Medium | 0 |
