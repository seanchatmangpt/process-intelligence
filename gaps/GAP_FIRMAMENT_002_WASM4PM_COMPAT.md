---
gap: FIRMAMENT_002_WASM4PM_COMPAT
project: wasm4pm-compat
date: 2026-06-02
status: OPEN
severity: BLOCKING
gate: Horse Gate
---

# Gap: wasm4pm-compat

## Summary

The wasm4pm-compat project has manufactured substantial closure work across all 6 declared gaps but none of it has been committed to git, none of the gap-closure annotation tokens appear in git history, three projection manifests reference a source ontology that does not exist, a test writes to a hardcoded absolute path in an external repository, and the 624-fixture trybuild ALIVE gate has no passing receipt. The audit machinery (audit-gap-decomposition.sh, audit-projection-receipts.sh) reports all 6 critical gaps as UNMAPPED and all projection receipts as unverified — not because the work was not done, but because the work is invisible to the audit layer. The wall cannot accept this project as ALIVE until all manufactured artifacts are committed, annotated, and receipted, the missing ontology and templates are manufactured, the boundary violation is excised, and the Horse Gate trybuild suite produces a signed passing receipt.

## Gap Register

### GAP_WASM4PM_COMPAT_001 — 35 uncommitted files invisible to the audit layer

- **Severity:** BLOCKING
- **Category:** UNCOMMITTED_WORK
- **Specific Blocker:** `git -C /Users/sac/wasm4pm-compat status --short` shows 35 dirty entries — 4 modified source files (src/powl.rs, src/ts/export.rs, src/ts/law_projection.rs, tests/graduation.rs) and 31 untracked files including entire new crates (c8-receipts, c8-market, c8-time, c8-instruments, c8-adversary), receipts/, scripts/, docs/ additions, adapter contracts, and agent reports. The audit-gap-decomposition.sh scans only `origin/main..HEAD` (1 commit) and classifies all 6 gaps as UNMAPPED because the closure work has never been committed.
- **Remediation:** Stage and commit all manufactured artifacts in logical groups: (1) new crates c8-receipts/c8-market/c8-time/c8-instruments/c8-adversary with commit type `checkpoint`; (2) modified source files src/ts/law_projection.rs, src/ts/export.rs, src/powl.rs with commit type `research-wasm4pm`; (3) graduation.rs cross-project write change with commit type `research-wasm4pm`; (4) each adapter contract and agent report separately. Each gap-closure commit must include a `[GAP_CLOSURE: <gap_id>]` token in the commit message for the audit scanner to register it.
- **Effort:** 1–4 hours

### GAP_WASM4PM_COMPAT_002 — Zero gap-closure annotation tokens in git history

- **Severity:** BLOCKING
- **Category:** MISSING_RECEIPTS
- **Specific Blocker:** `bash ggen/audits/audit-gap-decomposition.sh ggen/emitted/gap-ledger.yaml` produces FAIL gap-unmapped-critical for all 6 gaps: GAP_001, GAP_COMPONENT, GAP_LOSS, GAP_PROCESS_TREE, GAP_TS, GAP_WASM. All closure receipt files exist in ggen/emitted/ (iter1..iter5) but zero `[GAP_CLOSURE: <id>]` tokens appear in any `git log origin/main..HEAD` output. The audit scanner finds nothing to match against.
- **Remediation:** After committing all uncommitted work (GAP_WASM4PM_COMPAT_001 resolved), create 6 explicit gap-closure commits with the required annotation tokens: `chore(hardening): [GAP_CLOSURE: GAP_001] complete wasm4pm integration bridge`, `chore(hardening): [GAP_CLOSURE: GAP_COMPONENT] implement component model projection`, `chore(hardening): [GAP_CLOSURE: GAP_LOSS] formalize loss tracking in admission`, `chore(hardening): [GAP_CLOSURE: GAP_PROCESS_TREE] add typed process tree formalization`, `chore(hardening): [GAP_CLOSURE: GAP_TS] add TypeScript projection template and queries`, `chore(hardening): [GAP_CLOSURE: GAP_WASM] implement WASM component projection`. Re-run audit-gap-decomposition.sh to confirm all 6 are CLOSED.
- **Effort:** < 1 hour

### GAP_WASM4PM_COMPAT_003 — Missing source ontology, templates, and manifest field

- **Severity:** MAJOR
- **Category:** MISSING_ARTIFACTS
- **Specific Blocker:** `bash ggen/audits/audit-projection-receipts.sh .` exits 1 with: GAP: Source ontology missing: process-intelligence.ttl (reported 3 times for ts.projection.yaml, wasm.projection.yaml, component.projection.yaml), GAP: Template missing: wasm-projection.rs.tera, GAP: Template missing: component-model.tera, GAP: Output path missing in manifest (component.projection.yaml). The ggen/ontology/ directory contains audit-machinery.ttl, domain-*.ttl, ggen-substrate.ttl, papers.ttl, and wasm4pm-compat*.ttl but no process-intelligence.ttl. Only ts-projection.rs.tera exists in ggen/templates/; both wasm and component templates are absent.
- **Remediation:** Phase 1 — Create ggen/ontology/process-intelligence.ttl by composing the existing domain TTL files (domain-evidence-structure.ttl, domain-graduation-boundaries.ttl, domain-process-forms.ttl, domain-type-constraints.ttl) into a unified ontology. Phase 2 — Create ggen/templates/wasm-projection.rs.tera modeled on the existing wasm-boundary.rs.tera and the wasm.projection.yaml spec. Phase 3 — Create ggen/templates/component-model.tera modeled on the component.projection.yaml WIT interface spec. Phase 4 — Add output_dir field to ggen/projections/component.projection.yaml. Phase 5 — Run projection manufacture and commit receipt artifacts. Re-run audit-projection-receipts.sh to confirm all receipts verified.
- **Effort:** 1–2 days

### GAP_WASM4PM_COMPAT_004 — Cross-project boundary violation in graduation.rs

- **Severity:** MAJOR
- **Category:** BOUNDARY_VIOLATION
- **Specific Blocker:** tests/graduation.rs lines 65–66 — `std::fs::write(path2, &ts_output).unwrap()` where `path2 = "/Users/sac/pcp/src/types/bindings.d.ts"` — this test fails with an unwrap panic if the pcp project does not exist at that exact absolute path on the machine running the test. The original path (/Users/sac/process-intelligence/experiments/visualizer/bindings.d.ts) is also a cross-project write but remains within the same repository. The hardcoded pcp path creates a hard machine-specific cross-project coupling that will fail in any CI environment and on any machine that does not have pcp at that path.
- **Remediation:** Remove the hardcoded pcp write from tests/graduation.rs. The test must assert only on ts_output content (which it already does) and write output to a repo-local path under target/ or a documented fixtures directory. If pcp integration is required, implement it via a separate integration script outside the test suite. At minimum, wrap the write in a conditional that only writes if the path exists and documents the skip with a println — this prevents hard panics in CI.
- **Effort:** < 1 hour

### GAP_WASM4PM_COMPAT_005 — No passing receipt for the 624-fixture trybuild Horse Gate

- **Severity:** MAJOR
- **Category:** FAILING_TESTS
- **Specific Blocker:** No committed receipt proves that `cargo +nightly test --test ui_tests -- --ignored` passes all 624 trybuild compile-fail and compile-pass fixtures. FINAL_PARTIAL_ALIVE_001.md records ALIVE Gate Status as IN PROGRESS with no completion. Trybuild results are absent from receipts/ and checkpoints/. Four nightly toolchain versions are installed (2025-11-21, 2026-03-18, 2026-04-01, 2026-04-15) and the pinned toolchain is channel = nightly (unversioned), creating version drift risk between runs.
- **Remediation:** Run `cargo +nightly test --test ui_tests -- --ignored` to completion (estimated 10–30 minutes for 624 fixtures). If all pass: create receipts/ui_tests_alive_gate.yaml with toolchain version, fixture count, pass/fail counts, and timestamp, then commit with `checkpoint: ALIVE gate ui_tests receipt — 624/624 fixtures pass`. If any fixtures fail: document each failing fixture in gaps/ and repair the type laws. Additionally, pin the nightly toolchain to a specific date in rust-toolchain.toml (e.g., nightly-2026-04-15) to prevent drift across machines and CI runs.
- **Effort:** 1–4 hours

## ALIVE Conditions Blocked

The following ALIVE conditions for the Horse Gate cannot be met until the gaps above are closed:

- **All manufactured artifacts committed** — blocked by GAP_WASM4PM_COMPAT_001. The audit layer is scanning git history; uncommitted work is invisible.
- **All 6 critical gaps registered CLOSED in git history** — blocked by GAP_WASM4PM_COMPAT_002. The annotation tokens required by audit-gap-decomposition.sh are absent.
- **Projection receipts verified** — blocked by GAP_WASM4PM_COMPAT_003. Three projection manifests reference a non-existent source ontology; two templates are missing; one manifest has a structural defect.
- **Test suite portable across machines and CI** — blocked by GAP_WASM4PM_COMPAT_004. The graduation.rs boundary violation makes the test suite non-portable.
- **trybuild Horse Gate receipt signed** — blocked by GAP_WASM4PM_COMPAT_005. The type-law certification gate has no signed passing receipt in the repository.

## Resolution Path

1. **Resolve GAP_WASM4PM_COMPAT_004 first** (< 1 hour) — Remove the hardcoded pcp write from tests/graduation.rs so the test suite is portable. This must precede all commits or the committed test suite will be broken on CI.

2. **Resolve GAP_WASM4PM_COMPAT_001** (1–4 hours) — Stage and commit all 35 uncommitted files in logical groups with conventional commit messages. Include `[GAP_CLOSURE: <id>]` tokens in the relevant commit messages for the gap-closure commits.

3. **Resolve GAP_WASM4PM_COMPAT_002** (< 1 hour, follows step 2) — After all work is committed, create the 6 explicit gap-closure annotation commits. Re-run audit-gap-decomposition.sh to confirm all 6 gaps are CLOSED.

4. **Resolve GAP_WASM4PM_COMPAT_003** (1–2 days) — Manufacture ggen/ontology/process-intelligence.ttl, ggen/templates/wasm-projection.rs.tera, ggen/templates/component-model.tera, and the output_dir field in component.projection.yaml. Run projection manufacture and commit all receipt artifacts. Re-run audit-projection-receipts.sh to confirm clean exit.

5. **Resolve GAP_WASM4PM_COMPAT_005** (1–4 hours) — Pin the nightly toolchain to a specific date in rust-toolchain.toml. Run `cargo +nightly test --test ui_tests -- --ignored` to completion. If all 624 fixtures pass, commit the signed receipt to receipts/ui_tests_alive_gate.yaml. If any fail, fix and repeat.

6. **Final audit pass** — Run both audit scripts (audit-gap-decomposition.sh, audit-projection-receipts.sh) and confirm clean exit. Verify git log shows all 6 gap-closure tokens. Confirm receipts/ contains the trybuild receipt. Issue ALIVE_001 checkpoint.

## Doctrine Note

A manufacturing pipeline that produces closure work which is invisible to its own audit layer has not manufactured anything — evidence does not exist until it is committed, annotated, and receipted in the canonical record.
