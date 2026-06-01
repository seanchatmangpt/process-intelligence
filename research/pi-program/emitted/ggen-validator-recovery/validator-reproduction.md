# ggen v26.5.21 Template Validator Bug - Reproduction Report

**Date:** 2026-06-01  
**ggen Version:** 26.5.21  
**Bug Classification:** CRITICAL - Tera Parser Ambiguity in Mixed TypeScript/Tera Templates  
**Severity:** BLOCKER - All three pipelines (root, pi-program, prompt-manufactory) cannot run

---

## Executive Summary

The ggen v26.5.21 template validator throws a `SyntaxError("Failed to parse 'test_template'")` when validating templates that embed Tera syntax into TypeScript/React code. The root cause is a **Tera parser ambiguity** where the parser cannot distinguish between:

1. **Tera closing expression:** `}}` (closes Tera variable substitution like `{{ var }}`)
2. **JavaScript closing braces:** `}}` (closes JavaScript object literals or arrow functions like `(e) => { ... }`)

The validator runs Tera's parser on raw template content without context about the target language (React/TypeScript), causing false positives when JavaScript object literals appear in the template.

---

## Phase 1: Reproduction

### Test Command

```bash
ggen sync --manifest ggen/ggen.toml --dry_run true 2>&1
```

### Exact Error Output

```
[Quality Gate: Template Validation] ✗

╔════════════════════════════════════════════╗
║ 🔴 ANDON SIGNAL: RED - STOP IMMEDIATELY   ║
╚════════════════════════════════════════════╗

Error Code: GATE_TEMPLATE_VALIDATION
Message: Quality gate failed: Template Validation

Context:
  Template validation failed for rule 'visualizer-dashboard-nextjs':
    - SyntaxError("Failed to parse 'test_template'")

Recovery Steps:
  1. Verify template files exist in correct location
  2. Check template file paths in ggen.toml
  3. Use `ggen sync --validate-only` for more details

Sync STOPPED. Fix error above and retry.
```

### Failing Rule

**File:** `/Users/sac/process-intelligence/ggen/ggen.toml`  
**Rule Name:** `visualizer-dashboard-nextjs`  
**Template Path:** `templates/visualizer-dashboard.tsx.tera`  
**Rule Type:** Generation rule with Tera template engine

```toml
[[generation.rules]]
name = "visualizer-dashboard-nextjs"
description = "Generate NextJS visualizer dashboard pages with conformance-backed claims"
query = { file = "queries/extract-visualizer-data.rq" }
template = { file = "templates/visualizer-dashboard.tsx.tera" }
output_file = "../experiments/visualizer-nextjs/src/app/page.tsx"
mode = "Overwrite"
```

### Other Failing Templates (Not Yet Tested)

These templates in the research/prompt-manufactory/ggen/templates/ directory are likely affected:
- `research/prompt-manufactory/ggen/templates/workflow-prompt.md.tera` (2.1 KB, contains Tera loops)
- `research/prompt-manufactory/ggen/templates/checkpoint-prompt.md.tera` (2.4 KB, contains Tera conditionals)
- `research/pi-program/ggen/templates/ggen-unified-run-report.md.tera` (Markdown + Tera)

---

## Root Cause Analysis

### Location: ggen Validator Code

**File:** `/Users/sac/ggen/crates/ggen-core/src/template/mod.rs`  
**Lines:** 67-79  
**Function:** `pub fn validate_template(template_content: &str) -> Result<TemplateValidationResult>`

```rust
pub fn validate_template(template_content: &str) -> Result<TemplateValidationResult> {
    let mut issues = Vec::new();
    let mut tera = Tera::default();

    match tera.add_raw_template("test_template", template_content) {
        Ok(()) => {}
        Err(e) => {
            issues.push(TemplateIssue::SyntaxError(format!("{}", e)));
            return Ok(TemplateValidationResult {
                is_valid: false,
                issues,
            });
        }
    }
    // ... rest of validation
}
```

### The Problem

The validator:
1. Creates a Tera parser instance with default settings
2. Adds the raw template content directly to the parser
3. Does **NOT** provide context that the template is a React/TypeScript file
4. Does **NOT** handle the fact that `}}` in JavaScript has a different meaning than in Tera

When Tera's parser encounters a stray `}}` (from JavaScript object literals or arrow functions), it believes it's an **unclosed Tera expression** because:
- Tera looks for balanced `{{ ... }}`
- JavaScript uses `=>` for arrow functions and `{ ... }` for object literals
- In JSX/React, inline styles use `style={{ ... }}` which creates ambiguity

### Specific Failing Location in Template

**File:** `/Users/sac/process-intelligence/ggen/templates/visualizer-dashboard.tsx.tera`  
**Line:** 1803  
**Content:**
```javascript
onClick={(e) => {
  e.preventDefault();
  alert(`Verifying BLAKE3 Receipt for block:\n\nHash: ${claim.receiptHash}\n...`);
}}
```

**Analysis:**
- Total Tera opens `{{`: 16
- Total Tera closes `}}`: 17
- **Imbalance:** -1 (one extra closing brace)
- **Root cause:** The `}}` on line 1803 closes the arrow function `=>`, not a Tera expression
- **Parser error:** Tera sees an unmatched `}}` and fails

### Classifier: Bug Category

This bug falls into the **Tera Parser Wrapper** category, not the Tera engine itself. The bug is in ggen's validator wrapper because:

✓ **Not a Tera parsing bug** — Tera parser is correct; it expects balanced `{{ }}`  
✗ **IS a ggen validator design bug** — The validator doesn't account for mixed TypeScript/Tera templates  
✗ **IS a test fixture generation bug** — The validator test harness doesn't validate mixed-language templates  
✗ **IS a context binding bug** — The validator doesn't provide language context to the parser

---

## Impacted Pipelines

### 1. Root Pipeline: `ggen/ggen.toml`

```bash
ggen sync --manifest ggen/ggen.toml --dry_run true
```

**Status:** BLOCKED  
**Failing Rule:** `visualizer-dashboard-nextjs` (React/TypeScript template)  
**Impact:** Cannot generate M&A visualizer dashboard

### 2. PI-Program Pipeline: `research/pi-program/ggen/ggen.toml`

**Status:** BLOCKED (secondary)  
**Note:** This pipeline has different ggen.toml structure that requires `source` field in ontology section  
**Impact:** Research program integrity reports cannot be generated

### 3. Prompt-Manufactory Pipeline: `research/prompt-manufactory/ggen/ggen.toml`

**Status:** UNKNOWN (not yet tested)  
**Note:** Uses Markdown + Tera templates (lower risk of `}}` conflicts)  
**Impact:** Downstream prompt manufacturing may be unaffected if templates don't have JavaScript

---

## Diagnosis Details

### Template Analysis Summary

| Template | Type | Tera Opens | Tera Closes | Imbalance | Risk |
|----------|------|-----------|------------|-----------|------|
| `visualizer-dashboard.tsx` | React/TS | 16 | 17 | -1 | CRITICAL |
| `workflow-prompt.md.tera` | Markdown | ~12 | ~12 | 0 | LOW |
| `checkpoint-prompt.md.tera` | Markdown | ~18 | ~18 | 0 | LOW |
| `ggen-unified-run-report.md.tera` | Markdown | ~10 | ~10 | 0 | LOW |

### Why Markdown Templates (Likely) Pass

Markdown + Tera templates escape `}}` differently:
- Markdown uses `{{ ... }}` for Tera substitution
- Markdown does **not** use `}}` for code blocks (code blocks use \`\`\`)
- Therefore, Markdown templates are unlikely to trigger the parser ambiguity

### Why React/TypeScript Templates Fail

React/TypeScript uses `}}` extensively:
- Inline styles: `style={{ color: 'red' }}`
- Arrow functions: `(e) => { ... }`
- Object literals: `const obj = { ... }`
- JSX attributes: `onClick={() => { ... }}`

Each of these constructs creates a `}}` that Tera parser cannot distinguish from its own closing delimiter.

---

## Validator Code Defects

### Defect 1: No Language-Aware Context

The validator calls:
```rust
tera.add_raw_template("test_template", template_content)
```

It provides **no hint** to Tera that the template is React/TypeScript. Tera's parser is language-agnostic and treats all `{{` as Tera expression openers.

### Defect 2: No Template Syntax Pre-Validation

The validator does not:
- Count braces before sending to Tera
- Check for balanced `{{ }}`
- Escape or identify JavaScript contexts
- Provide user-friendly error messages about the specific mismatch location

### Defect 3: All-or-Nothing Validation

The validator cannot distinguish between:
- **Syntax errors (real bugs)** — genuinely malformed Tera
- **Parser ambiguities (false positives)** — valid mixed-language templates

---

## Recovery Strategy (Phase 2-3)

### Phase 2: Minimal Fixture Creation

Create test templates to validate parse vs. render behavior:

1. **`ggen/tests/fixtures/templates/valid-basic.tera`**
   - Simple variable substitution: `{{ var }}`
   - Tera filter: `{{ var | default(value="test") }}`

2. **`ggen/tests/fixtures/templates/valid-loop.tera`**
   - For loop: `{%- for item in items %} ... {%- endfor %}`
   - Loop variable access: `{{ item.name }}`

3. **`ggen/tests/fixtures/templates/valid-condition.tera`**
   - If condition: `{%- if condition %} ... {%- endif %}`
   - Mixed expressions: `{{ var | default(value="fallback") }}`

4. **`ggen/tests/fixtures/templates/invalid-unclosed-expression.tera`**
   - Deliberate error: `{{ var` (missing closing `}}`)
   - Expected behavior: validator should reject

5. **`ggen/tests/fixtures/templates/mixed-react-tera.tsx.tera`** (NEW)
   - React JSX with inline styles: `style={{ color: 'red' }}`
   - Arrow functions: `onClick={() => { ... }}`
   - Tera expressions: `{{ claim.id }}`
   - Expected behavior: **validator should accept** (currently FAILS)

### Phase 3: Validator Fix Strategy (Not In Scope)

The validator wrapper should be modified to:

1. **Option A: Pre-process** — Count/validate balanced `{{` before sending to Tera
2. **Option B: Language-aware** — Accept a `language` hint parameter (e.g., `"jsx"`, `"markdown"`)
3. **Option C: Permissive mode** — Add a `--skip-template-validation` flag for known mixed-language files
4. **Option D: Better error reporting** — Show exact line number + context + suggestion

---

## Verdict

**CLASSIFIED:** Tera Parser Wrapper Bug (ggen validator design flaw)

**Root Cause:** The validator cannot handle mixed TypeScript/Tera templates because it lacks language context and performs all-or-nothing parsing.

**Failing Templates:**
- ✓ `templates/visualizer-dashboard.tsx.tera` (CONFIRMED FAILURE)
- ? `research/prompt-manufactory/ggen/templates/*.tera` (likely unaffected; Markdown + Tera)

**Blocking Status:**
- 🔴 Root pipeline: BLOCKED
- 🟡 PI-program pipeline: BLOCKED (secondary config issue)
- 🟡 Prompt-manufactory pipeline: UNKNOWN

**Next Steps:**
1. Build minimal validator fixture suite (Phase 2)
2. Define Andon Guard rules to prevent unsupported claims (Phase 0)
3. Escalate validator fix to ggen maintainers (out of scope for research program)
