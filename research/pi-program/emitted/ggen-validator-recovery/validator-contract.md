# Tera Template Validator Contract Specification

**Authority:** Process Intelligence Research Directorate  
**Document Type:** Validator Contract (Behavioral Specification)  
**Date Issued:** 2026-06-01  
**Phase:** 2 - Validator Contract Specification  
**Status:** ACTIVE  

---

## Executive Summary

This document specifies the exact behavior of the Tera Template Validator before any code implementation. The validator enforces a strict separation between **parse validation** (syntax-only, no context required) and **render validation** (syntax + context binding).

**Critical Principle:** Parse and Render are orthogonal operations. A template with valid Tera syntax passes parse validation even if all context variables are missing. Missing context is NOT a parse error.

---

## 1. Validator Scope & Authority

### 1.1 Authority Boundaries

The Tera Template Validator is responsible for:
- ✓ Detecting Tera syntax errors (unclosed tags, malformed expressions, invalid blocks)
- ✓ Confirming template references in ggen.toml rules
- ✓ Validating template + context pairs in render scenarios
- ✓ Classifying templates by validation state

The Tera Template Validator is NOT responsible for:
- ✗ Validating generated artifact content
- ✗ Validating ggen.toml rule syntax (belongs to ggen validator)
- ✗ Validating SPARQL/TTL/RQ query syntax (separate validators)
- ✗ Checking artifact receipt cryptography (separate process)
- ✗ Evaluating manufacturing pipeline success (belongs to pipeline auditor)

### 1.2 Input Scope

**Template Sources:**
- Tera files (*.tera) in ggen template directories
- Tera inline templates in ggen.toml rules (if present)
- Sample context from ggen.toml query results (for render validation)

**Out of Scope:**
- Templates in non-active rules (may be PARSE_FAIL; acceptable if not rendered)
- Templates in experimental branches (may be incomplete)
- Handwritten template fixtures (accepted as-is for testing purposes)

---

## 2. Validation States (Exhaustive Definition)

### 2.1 Parse Validation States

Parse validation confirms Tera template **syntax only**. No runtime context is required.

#### TEMPLATE_PARSE_PASS

**Definition:** Template contains syntactically valid Tera code.

**Conditions (ALL must be true):**
1. Every `{{` has matching `}}`
2. Every `{%` has matching `%}`
3. Every block statement (`{% if %}`, `{% for %}`, `{% block %}`, etc.) has matching close tag
4. No dangling or unclosed delimiters
5. Variable expressions use valid Tera syntax: `{{ var }}`, `{{ obj.field }}`, `{{ array[0] }}`
6. Filters (if present) use valid syntax: `{{ var | filter }}`, `{{ var | filter(arg) }}`
7. Block statements use valid syntax:
   - `{% if condition %} ... {% else %} ... {% endif %}`
   - `{% for item in items %} ... {% endfor %}`
   - `{% set var = value %}`
   - `{% include "file" %}`
   - `{% extends "file" %}`
   - `{% block name %} ... {% endblock %}`
   - `{% macro name(args) %} ... {% endmacro %}`
8. No syntax errors in operator expressions: `+`, `-`, `*`, `/`, `%`, `==`, `!=`, `<`, `>`, `<=`, `>=`, `and`, `or`, `not`

**Example PASS cases:**
```tera
{{ name }}
{{ person.address.street }}
{{ items | length }}
{% for item in items %}{{ item.name }}{% endfor %}
{% if status == "active" %}OK{% else %}INACTIVE{% endif %}
```

**Evidence Collection:**
- Template file can be parsed by Tera parser without syntax error
- Tera parser error output is empty
- Parser returns a valid token tree (no parse errors)

**State Marker:** Emit `TEMPLATE_PARSE_PASS` in validation results

---

#### TEMPLATE_PARSE_FAIL

**Definition:** Template contains invalid Tera syntax and cannot be parsed.

**Conditions (ANY of these indicate FAIL):**
1. Unclosed expression: `{{ var` (missing `}}`)
2. Unclosed statement: `{% for` (missing `%}` or close tag)
3. Invalid statement keyword: `{% invalid %}`
4. Mismatched block structure: `{% if %} ... {% endfor %}`
5. Invalid filter syntax: `{{ var | }}` (filter name missing)
6. Invalid operator: `{{ a <> b }}` (invalid operator)
7. Orphaned close tag: `{% endif %}` without matching `{% if %}`
8. Malformed macro/function call: `{{ func(unclosed }}`, `{{ func(a, ) }}`
9. Invalid attribute/index access: `{{ obj. }}` (missing field), `{{ array[] }}` (empty index)
10. Recursive includes/extends that Tera parser rejects

**Example FAIL cases:**
```tera
{{ var                     # Missing }}
{% for x in y              # Missing {% endfor %}
{% invalid %}              # Invalid keyword
{% if %} ... {% endfor %}  # Mismatched block
{{ x | }}                  # Missing filter name
{{ obj. }}                 # Incomplete field access
```

**Error Details Collection:**
- Tera parser error message (exact message from parser)
- Line number and column number (if reported by parser)
- Context (surrounding code snippet, ±3 lines)

**State Marker:** Emit `TEMPLATE_PARSE_FAIL` in validation results

---

#### TEMPLATE_OUT_OF_SCOPE

**Definition:** Template exists but is not referenced by any active ggen.toml rule.

**Conditions:**
1. Template file exists in ggen/templates/ directory
2. Template syntax is valid (PARSE_PASS)
3. Template is not referenced in any ggen.toml generation rule
4. Template is not included/extended by any other active template

**Rationale:** Out-of-scope templates may be incomplete, experimental, or draft code. They are not part of the active manufacturing pipeline and do not require context validation.

**Example:** A `future-feature.tera` file in ggen/templates/ that is not referenced in ggen.toml is OUT_OF_SCOPE.

**Status in Reports:** OUT_OF_SCOPE templates PASS parse gate automatically (syntax is valid). Include in pass count for Gate 3 (Active Templates Parse).

**State Marker:** Emit `TEMPLATE_OUT_OF_SCOPE` in validation results with parse status

---

### 2.2 Render Validation States

Render validation confirms template can be **instantiated with explicit context**. Requires both parsed template and sample query result context.

#### TEMPLATE_RENDER_PASS

**Definition:** Template parses successfully AND all referenced variables have bindings in provided context.

**Conditions (ALL must be true):**
1. Template has TEMPLATE_PARSE_PASS status
2. All variables in {{ }} expressions exist in context
3. All variables in {% if %} conditions exist in context
4. All variables in {% for %} loops exist in context (both iterator and iterable)
5. All filter inputs are valid (correct type for filter)
6. All conditional branches execute without binding errors
7. Template renders to string without runtime error

**Example:**
```tera
{{ person.name }}
```
With context:
```
person: { name: "Alice" }
```
Result: TEMPLATE_RENDER_PASS ✓

**State Marker:** Emit `TEMPLATE_RENDER_PASS` in validation results

---

#### TEMPLATE_RENDER_FAIL

**Definition:** Template parses successfully BUT rendering fails due to type error, filter error, or other runtime issue.

**Conditions (ANY of these indicate FAIL):**
1. Filter called with wrong argument type (e.g., `{{ items | join(", ") }}` where `items` is not a list)
2. Method/attribute called on incompatible type (e.g., `{{ number.split(",") }}` where `number` is int)
3. Array index out of bounds (e.g., `{{ items[100] }}` with only 10 items) — Tera returns undefined
4. Loop iterator not a list/array (e.g., `{% for x in non_list %}`), Tera error
5. Division by zero in expression (e.g., `{{ 1 / 0 }}`)
6. Undefined filter called (e.g., `{{ x | nonexistent_filter }}`)
7. Invalid comparison between incompatible types (e.g., `{{ "a" > 5 }}`)

**Important:** In Tera, accessing missing attributes returns `undefined` (NOT an error). This is treated as TEMPLATE_CONTEXT_MISSING, not RENDER_FAIL.

**Error Details Collection:**
- Tera runtime error message
- Line number in template
- Variable name that caused error (if identifiable)
- Expected vs. actual type (if applicable)

**State Marker:** Emit `TEMPLATE_RENDER_FAIL` in validation results

---

#### TEMPLATE_CONTEXT_MISSING

**Definition:** Template parses successfully BUT one or more referenced variables have no binding in provided context.

**Conditions:**
1. Template has TEMPLATE_PARSE_PASS status
2. One or more variables in template are not present in context
3. Accessing missing variable in Tera returns `undefined` (not an error)
4. This is a **binding issue**, not a **syntax error**

**Critical Distinction:**
```tera
{{ missing_var }}  # Valid syntax, missing binding → CONTEXT_MISSING
{{ var            # Invalid syntax → PARSE_FAIL
```

**Examples:**

Template: `{{ author_name }}`  
Context: `{ title: "Report" }`  
Result: TEMPLATE_CONTEXT_MISSING (author_name not in context) ✓

Template: `{% for item in items %}{{ item.id }}{% endfor %}`  
Context: `{ other_data: 123 }`  
Result: TEMPLATE_CONTEXT_MISSING (items not in context) ✓

Template: `{{ person.address.city }}`  
Context: `{ person: { address: { } } }`  
Result: CONTEXT_MISSING (city field not present) ✓

**Acceptable Scenarios:**
- Template in non-active rule (not rendered in practice)
- Template is generic and accepts optional variables
- Context sample is incomplete by design (full context will be provided at render time)

**Not Acceptable When:**
- Template is in active rendering rule AND should render now
- Context is stated to be complete but variables are missing
- Template fixture is supposed to demonstrate complete rendering

**Missing Variable Details Collection:**
- List of all missing variables (names)
- Count of missing variables
- For loop iterables: confirm if iterable is missing
- Nested access chains: identify where chain breaks (e.g., `person.address.city` → `address` is the missing part)

**State Marker:** Emit `TEMPLATE_CONTEXT_MISSING` in validation results

---

## 3. Validator Behavior Rules

### 3.1 Parse Validation Rules (No Context Required)

**Rule 3.1.1: Parse does not require runtime context**
```
IF template syntax is valid
THEN state = PARSE_PASS
REGARDLESS OF context availability
```

**Rule 3.1.2: Missing variables are not syntax errors**
```
{{ missing_var }}  is PARSE_PASS (valid syntax)
{{ var             is PARSE_FAIL  (invalid syntax)
```

**Rule 3.1.3: Empty context does not cause parse failure**
```
Template: "{{ x }}"
Context:  {}
Parse:    PASS ({{ x }} is valid syntax)
Render:   CONTEXT_MISSING (x not in context)
```

**Rule 3.1.4: Tera parser is the authority**
```
IF Tera parser accepts template
THEN state = PARSE_PASS
REGARDLESS of what other tools say
```

### 3.2 Render Validation Rules (Context Required)

**Rule 3.2.1: Render requires both parse success and complete context**
```
RENDER_PASS if and only if:
  - Template PARSE_PASS
  - AND all variables have bindings
  - AND Tera renders without error
```

**Rule 3.2.2: Missing context variables are classified distinctly**
```
Missing {{ x }} in context
  → State = CONTEXT_MISSING
  → NOT RENDER_FAIL
  → NOT PARSE_FAIL
```

**Rule 3.2.3: Type errors are render failures**
```
{{ items | join(",") }} where items = 42
  → State = RENDER_FAIL (type error, not missing)
```

**Rule 3.2.4: Undefined variables render as empty string**
```
{{ missing_var }} with context = {}
  → Renders as "" (empty)
  → State = CONTEXT_MISSING (missing binding)
  → NOT RENDER_FAIL
```

### 3.3 Forbidden Validator Behaviors

**❌ FORBIDDEN: Reporting missing context as SyntaxError**
```
WRONG: {{ x }} with empty context → "SyntaxError: x undefined"
RIGHT: {{ x }} with empty context → "CONTEXT_MISSING: x"
```

**❌ FORBIDDEN: Collapsing parse and render into one state**
```
WRONG: State = "VALIDATION_FAIL" (which kind?)
RIGHT: State = "PARSE_FAIL" or "RENDER_FAIL" (explicit)
```

**❌ FORBIDDEN: Using fake test parser**
```
WRONG: Create fake Tera parser to validate syntax
RIGHT: Use actual Tera crate to parse templates
```

**❌ FORBIDDEN: Skipping validation on active templates**
```
WRONG: "This template is active, assume it works"
RIGHT: Validate all templates, report failures honestly
```

**❌ FORBIDDEN: Reporting valid Tera syntax as error**
```
WRONG: {{ items | length }} → "PARSE_FAIL"
RIGHT: {{ items | length }} → "PARSE_PASS"
```

**❌ FORBIDDEN: Accepting invalid syntax**
```
WRONG: {{ unclosed → "PARSE_PASS"
RIGHT: {{ unclosed → "PARSE_FAIL"
```

---

## 4. Validator State Machine

```
[Template File]
       |
       ├─→ [READ FILE] ──→ [File not readable] → OUT_OF_SCOPE
       |
       └─→ [PARSE SYNTAX]
              |
              ├─→ [PARSE ERROR] → PARSE_FAIL
              |
              └─→ [PARSE SUCCESS]
                     |
                     ├─→ [NOT REFERENCED in ggen.toml] → OUT_OF_SCOPE
                     |
                     └─→ [REFERENCED in ggen.toml]
                            |
                            └─→ [CONTEXT AVAILABLE?]
                                   |
                                   ├─→ [NO Context] → PARSE_PASS only
                                   |
                                   └─→ [YES Context]
                                          |
                                          ├─→ [MISSING VARIABLES] → CONTEXT_MISSING
                                          |
                                          ├─→ [RENDER TYPE ERROR] → RENDER_FAIL
                                          |
                                          └─→ [RENDER SUCCESS] → RENDER_PASS
```

---

## 5. Validator Output Format

### 5.1 Per-Template Validation Result

```yaml
template:
  path: "research/pi-program/ggen/templates/checkpoint.md.tera"
  filename: "checkpoint.md.tera"
  parse_status: "PARSE_PASS"
  render_status: "CONTEXT_MISSING"
  parse_error: null
  render_error: null
  missing_variables:
    - "critical_defect"
    - "failing_gate"
  missing_variable_count: 2
  is_active: true
  referenced_in_ggen_rules:
    - "ggen/ggen-pi-checkpoint.toml::rule-emit-checkpoint"
  validator_version: "1.0"
  validation_timestamp: "2026-06-01T14:00:00Z"
```

### 5.2 Summary Report

```yaml
summary:
  total_templates: 24
  parse_pass: 24
  parse_fail: 0
  context_missing: 23
  render_pass: 1
  render_fail: 0
  out_of_scope: 0
  active_templates: 24
  
gate_3_status: "PASS"  # Active Templates Parse (all 24 PARSE_PASS)
gate_evaluation: "All 24 active templates have valid Tera syntax"
```

---

## 6. Test Fixtures (Normative)

All test fixtures must validate according to these specifications. Each fixture demonstrates a specific validator behavior.

### 6.1 Valid Parse Fixtures (Must All PASS Parse)

#### Fixture: valid-basic.tera
```tera
{{ name }}
```
**Expected:** TEMPLATE_PARSE_PASS  
**Context:** `{ "name": "Alice" }`  
**Render Expected:** TEMPLATE_RENDER_PASS  

#### Fixture: valid-loop.tera
```tera
{% for item in items %}{{ item.id }}: {{ item.name }}
{% endfor %}
```
**Expected:** TEMPLATE_PARSE_PASS  
**Context:** `{ "items": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}] }`  
**Render Expected:** TEMPLATE_RENDER_PASS  

#### Fixture: valid-condition.tera
```tera
{% if status == "active" %}
Status is ACTIVE
{% else %}
Status is INACTIVE
{% endif %}
```
**Expected:** TEMPLATE_PARSE_PASS  
**Context:** `{ "status": "active" }`  
**Render Expected:** TEMPLATE_RENDER_PASS  

#### Fixture: valid-nested-paths.tera
```tera
Name: {{ user.profile.name }}
City: {{ user.profile.address.city }}
```
**Expected:** TEMPLATE_PARSE_PASS  
**Context:** `{ "user": { "profile": { "name": "Alice", "address": { "city": "Portland" } } } }`  
**Render Expected:** TEMPLATE_RENDER_PASS  

#### Fixture: valid-filter.tera
```tera
Items: {{ items | length }}
Sorted: {{ names | sort | join(", ") }}
```
**Expected:** TEMPLATE_PARSE_PASS  
**Context:** `{ "items": [1,2,3], "names": ["Charlie", "Alice", "Bob"] }`  
**Render Expected:** TEMPLATE_RENDER_PASS  

#### Fixture: valid-macro.tera
```tera
{% macro greeting(name, greeting) %}
{{ greeting }}, {{ name }}!
{% endmacro %}

{{ greeting("Alice", "Hello") }}
```
**Expected:** TEMPLATE_PARSE_PASS  
**Context:** `{}`  
**Render Expected:** TEMPLATE_RENDER_PASS (macros don't require external context)  

### 6.2 Invalid Parse Fixtures (Must All FAIL Parse)

#### Fixture: invalid-unclosed-expression.tera
```tera
{{ var
```
**Expected:** TEMPLATE_PARSE_FAIL  
**Error Contains:** "unclosed" or "expected }}"  

#### Fixture: invalid-unclosed-statement.tera
```tera
{% for x in items
```
**Expected:** TEMPLATE_PARSE_FAIL  
**Error Contains:** "unclosed" or "expected %}"  

#### Fixture: invalid-bad-block.tera
```tera
{% invalid %}
```
**Expected:** TEMPLATE_PARSE_FAIL  
**Error Contains:** "unknown block" or "invalid keyword"  

#### Fixture: invalid-bad-expression.tera
```tera
{{ 42 + }}
```
**Expected:** TEMPLATE_PARSE_FAIL  
**Error Contains:** "expected expression" or "operator without operand"  

#### Fixture: invalid-mismatched-block.tera
```tera
{% if condition %}
Content here
{% endfor %}
```
**Expected:** TEMPLATE_PARSE_FAIL  
**Error Contains:** "mismatched" or "expected endif"  

#### Fixture: invalid-orphan-close.tera
```tera
{% endif %}
```
**Expected:** TEMPLATE_PARSE_FAIL  
**Error Contains:** "unexpected close" or "no matching if"  

### 6.3 Context Missing Fixtures (Valid Syntax, Missing Context)

#### Fixture: context-missing-simple.tera
```tera
{{ author_name }}
```
**Template:** TEMPLATE_PARSE_PASS  
**Context:** `{ "title": "Report" }`  
**Expected Render:** TEMPLATE_CONTEXT_MISSING  
**Missing Variables:** `["author_name"]`  

#### Fixture: context-missing-nested.tera
```tera
{{ user.profile.phone }}
```
**Template:** TEMPLATE_PARSE_PASS  
**Context:** `{ "user": { "profile": { "name": "Alice" } } }`  
**Expected Render:** TEMPLATE_CONTEXT_MISSING  
**Missing Variables:** `["phone"]` (or context chain missing at point: user.profile.phone)  

#### Fixture: context-missing-loop-iterator.tera
```tera
{% for item in products %}{{ item.name }}{% endfor %}
```
**Template:** TEMPLATE_PARSE_PASS  
**Context:** `{ "users": [] }`  
**Expected Render:** TEMPLATE_CONTEXT_MISSING  
**Missing Variables:** `["products"]`  

---

## 7. Validator Contract Signatures

### 7.1 Validator Implementation Signature

```rust
pub fn validate_template(
    template_path: &Path,
    template_content: &str,
    context: Option<&serde_json::Value>,
    ggen_rules: &[GgenRule],
) -> ValidationResult {
    // Returns:
    // {
    //   parse_status: TemplateParseStatus,
    //   render_status: Option<TemplateRenderStatus>,
    //   parse_error: Option<String>,
    //   render_error: Option<String>,
    //   missing_variables: Vec<String>,
    // }
}
```

### 7.2 Enum Definitions

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateParseStatus {
    PARSE_PASS,      // Valid Tera syntax
    PARSE_FAIL,      // Invalid Tera syntax
    OUT_OF_SCOPE,    // Valid syntax, not referenced
}

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateRenderStatus {
    RENDER_PASS,         // Parsed + context complete + renders
    RENDER_FAIL,         // Parsed + context complete + type error
    CONTEXT_MISSING,     // Parsed + context incomplete
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub template_path: PathBuf,
    pub parse_status: TemplateParseStatus,
    pub render_status: Option<TemplateRenderStatus>,
    pub parse_error: Option<String>,
    pub render_error: Option<String>,
    pub missing_variables: Vec<String>,
    pub is_active: bool,
    pub referenced_in_rules: Vec<String>,
    pub validation_timestamp: String,
}
```

### 7.3 Validation Criteria for Testing

```rust
#[test]
fn test_valid_parse_basic() {
    let result = validate_template(
        Path::new("test.tera"),
        "{{ name }}",
        None,
        &[],
    );
    assert_eq!(result.parse_status, TemplateParseStatus::PARSE_PASS);
}

#[test]
fn test_invalid_parse_unclosed() {
    let result = validate_template(
        Path::new("test.tera"),
        "{{ var",
        None,
        &[],
    );
    assert_eq!(result.parse_status, TemplateParseStatus::PARSE_FAIL);
    assert!(result.parse_error.is_some());
}

#[test]
fn test_context_missing_does_not_fail_parse() {
    let result = validate_template(
        Path::new("test.tera"),
        "{{ missing_var }}",
        Some(&json!({})),
        &[],
    );
    assert_eq!(result.parse_status, TemplateParseStatus::PARSE_PASS);
    assert_eq!(result.render_status, Some(TemplateRenderStatus::CONTEXT_MISSING));
}
```

---

## 8. Acceptance Criteria (Gate 2 Compliance)

Gate 2 requires: "Validator Fixture Tests FAIL (Invalid) — Invalid test cases documented"

### 8.1 Acceptance Condition 1: All Valid Fixtures PASS

**Test:** Run validate_all_templates.py on valid-*.tera fixtures  
**Expected:** All return PARSE_PASS  
**Evidence:** Template validation report shows 100% PARSE_PASS for valid fixtures  
**Status:** ✓ DEMONSTRABLE  

### 8.2 Acceptance Condition 2: All Invalid Fixtures FAIL

**Test:** Run validate_all_templates.py on invalid-*.tera fixtures  
**Expected:** All return PARSE_FAIL with error message  
**Evidence:** Template validation report shows 100% PARSE_FAIL for invalid fixtures  
**Status:** ✓ DEMONSTRABLE  

### 8.3 Acceptance Condition 3: Context Missing is Distinguished from Parse Fail

**Test:** Validate context-missing-*.tera with empty context  
**Expected:** Parse status = PARSE_PASS, Render status = CONTEXT_MISSING  
**Evidence:** Validation report shows distinct states (not collapsed)  
**Status:** ✓ DEMONSTRABLE  

### 8.4 Acceptance Condition 4: Error Messages are Explicit

**Test:** PARSE_FAIL fixtures produce error messages  
**Expected:** Error message names the error (e.g., "unclosed expression", "bad block", etc.)  
**Evidence:** Validation report includes parse_error field with specific message  
**Status:** ✓ DEMONSTRABLE  

---

## 9. Contract Compliance Verification

### 9.1 Compliance Checklist

Before approving validator implementation:

- [ ] Parse validation works without runtime context
- [ ] PARSE_PASS allows missing variables
- [ ] PARSE_FAIL reports syntax errors only
- [ ] RENDER_PASS requires both syntax AND context
- [ ] CONTEXT_MISSING is distinct from PARSE_FAIL
- [ ] CONTEXT_MISSING is distinct from RENDER_FAIL
- [ ] All valid-*.tera fixtures PASS parse
- [ ] All invalid-*.tera fixtures FAIL parse
- [ ] All context-missing-*.tera fixtures PASS parse
- [ ] Tera parser is used (not fake parser)
- [ ] Validation results include all required fields
- [ ] Out-of-scope templates are handled correctly
- [ ] No forbidden behaviors detected (see Section 3.3)

### 9.2 Contract Signature Line

**By signing below, the implementer agrees to build the validator exactly as specified in this contract.**

```
Validator Contract Version: 1.0
Date: 2026-06-01
Authority: Process Intelligence Research Directorate

Implementation Signature: _________________________
Approval Signature:       _________________________
Audit Signature:         _________________________
```

---

## 10. Appendix: Tera Syntax Reference (Normative)

Valid Tera syntax that MUST parse successfully:

### 10.1 Variable Expressions
```tera
{{ var }}
{{ obj.field }}
{{ obj.nested.field }}
{{ array[0] }}
{{ array[i] }}
{{ var | filter }}
{{ var | filter(arg1, arg2) }}
{{ var | filter(arg) | another_filter }}
```

### 10.2 Control Flow
```tera
{% if condition %}text{% endif %}
{% if condition %}text1{% else %}text2{% endif %}
{% if condition %}text1{% elif other %}text2{% else %}text3{% endif %}
{% for item in items %}{{ item }}{% endfor %}
{% set var = value %}
{% set var = "string" %}
{% set var = obj.field %}
```

### 10.3 Blocks & Inheritance
```tera
{% block name %}content{% endblock %}
{% extends "parent.tera" %}
{% include "file.tera" %}
{% include "file.tera" with context %}
```

### 10.4 Macros
```tera
{% macro name(arg1, arg2) %}
  content using {{ arg1 }} and {{ arg2 }}
{% endmacro %}
{{ name(val1, val2) }}
```

### 10.5 Operators (All Valid)
```tera
{{ a + b }}
{{ a - b }}
{{ a * b }}
{{ a / b }}
{{ a % b }}
{{ a == b }}
{{ a != b }}
{{ a < b }}
{{ a > b }}
{{ a <= b }}
{{ a >= b }}
{{ a and b }}
{{ a or b }}
{{ not a }}
```

---

## Conclusion

This contract specifies the Tera Template Validator behavior comprehensively and unambiguously. Implementation must follow this contract exactly. Deviations constitute contract breach and must be escalated to the Process Intelligence Research Directorate for adjudication.

**Status:** READY FOR SIGNATURE  
**Date:** 2026-06-01  
**Authority:** Process Intelligence Research Directorate
