# ANDON Guard Rules — ggen-validator-recovery Phase 0

**Activation Date:** 2026-06-01  
**Authority:** Process Intelligence Research Program  
**Purpose:** Prevent unsupported claims and maintain research integrity during ggen validator recovery

---

## Mission

During the ggen v26.5.21 template validator recovery (Phases 1-3), the ANDON Guard enforces strict rules on what may and may not be claimed, committed, or advanced.

**Core Doctrine:** The validator bug is diagnosed but **not fixed**. No downstream work may proceed until fixtures are built and validator recovery strategy is confirmed.

---

## Allowed Write Surfaces

These surfaces may be modified ONLY for the stated purposes:

### 1. ggen Validator/Tooling

**Path:** `/Users/sac/ggen/crates/ggen-core/src/template/mod.rs` and related

**Allowed:**
- Diagnostic modifications to improve error messages
- Fixture test code additions
- No changes to validator logic that would suppress the error
- No workarounds that bypass validation

**Forbidden:**
- Patches that silence the validator without fixing the root cause
- Force-passing templates that fail validation
- Changes to the manifest parsing that skip template validation

### 2. ggen Tests

**Path:** `/Users/sac/ggen/tests/` and `crates/ggen-core/tests/`

**Allowed:**
- Fixture creation in `ggen/tests/fixtures/templates/`
- New test cases for mixed-language (React/Tera) validation
- Regression tests that validate parse vs. render behavior

**Forbidden:**
- Tests that depend on the broken validator passing
- Tests marked `#[ignore]` to skip validation
- Mock validators that don't use actual Tera parser

### 3. research/pi-program/emitted/ Output Directory

**Path:** `/Users/sac/process-intelligence/research/pi-program/emitted/ggen-validator-recovery/`

**Allowed:**
- Diagnostic reports (markdown)
- Fixture specs (yaml/markdown)
- This ANDON Guard document
- Recovery strategy documents
- Minimal reproducer test cases

**Forbidden:**
- Generated code from broken ggen pipeline
- Hand-written output masquerading as ggen-manufactured
- Claims about ALIVE status based on validator bypasses

### 4. ggen Source Surfaces

**Path:** Tera templates, RDF/TTL ontologies, SPARQL queries, ggen.toml configs

**Allowed:**
- Read operations to diagnose the bug
- Fixture template creation for test cases
- No modifications to existing production templates (visualizer-dashboard.tsx.tera, etc.)

**Forbidden:**
- Escaping or rewriting existing templates to work around the validator
- Modifying templates to hide the parser ambiguity
- Altering ggen.toml to skip validation

---

## Forbidden Claims

Until the ggen validator recovery is complete, these claims are **not permitted**:

### ❌ ALIVE Verdicts

**Forbidden:** Declaring any phase as ALIVE without validator fix

Rationale: The validator blocks all three pipelines. An ALIVE verdict without working validator infrastructure is not credible.

```markdown
# ❌ PROHIBITED
checkpoint: PROCESS_INTELLIGENCE_ALIVE_001
# This requires all downstream pipelines to work. Validator is broken. DENIED.
```

### ❌ File Count Signals

**Forbidden:** Claiming success based on file counts or artifact generation

Rationale: The broken validator prevents ggen from running. Any generated files are hand-written or from a previous working state.

```markdown
# ❌ PROHIBITED
Generated 12 visualizer dashboard artifacts ✓
# ggen sync doesn't even start. These don't exist. DENIED.
```

### ❌ Validator Bypasses

**Forbidden:** Committing code that works around the template validator

Rationale: The bug must be fixed upstream in ggen, not hidden downstream.

```rust
// ❌ PROHIBITED
if let Err(_) = validate_template(&content) {
    // Silently continue anyway
    eprintln!("WARNING: Validator bug, skipping check");
}
```

### ❌ Partial Evidence Claims

**Forbidden:** Claiming that fixtures or test cases represent working validator behavior

Rationale: Test fixtures are **diagnostic tools**, not proof that the validator is fixed.

```markdown
# ❌ PROHIBITED
doctrine: ggen template validator is functional
# Evidence: Created 5 fixture templates that parse correctly
# REASON: Fixtures test **expected** behavior, not current ggen v26.5.21 behavior
```

---

## Required Claims

These claims MUST appear in any Phase 1-3 work:

### ✓ Root Cause Citation

Every document in `ggen-validator-recovery/` must cite:

```markdown
**Root Cause:** Tera parser ambiguity in mixed TypeScript/Tera templates
**Failing Template:** `ggen/templates/visualizer-dashboard.tsx.tera` line 1803
**Error:** SyntaxError("Failed to parse 'test_template'") — unmatched `}}`
```

### ✓ Bug Classification

Every diagnostic report must classify the bug:

```markdown
**Classification:** Tera Parser Wrapper Bug (ggen validator design flaw)
**Category:** All-or-nothing template validation without language context
**Blocker Status:** CRITICAL — blocks all three pipelines
```

### ✓ No Workaround Claims

Any commit messages must be truthful about what was fixed:

```bash
# ✓ ALLOWED
research-compat: diagnose ggen validator parser ambiguity, build fixture suite
# Clear about what was done: diagnosis + fixtures, NOT a fix

# ❌ PROHIBITED
research-wasm4pm: fix ggen validator, enable downstream wasm4pm-compat refactor
# FALSE — validator is not fixed. Only diagnosed.
```

---

## Enforcement Checkpoints

### Checkpoint 1: Diagnostic Report (Phase 1)

**Gate:** Before committing any fixture or recovery code

**Verify:**
- [ ] validator-reproduction.md exists
- [ ] Root cause identified and classified
- [ ] Exact failing template line number cited
- [ ] No claims about validator being fixed
- [ ] No generated artifacts included (only diagnostics)

**Commit Message Required:**
```
research-compat: phase 1 - reproduce ggen v26.5.21 template validator bug

Root cause: Tera parser ambiguity in mixed TypeScript/Tera templates
Failing template: ggen/templates/visualizer-dashboard.tsx.tera:1803
Error: SyntaxError("Failed to parse 'test_template'") - unmatched }}

Output: research/pi-program/emitted/ggen-validator-recovery/validator-reproduction.md
```

### Checkpoint 2: Fixture Suite (Phase 2)

**Gate:** Before building validator recovery strategy

**Verify:**
- [ ] Fixture templates created in `ggen/tests/fixtures/templates/`
- [ ] Fixtures test **expected** behavior (not current broken behavior)
- [ ] Test cases include: valid-basic, valid-loop, valid-condition, invalid-error, mixed-react-tera
- [ ] No fixtures claim to pass the actual validator
- [ ] Fixture spec documents what each test validates

**Commit Message Required:**
```
research-compat: phase 2 - build minimal validator fixture suite

Fixtures test parse vs. render behavior:
  - valid-basic.tera: simple variable substitution
  - valid-loop.tera: Tera for loop syntax
  - valid-condition.tera: Tera if/else syntax
  - invalid-unclosed.tera: error case - missing closing }}
  - mixed-react-tera.tsx.tera: React JSX + Tera (currently fails v26.5.21)

Output: ggen/tests/fixtures/templates/
        research/pi-program/emitted/ggen-validator-recovery/fixture-report.md

Status: Fixtures demonstrate expected behavior. ggen v26.5.21 validator bug
prevents actual validation. No fix applied in this phase.
```

### Checkpoint 3: ANDON Guard Rules (This Document)

**Gate:** Before any downstream work claims to depend on validator fix

**Verify:**
- [ ] This ANDON Guard document is committed
- [ ] All team members are aware of allowed/forbidden surfaces
- [ ] No commits attempt to bypass the guards
- [ ] Forbidden claims are actively rejected

**Enforcement:** This document is canonical. Any commits violating these rules are reverted.

---

## Escalation Path

### If a Commit Violates ANDON Guard

**Step 1:** The commit is **reverted immediately**

```bash
git revert <commit-hash>
```

**Step 2:** Author is notified with reference to this document

**Step 3:** Corrected commit is submitted that complies with guards

### If the Bug Fix Proceeds Downstream Without This Recovery

**Step 1:** All downstream claims are invalidated

Example: If someone commits "fix: enable visualizer-dashboard generation", but the validator still fails:

```markdown
❌ INVALID CLAIM
This commit claims the visualizer dashboard is now generated.
But ggen sync still fails on validator gate: "SyntaxError: Failed to parse 'test_template'"
Commit reverted. Recovery must proceed through Phases 1-3 first.
```

**Step 2:** Checkpoint audits will flag the phase jump

---

## Authority & Scope

**Issued By:** Process Intelligence Research Program  
**Scope:** All work in `/Users/sac/process-intelligence/` related to ggen validator recovery  
**Scope:** All work in `/Users/sac/ggen/` related to template validator and fixtures  

**Not Applicable To:**
- Work on wasm4pm-compat (separate codebase)
- Work on other ggen features (outside validator recovery)
- Work in other users' repositories

---

## Revision History

| Date | Status | Change |
|------|--------|--------|
| 2026-06-01 | ACTIVE | Initial issuance - Phase 0 guards active |

---

## Related Documents

- `validator-reproduction.md` — Phase 1 diagnostic report
- `validator-fixture-report.md` — Phase 2 fixture specification (to be created)
- `COVENANT.md` — Parent authority doctrine (in process-intelligence root)
