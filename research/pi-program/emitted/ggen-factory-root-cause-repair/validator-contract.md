# Validator Contract Specification

**Authority:** Process Intelligence Research Directorate  
**Issued:** 2026-06-01  
**Phase:** PHASE 2 (Validator Contract Definition)  
**Binding Status:** FROZEN (supersedes any ggen v26.5.21 validator behavior)  
**Validation Mode:** Pre-implementation contract (written before code changes)

---

## Contract Authority

This contract specifies the exact behavior of the Tera template validator component in the PI GGEN manufacturing pipeline.

**Scope:** All Tera template validation logic (parse, render, output) in ggen v26.5.21 and forward.

**Binding:** Any validator code changes must follow this contract or require a new amendment with Process Intelligence Research Directorate authority.

**Exception:** Bug fixes to contract implementation are permitted only if they narrow (not expand) validator scope. Scope expansions require contract amendment.

---

## Validator Responsibility

The validator evaluates Tera template correctness in two sequential phases:

1. **PARSE VALIDATION** — Syntax and template structure (no runtime binding required)
2. **RENDER VALIDATION** — Runtime context availability and template rendering

---

## PARSE VALIDATION States

### State: TEMPLATE_PARSE_PASS

**Definition:** Tera template syntax is valid and structurally correct.

**Conditions:**
- Template file is readable (not corrupted, not truncated)
- Tera parser accepts syntax without errors
- No syntax errors detected in: tags, filters, expressions, whitespace
- Template structure is well-formed (no mismatched delimiters, no unclosed blocks)

**Evidence:**
- Tera parser returns OK (no SyntaxError)
- Parser output: "valid template" or equivalent

**Gate Reference:** Gate 3 (Active Templates Parse)

**Example:**
```tera
Dear {{ customer_name }},

Thank you for your order:
{% for item in items %}
  - {{ item.name }}: ${{ item.price }}
{% endfor %}

Best regards,
Sales Team
```
→ TEMPLATE_PARSE_PASS (valid Tera syntax)

---

### State: TEMPLATE_PARSE_FAIL

**Definition:** Tera template syntax is invalid and structurally malformed.

**Conditions:**
- Tera parser rejects template with SyntaxError
- Error type: malformed tag, invalid filter, unclosed block, bad expression, or invalid whitespace
- Parser cannot proceed to rendering phase

**Evidence:**
- Tera parser returns error code / SyntaxError exception
- Error message: Identifies specific syntax violation (e.g., "unclosed tag", "invalid filter")

**Gate Reference:** Gate 3 (Active Templates Parse), Gate 2 (Invalid test case documentation)

**Example:**
```tera
Dear {{ customer_name,

{# unclosed tag and invalid syntax #}
{% for item in items
  - {{ item.name }}
```
→ TEMPLATE_PARSE_FAIL (Tera parser rejects; syntax error)

---

## RENDER VALIDATION States

### State: TEMPLATE_RENDER_PASS

**Definition:** Template parses successfully AND renders to complete output with all context variables bound.

**Conditions:**
- TEMPLATE_PARSE_PASS condition satisfied ✓
- Rendering context provided (explicit sample data or production data)
- All template variables exist in context (no missing keys)
- All template filters are applicable to their values (no type mismatches)
- Rendering completes without error
- Output is non-empty and well-formed

**Evidence:**
- Tera renderer returns OK (no error)
- Rendered output is captured and validated
- Context coverage: all template variables are bound in context data

**Gate Reference:** Gate 9 (At Least One ggen Pipeline Executed), Gate 10 (Prompt Manufactory Warrant Path Executed), Gate 11 (Receipts Emitted)

**Example:**
```tera
Dear {{ customer_name }},
Total: ${{ order.total }}
```

With context:
```json
{
  "customer_name": "Alice",
  "order": { "total": 150.00 }
}
```
→ TEMPLATE_RENDER_PASS (all variables bound, output: "Dear Alice, Total: $150.00")

---

### State: TEMPLATE_RENDER_FAIL

**Definition:** Template parses successfully but rendering fails (context error, filter mismatch, or renderer bug).

**Conditions:**
- TEMPLATE_PARSE_PASS condition satisfied ✓
- Rendering context provided, but rendering fails due to:
  - Filter application error (e.g., applying string filter to non-string)
  - Variable type mismatch (e.g., iterating over non-iterable)
  - Renderer exception or crash
  - Invalid context data structure (malformed JSON, circular reference, etc.)

**Evidence:**
- Tera renderer returns error (not SyntaxError, but RenderError or FilterError)
- Error message identifies the rendering failure (not parse failure)
- Context was provided but was incompatible

**Gate Reference:** Gate 10 (Prompt Manufactory Warrant Path), Gate 11 (Receipts)

**Example:**
```tera
Total items: {{ items | length }}
```

With context:
```json
{
  "items": "not_an_array"  # ← Type mismatch
}
```
→ TEMPLATE_RENDER_FAIL (filter "length" expects array, got string)

---

### State: TEMPLATE_CONTEXT_MISSING

**Definition:** Template parses successfully but context is incomplete or not provided (missing required variable).

**Conditions:**
- TEMPLATE_PARSE_PASS condition satisfied ✓
- Rendering context not provided, OR
- Context provided but missing required template variable(s)
- Examples:
  - {{ customer_name }} in template but not in context
  - {{ order.total }} in template but context missing "order" key
  - {{ items | length }} in template but "items" not in context

**Evidence:**
- Tera renderer returns error: "undefined variable: X" or "key not found: Y"
- Error type: variable/key missing, not syntax or type error
- Indicates missing context, not validator bug

**Important Distinction:** This is NOT a PARSE error. Syntax is valid, but context is incomplete.

**Gate Reference:** Gate 2 (Invalid test case), Gate 10 (Warrant Path), Gate 11 (Receipts)

**Example:**
```tera
Dear {{ customer_name }},
Your account balance: ${{ account.balance }}
```

With context:
```json
{
  "customer_name": "Bob"
  # ← Missing "account" key
}
```
→ TEMPLATE_CONTEXT_MISSING (context incomplete, variable "account" not provided)

---

### State: TEMPLATE_OUT_OF_SCOPE

**Definition:** Template file exists but is not active in any ggen generation rule.

**Conditions:**
- Template file (.tera) found in source tree
- No generation rule references this template
- Template is either:
  - Legacy code (not used in current manufacturing)
  - Orphaned (prior refactoring left it behind)
  - Test/example fixture (not production)

**Evidence:**
- File exists on disk (e.g., sources/ggen/templates/unused-template.tera)
- Zero references in generation rule manifests or gate evidence

**Gate Reference:** Gate 6 (Every Legacy .ggen Classified)

**Note:** OUT_OF_SCOPE templates are not validator errors. They are project management issues (unused code). Document in MANIFEST.md with status "OUT_OF_SCOPE — removed from rule set as of [date]".

**Example:**
```
sources/ggen/templates/old-deprecated-report.tera
```
→ TEMPLATE_OUT_OF_SCOPE (not referenced by any generation rule)

---

## Validation Flow (Contract Sequence)

```
1. PARSE VALIDATION (no context required)
   ├─ Read template file
   ├─ Pass to Tera parser
   ├─ Outcome: TEMPLATE_PARSE_PASS or TEMPLATE_PARSE_FAIL
   └─ If PARSE_FAIL → STOP (cannot proceed to render)

2. RENDER VALIDATION (requires context)
   ├─ Retrieve sample/production context
   ├─ If context not available
   │  └─ Outcome: TEMPLATE_CONTEXT_MISSING (not an error, just missing data)
   ├─ If context available
   │  ├─ Pass template + context to Tera renderer
   │  ├─ Outcome: TEMPLATE_RENDER_PASS or TEMPLATE_RENDER_FAIL
   │  └─ Capture rendered output

3. CLASSIFICATION (out-of-scope check)
   ├─ Check if template is active in any rule
   ├─ If not active
   │  └─ Classify as TEMPLATE_OUT_OF_SCOPE
   └─ Document in MANIFEST.md
```

---

## Validator Output States (Summary)

| State | Requirement | Context | Error Type | Gate Impact |
|-------|-------------|---------|-----------|-------------|
| TEMPLATE_PARSE_PASS | Tera syntax valid | Not required | None | Gate 3 ✓ |
| TEMPLATE_PARSE_FAIL | Tera syntax invalid | Not required | SyntaxError | Gate 3 ✗ |
| TEMPLATE_RENDER_PASS | Parse valid + context complete | Required | None | Gates 9, 10, 11 ✓ |
| TEMPLATE_RENDER_FAIL | Parse valid + context incompatible | Required | RenderError/FilterError | Gates 9, 10, 11 ✗ |
| TEMPLATE_CONTEXT_MISSING | Parse valid + context incomplete | Required | VariableUndefinedError | Gates 9, 10, 11 ⚠ |
| TEMPLATE_OUT_OF_SCOPE | Template exists but unused | Not required | None (classification) | Gate 6 (classification) |

---

## Forbidden Validator Behaviors

The following behaviors are explicitly FORBIDDEN by this contract:

### Forbidden: Report Missing Context as SyntaxError

**Forbidden:**
```
Template: "Dear {{ customer_name }}"
Context: missing "customer_name"
Validator Output: "SyntaxError: undefined variable customer_name"
```

**Correct:**
```
Template: "Dear {{ customer_name }}"
Context: missing "customer_name"
Validator Output: "TEMPLATE_CONTEXT_MISSING: variable 'customer_name' not in context"
```

**Reason:** Missing context is not a syntax error. Conflating these prevents correct diagnosis.

### Forbidden: Accept Invalid Tera Syntax as Valid

**Forbidden:**
```
Template: "{% for item in items %} ... {# unclosed #}"
Validator Output: "TEMPLATE_PARSE_PASS"
```

**Correct:**
```
Template: "{% for item in items %} ... {# unclosed #}"
Validator Output: "TEMPLATE_PARSE_FAIL: unclosed block 'for'"
```

**Reason:** Invalid syntax must be rejected. Accepting it hides errors.

### Forbidden: Skip Validation Phases

**Forbidden:**
```
Validator runs only PARSE validation
→ Missing RENDER validation
→ Unknown templates silently fail in production
```

**Correct:**
```
Validator runs PARSE validation
→ For active templates: run RENDER validation with sample context
→ Document RENDER results separately from PARSE results
```

**Reason:** Both phases are required for manufacturing assurance.

### Forbidden: Use "test_template" Fake Parser Path

**Forbidden:**
```
if template.name == "test_template":
    return PARSE_PASS  # Bypass real validation
```

**Correct:**
```
if template.name == "test_template":
    return PARSE_FAIL  # Apply same rules to all templates
```

**Reason:** Test templates must pass same validation as production. No special cases.

### Forbidden: Report Valid Tera as SyntaxError

**Forbidden:**
```
Template: "{{ x | default(empty_string) }}"
Validator Output: "SyntaxError: 'empty_string' is undefined"
```

**Correct:**
```
Template: "{{ x | default(empty_string) }}"
Validator Output: "TEMPLATE_PARSE_PASS" (valid Tera syntax)
→ If rendering: check if 'empty_string' exists in context
  If missing: TEMPLATE_CONTEXT_MISSING (not PARSE_FAIL)
```

**Reason:** Tera's default() filter is valid syntax. Context binding is separate from syntax.

---

## Contract Amendment Process

Any change to validator behavior must follow this process:

1. **Issue Identification:** Document the validator behavior issue (bug, ambiguity, or gap)
2. **Contract Amendment Proposal:** Write proposed change to this contract
3. **Authority Review:** Process Intelligence Research Directorate review and approval
4. **Contract Amendment:** Freeze new contract version
5. **Implementation:** Code changes to match amended contract
6. **Validation Gate:** Re-run affected gates (3, 9, 10, 11) to verify compliance

**No validator code changes are permitted until this contract is frozen and approved.**

---

## Gate Binding

This contract binds the following conformance gates:

| Gate | Requirement | Contract Sections |
|------|-------------|-------------------|
| Gate 2 | Validator rejects invalid templates | TEMPLATE_PARSE_FAIL, TEMPLATE_RENDER_FAIL |
| Gate 3 | All *.tera files parse successfully | TEMPLATE_PARSE_PASS |
| Gate 6 | Every legacy .ggen classified | TEMPLATE_OUT_OF_SCOPE |
| Gate 9 | ≥1 pipeline produces ≥1 artifact | TEMPLATE_RENDER_PASS |
| Gate 10 | Prompt manufactory warrant path | TEMPLATE_RENDER_PASS flow |
| Gate 11 | Receipts emitted | TEMPLATE_RENDER_PASS output capture |

---

## Testing Requirements

This contract specifies test case categories:

### Valid Tera Syntax Test Cases (PARSE_PASS)

Test cases MUST pass TEMPLATE_PARSE_PASS validation:
- Simple variable interpolation: `{{ x }}`
- Filters: `{{ x | upper }}`, `{{ x | default("") }}`
- Tags: `{% if x %} ... {% endif %}`
- Loops: `{% for item in items %} ... {% endfor %}`
- Comments: `{# comment #}`
- Whitespace control: `{%- expr -%}`
- Nested structures: `{% if a %}{% for b %}...{% endfor %}{% endif %}`

**Test Fixture:** warrant-path-proof.md (Gate 2) must document all valid test cases

---

### Invalid Tera Syntax Test Cases (PARSE_FAIL)

Test cases MUST fail TEMPLATE_PARSE_FAIL validation:
- Unclosed tags: `{{ x` or `{% if x %}`
- Invalid filter syntax: `{{ x | }}` or `{{ x | bad_filter }}`
- Malformed expressions: `{{ x y }}`
- Bad whitespace: `{%  %}`
- Circular or recursive: `{% for x in x %}`

**Test Fixture:** warrant-path-proof.md (Gate 2) must document all invalid test cases with expected errors

---

### Render Context Test Cases (RENDER_PASS vs CONTEXT_MISSING)

Test cases MUST distinguish between valid context and missing context:
- Complete context → TEMPLATE_RENDER_PASS
- Missing variable → TEMPLATE_CONTEXT_MISSING
- Type mismatch → TEMPLATE_RENDER_FAIL

**Test Fixture:** ggen-pipeline-execution-ledger.yaml (Gates 9, 10) must document context availability

---

## Contract Freeze Declaration

**This contract is FROZEN as of 2026-06-01.**

**Effective immediately:** All validator code must conform to these specifications before Phase 12 remediation begins.

**No code changes to validator logic are permitted until this contract is approved and authority confirms compliance.**

---

## Conclusion

This contract specifies the exact validator behavior required for the PI GGEN manufacturing pipeline.

**Key Principles:**
1. PARSE validation requires no context; RENDER validation requires explicit context
2. Missing context is TEMPLATE_CONTEXT_MISSING, not PARSE_FAIL
3. All template states are binary (PASS/FAIL/MISSING/OUT_OF_SCOPE)
4. No fake/bypass paths; all templates evaluated equally
5. Validator behavior is immutable until contract amendment

**Next Steps:**
- Phase 2 Gate: Contract signature (this document)
- Phase 12: Gap remediation (validator bug fixes)
- Phase 13: Re-run Gates 2, 3, 9, 10, 11 with corrected validator

---

**Contract Authority:** Process Intelligence Research Directorate  
**Issue Date:** 2026-06-01  
**Status:** FROZEN — Ready for Phase 12 Implementation  
**Next Review:** Phase 13 (Post-remediation audit)

---

