# DOWNSTREAM WASM4PM-COMPAT GAP CLOSE — Authorized Mandate

**Authority Source:** process-intelligence Gap Register Agent  
**Target crate:** wasm4pm-compat (`/Users/sac/wasm4pm-compat`)  
**Gap Register:** gaps/GAP_REGISTER.md (GAP_007, GAP_008)  
**Red Team Findings:** adversarial/RED_TEAM_FINDINGS_001.md (Finding 003), adversarial/RED_TEAM_FINDINGS_002.md

---

## Mandate

Four gap closure tasks for the wasm4pm-compat crate itself. These are self-contained — they do not require wasm4pm changes and are not blocked by GAP_001.

---

## Task 1 — Fix WfNet::attest_witnessed() forgeability (GAP_007)

**Severity:** MINOR (but a type-law integrity defect)

`WfNet::attest_witnessed()` is currently a public method. Any caller can invoke it and obtain an attested WF-net without going through the lawful soundness verification path. This makes the attestation forgeable and contradicts the type-law covenant.

`WfNetConst<SOUNDNESS>` is the correct non-forgeable surface. It uses const-generic type parameterization to make soundness non-forgeable at compile time.

**Required change:**

Remove `WfNet::attest_witnessed()` from the public API, or:
- Gate it as `pub(crate)` to prevent external callers, OR
- Annotate it `#[deprecated = "Use WfNetConst<SOUNDNESS> — the non-forgeable soundness surface"]` as an interim step before removal

**Completion criterion:** No external caller can produce an attested WF-net by calling `attest_witnessed()` directly. The only non-forgeable path to WF-net attestation is through `WfNetConst<SOUNDNESS>`.

**Fixture required:** A compile-fail fixture demonstrating that `WfNet::attest_witnessed()` is inaccessible from outside the crate (E0616 — field is private, or E0624 — method is private, depending on implementation choice).

---

## Task 2 — Replace E0425 absence-proof fixtures with true type-law fixtures (GAP_008)

**Severity:** MINOR (but type-law receipt quality defect)

Some compile_fail fixtures in `tests/ui/compile_fail/` fail because a type is not found (E0425), not because a structural law prevents the operation. These are not valid type-law receipts.

**Diagnosis:** Run `cargo test --test ui_tests -- --ignored` and inspect the `.stderr` files. Any fixture whose `.stderr` contains `error[E0425]` or `error[E0432]` is an import error, not a type-law receipt.

**Required change:** For each E0425/E0432 fixture, replace it with a fixture that:
1. Has the correct `use` statements so the types are in scope
2. Attempts the operation that is supposed to be forbidden by law
3. Fails on a structural error code:
   - `E0308` — type mismatch (wrong state token or wrong witness type)
   - `E0599` — method not found for type (law surface not available for this type)
   - `E0277` — trait bound not satisfied (compile-time predicate rejected)
   - `E0080` — const eval failure (`generic_const_exprs` bound violated)

**Completion criterion:** Zero E0425 or E0432 errors in any `.stderr` file in `tests/ui/compile_fail/`. Every fixture fails because the law structurally prevents the operation, not because of a missing import.

---

## Task 3 — Add cross-witness compile-fail fixtures

**Severity:** Enhancement (strengthens the type-law receipt surface)

The type system distinguishes `Admission<T, Ocel20>` from `Admission<T, Xes1849>` at the type level. The witness marker prevents confusion between process evidence admitted under different standards. This is a core type-law claim.

There are currently no compile-fail fixtures that demonstrate this witness isolation property.

**Required fixture:** A fixture in `tests/ui/compile_fail/` that:
1. Produces an `Admission<SomeData, Ocel20>` through the lawful `Admit::admit()` path
2. Attempts to use it where `Admission<SomeData, Xes1849>` is expected
3. Fails on E0308 (type mismatch — witness types differ)
4. Has a `.stderr` file confirming the E0308 error

**Completion criterion:** At least one cross-witness compile-fail fixture exists, fails on E0308, and has a matching `.stderr` file.

---

## Task 4 — Verify all COVERED_BY_TYPE claims have actual types in src/

**Severity:** Audit (addresses Red Team Finding 002 / adversarial challenge category 2)

The paper corpus classification uses `COVERED_BY_TYPE` as the strongest coverage label. Every `COVERED_BY_TYPE` claim asserts that a specific Rust type in `wasm4pm-compat/src/` structurally encodes the paper's contribution.

**Required verification:**

For each paper classified as `COVERED_BY_TYPE`:
1. Name the specific Rust type (struct, enum, or trait)
2. Name the module (`src/law.rs`, `src/petri.rs`, etc.)
3. Identify the specific field, const bound, or trait bound that encodes the paper's structural contribution

If a `COVERED_BY_TYPE` label cannot be backed by a specific type, it must be downgraded to:
- `COVERED_BY_STRUCTURE` — if the structure exists but no single named type encodes it
- `COVERED_BY_WITNESS` — if only a witness marker naming the paper exists
- `RESEARCH_CLAIM` — if the paper's contribution is planned but not yet implemented

**Completion criterion:** Every `COVERED_BY_TYPE` label has a documented (type, module, field/bound) triple. Any label that cannot be backed is downgraded with a documented reason.

---

## Summary

| Task | Gap | Effort | Dependency |
|------|-----|--------|------------|
| Fix WfNet::attest_witnessed() | GAP_007 | Low | Independent |
| Replace E0425 fixtures | GAP_008 | Medium | Independent (benefits from GAP_001 import paths) |
| Add cross-witness fixtures | Enhancement | Low | Independent |
| Verify COVERED_BY_TYPE claims | Audit | Medium | Independent |

All four tasks are self-contained within wasm4pm-compat. None require wasm4pm changes. All can be executed before or in parallel with the GAP_001 bridge work.
