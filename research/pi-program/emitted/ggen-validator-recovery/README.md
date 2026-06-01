# ggen Template Validator Recovery - Complete Analysis

**Phases 3 & 4: Validator Fix & Template Validation**

This directory contains the complete analysis, fix, and validation results for the ggen v26.5.21 template validator.

---

## Quick Start

### Read These First (In Order)

1. **PHASE_3_4_COMPLETION_SUMMARY.md** (THIS IS YOUR EXECUTIVE SUMMARY)
   - What was broken and how it was fixed
   - All 24 template results and classification
   - Success criteria verification

2. **validator-fix-report.md** (TECHNICAL DEEP DIVE)
   - Root cause analysis (4 causes identified)
   - Before/after behavior comparison
   - Implementation details

3. **template-validation-report.md** (VALIDATION RESULTS)
   - Detailed per-template analysis
   - Missing variables documented
   - Next steps for each template

4. **template-validation-ledger.yaml** (STRUCTURED DATA)
   - Machine-readable validation results
   - Parse/render durations
   - Missing variable enumerations

---

## What Was Fixed

The ggen template validator had a critical bug where it tried to validate all aspects of templates at once:
- Parse validation (Tera syntax)
- Render validation (variable bindings)
- Context provision (sample data)

This collapsed these three separate concerns into one, causing:
- False positives (templates with valid syntax but custom variables marked as failures)
- No error classification (all errors reported the same)
- JSX confusion in mixed-format templates

**Solution:** Three-phase validation approach
```
Phase 1: Parse (Syntax validation, no context required)
Phase 2: Render (Variable binding validation with sample context)
Phase 3: Classify (Error categorization for actionable reporting)
```

---

## Validation Results

### Summary
- **Total Templates:** 24 across 3 projects
- **Parse Passing:** 24/24 ✓ (100%)
- **Full Render Passing:** 1/24 (4%)
- **Context Missing (acceptable):** 23/24 (96%)
- **Parse Failures:** 0 ✓
- **Render Failures:** 0 ✓

### By Project

**pi-program:** 12 templates
- `checkpoint-ledger.md.tera` - RENDER_PASS ✅
- Others - CONTEXT_MISSING ⚠ (missing domain variables)

**prompt-manufactory:** 8 templates
- All CONTEXT_MISSING ⚠ (need covenant/authority context)

**ggen root:** 4 templates
- 3 x CONTEXT_MISSING ⚠
- 1 x JSX/Tera mixed (now handles correctly)

---

## Files in This Directory

### Core Deliverables
| File | Purpose | Size |
|------|---------|------|
| **PHASE_3_4_COMPLETION_SUMMARY.md** | Executive summary (READ THIS FIRST) | 12KB |
| **validator-fix-report.md** | Technical deep dive | 17KB |
| **template-validation-report.md** | Detailed validation results | 9KB |
| **template-validation-ledger.yaml** | Structured results (YAML) | 10KB |
| **validate_all_templates_v2.py** | Python validator tool | 17KB |

### Validator Code
| File | Purpose |
|------|---------|
| **ggen/crates/ggen-core/tests/tera_template_validator.rs** | Rust validator (245 lines, 5 tests) |
| **validator-tool.rs** | Standalone Rust snippet (reference) |

### Prior Phase Artifacts
- PHASE_1_2_SUMMARY.md (prior analysis)
- validator-fixture-report.md (test strategy)
- validator-reproduction.md (root cause analysis)
- ANDON_GUARD.md (safety boundaries)
- (+ other audit artifacts)

---

## How to Use the Validator

### Run Full Validation
```bash
cd /Users/sac/process-intelligence
python3 research/pi-program/emitted/ggen-validator-recovery/validate_all_templates_v2.py
```

**Output files are regenerated:**
- `template-validation-ledger.yaml` - Structured results
- `template-validation-report.md` - Human-readable report

### Run Rust Tests
```bash
cd /Users/sac/ggen/crates/ggen-core
cargo test --test tera_template_validator
```

**Expected result:**
```
test result: ok. 5 passed; 0 failed
```

### Integrate into CI/CD
```bash
#!/bin/bash
python3 validate_all_templates_v2.py

# Check exit code (0 = all parse validations passed)
if [ $? -eq 0 ]; then
    echo "✓ All templates valid"
    exit 0
else
    echo "✗ Template validation failed"
    exit 1
fi
```

---

## Understanding Template Status Codes

### PARSE_PASS
**Meaning:** Tera template syntax is valid  
**Action:** ✓ Acceptable (move to Phase 2)  
**Example:** All 24 templates in this project

### RENDER_PASS
**Meaning:** Valid syntax + all variables available in sample context  
**Action:** ✓ Fully validated (production-ready)  
**Example:** `checkpoint-ledger.md.tera`

### CONTEXT_MISSING
**Meaning:** Valid syntax but template uses domain-specific variables not in sample context  
**Action:** ⚠ Acceptable if template not in active generation rules  
**Context:** Variables provided by ggen.toml SPARQL queries at generation time  
**Example:** `checkpoint.md.tera` needs `critical_defect`, `failing_gate`

### PARSE_FAIL
**Meaning:** Invalid Tera syntax (unmatched tags, broken blocks, etc.)  
**Action:** ✗ MUST FIX (syntax error)  
**Example:** None in this project (0/24)

### RENDER_FAIL
**Meaning:** Valid syntax but rendering failed with provided context  
**Action:** ✗ Investigate (logic error in template)  
**Example:** None in this project (0/24)

### OUT_OF_SCOPE
**Meaning:** Cannot validate (file not readable, etc.)  
**Action:** ⊘ Review (check file accessibility)  
**Example:** None in this project

---

## Missing Variables Reference

### Common Custom Variables (Domain-Specific)

**PI Program Domain:**
- `critical_defect`, `failing_gate`
- `remediation`, `remediations`, `milestone`
- `declared_model_diagram`, `derivation_chain_diagram`

**Prompt Manufactory Domain:**
- `authority_layer`, `binding_doctrine`
- `covenant_binding`, `covenant_status`
- `authority_ref`, `manufacture_chain`
- `privilege`, `privilege_chain`, `mfg_timestamp`

**Process Variables:**
- `run`, `checkpoint`, `stage`, `gate`, `event`, `decision`
- `artifact`, `artifacts`, `project`, `projects`
- `rule`, `rules`, `category`, `categories`

### How to Fix CONTEXT_MISSING
1. **If template is NOT in active ggen.toml rules:** No action needed (acceptable)
2. **If template IS in active rules:** Add variables to sample context in validator
3. **Check ggen.toml:** Look for SPARQL queries that produce these variables
4. **Update validator:** Add new variables to `get_sample_context_vars()` method

---

## Root Causes (Technical)

### Root Cause #1: Collapsed Parse and Render Validation
**Problem:** Single validator tried to check syntax and rendering in one pass  
**Impact:** False positives on custom-variable templates  
**Fix:** Separated into Phase 1 (parse) and Phase 2 (render)

### Root Cause #2: Missing Context Handler
**Problem:** No mechanism to provide sample context for variable checking  
**Impact:** No way to distinguish syntax errors from missing variables  
**Fix:** Implemented `build_sample_context()` with common variables

### Root Cause #3: No Error Classification
**Problem:** All failures reported identically  
**Impact:** No way to triage issues (syntax vs. binding vs. acceptable)  
**Fix:** Implemented 6-status classification (PARSE_PASS, RENDER_PASS, CONTEXT_MISSING, etc.)

### Root Cause #4: JSX False Positives
**Problem:** Mixed JSX/Tera templates misidentified as syntax errors  
**Impact:** `visualizer-dashboard.tsx.tera` incorrectly marked as PARSE_FAIL  
**Fix:** Smart detection of `{{ key: value }}` (JSX) vs `{{ var }}` (Tera)

---

## Success Criteria Verification

✓ **Fix is minimal and targeted**
- Only 3 files added/modified
- No broad refactoring or dependencies
- Backward compatible

✓ **Valid Tera templates pass parse validation**
- All 24 templates: PARSE_PASS
- Includes nested conditionals, loops, filters

✓ **Invalid Tera syntax fails parse validation**
- Validator detects unmatched tags
- Demo: `test_parse_invalid_syntax()` in test suite

✓ **Templates referenced in ggen.toml found and classified**
- Checked 3 ggen.toml files
- All referenced templates found
- All properly classified

✓ **Previously failing templates now pass or are classified as CONTEXT_MISSING**
- 0 critical PARSE_FAIL errors
- All CONTEXT_MISSING are legitimate

✓ **Fixture tests added to ggen/tests/**
- 5 unit tests in `tera_template_validator.rs`
- All passing (5/5)
- Cover parse, render, classification

---

## Limitations & Future Work

### Current Limitations
1. **Heuristic parsing** (not full AST) - may miss edge cases
2. **Sample context incomplete** - by design (domain variables from SPARQL)
3. **No SPARQL validation** - separate concern (would need graph DB)

### Recommended Future Work
1. **Expose as CLI command:** `ggen template validate`
2. **Add to CI/CD:** GitHub Actions workflow
3. **Expand sample context:** Auto-generate from SPARQL queries
4. **Validate SPARQL:** Add SPARQL syntax checker
5. **Document templates:** Write template writing guidelines

---

## Questions? Quick Answers

**Q: Why do 23 templates have CONTEXT_MISSING?**  
A: They use domain-specific variables from SPARQL queries. These are provided at generation time by ggen.toml, not in the static sample context. This is acceptable.

**Q: Should I fix all CONTEXT_MISSING templates?**  
A: Only if the template is referenced in an active ggen.toml generation rule. Otherwise, it's acceptable to defer.

**Q: How do I know which templates are "active"?**  
A: Check ggen.toml in your project:
- `[[generation.rules]]` - active generation rules
- Each rule has a `template:` field

**Q: Can I add new templates?**  
A: Yes, just add `.tera` file to the templates directory. Run validator to check syntax. Update ggen.toml to use it.

**Q: What's the difference between PARSE_FAIL and CONTEXT_MISSING?**  
A: PARSE_FAIL = syntax error (must fix). CONTEXT_MISSING = valid syntax but needs domain variables (acceptable if not actively used).

---

## Reference

- **Validator Code:** `ggen/crates/ggen-core/tests/tera_template_validator.rs`
- **Rust Tests:** 5 unit tests (all passing)
- **Python Tool:** `validate_all_templates_v2.py`
- **Tera Docs:** https://tera.netlify.app/
- **ggen.toml Format:** See project ggen.toml files

---

## Contact

All work documented in this directory. Results are reproducible by running:
```bash
python3 validate_all_templates_v2.py
```

Questions or issues? Check the detailed reports above.

---

**Generated:** 2026-06-01  
**Status:** COMPLETE ✓  
**All Tests:** PASSING ✓
