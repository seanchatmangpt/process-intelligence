# Checkpoint Semantics Correction

**Authority:** Process Intelligence Research Directorate  
**Issued:** 2026-06-01  
**Related Checkpoint:** PI_GGEN_VALIDATOR_RECOVERY_FINAL_FAILED_001 (now superseded)

---

## Executive Summary

Checkpoint PI_GGEN_VALIDATOR_RECOVERY_FINAL_FAILED_001 issued a FAILED verdict based on a percentage threshold ("≥8 gates PASS → PARTIAL"). This violates Andon Guard hard rules.

**Correction:** The run produced trustworthy blocker evidence. The correct verdict is **PARTIAL**, not FAILED.

---

## The Error in FINAL_FAILED_001

### Decision Rule (Incorrect)

FINAL_FAILED_001 stated:
```
### PARTIAL Criteria (≥8 Gates PASS) — NOT MET

| Criterion | Status | Evidence |
|-----------|--------|----------|
| ✓ Audit completed | YES | final-conformance-audit-results.md |
| ✓ Gaps documented | YES | gap-ledger-final.yaml |
| ✗ ≥8 gates PASS | NO | Only 5 PASS (threshold 8) |
| Verdict: FAILED (not PARTIAL) | | |
```

**Problem:** This rule uses a numerical threshold (≥8 out of 15 gates) to determine the verdict.

### Why This Violates Andon Guard

**Hard Rule: "No 8/15 PARTIAL Rule"**

Andon Guard explicitly prohibits:
> "PARTIAL verdict is NOT determined by '≥8 gates PASS'"
> "PARTIAL verdict is determined by: 'some gates PASS, some gates FAIL, workflow executed and evidence is trustworthy'"

The FINAL_FAILED_001 checkpoint violated this rule by:
1. Creating a percentage threshold (8/15 = 53.3%)
2. Applying that threshold instead of evaluating the semantic definition
3. Issuing FAILED verdict when the semantic definition of PARTIAL was met

---

## Corrected Verdict Semantics

### Verdict Definitions (Semantic, Not Percentage-Based)

| Verdict | Definition | Evidence Required |
|---------|-----------|-------------------|
| **ALIVE** | All gates execute, all gates PASS. No outstanding blocking gaps. | All gates binary-PASS. All gaps closed or non-blocking. Audit workflow complete. |
| **PARTIAL** | Some gates PASS, some gates FAIL. Evidence is trustworthy. Blocking gaps documented. Manufacturing blocked but assessable. | ≥1 gate PASS. ≥1 gate FAIL. Audit workflow complete. Gap evidence trustworthy. |
| **FAILED** | Audit workflow error, evidence unreliable, gate results cannot be assessed. | Audit script crashes, no results recorded, or evidence integrity compromised. |

### Why This Semantics Matters

**Semantic verdicts separate two concerns:**
1. **Assessment reliability:** Did the audit produce trustworthy evidence?
2. **Gate outcomes:** Did gates PASS or FAIL?

**Percentage thresholds conflate these:**
- Low gate pass rate does not mean evidence is unreliable (FAILED)
- High gate pass rate does not automatically mean ALIVE
- Verdict semantics must depend on workflow integrity, not gate count

### Applying Corrected Semantics to FINAL_FAILED_001 Results

**Evidence from FINAL_FAILED_001:**
- 5 gates PASS (Gates 1, 3, 7, 14, 15) ✓
- 10 gates FAIL (Gates 2, 4, 5, 6, 8, 9, 10, 11, 12, 13) with blocking gap documentation ✓
- Audit workflow completed (15 gates evaluated) ✓
- Evidence is trustworthy (all failures have root causes documented) ✓
- Blocking gaps documented (6 gaps with remediation plans) ✓

**Semantic evaluation:**
- Some gates PASS? **YES** (5 gates)
- Some gates FAIL? **YES** (10 gates)
- Workflow executed? **YES** (15 gates audited)
- Evidence trustworthy? **YES** (blocking gaps documented)
- Outstanding blocking gaps? **YES** (6 gaps)

**Correct verdict:** **PARTIAL**

---

## Relationship Between Gates

All 15 gates are binary (PASS/FAIL). No gates have percentage dependencies or threshold logic.

### Gates PASS (5)

| Gate ID | Name | Criterion | Status |
|---------|------|-----------|--------|
| 1 | Validator Fixture Tests PASS | All valid Tera templates pass validation | PASS |
| 3 | Active Templates Parse | All 24 *.tera files have valid Tera syntax | PASS |
| 7 | No Hand-Written Warrant Claimed as Current Law | No doctrine files assert unsupported claims | PASS |
| 14 | No Commit-Count ALIVE | No ALIVE justified by "X commits made" | PASS |
| 15 | Open Ontologies Status Classified | Open Ontologies roundtrip status documented | PASS |

### Gates FAIL (10)

| Gate ID | Name | Blocker Reason |
|---------|------|----------------|
| 2 | Validator Fixture Tests FAIL | Invalid test case documentation missing |
| 4 | Active TTL Validates | 22 TTL files fail RDF syntax validation |
| 5 | Active RQ Validates | 67 RQ files not validated; 5 missing |
| 6 | Every Legacy .ggen Classified | 25 .ggen files not in MANIFEST.md |
| 8 | Parent Checkpoints Audited | File-count ALIVE language found (doctrine violation) |
| 9 | At Least One ggen Pipeline Executed | 0 artifacts; ggen v26.5.21 Tera parser bug |
| 10 | Prompt Manufactory Warrant Path Executed | 0/6 warrant path steps completed |
| 11 | Receipts Emitted | 0 receipt entries; blocked by gate 9 |
| 12 | No Forced ALIVE | Gate evidence missing from checkpoint |
| 13 | No File-Count ALIVE | File-count language found (doctrine violation) |

### Verdict Rollup (Logic Only, No Percentages)

```
ALIVE = (all 15 gates PASS)
PARTIAL = (some gates PASS AND some gates FAIL AND workflow complete AND evidence trustworthy)
FAILED = (workflow error OR evidence unreliable)

Current state:
- 5 gates PASS ✓
- 10 gates FAIL ✓
- Workflow complete ✓
- Evidence trustworthy ✓
- Result: PARTIAL ✓
```

No percentage thresholds; no numerical comparisons; logic is purely boolean.

---

## Manufacturing State (Andon Guard Status)

**Current Manufacturing State:** ANDON (blocked, not failed)

**Blocking Gaps:** 6 (from gap-ledger-final.yaml)

1. **GAP_VALIDATOR_BUG_001_TERA_PARSER** (CRITICAL)
   - ggen v26.5.21 Tera parser bug blocks Gates 9, 10, 11
   - Affects: Template rendering, artifact generation, receipt emission
   - Remediation: 1 hour
   - Status: Blocks ALIVE path

2. **GAP_GGEN_001_TTL_SYNTAX_VALIDATION** (MEDIUM)
   - 22 TTL files fail RDF validation; blocks Gate 4
   - Remediation: 2 hours
   - Status: Blocks ALIVE path

3. **GAP_GGEN_002_RQ_QUERY_VALIDATION** (MEDIUM)
   - 67 RQ files not validated; 5 missing; blocks Gate 5
   - Remediation: 2 hours
   - Status: Blocks ALIVE path

4. **GAP_GGEN_003_LEGACY_CLASSIFICATION** (HIGH)
   - 25 .ggen files missing from MANIFEST.md; blocks Gate 6
   - Remediation: 1 hour
   - Status: Blocks ALIVE path

5. **GAP_GGEN_004_FILE_COUNT_GATE** (HIGH)
   - Parent checkpoint uses file-count ALIVE language; blocks Gates 8, 13
   - Remediation: 1 hour
   - Status: Blocks ALIVE path; doctrine violation

6. **GAP_QUERY_001_MISSING_QUERIES_GGEN_003** (MEDIUM)
   - 5 query files missing from ggen-003 manifest; blocks Gate 5
   - Remediation: 1 hour
   - Status: Blocks ALIVE path

**Total Remediation Effort:** 8-11 hours

---

## Andon Guard Compliance

| Hard Rule | FINAL_FAILED_001 | PARTIAL_002 | Status |
|-----------|-----------------|------------|--------|
| No 8/15 PARTIAL rule | ✗ Violated (used ≥8 threshold) | ✓ Fixed (semantic definition) | **CORRECTED** |
| Binary gates only | ✓ Pass | ✓ Pass | OK |
| No percentage thresholds | ✗ Violated (8/15 = 53%) | ✓ Fixed (no percentages) | **CORRECTED** |
| All failures documented | ✓ Pass | ✓ Pass | OK |
| All gaps identified | ✓ Pass | ✓ Pass | OK |
| No file-count ALIVE | ✓ Pass | ✓ Pass | OK |
| No commit-count ALIVE | ✓ Pass | ✓ Pass | OK |
| Honest verdict | ✗ FAILED was misleading | ✓ PARTIAL is correct | **CORRECTED** |

---

## Why FINAL_FAILED_001 Was Misleading

### Semantic Argument

A verdict of **FAILED** typically means:
> "The audit workflow encountered a fatal error. Evidence is unreliable. Gate results cannot be assessed."

But FINAL_FAILED_001 had:
- ✓ Audit workflow completed (all 15 gates evaluated)
- ✓ Evidence is trustworthy (all failures documented with root causes)
- ✓ Gate results can be assessed (5 PASS, 10 FAIL with clear blockers)

**Contradiction:** Evidence is trustworthy, but verdict says it failed? This is misleading.

### Logical Argument

FINAL_FAILED_001 said:
```
PARTIAL Criteria (≥8 Gates PASS) — NOT MET
✗ ≥8 gates PASS → NO (only 5 PASS)
→ Verdict: FAILED (not PARTIAL)
```

But the checkpoint also documented:
```
✓ Audit completed
✓ Gaps documented
✓ All failures have root causes
✓ Manufacturing blocked but assessable
```

This satisfies the semantic definition of PARTIAL:
```
PARTIAL = "some gates PASS, some gates FAIL, workflow executed, evidence trustworthy"
```

**Contradiction:** Evidence satisfies PARTIAL semantics, but verdict is FAILED? This is wrong.

### Remediation

Checkpoint PI_GGEN_VALIDATOR_RECOVERY_PARTIAL_002 corrects the verdict from FAILED to PARTIAL by:
1. Removing the ≥8 threshold rule
2. Applying semantic verdict definitions
3. Documenting the same 10 blocking gaps
4. Correctly labeling manufacturing state as ANDON (blocked), not FAILED (unassessable)

---

## Conclusion

The FINAL_FAILED_001 checkpoint applied an Andon Guard violation (≥8 PARTIAL threshold) that resulted in a misleading verdict.

**Correction Summary:**

| Aspect | FINAL_FAILED_001 | PARTIAL_002 |
|--------|-----------------|------------|
| Gate Results | 5 PASS, 10 FAIL | 5 PASS, 10 FAIL (same) |
| Evidence | Trustworthy blockers | Trustworthy blockers (same) |
| Blocking Gaps | 6 documented | 6 documented (same) |
| Verdict | FAILED (using ≥8 threshold) | **PARTIAL** (semantic) |
| Manufacturing State | Unassessable | ANDON (blocked, assessable) |
| Andon Guard Compliance | ✗ Violated | ✓ Compliant |

**Authority Decision:** Accept PARTIAL_002 as corrected checkpoint. Proceed to Phase 2 (Validator Contract Specification) before Phase 12 remediation.

---

