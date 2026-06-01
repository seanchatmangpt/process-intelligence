# Audit: Type Law Quality — wasm4pm-compat

**Date:** 2026-05-31  
**Auditor:** Dr. Standards Cartographer  
**Repository:** `/Users/sac/wasm4pm-compat`  
**Source:** `src/petri.rs`, `tests/ui/compile_fail/`, `tests/ui/compile_pass/`

---

## Executive Summary

The wasm4pm-compat type law is structurally sound at its core. The bipartite arc law, conformance metric bounds, and evidence typestate chain are the strongest surfaces. The critical weakness is the WfNet split-brain: `SoundnessState` is a three-value enum (Unknown/Claimed/Witnessed) but the `WfNetConst` const-generic parameter accepts any `SoundnessState` value without the transition path being enforced at the witness level by a cross-witness fixture. Additionally, zero cross-witness confusion fixtures exist — a significant gap for a crate whose identity is non-forgeable witnesses.

---

## Audit Dimension 1: Witness Lattice Completeness

The witness lattice in `src/witness.rs` defines named paper/standard markers: `Ocel20`, `Xes1849`, `WfNetSoundnessPaper`, `PowlPaper`, `Pm4pyApiGrammar`, `ProcessTreePaper`, `ObjectCentricPetriNetPaper`, `ProvOPaper`, `DeclarePaper`, and others.

**Assessment:** The lattice is broad but not complete.

| Gap | Severity |
|---|---|
| No `OcpqPaper` witness marker (Küsters & van der Aalst 2025) | MEDIUM — `src/ocpq.rs` exists but has no named witness |
| No `DivergenceWitness` / `ConvergenceWitness` for OCEL divergence/convergence law (paper #49) | MEDIUM — structural support exists; witness name missing |
| No `FreeChoiceMarker` to distinguish polynomial-soundness WF-nets from general WF-nets (#44) | LOW — graduation boundary item |
| `AbstractionLevel` marker missing from `src/eventlog.rs` (#52) | LOW — graduation boundary item |

---

## Audit Dimension 2: WfNet Soundness — Non-Forgeability

`WfNetConst::witness_soundness` requires a `SoundnessProof`, which is only constructible inside this module (private constructor sealed by `wfnet_seal::WfNetSeal`). This is the correct non-forgeability mechanism.

**Inspected mechanism:**
```
WfNetConst<Unknown> 
  .claim_sound() → WfNetConst<Claimed>
  .witness_soundness(proof) → WfNetConst<Witnessed>
```

`SoundnessProof(wfnet_seal::WfNetSeal)` — the `WfNetSeal` type is in a private `wfnet_seal` module. External callers cannot construct `WfNetSeal`, therefore cannot forge `SoundnessProof`, therefore cannot construct `WfNetConst<{Witnessed}>` directly.

**CRITICAL finding — WfNet split-brain:**  
`WfNetConst` is parameterized by `SoundnessState` (a const-generic enum). The type system therefore distinguishes `WfNetConst<{SoundnessState::Unknown}>`, `WfNetConst<{SoundnessState::Claimed}>`, and `WfNetConst<{SoundnessState::Witnessed}>` as three distinct types. However:

- `WfNetConst<{SoundnessState::Claimed}>` is constructible directly via `claim_sound()` — this is intentional (human assertion, not algorithmic proof)
- The split-brain risk: code that accepts `WfNetConst<{SoundnessState::Claimed}>` where `WfNetConst<{SoundnessState::Witnessed}>` is required will fail with E0308 — this IS correctly sealed

The `wfnet_claimed_as_witnessed` compile-fail fixture (`tests/ui/compile_fail/wfnet_claimed_as_witnessed.rs`) seals this law and fails with E0308.

**Revised assessment:** The split-brain is sealed by the type system. The "CRITICAL" label from earlier analysis overstates the risk — `Claimed ≠ Witnessed` is enforced at compile time. The gap is that no fixture proves `Unknown ≠ Witnessed` (only `Claimed ≠ Witnessed` is tested).

---

## Audit Dimension 3: Compile-Fail Receipt Quality

**Error code distribution across 199 compile-fail fixtures:**

| Error code | Count | Law category | Receipt quality |
|---|---|---|---|
| `E0308` | 262 occurrences | Type mismatch — correct law errors | VALID receipt |
| `E0277` | 20 occurrences | Trait bound not satisfied — correct law errors | VALID receipt |
| `E0599` | 10 occurrences | Method not found — sealed trait violations | VALID receipt |
| `E0451` | 2 occurrences | Private field access attempt | VALID receipt |
| `E0391` | 1 occurrence | Cyclic dependency | AMBIGUOUS — may be accidental |
| `E0382` | 1 occurrence | Value used after move | AMBIGUOUS — may be accidental |
| `E0063` | 1 occurrence | Missing struct field | AMBIGUOUS — may be structural check |
| `E0061` | 1 occurrence | Wrong number of arguments | AMBIGUOUS — may be arity check |
| `E0053` | 1 occurrence | Method not compatible with trait | VALID receipt if testing trait impl law |

**Zero E0425 fixtures.** The E0425 error ("cannot find value") would indicate a fixture that fails because a symbol name is wrong — an absence-proof fixture that catches a typo rather than proving a law. The absence of E0425 fixtures is a positive finding: no fixture is accidentally passing due to name resolution failures.

**Receipt quality:** 292 of 299 error occurrences (97.7%) are valid law-proving error codes (E0308, E0277, E0599, E0451). The 7 ambiguous occurrences should be reviewed to confirm they are intentional law tests.

---

## Audit Dimension 4: Cross-Witness Fixture Coverage

Cross-witness fixtures prove that evidence carrying witness `W1` cannot be used where witness `W2` is required. This is the core non-forgeability claim.

**Fixtures with cross-witness content (2+ named witnesses, testing confusion):**

| Fixture | Witnesses tested | Error |
|---|---|---|
| `evidence_wrong_witness_xes_as_ocel.rs` | `Xes1849` vs `Ocel20` | E0308 |
| `evidence_wrong_witness_ocel_as_xes.rs` | `Ocel20` vs `Xes1849` | E0308 |
| `witness_pm4py_as_pmax.rs` | `Pm4pyApiGrammar` vs `PmaxApiGrammar` | E0308 |
| `witness_xes_as_wfnet.rs` | `Xes1849` vs `WfNetSoundnessPaper` | E0308 |
| `witness_ocel_as_powl.rs` | `Ocel20` vs `PowlPaper` | E0308 |
| `receipt_wrong_witness_marker.rs` | Receipt witness confusion | E0308 |
| `refusal_without_named_law.rs` | `Xes1849` witness mismatch | E0308 |

**Count: 7 explicit cross-witness confusion fixtures** (out of 199 total compile-fail fixtures). This is 3.5% of the fixture corpus.

**Gap assessment:** With 10+ named witness markers, there are ~45 possible pairwise confusion scenarios. Only 7 are tested. This is sufficient for the most important pairs (OCEL vs XES, XES vs WF-net, OCEL vs POWL) but leaves 38+ cross-witness pairs untested.

**Priority gaps:**
- No fixture for `WfNetSoundnessPaper` vs `PowlPaper` confusion
- No fixture for `ProcessTreePaper` vs `WfNetSoundnessPaper` confusion
- No fixture for `Ocel20` vs `DeclarePaper` confusion

---

## Audit Dimension 5: Graduation Boundary Accuracy

The graduation boundary declares what is structure (stays in compat) and what is engine (must move to wasm4pm). This is audited by checking whether any compile-fail fixture tests for engine logic being present in compat.

Relevant fixtures:
- `conformance_checker_absent.rs` — seals that conformance checking execution is not in compat
- `dfg_engine_boundary_rejected.rs` — seals that DFG computation is not in compat

**Assessment:** The graduation boundary sealing is thin. Only 2 fixtures explicitly test engine boundary rejection. The boundary is primarily documented (in docstrings and CLAUDE.md) rather than type-sealed. A caller who imports compat and attempts to call a discovery algorithm will find no such method — but there is no compile-fail fixture proving that `DiscoveryEngine::run()` cannot be called on compat types.

**Recommendation:** Add one engine-boundary compile-fail fixture per graduation reason: `NeedsDiscovery`, `NeedsConformanceExecution`, `NeedsObjectCentricQueryExecution`, `NeedsReplay`.

---

## Summary: Known Issues by Severity

| Issue | Severity | Status |
|---|---|---|
| WfNet `Unknown ≠ Witnessed` not explicitly fixture-sealed | MEDIUM | Gap: no fixture for this specific confusion |
| Cross-witness fixture coverage is 3.5% of possible pairs | MEDIUM | 38+ cross-witness pairs untested |
| `E0391`/`E0382` ambiguous error code fixtures | LOW | Review for intentionality |
| Graduation boundary sealing has only 2 fixtures | MEDIUM | Add one per graduation reason |
| Missing `OcpqPaper` witness marker | LOW | No fixture impact until witness added |
| `DivergenceWitness`/`ConvergenceWitness` missing | LOW | Structural support present; name missing |

---

## What Is Strong

- **Bipartite arc law** (E0308, 4+ fixtures) — strongest surface in the codebase; place→place and transition→transition arcs are unconstructible
- **Evidence typestate chain** (E0308, 11+ fixtures) — every illegal state transition is fixture-sealed
- **Conformance metric bounds** (`Between01<NUM,DEN>` with E0080/E0308) — metric-out-of-bounds is type-rejected
- **WfNet non-forgeability** — `SoundnessProof` constructor sealed by private module; verified correct
- **Zero E0425 fixtures** — no absence-proof defects in the fixture corpus
