# Tera Template Validator State Machine & Diagrams

**Authority:** Process Intelligence Research Directorate  
**Document Type:** Validator State Machine Specification  
**Date Issued:** 2026-06-01  
**Phase:** 2 - Validator Contract Specification  
**Status:** ACTIVE  

---

## 1. Validator State Machine (Comprehensive)

```
                          [START: Template File]
                                    |
                                    v
                          [READ TEMPLATE FILE]
                                    |
                    ________________|________________
                   |                |                |
                   v                v                v
           [File exists?]    [File readable?]  [File is *.tera?]
                   |NO              |NO              |NO
                   |                |                |
          [OUT_OF_SCOPE]   [OUT_OF_SCOPE]  [OUT_OF_SCOPE]
                   |                |                |
                   v                v                v
              [END: SKIP]       [END: SKIP]     [END: SKIP]
                                    
                                    ^
                                    |
                    [File exists, readable, is .tera]
                                    |
                                    v
                          [PARSE TEMPLATE SYNTAX]
                                    |
                    ________________|________________
                   |                                 |
                   v                                 v
            [PARSE SUCCESS?]                  [PARSE ERROR]
                   |                                 |
                   |YES                              |
                   |                            [Get error details]
                   |                                 |
                   v                                 v
         [TEMPLATE_PARSE_PASS]          [TEMPLATE_PARSE_FAIL]
                   |                                 |
                   |                           [record: parse_error]
                   |                           [record: line number]
                   |                           [record: error text]
                   |                                 |
                   |                                 v
                   |                            [END: FAIL]
                   |
                   v
        [IS REFERENCED IN ggen.toml?]
                   |
        ___________|___________
       |                       |
      NO                      YES
       |                       |
       v                       v
   [TEMPLATE_          [NEED RENDER CONTEXT?]
    OUT_OF_SCOPE]           |
       |              _______|_______
       |             |               |
       |           NO              YES
       |             |               |
       |             v               v
       |          [Return:    [Context available?]
       |        PARSE_PASS           |
       |        OUT_OF_SCOPE]     ____|____
       |             |          |         |
       |             |        NO        YES
       |             |         |         |
       |             |         |         v
       |             |         |    [EXTRACT VARIABLES
       |             |         |     FROM CONTEXT]
       |             |         |         |
       |             |         |         v
       |             |         |    [IDENTIFY MISSING
       |             |         |     VARIABLES]
       |             |         |         |
       |             |         |    _____|_____
       |             |         |   |           |
       |             |         |  NO          YES
       |             |         |   |           |
       |             |         |   |           v
       |             |         |   |    [TEMPLATE_
       |             |         |   |    CONTEXT_MISSING]
       |             |         |   |           |
       |             |         |   |      [record: missing vars]
       |             |         |   |           |
       |             |         |   |           v
       |             |         |   |      [END: CONTEXT_MISSING]
       |             |         |   |
       |             |         |   v
       |             |         |  [ATTEMPT RENDER
       |             |         |   WITH CONTEXT]
       |             |         |         |
       |             |         |    _____|_____
       |             |         |   |           |
       |             |         | ERROR      SUCCESS
       |             |         |   |           |
       |             |         |   v           v
       |             |         | [Check error  [TEMPLATE_
       |             |         |  type]        RENDER_PASS]
       |             |         |   |           |
       |             |         |   |      [record: success]
       |             |         |   v      [return: context OK]
       |             |         | [Is type     |
       |             |         |  error?]     v
       |             |         |   |      [END: RENDER_PASS]
       |             |         | YES|NO
       |             |         |   | |
       |             |         |   | v
       |             |         |   |[Missing var?]
       |             |         |   | |
       |             |         |   |YES|NO
       |             |         |   | | |
       |             |         |   v | v
       |             |         |[TYPE   [Other
       |             |         | ERROR] error]
       |             |         |   |     |
       |             |         |   |     v
       |             |         |   v [TEMPLATE_
       |             |         |[TEMPLATE_RENDER_FAIL]
       |             |         | CONTEXT_        |
       |             |         | MISSING]   [record: error]
       |             |         |   |        [return: error type]
       |             |         |   |              |
       |             |         |   v              v
       |             |         |[END:         [END:
       |             |         | CONTEXT_     RENDER_FAIL]
       |             |         | MISSING]
       |             |         |
       v             v         v
    [RETURN RESULTS TO AGGREGATOR]
         |
         v
   [TEMPLATE VALIDATION REPORT]
         |
         v
    [END: COMPLETE]
```

---

## 2. Parse vs. Render Separation (Critical)

```
┌──────────────────────────────────────────────────────────────┐
│               VALIDATOR OPERATION PHASES                      │
└──────────────────────────────────────────────────────────────┘

                    ┌─────────────────────┐
                    │  PARSE PHASE        │
                    │  (No context req.)  │
                    └─────────────────────┘
                              |
                    ┌─────────v──────────┐
                    │ Tera Parser        │
                    │ (syntax check)     │
                    └─────────────────────┘
                              |
                    ┌─────────v──────────┐
                    │ Check:             │
                    │ - Closed tags      │
                    │ - Valid blocks     │
                    │ - Valid operators  │
                    └─────────────────────┘
                              |
                    ┌─────────v──────────┐
                    │ Emit:              │
                    │ ✓ PARSE_PASS       │
                    │ ✗ PARSE_FAIL       │
                    │ ⊘ OUT_OF_SCOPE     │
                    └─────────────────────┘
                              |
                              |
         [PARSE_PASS only; cannot proceed to render]
         [unless context explicitly provided]
                              |
                              |
                    ┌─────────v──────────┐
                    │  RENDER PHASE      │
                    │  (Context req.)    │
                    └─────────────────────┘
                              |
                    ┌─────────v──────────┐
                    │ Require:           │
                    │ 1. PARSE_PASS      │
                    │ 2. Context object  │
                    └─────────────────────┘
                              |
                    ┌─────────v──────────┐
                    │ Variable Binding   │
                    │ - Extract vars     │
                    │ - Match to context │
                    │ - Identify missing │
                    └─────────────────────┘
                              |
                    ┌─────────v──────────┐
                    │ Missing variables? │
                    └─────────────────────┘
                         |            |
                        YES           NO
                         |            |
            ┌────────────v──┐  ┌──────v────────┐
            │ CONTEXT_      │  │ Tera Render   │
            │ MISSING       │  │ Engine        │
            └───────────────┘  └──────────────┘
                                      |
                                ┌─────v─────┐
                                │ Render OK? │
                                └─────┬─────┘
                                  |       |
                               YES      NO
                                |       |
                        ┌───────v──┐  ┌─v──────────┐
                        │ RENDER_   │  │ Error Type │
                        │ PASS      │  └──┬─────────┘
                        └───────────┘     |
                                     ┌────v────┐
                                     │ Type    │
                                     │ error?  │
                                     └────┬────┘
                                      |       |
                                    YES      NO
                                     |       |
                            ┌────────v──┐ ┌─v──────────┐
                            │RENDER_    │ │RENDER_     │
                            │FAIL       │ │FAIL        │
                            │(type)     │ │(other)     │
                            └───────────┘ └────────────┘
```

---

## 3. Orthogonal State Axes

The validator operates on two independent axes:

```
┌─────────────────────────────────────────────────────────────┐
│                  PARSE STATE (Independent)                  │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  PARSE_PASS:        Template syntax is valid                │
│                     (can parse with Tera parser)            │
│                                                               │
│  PARSE_FAIL:        Template syntax is invalid              │
│                     (Tera parser rejects)                   │
│                                                               │
│  OUT_OF_SCOPE:      Template valid but not referenced       │
│                     in any active ggen.toml rule            │
│                                                               │
└─────────────────────────────────────────────────────────────┘
         ∧                          ∧
         │                          │
      INDEPENDENT              INDEPENDENT
         │                          │
         ∨                          ∨
┌─────────────────────────────────────────────────────────────┐
│               RENDER STATE (Dependent on Parse)             │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  RENDER_PASS:       Valid syntax + complete context         │
│                     (REQUIRES PARSE_PASS)                   │
│                                                               │
│  RENDER_FAIL:       Valid syntax + type/runtime error       │
│                     (REQUIRES PARSE_PASS)                   │
│                                                               │
│  CONTEXT_MISSING:   Valid syntax + incomplete context       │
│                     (REQUIRES PARSE_PASS)                   │
│                                                               │
│  (no render state if PARSE_FAIL)                            │
│                                                               │
└─────────────────────────────────────────────────────────────┘

KEY INSIGHT:
  Parse and Render states are NOT mutually exclusive on the same axis.
  A template can be:
    - PARSE_PASS + CONTEXT_MISSING (valid syntax, missing variables)
    - PARSE_PASS + RENDER_PASS (valid syntax, all variables bound)
    - PARSE_PASS + RENDER_FAIL (valid syntax, type error)
    - PARSE_FAIL + (no render state)
```

---

## 4. Template State Transition Diagram

```
                    ┌──────────────────┐
                    │  Template File   │
                    │ with Tera Code   │
                    └────────┬─────────┘
                             │
                             v
                    ┌──────────────────┐
                    │  S0: UNVALIDATED │
                    └────────┬─────────┘
                             │
                    [Run: parse_template()]
                             │
              ┌──────────────┴───────────────┐
              │                              │
              v                              v
    ┌────────────────────┐      ┌────────────────────┐
    │  S1: PARSE_PASS    │      │  S2: PARSE_FAIL    │
    │ (syntax valid)     │      │ (syntax invalid)   │
    └────────┬───────────┘      └─────────────────────┘
             │
    [Check: referenced in ggen.toml?]
             │
    ┌────────┴────────┐
    │                 │
   NO                YES
    │                 │
    v                 v
┌──────────────┐  ┌──────────────────┐
│ S3:          │  │  S4:             │
│ OUT_OF_SCOPE │  │ PARSE_PASS +     │
│              │  │ ACTIVE           │
└──────────────┘  └────────┬─────────┘
                           │
                [Render context available?]
                           │
                  ┌────────┴────────┐
                  │                 │
                 NO                YES
                  │                 │
                  v                 v
            ┌──────────────┐  ┌──────────────┐
            │  S5:         │  │  Call:       │
            │  PARSE_PASS  │  │  render()    │
            │  (no render) │  │              │
            └──────────────┘  └──────┬───────┘
                                     │
                        ┌────────────┴────────────┐
                        │                         │
                     SUCCESS                  FAILURE
                        │                         │
                        v                         v
                  ┌───────────────┐    ┌──────────────────┐
                  │ S6:           │    │ Check error type │
                  │ RENDER_PASS   │    └────────┬─────────┘
                  │ (all vars OK) │             │
                  └───────────────┘     ┌───────┴────────┐
                                        │                │
                                   Missing var?      Type error?
                                        │                │
                                       YES              YES
                                        │                │
                        ┌───────────────v──────┐  ┌──────v────────┐
                        │  S7:                 │  │  S8:          │
                        │  CONTEXT_MISSING     │  │  RENDER_FAIL  │
                        │  (incomplete context)│  │  (runtime err)│
                        └──────────────────────┘  └───────────────┘


TRANSITIONS:
  S0 → S1: PARSE_PASS (Tera parser succeeds)
  S0 → S2: PARSE_FAIL (Tera parser fails)
  S1 → S3: OUT_OF_SCOPE (valid syntax, not active)
  S1 → S4: PARSE_PASS + ACTIVE (valid syntax, is active)
  S4 → S5: PARSE_PASS (no context provided, cannot render)
  S4 → S6: RENDER_PASS (render succeeds, all context OK)
  S4 → S7: CONTEXT_MISSING (render fails due to missing variables)
  S4 → S8: RENDER_FAIL (render fails due to type error)
  S2 → (terminal): PARSE_FAIL (terminal state)
  S3 → (terminal): OUT_OF_SCOPE (terminal state)
  S5 → (terminal): PARSE_PASS (no render attempted)
  S6 → (terminal): RENDER_PASS (success)
  S7 → (terminal): CONTEXT_MISSING (documented)
  S8 → (terminal): RENDER_FAIL (error)
```

---

## 5. Decision Tree: What State is This Template In?

```
START: You have a template file

  ┌─────────────────────────────────────────────────┐
  │ Question 1: Does Tera parser accept the syntax? │
  └─────────────────────────────────────────────────┘
  
           ┌──────────────┬──────────────┐
          NO             YES
           │              │
           v              v
  ┌──────────────┐  ┌──────────────────────────────────┐
  │ PARSE_FAIL   │  │ Question 2: Is template          │
  │              │  │ referenced in ggen.toml rules?   │
  │ ✗ Template   │  └──────────────────────────────────┘
  │   syntax is  │
  │   broken     │           ┌──────────────┬──────────────┐
  │              │          NO             YES
  │              │           │              │
  │ → See error  │           v              v
  │   message    │  ┌────────────────┐  ┌────────────────────┐
  │              │  │ OUT_OF_SCOPE   │  │ Question 3: Do you │
  │ → Fix syntax │  │                │  │ have a rendering   │
  └──────────────┘  │ ✓ Syntax valid │  │ context object?    │
                    │ ✗ Not in rules │  └────────────────────┘
                    │                │
                    │ → OK to ignore │           ┌──────┬──────┐
                    │   if not used  │          NO    YES
                    └────────────────┘           │      │
                                                v      v
                                      ┌──────────────┐┌────────────┐
                                      │ PARSE_PASS   ││ Question 4:│
                                      │ (no render)  ││ Do all     │
                                      │              ││ variables  │
                                      │ ✓ Syntax OK  ││ in         │
                                      │ ✗ Context    ││ template   │
                                      │   not given  ││ exist in   │
                                      │              ││ context?   │
                                      │ → Store for  ││            │
                                      │   later use  ││ └──────┬───┘
                                      └──────────────┘        │
                                                   ┌──────────┴─────────┐
                                                  NO                  YES
                                                   │                   │
                                                   v                   v
                                        ┌─────────────────┐  ┌──────────────────┐
                                        │ CONTEXT_MISSING │  │ Question 5:      │
                                        │                 │  │ Does render work  │
                                        │ ✓ Syntax valid  │  │ without errors?   │
                                        │ ✓ Context given │  └──────┬───────────┘
                                        │ ✗ Vars missing  │         │
                                        │                 │  ┌──────┴─────────┐
                                        │ → Add missing   │ YES              NO
                                        │   variables to  │  │                │
                                        │   context or    │  v                v
                                        │   remove from   │┌──────────────┐ ┌────────────┐
                                        │   template      ││ RENDER_PASS  │ │RENDER_FAIL │
                                        └─────────────────┘│              │ │            │
                                                          │✓ All syntax  │ │✓ Syntax OK │
                                                          │  OK          │ │✓ Context   │
                                                          │✓ All vars    │ │  OK        │
                                                          │  bound       │ │✗ Runtime   │
                                                          │✓ Renders OK  │ │  error     │
                                                          │              │ │            │
                                                          │→ Template    │ │→ Check     │
                                                          │  ready to    │ │  error     │
                                                          │  use!        │ │  message:  │
                                                          │              │ │  - type    │
                                                          └──────────────┘ │    error   │
                                                                          │  - filter  │
                                                                          │    error   │
                                                                          │  - other   │
                                                                          └────────────┘
```

---

## 6. Error Classification Hierarchy

```
┌─────────────────────────────────────────────────────┐
│            VALIDATOR ERROR CLASSIFICATION            │
└─────────────────────────────────────────────────────┘

PARSE ERRORS (Tera parser rejects)
├─ Unclosed Delimiters
│  ├─ {{ without }}
│  ├─ {% without %}
│  └─ Block open without close (if/endif, for/endfor, etc.)
├─ Invalid Keywords
│  ├─ Unknown block name (e.g., {% invalid %})
│  ├─ Unexpected token in block
│  └─ Invalid operator syntax
├─ Mismatched Blocks
│  ├─ if...endfor
│  ├─ for...endif
│  └─ Nested block mismatch
├─ Syntax Errors in Expressions
│  ├─ Operator without operand ({{ 42 + }})
│  ├─ Missing field name ({{ obj. }})
│  ├─ Empty array index ({{ items[] }})
│  └─ Invalid filter syntax ({{ x | }})
└─ Malformed Structure
   ├─ Orphan close tag ({% endif %} without {% if %})
   ├─ Triple or more braces ({{{ x }}})
   └─ Unclosed string literal in expression

RENDER ERRORS (Parser OK, runtime fails)
├─ Type Errors
│  ├─ Filter applied to wrong type
│  │  └─ join() on integer
│  ├─ Array index on non-array
│  └─ Method call on incompatible type
├─ Undefined Variables (Tera behavior)
│  ├─ Variable not in context
│  └─ Field missing in nested object
│  CLASSIFY AS: CONTEXT_MISSING (not RENDER_FAIL)
├─ Filter Errors
│  ├─ Unknown filter name
│  ├─ Filter with wrong argument count
│  └─ Filter with invalid argument
└─ Expression Errors
   ├─ Division by zero
   └─ Invalid comparison type

NOT VALIDATOR ERRORS (Out of scope)
├─ Generated artifact content
├─ ggen.toml syntax
├─ SPARQL/TTL/RQ syntax
└─ Manufacturing pipeline results
```

---

## 7. State Table (Comprehensive)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Template Validator State Reference Table                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│ State            │ Parse │ Render │ Root Cause         │ Action             │
├──────────────────┼───────┼────────┼────────────────────┼────────────────────┤
│ PARSE_PASS       │ PASS  │ -      │ Syntax is valid    │ OK; may render     │
│ OUT_OF_SCOPE     │ PASS  │ -      │ Not in rules       │ OK to skip         │
│ PARSE_FAIL       │ FAIL  │ none   │ Syntax error       │ Fix syntax         │
├──────────────────┼───────┼────────┼────────────────────┼────────────────────┤
│ RENDER_PASS      │ PASS  │ PASS   │ All OK             │ Ready to use       │
│ CONTEXT_MISSING  │ PASS  │ MISSING│ Missing variables  │ Add context vars   │
│ RENDER_FAIL      │ PASS  │ FAIL   │ Type/runtime error │ Fix expression     │
└─────────────────────────────────────────────────────────────────────────────┘

CARDINALITY:
  PARSE_PASS         → 1..N (may have 0, 1, or many render states)
  PARSE_FAIL         → Exactly 1 (terminal, no render state)
  OUT_OF_SCOPE       → Exactly 1 (valid syntax, not tested)
  RENDER_PASS        → 0..1 (if context provided, may pass or fail)
  CONTEXT_MISSING    → 0..1 (if context provided but incomplete)
  RENDER_FAIL        → 0..1 (if context provided and type error)

Note: A template can have PARSE_PASS + multiple render states
in different test scenarios (with different contexts).
```

---

## 8. Context Availability States

```
┌────────────────────────────────────────────────────┐
│   When is render validation performed?             │
└────────────────────────────────────────────────────┘

SCENARIO 1: No context provided
  Input: template + no context object
  Parse:  Always execute
  Render: Not executed
  State:  PARSE_PASS (if syntax valid)
          PARSE_FAIL (if syntax invalid)
  Output: Only parse status reported

SCENARIO 2: Empty context provided
  Input: template + context = {}
  Parse:  Execute
  Render: Execute (all variables will be missing)
  State:  PARSE_PASS + CONTEXT_MISSING
  Output: Both parse and render status reported
          Missing variable list included

SCENARIO 3: Partial context provided
  Input: template + context = { some_vars: {...} }
  Parse:  Execute
  Render: Execute (some variables present, some missing)
  State:  PARSE_PASS + CONTEXT_MISSING (if any vars missing)
          PARSE_PASS + RENDER_PASS (if all vars present)
  Output: Parse and render status reported
          Missing variable list included (if any)

SCENARIO 4: Complete context provided
  Input: template + context = { all_required_vars: {...} }
  Parse:  Execute
  Render: Execute
  State:  PARSE_PASS + RENDER_PASS (if no type errors)
          PARSE_PASS + RENDER_FAIL (if type errors)
  Output: Parse and render status reported
          Error details included (if render fails)

DECISION:
  IF (context is null or undefined)
    Emit: parse_status only
  ELSE IF (context is object)
    Emit: parse_status + render_status + missing_variables
```

---

## 9. Quick Reference: What Each State Means

```
PARSE_PASS
  Meaning:  Tera parser accepts the template syntax
  Example:  {{ name }} is valid Tera
  Implication: Template can be used once context is provided
  Action:   Move to render validation if context available

PARSE_FAIL
  Meaning:  Tera parser rejects the template syntax
  Example:  {{ name is not valid Tera
  Implication: Template cannot be used until syntax is fixed
  Action:   Fix the syntax error

OUT_OF_SCOPE
  Meaning:  Template syntax is valid, but not referenced in active rules
  Example:  future-feature.tera exists but ggen.toml doesn't use it
  Implication: Template may be incomplete or experimental; acceptable to skip
  Action:   Can safely ignore if not part of manufacturing pipeline

RENDER_PASS
  Meaning:  Template parsed successfully AND rendered with provided context
  Example:  {{ name }} rendered with context { name: "Alice" }
  Implication: Template is production-ready for this context
  Action:   Approve for use; no further validation needed

CONTEXT_MISSING
  Meaning:  Template parsed successfully BUT context is incomplete
  Example:  {{ author }} with context { title: "Report" }
  Implication: Template is valid but needs more context variables
  Action:   Provide missing variables or remove from template

RENDER_FAIL
  Meaning:  Template parsed successfully BUT render failed with provided context
  Example:  {{ items | length }} where items = 42 (not an array)
  Implication: Template has type mismatch with context data
  Action:   Fix the template or provide correct context type
```

---

## 10. Validation Flow Checklist

```
Template Validator Execution Checklist
═════════════════════════════════════════════════════════════

FOR EACH TEMPLATE:

PHASE 1: SETUP
  ☐ Read template file
  ☐ Check file is readable
  ☐ Check file ends with .tera
  ☐ Extract template content

PHASE 2: PARSE
  ☐ Pass template to Tera parser
  ☐ Capture parse result (success or error)
  ☐ If error: Emit PARSE_FAIL with error message → STOP
  ☐ If success: Emit PARSE_PASS → Continue to Phase 3

PHASE 3: SCOPE CHECK
  ☐ Search ggen.toml for references to this template
  ☐ If not referenced: Emit OUT_OF_SCOPE → STOP
  ☐ If referenced: Continue to Phase 4

PHASE 4: RENDER (Only if context available)
  ☐ Check if context object is provided
  ☐ If no context: Emit PARSE_PASS (no render) → STOP
  ☐ If context provided: Extract variable bindings
  ☐ Identify variables referenced in template
  ☐ Find missing variables (in template but not in context)
  ☐ If any missing: Emit CONTEXT_MISSING → STOP
  ☐ If complete: Attempt to render

PHASE 5: RENDER EXECUTION
  ☐ Call Tera render engine with template + context
  ☐ If render succeeds: Emit RENDER_PASS → STOP
  ☐ If render fails:
     ☐ Classify error type (type error, filter error, etc.)
     ☐ If missing variable: Emit CONTEXT_MISSING (go back to Phase 4)
     ☐ If type error: Emit RENDER_FAIL → STOP
     ☐ If other: Emit RENDER_FAIL with error details → STOP

PHASE 6: REPORT
  ☐ Emit validation result with all captured details
  ☐ Include: path, parse_status, render_status, errors, missing_vars
  ☐ Aggregate results to validator report
```

---

## Conclusion

This document specifies the complete validator state machine. All implementation must follow these diagrams and state definitions exactly. Deviations constitute contract breach.

**Status:** READY FOR REFERENCE  
**Authority:** Process Intelligence Research Directorate  
**Date:** 2026-06-01
