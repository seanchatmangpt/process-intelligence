# Conformance Audit Results — Phase 8

**Timestamp:** 2026-06-01T13:16:38.154828
**Phase:** 8 (Conformance Audits)
**Checkpoint Verdict:** `FAILED`

## Summary

| Metric | Count |
|--------|-------|
| **Total Audits** | 14 |
| **Passed (PASS)** | 7 |
| **Failed (FAIL)** | 5 |
| **Skipped (SKIP)** | 2 |
| **Verdict** | `FAILED` |

### Verdict Reason

5 audits failed → checkpoint FAILED

---

## Audit Results

### By Status

#### Passed (7/14)

- ✓ `audit-no-invalid-new-ggen-source.rq` — No Invalid New Ggen Source
- ✓ `audit-no-file-count-alive.rq` — No File Count Alive
- ✓ `audit-no-forced-alive.rq` — No Forced Alive
- ✓ `audit-every-rendered-artifact-has-source-trace.rq` — Every Rendered Artifact Has Source Trace
- ✓ `audit-no-hand-written-research-warrant.rq` — No Hand Written Research Warrant
- ✓ `audit-no-commit-count-alive.rq` — No Commit Count Alive
- ✓ `audit-no-manual-prompt-writing.rq` — No Manual Prompt Writing

#### Failed (5/14)

- ✗ `audit-all-legacy-ggen-classified.rq`
  - **Name:** All Legacy Ggen Classified
  - **Passes If:** All legacy ggen files are documented with classification status
  - **Root Cause:** Evidence not found in loaded ontology graph

- ✗ `audit-warrant-path-exists.rq`
  - **Name:** Warrant Path Exists
  - **Passes If:** At least one READY rule traces to a SEALED checkpoint through template
  - **Root Cause:** Evidence not found in loaded ontology graph

- ✗ `audit-validator-passes-valid-tera.rq`
  - **Name:** Validator Passes Valid Tera
  - **Passes If:** At least one test case with valid Tera syntax passes validation
  - **Root Cause:** Evidence not found in loaded ontology graph

- ✗ `audit-validator-rejects-invalid-tera.rq`
  - **Name:** Validator Rejects Invalid Tera
  - **Passes If:** At least one test case with invalid Tera syntax fails validation
  - **Root Cause:** Evidence not found in loaded ontology graph

- ✗ `audit-partial-checkpoint-possible.rq`
  - **Name:** Partial Checkpoint Possible
  - **Passes If:** For each PARTIAL checkpoint, at least one gate passed and one gate failed
  - **Root Cause:** Evidence not found in loaded ontology graph


#### Skipped (2/14)

- ⊘ `audit-every-generation-rule-has-query-template-output.rq` — Every Generation Rule Has Query Template Output
  - **Reason:** Query execution error: Expected AskQuery, found 'FILTER'  (at char 414), (line:11, col:5)

- ⊘ `audit-checkpoints-have-receipts-or-explicit-missing.rq` — Checkpoints Have Receipts Or Explicit Missing
  - **Reason:** Query execution error: Expected AskQuery, found 'FILTER'  (at char 438), (line:11, col:5)

---

## Detailed Audit Definitions

### 1. audit-no-invalid-new-ggen-source

**Status:** ✓ PASS

**Purpose:** No Invalid New ggen Source Files

**Passes If:** No unclassified .ggen files discovered in projects


### 2. audit-all-legacy-ggen-classified

**Status:** ✗ FAIL

**Purpose:** All Legacy GGEN Files Are Classified

**Passes If:** All legacy ggen files are documented with classification status

**Failure Details:**

- **Reason:** Audit condition returned false
- **Likely Cause:** Required data not present in ontology graph
- **Remediation:** Check that all upstream manufacturing steps completed successfully


### 3. audit-no-file-count-alive

**Status:** ✓ PASS

**Purpose:** No File Count Inflation in ALIVE Checkpoints

**Passes If:** File counts in ledgers match declared counts in program registry


### 4. audit-no-forced-alive

**Status:** ✓ PASS

**Purpose:** No Forced ALIVE Verdicts

**Passes If:** All ALIVE checkpoints explicitly report gates met with zero arbitration


### 5. audit-every-generation-rule-has-query-template-output

**Status:** ⊘ SKIP

**Purpose:** Every Generation Rule Has Query, Template, and Output

**Passes If:** All READY rules declare query, template, and outputFile properties

**Skip Details:**

- **Reason:** Query execution error: Expected AskQuery, found 'FILTER'  (at char 414), (line:11, col:5)
- **Error:** Expected AskQuery, found 'FILTER'  (at char 414), (line:11, col:5)


### 6. audit-every-rendered-artifact-has-source-trace

**Status:** ✓ PASS

**Purpose:** Every Rendered Artifact Has Source Trace

**Passes If:** Each outputFile is reachable through rule->template->artifact path


### 7. audit-checkpoints-have-receipts-or-explicit-missing

**Status:** ⊘ SKIP

**Purpose:** Checkpoints Have Receipts or Explicit Missing Declaration

**Passes If:** All checkpoints declare receiptReference OR explicitly note receipt unavailable

**Skip Details:**

- **Reason:** Query execution error: Expected AskQuery, found 'FILTER'  (at char 438), (line:11, col:5)
- **Error:** Expected AskQuery, found 'FILTER'  (at char 438), (line:11, col:5)


### 8. audit-warrant-path-exists

**Status:** ✗ FAIL

**Purpose:** Warrant Path Exists (Rule -> Template -> Checkpoint)

**Passes If:** At least one READY rule traces to a SEALED checkpoint through template

**Failure Details:**

- **Reason:** Audit condition returned false
- **Likely Cause:** Required data not present in ontology graph
- **Remediation:** Check that all upstream manufacturing steps completed successfully


### 9. audit-no-hand-written-research-warrant

**Status:** ✓ PASS

**Purpose:** No Hand-Written Research Warrant

**Passes If:** Every research warrant in doctrine/ has prov:derivedFrom pointing to source


### 10. audit-validator-passes-valid-tera

**Status:** ✗ FAIL

**Purpose:** Validator Passes Valid Tera

**Passes If:** At least one test case with valid Tera syntax passes validation

**Failure Details:**

- **Reason:** Audit condition returned false
- **Likely Cause:** Required data not present in ontology graph
- **Remediation:** Check that all upstream manufacturing steps completed successfully


### 11. audit-validator-rejects-invalid-tera

**Status:** ✗ FAIL

**Purpose:** Validator Rejects Invalid Tera

**Passes If:** At least one test case with invalid Tera syntax fails validation

**Failure Details:**

- **Reason:** Audit condition returned false
- **Likely Cause:** Required data not present in ontology graph
- **Remediation:** Check that all upstream manufacturing steps completed successfully


### 12. audit-no-commit-count-alive

**Status:** ✓ PASS

**Purpose:** No Commit Count ALIVE

**Passes If:** No ALIVE checkpoint cites commitCount as sole or primary criterion


### 13. audit-no-manual-prompt-writing

**Status:** ✓ PASS

**Purpose:** No Manual Prompt Writing

**Passes If:** All prompts/ .md files marked as "current law" have prov:wasGeneratedBy


### 14. audit-partial-checkpoint-possible

**Status:** ✗ FAIL

**Purpose:** Partial Checkpoint Possible

**Passes If:** For each PARTIAL checkpoint, at least one gate passed and one gate failed

**Failure Details:**

- **Reason:** Audit condition returned false
- **Likely Cause:** Required data not present in ontology graph
- **Remediation:** Check that all upstream manufacturing steps completed successfully


---

## Classification

**Phase 8 Verdict:** `FAILED`

### Audit Passage Criteria

- **ALIVE:** ≥12 audits PASS, 0 audits FAIL
- **PARTIAL:** ≥8 audits PASS, ≤3 audits FAIL (failures documented with root cause)
- **FAILED:** <8 audits PASS OR undocumented failures

**Current Status:** 7 PASS, 5 FAIL, 2 SKIP → `FAILED`

---

## Next Steps

1. ✗ Critical conformance failures detected
2. Analyze failed audits
3. Implement remediation in targeted commits
4. Re-run Phase 8 audits

Checkpoint authorization blocked until failures resolved.

---

**Generated:** 2026-06-01T13:16:38.154828
**Tool:** rdflib via Python
**Query Directory:** /Users/sac/process-intelligence/research/pi-program/ggen/queries
**Ontology Directory:** /Users/sac/process-intelligence/research/pi-program/ggen/ontology
