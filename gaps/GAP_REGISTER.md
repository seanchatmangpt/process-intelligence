# GAP REGISTER — Master Gap Index

**Status:** Active  
**Scope:** wasm4pm-compat ↔ wasm4pm integration gaps, type-law quality gaps  
**Maintained by:** process-intelligence Gap Register Agent

---

## GAP_001 — CRITICAL — compat-wasm bridge missing

**Severity:** CRITICAL  
**Status:** Open

wasm4pm does NOT import wasm4pm-compat. Two parallel type universes exist with no connection.

- wasm4pm accepts raw `EventLog` in all algorithm functions
- Graduation bridge in compat is one-sided (no intake in wasm4pm)
- `GraduationCandidate` is defined in compat but wasm4pm has no function that accepts it
- Process evidence admitted through compat type law is thrown away at the wasm4pm boundary

**Resolution:** Add wasm4pm-compat as a dependency in wasm4pm `Cargo.toml`. Change all algorithm function inputs from raw `EventLog` to `Admitted` types sourced from compat. Implement an intake path in wasm4pm that consumes `GraduationCandidate`.

---

## GAP_002 — CRITICAL — named law refusals missing in wasm4pm

**Severity:** CRITICAL  
**Status:** Open

`ValidationError(String)` is not a named law refusal. It carries no structural law identity, no witness type, and no specific reason type. This makes wasm4pm non-conformant with the compat type-law covenant.

**Current state:** wasm4pm returns `ValidationError(String)` across its validation surfaces.  
**Required state:** Named `Refusal<R, W>` where `R` is a specific structural law (e.g. `DanglingEventObjectLink`, `MissingFinalMarking`) and `W` is a witness marker.

**Resolution:** Replace `ValidationError(String)` with compat `Refusal` types. Requires GAP_001 closed first (compat must be imported before its types can be used).

---

## GAP_003 — MAJOR — Inductive Miner missing from wasm4pm

**Severity:** MAJOR  
**Status:** Open

Alpha Miner is present in wasm4pm but produces unsound WF-nets. Inductive Miner provides the soundness guarantee that Alpha Miner lacks. Without Inductive Miner, wasm4pm cannot produce process trees with guaranteed soundness.

**Gap:** No `InductiveMiner` implementation exists. No `TypedProcessTree` output from discovery.

**Resolution:** Implement `InductiveMiner` returning `TypedProcessTree` (using the `process_tree.rs` types from compat). This is independent of GAP_001 at the algorithm level but requires compat types for the return type.

---

## GAP_004 — MAJOR — alignment-based conformance missing from wasm4pm

**Severity:** MAJOR  
**Status:** Open

Token replay is present in wasm4pm but alignment-based conformance is the superior method. Token replay produces a fitness approximation; alignment produces an exact optimal-alignment fitness with precision.

**Gap:** No `AlignmentConformance` implementation. No `Metric<FITNESS, N, D>` output from conformance checking.

**Resolution:** Implement `AlignmentConformance` returning `Metric<FITNESS, N, D>` (using the `conformance.rs` `Metric` type from compat with `Between01` bounds). Independent of GAP_001 at the algorithm level.

---

## GAP_005 — MAJOR — OCPQ evaluation missing from wasm4pm

**Severity:** MAJOR  
**Status:** Open

OCPQ structural shapes are defined in wasm4pm-compat `ocpq.rs`. There is no execution engine in wasm4pm that evaluates OCPQ queries against an object-centric event log.

**Gap:** `OcpqEvaluator` does not exist in wasm4pm. The structural types exist in compat but have no runtime counterpart.

**Resolution:** Implement `OcpqEvaluator` in wasm4pm. This is logically independent but benefits from GAP_001 being closed so that admitted OCEL logs flow in rather than raw structures.

---

## GAP_006 — MINOR — POWL discovery missing from wasm4pm

**Severity:** MINOR  
**Status:** Open

POWL types are defined in wasm4pm-compat `powl.rs`. No `mineDG` miner or equivalent POWL discovery algorithm exists in wasm4pm.

**Gap:** No `PowerMiner` or equivalent. No path from an event log to a `TypedPowl` artifact.

**Resolution:** Implement `PowerMiner` returning `TypedPowl`. Uses the sealed `TreeProjectable` trait from compat `powl.rs`. Independent of GAP_001 at the algorithm level.

---

## GAP_007 — MINOR — WfNet split-brain in wasm4pm-compat

**Severity:** MINOR  
**Status:** Open

`WfNet::attest_witnessed()` is forgeable (the method is public). `WfNetConst<SOUNDNESS>` is sealed (correct). The two exist in the same crate with contradictory forgeability guarantees.

**Gap:** A caller can call `WfNet::attest_witnessed()` and obtain an attested WF-net without going through the lawful soundness verification path. This undermines the type-law receipt claim for WF-net soundness.

**Resolution:** Remove `WfNet::attest_witnessed()` or gate it behind a `pub(crate)` or `#[doc(hidden)]` guard with a `#[deprecated]` annotation until removal. `WfNetConst<SOUNDNESS>` is the correct non-forgeable surface.

---

## GAP_008 — MINOR — E0425 absence-proof fixtures in wasm4pm-compat

**Severity:** MINOR  
**Status:** Open

Some `compile_fail` fixtures in `tests/ui/compile_fail/` fail because a type is not found (E0425 — cannot find type in scope), not because a law structurally prevents the operation. An E0425 failure is an import error, not a type-law receipt.

**Gap:** Fixtures failing on E0425 are not valid type-law receipts. They would pass if someone added the correct `use` statement, regardless of the law's intent.

**Resolution:** Replace E0425-failing fixtures with structural fixtures that fail on:
- `E0308` (type mismatch — wrong state or witness type)
- `E0599` (method not found for type — law not satisfied)
- `E0277` (trait bound not satisfied — compile-time predicate rejected)
- `E0080` (const eval failure — `generic_const_exprs` bound violated)

---

## Summary Table

| GAP | Severity | Status | Priority |
|-----|----------|--------|----------|
| GAP_001 | CRITICAL | Open | P1 |
| GAP_002 | CRITICAL | Open | P1 |
| GAP_003 | MAJOR | Open | P2 |
| GAP_004 | MAJOR | Open | P2 |
| GAP_005 | MAJOR | Open | P3 |
| GAP_006 | MINOR | Open | P3 |
| GAP_007 | MINOR | Open | P1 |
| GAP_008 | MINOR | Open | P2 |
