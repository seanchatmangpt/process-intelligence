# Remediation Plan — Phase 12 Execution

**Authority:** Process Intelligence Research Directorate  
**Source Verdict:** PI_GGEN_VALIDATOR_RECOVERY_FINAL_FAILED_001  
**Date Issued:** 2026-06-01  
**Total Effort:** 11 hours (8 hours critical path + 3 hours ALIVE threshold)  

---

## Overview

Phase 9-11 audit identified 6 blocking gaps preventing ALIVE verdict.
This remediation plan provides step-by-step instructions to close all gaps.

**Current State:** 5/15 gates PASS (FAILED)  
**Target State:** 15/15 gates PASS (ALIVE)  

---

## Critical Path (8 Hours)

Execute these 6 gaps in order. Each closes one or more gates.

### Phase 12-1: Fix ggen Tera Parser Bug (1 hour)

**Gap:** GAP_VALIDATOR_BUG_001_TERA_PARSER  
**Gates Unblocks:** 9, 10, 11  
**Effort:** 1 hour  

**Step 1: Check ggen version**

```bash
ggen --version
```

Expected output shows v26.5.21 or similar.

**Step 2: Try upgrade to v26.6+**

```bash
# If installed via cargo
cargo install ggen@26.6 --force

# Or via package manager
brew upgrade ggen
```

If v26.6+ available, test:
```bash
cd /Users/sac/process-intelligence/research/pi-program
ggen validate ggen.toml
```

**Step 3: If upgrade unavailable, apply workaround**

Use direct Tera CLI rendering instead of ggen validator:

```bash
# Install Tera CLI
cargo install tera-cli

# For each template, render directly
cd /Users/sac/process-intelligence/ggen/templates
for f in *.tera; do
  tera-cli render "$f" --strict
done
```

**Step 4: Update execution ledger**

Document in ggen-pipeline-execution-ledger.yaml:
- Method applied (upgrade or workaround)
- Status of each pipeline after fix
- Any new errors encountered

**Success Criteria:**
- At least 1 pipeline produces 1 artifact
- No template validator errors
- artifact count > 0 in ledger

---

### Phase 12-2: Install and Validate TTL Files (2 hours)

**Gap:** GAP_GGEN_001_TTL_SYNTAX_VALIDATION  
**Gate Unblocks:** 4  
**Effort:** 2 hours  

**Step 1: Install validation tools**

```bash
# macOS
brew install raptor2

# Verify installation
rapper --version
```

**Step 2: Validate all TTL files**

```bash
cd /Users/sac/process-intelligence
find . -name "*.ttl" -print0 | while IFS= read -r -d '' f; do
  echo "Validating: $f"
  rapper -c "$f" > /dev/null 2>&1
  if [ $? -eq 0 ]; then
    echo "  ✓ PASS"
  else
    echo "  ✗ FAIL"
    rapper -c "$f" 2>&1 | head -5
  fi
done > ttl-validation-report.txt 2>&1
```

**Step 3: Fix syntax errors**

For each failed file, examine the rapper error output:

```bash
rapper -c /path/to/file.ttl
```

Common fixes:
- Missing `@prefix` declarations → add them at file top
- Malformed RDF syntax → check Turtle specification
- Invalid URI format → fix URI escaping

**Step 4: Document results**

Create `/Users/sac/process-intelligence/research/pi-program/emitted/ggen-validator-recovery/ttl-validation-report.md`:

```markdown
# TTL Validation Report

**Date:** 2026-06-02
**Tool:** rapper (from raptor2)

## Summary
- Files validated: X
- Files passing: Y
- Files failing: Z

## Passing Files
[list]

## Failing Files
[list with error details]

## Fixes Applied
[document each fix]
```

**Success Criteria:**
- All 22 TTL files validate without errors
- Validation report generated
- Gate 4 → PASS

---

### Phase 12-3: Validate and Create Missing RQ Files (2 hours)

**Gap:** GAP_GGEN_002_RQ_QUERY_VALIDATION + GAP_QUERY_001_MISSING_QUERIES  
**Gate Unblocks:** 5  
**Effort:** 2 hours  

**Step 1: Validate existing RQ files**

```bash
cd /Users/sac/process-intelligence
find . -name "*.rq" -print0 | while IFS= read -r -d '' f; do
  echo "Validating: $f"
  rapper -q "$f" > /dev/null 2>&1
  if [ $? -eq 0 ]; then
    echo "  ✓ PASS"
  else
    echo "  ✗ FAIL"
    rapper -q "$f" 2>&1 | head -5
  fi
done > rq-validation-report.txt 2>&1
```

**Step 2: Identify missing queries**

```bash
cd /Users/sac/process-intelligence/research/prompt-manufactory/ggen
grep -r "query:" ggen.toml | awk -F'"' '{print $2}' | sort | uniq > required_queries.txt
find . -name "*.rq" -exec basename {} \; | sed 's/.rq$//' | sort | uniq > existing_queries.txt
comm -23 required_queries.txt existing_queries.txt > missing_queries.txt
```

This shows which 5 queries are missing.

**Step 3: Infer missing query requirements**

For each missing query in missing_queries.txt, examine ggen.toml to understand what data it should select:

```bash
grep -A10 -B2 "select-workflow-prompts" ggen.toml
```

Example inference for missing query `select-subagent-prompts.rq`:
- Rule expects: subagent data (id, name, contract, capabilities)
- Ontology has: SubAgent class with properties
- Query should: SELECT all SubAgent instances with relevant properties

**Step 4: Create missing SPARQL files**

For each missing file, create `/Users/sac/process-intelligence/research/prompt-manufactory/ggen/queries/MISSING_FILE.rq`:

```sparql
PREFIX pm: <https://pi-research.dev/ontology#>
PREFIX dct: <http://purl.org/dc/terms/>

SELECT ?subagentId ?name ?contract
WHERE {
  ?subagent a pm:SubAgent ;
    dct:identifier ?subagentId ;
    dct:title ?name ;
    pm:contractUri ?contract .
}
```

**Step 5: Test queries**

```bash
for f in /Users/sac/process-intelligence/research/prompt-manufactory/ggen/queries/*.rq; do
  echo "Testing: $(basename $f)"
  rapper -q "$f" > /dev/null 2>&1 && echo "  ✓ PASS" || echo "  ✗ FAIL"
done
```

**Step 6: Document**

Create `/Users/sac/process-intelligence/research/pi-program/emitted/ggen-validator-recovery/rq-validation-report.md`:

```markdown
# RQ/SPARQL Validation Report

**Date:** 2026-06-02
**Tool:** rapper

## Summary
- Files validated: 67
- Files passing: X
- Files failing: Y
- Missing files created: 5

## Created Files
- select-subagent-prompts.rq
- select-skill-prompts.rq
- select-hook-policies.rq
- select-checkpoint-prompts.rq
- select-legacy-ggen-files.rq

## Validation Results
[detailed results]
```

**Success Criteria:**
- All 67 RQ files validate
- 5 missing files created and validating
- All queries executable against ontology
- Gate 5 → PASS

---

### Phase 12-4: Create ggen/MANIFEST.md (1 hour)

**Gap:** GAP_GGEN_003_LEGACY_CLASSIFICATION  
**Gate Unblocks:** 6  
**Effort:** 1 hour  

**Step 1: List all .ggen files**

```bash
cd /Users/sac/process-intelligence
find . -name "*.ggen" | sort
```

This should find all 25 .ggen files.

**Step 2: Classify each file**

For each file, determine its status:
- **ACTIVE:** Currently used by generation rules, maintained
- **DEPRECATED:** Superseded but kept for reference
- **LEGACY:** Old, not used, kept for audit trail
- **MOVED:** Relocated to another path

**Step 3: Create ggen/MANIFEST.md**

```markdown
# GGEN File Manifest

**Date:** 2026-06-02
**Authority:** Process Intelligence Research Directorate
**Total Files:** 25

## File Inventory

| File | Status | Created | Purpose | Last Modified | Notes |
|------|--------|---------|---------|---------------|-------|
| ggen/templates/visualizer.tera | ACTIVE | 2026-05-15 | Dashboard visualization | 2026-06-01 | Used by rule ggen-001 |
| ggen/templates/blue-river.tera | ACTIVE | 2026-05-01 | Lifecycle state template | 2026-06-01 | Used by rule ggen-002 |
| ggen/queries/extract-lifecycle.rq | ACTIVE | 2026-05-15 | Lifecycle extraction | 2026-06-01 | Used by rule ggen-001 |
| ... (22 more files) | | | | | |

## Status Summary
- ACTIVE: X files
- DEPRECATED: Y files
- LEGACY: Z files
- MOVED: W files

## Classification Rules
- ACTIVE: used in active generation rules
- DEPRECATED: superseded by newer versions
- LEGACY: retained for audit trail only
- MOVED: relocated, reference provided
```

**Step 4: Validate manifest**

```bash
# Count files in manifest
grep "^| " ggen/MANIFEST.md | wc -l
# Should equal 25 + header line

# Verify all .ggen files mentioned
find . -name "*.ggen" | while read f; do
  basename "$f"
done | while read fname; do
  if grep -q "$fname" ggen/MANIFEST.md; then
    echo "✓ $fname documented"
  else
    echo "✗ $fname missing from manifest"
  fi
done
```

**Success Criteria:**
- All 25 .ggen files documented in MANIFEST.md
- Each file has status (ACTIVE/DEPRECATED/LEGACY/MOVED)
- Each file has purpose and metadata
- Gate 6 → PASS

---

### Phase 12-5: Audit and Fix Parent Checkpoint (1 hour)

**Gap:** GAP_GGEN_004_FILE_COUNT_GATE  
**Gate Unblocks:** 8, 13  
**Effort:** 1 hour  

**Step 1: Identify problematic checkpoint**

```bash
grep -l "file count\|files generated\|therefore alive" \
  /Users/sac/process-intelligence/checkpoints/*.md
```

This should identify ALIVE_GATE_ASSESSMENT.md (or similar).

**Step 2: Read the checkpoint**

```bash
cat /Users/sac/process-intelligence/checkpoints/ALIVE_GATE_ASSESSMENT.md | head -100
```

Look for patterns:
- "X files generated" → forbidden
- "file count" → forbidden
- "therefore ALIVE" → requires gate evidence
- Explicit gate enumeration → required

**Step 3: Extract gate evidence**

If the checkpoint has legitimate gate evidence mixed with file-count language:

```bash
# Extract gate mentions
grep -i "gate\|pass\|fail" /path/to/checkpoint.md
```

Document which gates actually passed/failed.

**Step 4: Rewrite checkpoint**

Create corrected version with:
- ✗ Remove: "X files generated" language
- ✓ Add: "Gate 1: PASS", "Gate 2: FAIL", etc.
- ✓ Add: "Reason for ALIVE verdict: All 15 gates PASS"

Example corrected section:
```markdown
## Gate Assessment

| Gate | Name | Status | Evidence |
|------|------|--------|----------|
| 1 | Validator Fixture Tests PASS | PASS | 24 templates validated |
| 2 | Validator Fixture Tests FAIL | PASS | Invalid tests documented |
| ... | | | |

## Verdict Justification

ALIVE issued because: All 15 gates PASS (see table above).

NOT based on:
- ✗ File count metrics
- ✗ Commit frequency
- ✓ Gate evidence only
```

**Step 5: Update checkpoint file**

Replace or create new version. If replacing, ensure you preserve git history:

```bash
cd /Users/sac/process-intelligence
git add checkpoints/ALIVE_GATE_ASSESSMENT.md
git commit -m "fix(checkpoint): remove file-count language, add gate evidence"
```

**Success Criteria:**
- File-count language removed
- Gate evidence clearly documented
- Checkpoint language complies with doctrine
- Gate 8 → PASS
- Gate 13 → PASS

---

### Phase 12-6: Document Invalid Template Test Cases (1 hour)

**Gap:** Warrant Path Proof Documentation  
**Gate Unblocks:** 2  
**Effort:** 1 hour  

**Step 1: Create invalid template fixtures**

Identify templates with known-invalid Tera syntax:

```bash
# Example invalid template: template syntax inside JSX object literal
# File: ggen/templates/example-invalid.tera
# Content: {{ { nested: "{{ not valid }}" } }}  # Invalid: double nesting
```

**Step 2: Test validation**

```bash
tera-cli render /path/to/invalid.tera --strict 2>&1 | tee invalid_test_result.txt
```

Should produce error like:
```
Error: Unexpected token in template
...
```

**Step 3: Document in warrant-path-proof.md**

Add section:

```markdown
## Step X: Invalid Template Validation

### Test Case 1: Nested Tera Syntax

**Template:** `ggen/templates/example-invalid.tera`

**Syntax:**
```tera
{{ { nested: "{{ invalid }}" } }}
```

**Validation Result:**
```
Error: Unexpected token
...
```

**Conclusion:** Invalid templates correctly rejected by validator.

### Test Case 2: [Additional invalid templates...]

[Continue for each invalid template]

## Summary
- Valid templates: ✓ PASS
- Invalid templates: ✓ FAIL (as expected)
```

**Success Criteria:**
- At least 3 invalid template test cases documented
- Each shows the invalid syntax
- Each shows validation rejected it
- Gate 2 → PASS

---

## Post-Critical-Path (3 Hours for ALIVE)

After completing critical path (gates 2-6 PASS), you'll have:
- Gates passing: 5 + 6 = 11
- Gates failing: 4
- Verdict: PARTIAL (not yet ALIVE)

To reach ALIVE, fix remaining 4 gates:

### Remaining Gate Failures

**Gate 1 (Already PASS):** Validator Fixture Tests PASS  
**Gate 7 (Already PASS):** No Hand-Written Warrant Claimed  
**Gate 14 (Already PASS):** No Commit-Count ALIVE  
**Gate 15 (Already PASS):** Open Ontologies Status Classified  

**Still Failing (4 gates):**
- Gate 9: Pipeline execution (needs Phase 12-1 fix)
- Gate 10: Warrant path execution (needs Phase 12-1 + test)
- Gate 11: Receipts emitted (needs artifact generation)
- Gate 12: No forced ALIVE (checkpoint audit)

**Effort to fix remaining 4:** ~2-3 hours
- Run pipelines with fixed validator (30 min)
- Generate artifacts and receipts (30 min)
- Verify all 15 gates PASS (30 min)
- Issue final ALIVE checkpoint (30 min)

---

## Execution Timeline

| Phase | Task | Hours | Completion |
|-------|------|-------|------------|
| 12-1 | Fix ggen Tera parser bug | 1 | 2026-06-02 10:00 |
| 12-2 | Validate TTL files | 2 | 2026-06-02 12:00 |
| 12-3 | Validate/create RQ files | 2 | 2026-06-02 14:00 |
| 12-4 | Create ggen/MANIFEST.md | 1 | 2026-06-02 15:00 |
| 12-5 | Fix parent checkpoint | 1 | 2026-06-02 16:00 |
| 12-6 | Document invalid templates | 1 | 2026-06-02 17:00 |
| **Total Critical Path** | | **8** | |
| 12-7 | Run corrected pipelines | 1 | 2026-06-02 18:00 |
| 12-8 | Final gate audit & checkpoint | 1 | 2026-06-02 19:00 |
| **Total with ALIVE** | | **10** | |

---

## Success Criteria for Each Phase

**Phase 12-1:** ✓ ggen validates with no Tera errors; 1+ artifacts generated  
**Phase 12-2:** ✓ All 22 TTL files validate without errors  
**Phase 12-3:** ✓ All 67 RQ files validate; 5 missing files created  
**Phase 12-4:** ✓ ggen/MANIFEST.md documents all 25 .ggen files  
**Phase 12-5:** ✓ Parent checkpoint revised to remove file-count language  
**Phase 12-6:** ✓ warrant-path-proof.md documents invalid template test cases  
**Phase 12-7:** ✓ All 3 pipelines execute; receipts emitted  
**Phase 12-8:** ✓ All 15 gates PASS; ALIVE checkpoint issued  

---

## Rollback Plan (If Needed)

If any phase fails:

1. Document the failure in gap-ledger-final.yaml
2. Create issue or sub-task
3. Escalate to Phase 13 (extended remediation)
4. Issue PARTIAL_003 checkpoint with updated gaps

---

## Andon Guard Compliance

This remediation plan:
- ✓ Honest assessment of blockers
- ✓ Specific, step-by-step instructions
- ✓ Clear success criteria
- ✓ No arbitrary percentages or padding
- ✓ Binary gate verification
- ✓ Traceable evidence collection

---

## Next Checkpoint

After completing Phase 12:

**If all phases successful:**
```
PI_GGEN_VALIDATOR_RECOVERY_ALIVE_001
  - All 15 gates PASS
  - All 6 blocking gaps CLOSED
  - Manufacturing pipelines active
  - Prompt Manufactory warrant path proven
  - Receipts emitted
```

**If partial success:**
```
PI_GGEN_VALIDATOR_RECOVERY_PARTIAL_002
  - 8-14 gates PASS
  - Documented gaps with remediation plan
  - Clear path to ALIVE identified
```

---

**Plan Created:** 2026-06-01T13:39:32  
**Authority:** Process Intelligence Research Directorate  
**Ready for Execution:** Yes

