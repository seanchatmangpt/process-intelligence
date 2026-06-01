# ggen Validator Fixture Strategy & Specification — Phase 2

**Date:** 2026-06-01  
**Phase:** 2 (Minimal Validator Fixture Building)  
**Scope:** Define test fixtures to validate Tera parser behavior and expose validator ambiguity

---

## Objective

Build a **minimal fixture suite** that:

1. Tests parse vs. render behavior for each Tera construct
2. Isolates the template validator's exact failure modes
3. Demonstrates the parser ambiguity with mixed TypeScript/Tera templates
4. Provides concrete test cases for ggen validator recovery (Phase 3)

---

## Validator Architecture (Current)

**Location:** `/Users/sac/ggen/crates/ggen-core/src/template/mod.rs:67-79`

Current validation flow:
```
Template Content
    ↓
validate_template()
    ↓
tera.add_raw_template("test_template", content)
    ↓
[IF ERROR] → return SyntaxError
[IF OK] → return Valid
```

**Limitations:**
- No context about target language (React, Markdown, etc.)
- All-or-nothing: either parse succeeds or entire template rejected
- No location information for errors (which line?)
- No distinction between syntax errors vs. parser ambiguities

---

## Fixture Suite Design

### Fixture 1: `valid-basic.tera`

**Purpose:** Validate that simple variable substitution works

**Content:**
```tera
{# Simple variable substitution test #}
Hello {{ user_name }}!
This is a basic greeting.
Price: ${{ price | default(value=99.99) }}
```

**Expected:** ✓ PASS

**What It Tests:**
- Basic `{{ var }}` syntax
- Filter syntax `{{ var | default(...) }}`
- Comment syntax `{# ... #}`

**Parse vs. Render:**
- Parse: Tera should accept this template syntax
- Render: With `user_name="Alice", price=42.50`, output should be:
  ```
  Hello Alice!
  This is a basic greeting.
  Price: $42.50
  ```

---

### Fixture 2: `valid-loop.tera`

**Purpose:** Validate that loop constructs work

**Content:**
```tera
{# Loop test #}
Shopping List:
{%- for item in items %}
- {{ item.name }}: ${{ item.price }}
{%- endfor %}

Total items: {{ items | length }}
```

**Expected:** ✓ PASS

**What It Tests:**
- For loop syntax `{% for ... in ... %}`
- Loop variable access `{{ item.field }}`
- Filter chaining `{{ items | length }}`
- Whitespace control `{%-` and `-%}`

**Parse vs. Render:**
- Parse: Tera should accept this template syntax
- Render: With `items=[{name: "Milk", price: 3.50}, {name: "Bread", price: 2.99}]`, output should be:
  ```
  Shopping List:
  - Milk: $3.50
  - Bread: $2.99
  
  Total items: 2
  ```

---

### Fixture 3: `valid-condition.tera`

**Purpose:** Validate that conditional constructs work

**Content:**
```tera
{# Conditional test #}
User Profile:
Name: {{ profile.name }}

{%- if profile.is_admin %}
Status: Administrator
Permissions: All
{%- elif profile.is_moderator %}
Status: Moderator
Permissions: Limited
{%- else %}
Status: User
Permissions: Read-Only
{%- endif %}

Profile active: {{ profile.is_active | default(value=false) }}
```

**Expected:** ✓ PASS

**What It Tests:**
- If/elif/else syntax `{% if ... %} ... {% elif ... %} ... {% else %} ... {% endif %}`
- Boolean conditions
- Mixed filters and conditionals

**Parse vs. Render:**
- Parse: Tera should accept this template syntax
- Render: With profile data, output should correctly select branch

---

### Fixture 4: `invalid-unclosed-expression.tera`

**Purpose:** Validate that the validator **rejects** malformed syntax

**Content:**
```tera
{# Invalid syntax test #}
This is a test: {{ unclosed_var
The brace above is not closed.
```

**Expected:** ✗ FAIL (parser should reject this)

**What It Tests:**
- The validator **should** reject genuinely malformed templates
- Confirms validator is not completely broken
- Tests error handling path

**Parse vs. Render:**
- Parse: Tera should **reject** this (missing `}}`)
- Render: N/A (should fail at parse time)

**Expected Error Message:**
```
SyntaxError: Unclosed variable expression starting at line 2
or similar Tera-specific error about mismatched braces
```

---

### Fixture 5: `mixed-react-tera.tsx.tera` ⚠️ CRITICAL

**Purpose:** Expose the validator ambiguity with mixed TypeScript/Tera templates

**Content:**
```tera
{# React component with embedded Tera expressions #}
"use client";

import React, { useState } from "react";

interface Props {
  claim: {
    id: string;
    receiptHash: string;
    receiptTimestamp: string;
  };
}

export default function ClaimCard({ claim }: Props) {
  const handleVerify = (e: React.MouseEvent) => {
    e.preventDefault();
    alert(`Verifying receipt: ${claim.receiptHash}`);
  };

  return (
    <div style={{ backgroundColor: '#f0f0f0', padding: '16px' }}>
      <h2>{{ claim.id }}</h2>
      <p>Hash: {{ claim.receiptHash }}</p>
      <p>Time: {{ claim.receiptTimestamp }}</p>
      
      <button onClick={handleVerify}>
        Verify Receipt
      </button>

      <script>
        const config = {{ 
          hash: "{{ claim.receiptHash }}", 
          timestamp: "{{ claim.receiptTimestamp }}"
        }};
      </script>
    </div>
  );
}
```

**Expected (Current ggen v26.5.21):** ✗ FAIL with SyntaxError

**Expected (After Fix):** ✓ PASS

**Why It Fails (Current):**

The validator sees multiple `}}` that confuse the Tera parser:

1. **Line 19:** `style={{ backgroundColor: '#f0f0f0', ... }}`
   - The `}}` closes the React inline style object
   - Tera parser sees this as an unclosed expression

2. **Line 20-26:** Regular Tera substitutions work fine

3. **Line 28-31:** JavaScript object literal with Tera substitutions
   ```javascript
   const config = {{ 
     hash: "{{ claim.receiptHash }}", 
     timestamp: "{{ claim.receiptTimestamp }}"
   }};
   ```
   - The outer `}}` closes the object
   - The inner `}}` closes the Tera expressions
   - Parser gets confused about nesting

**What It Tests:**
- React inline styles with object literals
- JavaScript object literals with embedded Tera
- Nested template expressions
- The parser ambiguity that blocks visualizer-dashboard.tsx.tera

**Parse vs. Render:**
- **Current (v26.5.21):** Parse FAILS with `SyntaxError("Failed to parse 'test_template'")`
- **Expected (fixed):** Parse succeeds, render produces valid React component with substituted values

---

## Fixture Validation Strategy

### Approach 1: Direct Tera Parsing Test

```rust
#[test]
fn test_valid_basic_fixture_parses() {
    let template = include_str!("fixtures/templates/valid-basic.tera");
    let mut tera = Tera::default();
    
    assert!(
        tera.add_raw_template("test_basic", template).is_ok(),
        "valid-basic.tera should parse successfully"
    );
}

#[test]
fn test_mixed_react_tera_fixture_parses() {
    let template = include_str!("fixtures/templates/mixed-react-tera.tsx.tera");
    let mut tera = Tera::default();
    
    assert!(
        tera.add_raw_template("test_react", template).is_ok(),
        "mixed-react-tera.tsx.tera should parse successfully (currently fails in v26.5.21)"
    );
}

#[test]
fn test_invalid_syntax_fixture_rejects() {
    let template = include_str!("fixtures/templates/invalid-unclosed-expression.tera");
    let mut tera = Tera::default();
    
    assert!(
        tera.add_raw_template("test_invalid", template).is_err(),
        "invalid-unclosed-expression.tera should be rejected by parser"
    );
}
```

### Approach 2: Integration with ggen Validator

```rust
#[test]
fn test_ggen_validator_accepts_basic_template() {
    let template = include_str!("fixtures/templates/valid-basic.tera");
    let result = ggen_core::template::validate_template(template);
    
    assert!(result.is_ok(), "validator should accept basic template");
    assert!(result.unwrap().is_valid);
}

#[test]
#[should_panic(expected = "mixed-react-tera")]
fn test_ggen_validator_fails_react_template_current_bug() {
    let template = include_str!("fixtures/templates/mixed-react-tera.tsx.tera");
    let result = ggen_core::template::validate_template(template);
    
    // This SHOULD PASS after Phase 3 fix
    // Currently it FAILS due to parser ambiguity
    assert!(
        result.is_ok() && result.unwrap().is_valid,
        "validator should accept mixed React/Tera template (currently fails - BUG)"
    );
}
```

---

## Fixture File Locations

### To Create

```
ggen/tests/fixtures/templates/
├── valid-basic.tera
├── valid-loop.tera
├── valid-condition.tera
├── invalid-unclosed-expression.tera
└── mixed-react-tera.tsx.tera

ggen/tests/integration/template_validator_fixtures.rs
└── Tests that exercise the above fixtures
```

### Directory Permissions

- All fixture files: **Read-only** (`.tera` files)
- Test code: **Readable/executable**
- No generated outputs

---

## Fixture Properties

| Fixture | Type | Size | Status | Notes |
|---------|------|------|--------|-------|
| `valid-basic.tera` | Simple | ~200 bytes | ✓ Pass | Basic substitution |
| `valid-loop.tera` | Loop | ~250 bytes | ✓ Pass | For loop construct |
| `valid-condition.tera` | Conditional | ~350 bytes | ✓ Pass | If/elif/else construct |
| `invalid-unclosed-expression.tera` | Error | ~100 bytes | ✗ Reject | Deliberate syntax error |
| `mixed-react-tera.tsx.tera` | React+Tera | ~800 bytes | ✗ FAIL (BUG) | Exposes parser ambiguity |

---

## Parse vs. Render Validation

Each fixture should pass **two** levels of validation:

### Level 1: Parse (Tera.add_raw_template)
- Template syntax is valid Tera
- No unclosed expressions
- All tags are balanced

### Level 2: Render (Tera.render_str)
- Template can be rendered with provided context
- Variable references are satisfied
- Filters produce expected output

**Current Problem:** Level 1 fails for `mixed-react-tera.tsx.tera` due to parser ambiguity.

---

## Success Criteria (Phase 2)

### ✓ All Fixtures Created

- [ ] `valid-basic.tera` exists and is readable
- [ ] `valid-loop.tera` exists and is readable
- [ ] `valid-condition.tera` exists and is readable
- [ ] `invalid-unclosed-expression.tera` exists and is readable
- [ ] `mixed-react-tera.tsx.tera` exists and is readable

### ✓ Fixture Tests Pass

- [ ] `test_valid_basic_fixture_parses()` — PASS
- [ ] `test_valid_loop_fixture_parses()` — PASS
- [ ] `test_valid_condition_fixture_parses()` — PASS
- [ ] `test_invalid_unclosed_fixture_rejects()` — PASS (correctly rejects)
- [ ] `test_mixed_react_tera_fixture_parses()` — FAIL (expected; documents the bug)

### ✓ Documentation Complete

- [ ] This fixture report exists
- [ ] Each fixture has inline comments explaining its purpose
- [ ] Test cases document expected behavior
- [ ] No claims that the validator is fixed

### ✓ ANDON Guard Compliance

- [ ] No fixtures are generated from broken ggen pipeline
- [ ] All fixtures are hand-written reference implementations
- [ ] No claims that fixtures represent current ggen v26.5.21 behavior
- [ ] Fixture report cites root cause diagnosis

---

## Next Steps (Phase 3)

After fixtures are complete, Phase 3 can begin:

1. **Analyze Validator Results**
   - Run test suite against ggen v26.5.21
   - Document which tests pass/fail
   - Confirm mixed-react-tera.tsx.tera fails as expected

2. **Design Recovery Strategy**
   - Option A: Pre-process templates (count braces)
   - Option B: Language-aware validation (accept language hint)
   - Option C: Permissive mode (skip validation for known mixed files)
   - Option D: Better error messages (line number + context)

3. **Escalate to ggen Maintainers**
   - File issue with root cause analysis
   - Provide minimal reproducer (mixed-react-tera.tsx.tera)
   - Suggest recovery strategies
   - Request fix in next ggen release

---

## Risk Assessment

### What Could Go Wrong

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|-----------|
| Fixtures are too simple; don't expose bug | LOW | Misses key test case | Include mixed-react-tera.tsx.tera |
| Fixtures are too complex; unrelated failures | LOW | Confuses diagnosis | Keep each fixture focused on one construct |
| Test harness is broken | MEDIUM | Can't validate fixtures | Use standard Rust cargo test framework |
| ggen version changes during Phase 2 | LOW | Fixtures become stale | Document ggen v26.5.21 requirement |

### What We Know

✓ **Bug is reproducible:** `ggen sync --manifest ggen/ggen.toml --dry_run true` fails predictably  
✓ **Root cause is identified:** Line 1803 in visualizer-dashboard.tsx.tera has unmatched `}}`  
✓ **Validator logic is correct:** The test harness itself is sound, just incomplete  
✓ **Fixtures are minimal:** No external dependencies, just plain `.tera` files

---

## Acceptance Criteria

**Phase 2 is complete when:**

1. All five fixture templates exist in `ggen/tests/fixtures/templates/`
2. All fixture tests are written and documented
3. Test results show:
   - ✓ `valid-*.tera` fixtures parse successfully
   - ✗ `invalid-*.tera` fixture is correctly rejected
   - ✗ `mixed-react-tera.tsx.tera` fails (documents the bug)
4. This report is finalized with test execution results
5. No commits claim the validator is fixed

---

## Appendix: Template Syntax Reference

### Tera Comments
```tera
{# This is a comment #}
{%- This is also a comment (with whitespace control) -%}
```

### Tera Expressions
```tera
{{ variable }}
{{ variable.field }}
{{ variable[0] }}
{{ variable | filter }}
{{ variable | filter(param=value) }}
{{ variable | default(value="fallback") }}
```

### Tera Tags
```tera
{% if condition %} ... {% endif %}
{% if cond1 %} ... {% elif cond2 %} ... {% else %} ... {% endif %}
{% for item in collection %} ... {% endfor %}
{% for item in collection %} ... {% else %} ... {% endfor %}
{% block name %} ... {% endblock %}
{% extends "parent.tera" %}
```

### Whitespace Control
```tera
{%- var -%}    {# Strip all whitespace around tag #}
{{- var -}}    {# Strip all whitespace around expression #}
{%~ var ~%}    {# Strip leading/trailing whitespace separately #}
```

---

## References

- **Root Cause Report:** `validator-reproduction.md`
- **ANDON Guard Rules:** `ANDON_GUARD.md`
- **ggen Validator Code:** `/Users/sac/ggen/crates/ggen-core/src/template/mod.rs:67-79`
- **Failing Template:** `/Users/sac/process-intelligence/ggen/templates/visualizer-dashboard.tsx.tera`
- **Tera Documentation:** https://keats.github.io/tera/
