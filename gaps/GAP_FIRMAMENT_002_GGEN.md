---
gap: FIRMAMENT_002_GGEN
project: ggen
date: 2026-06-02
status: OPEN
severity: BLOCKING
gate: Dung Gate
---

# Gap: ggen

## Summary

ggen cannot achieve an ALIVE verdict in its current state. The workspace fails the `cargo clippy --workspace --all-targets -- -D warnings` gate with 17 violations across ggen-core, 2 fixture-validation tests fail due to Tera 1.20.1 grammar incompatibilities in 15 production templates, 6 files (including a functional change to syntax_validator.rs) have never been committed, and the GALL-CONFORM-001 conformance plan has no completion receipt despite its mine test passing. Until these gates clear, ggen is not a defensible artifact: its manufacturing pipeline cannot be proved conformant, its template corpus cannot be proved parseable, and its proof chain has an unwitnessed gap.

## Gap Register

### GAP_GGEN_001 — 17 clippy violations in ggen-core block the workspace lint gate

- **Severity:** BLOCKING
- **Category:** FAILING_TESTS
- **Specific Blocker:** `cargo clippy --workspace --all-targets -- -D warnings` exits non-zero: `error: could not compile 'ggen-core' (lib) due to 17 previous errors`. Violations span 10 files: unwrap() in show.rs:133; too-many-arguments (8/7) in genesis.rs:570; elided lifetimes in genesis.rs:640,650; format!-from-iter in graph/construct.rs:159; Drop-in-if-let in graph/core.rs:484,498; struct-excessive-bools in lifecycle/dx.rs:15, template_types.rs:42, security/logging.rs:68; panic! in prompt_mfg/emitter.rs:136, prompt_mfg/mod.rs:156, rdf/template_metadata.rs:604, templates/frozen.rs:483-485, types/path_protection.rs:174; underscore-prefixed binding in telemetry.rs:97.
- **Remediation:** Fix each of the 17 violations: (1) Replace unwrap() at show.rs:133 with proper error propagation; (2) Extract genesis.rs:570 Construct8::new 8-arg function into a Builder struct; (3) Elide lifetimes at genesis.rs:640,650; (4) Replace format!() iterator at construct.rs:159 with collect(); (5) Bind the Drop-holding value before the if-let at core.rs:484,498; (6) Convert excessive-bool structs to enums/bitflags at dx.rs:15, template_types.rs:42, security/logging.rs:68; (7) Replace panic! with Result returns or unwrap_or_else with graceful fallback at all 6 panic sites; (8) Remove underscore prefix from telemetry.rs:97 or suppress with #[allow] with documented reason.
- **Effort:** 1-4 hours

### GAP_GGEN_002 — 2 failing tests in fixture_validation_proof.rs due to Tera 1.20.1 grammar and error-format mismatches

- **Severity:** BLOCKING
- **Category:** FAILING_TESTS
- **Specific Blocker:** `tests/fixture_validation_proof.rs`: (1) `test_all_active_project_templates_parse` fails because 15 of 47 .tera templates fail Tera 1.20.1 `add_raw_template()` parsing — root cause is pipe-inside-function-arg syntax such as `default(value=now() | date(format="..."))` and `default(value=changes | length)` which is invalid Tera grammar; (2) `test_missing_context_variable_classified` fails because `extract_missing_variables()` checks for the string `'undefined variable'` but Tera 1.20.1 emits `'Variable \`x\` not found in context while rendering'`, and `e.to_string()` returns only the outer `'Failed to render'` wrapper, not the inner cause.
- **Remediation:** For `test_all_active_project_templates_parse`: fix all 15 failing templates by rewriting pipe-inside-function-arg expressions — e.g. change `default(value=now() | date(format="..."))` to `{% set ts = now() | date(format="...") %}{{ ts | default(value="N/A") }}`. Confirm the full list by running the test with RUST_LOG=trace. For `test_missing_context_variable_classified`: change `extract_missing_variables()` at tests/fixture_validation_proof.rs:144 to match `'not found in context'` AND traverse the full error chain using `e.source()` to reach the inner Tera error before calling `to_string()`.
- **Effort:** 1-4 hours

### GAP_GGEN_003 — Uncommitted test file, fixtures directory, and 4 modified source files have no commit receipt

- **Severity:** BLOCKING
- **Category:** UNCOMMITTED_WORK
- **Specific Blocker:** `git status` shows: `?? tests/fixture_validation_proof.rs`, `?? tests/fixtures/templates/`, `M crates/ggen-core/src/validation/syntax_validator.rs`, `M crates/ggen-core/src/genesis.rs`, `M crates/ggen-core/src/domain/generation/headers.rs`, `M crates/ggen-cli/tests/sync_command_test.rs`. The syntax_validator.rs modification is a functional change (Tera syntax validation added via gray_matter); genesis.rs and headers.rs are rustfmt reformatting only.
- **Remediation:** After fixing GAP_GGEN_001 and GAP_GGEN_002: (1) run `cargo fmt --all` to normalize all formatting; (2) stage and commit tests/fixture_validation_proof.rs, tests/fixtures/templates/, and the 4 modified source files under a single conventional commit (e.g. `test(validation): add Tera template parse proof test`). The syntax_validator.rs functional change must be documented in the commit body with evidence that it does not regress existing tests.
- **Effort:** < 1 hour

### GAP_GGEN_004 — GALL-CONFORM-001 has no completion receipt; conformance plan is unwitnessed

- **Severity:** BLOCKING
- **Category:** MISSING_RECEIPTS
- **Specific Blocker:** `docs/receipts/GALL_CONFORM_001_PRE_INVENTORY.md` exists but `docs/receipts/GALL_CONFORM_001_RECEIPT.md` does not. `GGEN_FINISH_GAPS_RECEIPT.md` §conform-plan verdict: BLOCKED/RED as of 2026-05-30. The `intel::mine::tests::mine_promotes_a_conformant_route_with_measured_success` test passes in current HEAD but there is no committed receipt proving: (a) clippy/fmt are green for ggen-lsp after conform-plan changes, (b) the 6-link OCEL chain is mineable via the real wpm oracle, or (c) the ocel-core workspace-manifest ANDON (ambiguous git+path dep) was resolved.
- **Remediation:** Re-run the 4-gate proof for CONFORM-001: (1) `cargo make check` green; (2) `cargo test -p ggen-lsp` all pass (mine test confirmed green, but re-verify after GAP_GGEN_001 clippy fixes); (3) `cargo clippy -p ggen-lsp --no-deps -- -D warnings` exit 0; (4) `cargo fmt -p ggen-lsp -- --check` exit 0. Verify the ocel-core Cargo.toml dep is single-source (either git OR path, not both). Then write `docs/receipts/GALL_CONFORM_001_RECEIPT.md` with gate tails and commit.
- **Effort:** 1-4 hours

### GAP_GGEN_005 — 15 production templates contain invalid Tera 1.20.1 syntax and cannot be parsed

- **Severity:** MAJOR
- **Category:** MISSING_ARTIFACTS
- **Specific Blocker:** `cargo test --test fixture_validation_proof test_all_active_project_templates_parse` — 15 templates emit `'Failed to parse _test'` at Tera `add_raw_template()` time. Affected templates: cli-command.tera, ontology-diff-report.tera, c4-component-diagrams.tera, receipt-report.tera, dod-compliance-report.tera, code-review-prompt.tera, dod-checklist.tera, erlang-adapter.tera, type-registry.tera, kubernetes-deployment.tera, runbook-template.tera, openapi-from-registry.tera, slo-dashboard.tera, rust-struct-from-ontology.tera, ontology-explorer-dashboard.tera. Primary root cause: pipe-inside-function-argument expressions; secondary cause: `map(attr='value')` uses wrong argument name (should be `attribute`).
- **Remediation:** For each of the 15 failing templates: (1) Replace all `default(value=expr | filter(...))` patterns with a preceding `{% set tmp = expr | filter(...) %}` then `{{ var | default(value=tmp) }}`; (2) Replace all `map(attr='...')` with `map(attribute='...')`; (3) Run the fixture_validation_proof test after each fix to confirm the template is removed from the failure list. Commit the template fixes as a separate `fix(templates): repair invalid Tera syntax in 15 production templates` commit.
- **Effort:** 1-4 hours

### GAP_GGEN_006 — 6 debug/scratch files are untracked, polluting git status and future ANDON analysis

- **Severity:** MINOR
- **Category:** UNCOMMITTED_WORK
- **Specific Blocker:** `git status` shows: `?? analyze_failing_template.py`, `?? debug_tera_error.rs`, `?? test_cli_command.rs`, `?? test_map_filter.rs`, `?? test_tera_parse.rs`, `?? crates/ggen-core/tests/tera_template_validator.rs`. These were created during investigation of template parse failures but are neither committed nor gitignored.
- **Remediation:** Decide: either (a) add all 6 debug/scratch files to .gitignore if they are not intended as committed artifacts, or (b) promote `crates/ggen-core/tests/tera_template_validator.rs` to a committed test if it adds coverage beyond fixture_validation_proof.rs. The 5 root-level debug scripts should be gitignored or deleted.
- **Effort:** < 1 hour

## ALIVE Conditions Blocked

The following ALIVE conditions cannot be met until the gaps above are closed:

- **Lint gate** — `cargo clippy --workspace --all-targets -- -D warnings` must exit 0. Blocked by GAP_GGEN_001.
- **Test gate** — all tests in the workspace must pass. Blocked by GAP_GGEN_002 (2 fixture-validation test failures).
- **Template corpus parseability** — all production templates must be parseable by the runtime Tera engine. Blocked by GAP_GGEN_005 (and indirectly by GAP_GGEN_002).
- **Proof chain completeness** — every manufacturing stage must have a committed receipt. Blocked by GAP_GGEN_004 (GALL-CONFORM-001 completion receipt absent).
- **Clean working tree** — no uncommitted source changes or untracked test files may exist at checkpoint time. Blocked by GAP_GGEN_003 and GAP_GGEN_006.

## Resolution Path

Execute in this order to bring ggen from current state to ALIVE:

1. **Fix GAP_GGEN_001** — Resolve all 17 clippy violations in ggen-core. Confirm `cargo clippy --workspace --all-targets -- -D warnings` exits 0.
2. **Fix GAP_GGEN_005** — Repair all 15 production templates with invalid Tera syntax. Commit as `fix(templates): repair invalid Tera syntax in 15 production templates`.
3. **Fix GAP_GGEN_002** — Update `extract_missing_variables()` to match Tera 1.20.1 error format. Confirm both fixture-validation tests pass.
4. **Fix GAP_GGEN_003** — Run `cargo fmt --all`, stage and commit all 6 uncommitted files under a single `test(validation): add Tera template parse proof test` commit.
5. **Fix GAP_GGEN_006** — Gitignore or delete the 6 debug/scratch files; promote tera_template_validator.rs if warranted.
6. **Fix GAP_GGEN_004** — Re-run the 4-gate CONFORM-001 proof, resolve the ocel-core dep ambiguity, write and commit `docs/receipts/GALL_CONFORM_001_RECEIPT.md`.
7. **Verify full workspace** — Run `cargo make check` and confirm all tests pass, lint is green, and git status is clean.
8. **Issue GGEN_ALIVE_001 checkpoint receipt** — Write and commit the checkpoint document under `checkpoints/` and `receipts/`.

## Doctrine Note

A manufacturing pipeline whose template corpus cannot be parsed by its own runtime engine has not manufactured anything — it has only claimed to.
