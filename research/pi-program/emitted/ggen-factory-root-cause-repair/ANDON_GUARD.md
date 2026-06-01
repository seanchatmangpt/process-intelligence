# ANDON Guard — PI GGEN Factory Root Cause Repair

**Authority:** Process Intelligence Research Directorate  
**Issued:** 2026-06-01  
**Scope:** PHASE 0 gate specification for PI GGEN Validator Recovery remediation

---

## Hard Rules

### No Broad Census

- Gates evaluate specific, testable criteria only
- No "audit all files" unless scope is numerically bounded and documented
- Census output (file counts, totals) is evidence source only, not verdict justification
- Example FORBIDDEN: "5,000 files checked, 4,500 valid → ALIVE"
- Example PERMITTED: "All 5 tera files found in sources/ggen/templates/ have valid syntax (5/5 PASS)"

### No Downstream Execution Before Validator Contract Passes

- Validator contract specification (PHASE 2) must be written BEFORE any code changes
- Code changes to validator logic, templates, or output are FORBIDDEN until contract is signed
- Validator behavior is bound to contract; changes invalidate prior audit evidence
- Exception: Bug fixes to contract implementation are permitted ONLY if they narrow (not expand) validator scope

### No Manual Rendering as ALIVE

- ALIVE verdicts are automated: all gates execute, all gates PASS
- ALIVE verdicts are never issued by human decision or override
- ALIVE verdicts are never issued with outstanding blocking gaps
- If gates fail → verdict is PARTIAL or FAILED (never manual ALIVE)

### No Percentage Thresholds

- Verdict law does not include percentage-based thresholds (e.g., "75% gates PASS → ALIVE")
- Verdict is binary per gate (PASS/FAIL)
- Verdict rollup is logical (all PASS → ALIVE; some FAIL → PARTIAL; workflow failure → FAILED)
- Example FORBIDDEN: "10/15 gates PASS (67%) → PARTIAL"
- Example PERMITTED: "Gates {1,3,7,14,15} PASS; Gates {2,4,5,6,8,9,10,11,12,13} FAIL → PARTIAL"

### No 8/15 PARTIAL Rule

- PARTIAL verdict is NOT determined by "≥8 gates PASS"
- PARTIAL verdict is determined by: "some gates PASS, some gates FAIL, workflow executed and evidence is trustworthy"
- Gate count (15) is implementation detail, not verdict logic
- Example FORBIDDEN: "If ≥8 gates PASS → PARTIAL"
- Example PERMITTED: "5 gates PASS, 10 gates FAIL, blocker evidence documented → PARTIAL with Andon status"

### No File-Count ALIVE

- ALIVE verdict is never justified by "X files generated" or "Y files classified"
- ALIVE verdict is never justified by file counts, line counts, or census metrics
- File counts are audit evidence only, not verdict justification
- Example FORBIDDEN: "Generated 500 artifacts → ALIVE"
- Example PERMITTED: "All 15 conformance gates executed and passed → ALIVE"

### No Commit-Count ALIVE

- ALIVE verdict is never justified by "X commits made" or "Y pull requests merged"
- ALIVE verdict is never justified by commit frequency, velocity, or history metrics
- Commits are immutable evidence sources only, not verdict justification
- Example FORBIDDEN: "Made 12 commits → ALIVE"
- Example PERMITTED: "All 15 gates PASS with evidence from 12 committed fixes → ALIVE"

### Validator Contract Must Be Written Before Code Changes

- Validator contract (PHASE 2) specifies exact behavior: parse, render, output states
- Contract is frozen before any validator code is modified
- Contract changes require new phase/checkpoint/authority decision
- Behavior changes must be documented as contract amendments, not silent code updates
- All gates that depend on validator logic must reference specific contract sections

---

## Compliance Checklist

| Rule | Status | Verification Method |
|------|--------|----------------------|
| No broad census | ENFORCED | Audit results specify exact file paths or bounded sets |
| No downstream execution before contract | ENFORCED | Contract signed (PHASE 2) before code changes |
| No manual ALIVE rendering | ENFORCED | ALIVE verdict only if all gates automated-PASS |
| No percentage thresholds | ENFORCED | All verdict logic is binary (PASS/FAIL), not percentage-based |
| No 8/15 PARTIAL rule | ENFORCED | PARTIAL = some PASS + some FAIL + trustworthy evidence |
| No file-count ALIVE | ENFORCED | Verdicts reference gate results, not file metrics |
| No commit-count ALIVE | ENFORCED | Verdicts reference gate results, not commit metrics |
| Validator contract first | ENFORCED | Contract written (PHASE 2) before code review/changes |

---

## Authority

**Issued by:** Process Intelligence Research Directorate  
**Effective date:** 2026-06-01  
**Scope:** PHASE 0 through Phase 13 (closure)

This document is immutable. Violations require new checkpoint authority.

---

