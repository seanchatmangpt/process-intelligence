---
gap: FIRMAMENT_002_WASM4PM
project: wasm4pm
date: 2026-06-02
status: OPEN
severity: MAJOR
gate: Inspection Gate
---

# Gap: wasm4pm

## Summary

wasm4pm holds an ALIVE status reached on a feature branch (finish-wip-primitives) with five unresolved caveats that collectively undermine the provenance, accuracy, and reproducibility of that verdict. The admission gate receipt contains a placeholder Git SHA, references a test file that does not exist, and reports a test count that does not match the actual test suite. The project requires a nightly Rust toolchain due to a dependency on generic_const_exprs, meaning the stable toolchain fails entirely. The default algorithm (simd_streaming_dfg) produces a shape mismatch error on the canonical small.xes fixture. Until these caveats are resolved, the ALIVE claim is unverifiable and the project cannot be admitted to the doctoral defense corpus.

## Gap Register

### GAP_WASM4PM_CAVEAT_001 — Incomplete provenance receipt: Git SHA placeholder never finalized

- **Severity:** MAJOR
- **Category:** MISSING_ARTIFACTS
- **Specific Blocker:** ADMISSION_GATE_RECEIPT.md contains the string '(to be confirmed after commit)' in the Git SHA field, meaning the receipt was authored before the commit it attests to and was never updated with the actual commit hash. The receipt is therefore an incomplete provenance document and cannot serve as a valid proof gate artifact.
- **Remediation:** Locate the actual commit hash for the commit the receipt is intended to attest, update the Git SHA field with the real value, and commit the corrected receipt under a new checkpoint commit so the receipt attests a real, traceable object.
- **Effort:** 1-4 hours

### GAP_WASM4PM_CAVEAT_002 — Receipt test count mismatch and non-existent referenced file

- **Severity:** MAJOR
- **Category:** MISSING_ARTIFACTS
- **Specific Blocker:** The admission gate receipt claims '9 conformance tests fixed' but the actual test file (mcpp-admission-gate.test.ts) contains 42 passing tests. The file path cited in the receipt (conformance-mcpp-admission.test.ts) does not exist in the repository. The receipt attests to a test state that cannot be reproduced or audited.
- **Remediation:** Audit the actual test file layout, determine the correct test file path and passing count, rewrite the receipt with accurate values, and verify the corrected receipt against the live test run output before committing.
- **Effort:** 1-4 hours

### GAP_WASM4PM_CAVEAT_003 — Stable toolchain build failure: nightly-only dependency

- **Severity:** MAJOR
- **Category:** MISSING_ARTIFACTS
- **Specific Blocker:** Running `cargo test` on the stable Rust toolchain fails with E0554 because wasm4pm-compat requires the `generic_const_exprs` feature, which is gated behind nightly. The project compiles and tests only under nightly. This is an undocumented toolchain constraint that makes the build non-reproducible on standard CI environments and any machine without nightly installed.
- **Remediation:** Either pin an explicit nightly toolchain in rust-toolchain.toml with a known-good nightly date, or refactor the wasm4pm-compat dependency to remove the generic_const_exprs usage so the project builds on stable. Document the toolchain requirement explicitly in CLAUDE.md and any admission gate receipt.
- **Effort:** 1-4 hours

### GAP_WASM4PM_CAVEAT_004 — Default algorithm fails on canonical fixture

- **Severity:** MAJOR
- **Category:** MISSING_ARTIFACTS
- **Specific Blocker:** The default `wpm run` command invokes the simd_streaming_dfg algorithm, which produces a shape mismatch error when run against the small.xes fixture. Only non-default algorithms (inductive, heuristic) succeed. The default execution path is broken, meaning a user following the standard invocation pattern cannot reproduce the ALIVE result.
- **Remediation:** Either fix the simd_streaming_dfg algorithm to handle the small.xes fixture correctly, or change the default algorithm to one that succeeds on the canonical fixture and document the change. Update any ALIVE receipt or checkpoint that references successful default execution.
- **Effort:** 1-4 hours

### GAP_WASM4PM_CAVEAT_005 — ALIVE status on feature branch, not main

- **Severity:** MAJOR
- **Category:** MISSING_ARTIFACTS
- **Specific Blocker:** ALIVE status was declared and checkpointed on the branch finish-wip-primitives. The project's main branch does not contain this work. ALIVE verdicts issued against a feature branch are not stable: the branch may diverge, be rebased, or remain unmerged indefinitely, and downstream consumers relying on main cannot reproduce the ALIVE state.
- **Remediation:** Merge finish-wip-primitives into main after all other caveats are resolved, re-run the full ALIVE gate verification on main, and issue a new checkpoint commit attesting to ALIVE status on the main branch.
- **Effort:** 1-4 hours

## ALIVE Conditions Blocked

The following ALIVE conditions cannot be met until these gaps are closed:

1. **Provenance completeness** — GAP_WASM4PM_CAVEAT_001 blocks: no receipt can be admitted to the proof chain with a placeholder SHA field.
2. **Test count accuracy** — GAP_WASM4PM_CAVEAT_002 blocks: the stated conformance test count is demonstrably incorrect and the cited file does not exist, invalidating the conformance attestation.
3. **Stable reproducibility** — GAP_WASM4PM_CAVEAT_003 blocks: ALIVE requires independent replay on a reproducible build environment; nightly-only builds are not reproducibly stable across time.
4. **Default path correctness** — GAP_WASM4PM_CAVEAT_004 blocks: an ALIVE project must execute its default command successfully on the canonical test fixture.
5. **Main branch canonicity** — GAP_WASM4PM_CAVEAT_005 blocks: ALIVE verdicts must be anchored to the canonical branch; feature-branch verdicts are not admissible as final status.

## Resolution Path

1. Resolve GAP_WASM4PM_CAVEAT_003 first — stabilize the toolchain by pinning nightly in rust-toolchain.toml so all subsequent verification steps run against a reproducible build.
2. Resolve GAP_WASM4PM_CAVEAT_004 — fix or replace the simd_streaming_dfg default algorithm so `wpm run` succeeds on small.xes.
3. Resolve GAP_WASM4PM_CAVEAT_002 — audit the actual test file path and passing count, then rewrite the receipt with accurate values and verify against a live test run.
4. Resolve GAP_WASM4PM_CAVEAT_005 — merge finish-wip-primitives into main after steps 1-3 are complete and verified.
5. Resolve GAP_WASM4PM_CAVEAT_001 — after the merge commit exists on main, update ADMISSION_GATE_RECEIPT.md with the actual commit SHA and commit the corrected receipt.
6. Re-run the full ALIVE gate verification on main and issue a new checkpoint commit (C8_WASM4PM_ALIVE_002 or equivalent) attesting to ALIVE status with all five caveats resolved.

## Doctrine Note

A receipt that cannot be independently verified against a real commit hash, a real file, and a real test count is not a receipt — it is a claim, and claims without evidence are inadmissible under the Van der Aalst Constitution.
