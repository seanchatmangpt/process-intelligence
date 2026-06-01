# Remediation Plan — PI GGEN Conformance Audit 2026-06-01

**Plan ID:** PI_GGEN_REMEDIATION_PLAN_001  
**Date:** 2026-06-01  
**Audit Run:** PI_GGEN_CONFORMANCE_AUDIT_2026_06_01  
**Total Gaps:** 6  
**Total Violations:** 101  
**Estimated Effort:** 9 hours (single FTE, 1.1 days)

---

## Phase 1: Tool Installation & Environment Setup (2 hours)

### Objective
Install RDF/SPARQL validation tooling required to validate TTL and RQ files.

### Steps

#### 1.1 Install librdf/rapper
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y librdf-utils

# macOS
brew install raptor2

# Verify installation
rapper --version
```

**Expected:** rapper installed and available on PATH

**Time:** 20 minutes

#### 1.2 Validate TTL Files
```bash
# Find all TTL files
find /Users/sac/process-intelligence -name "*.ttl" -type f

# Validate each TTL file
for ttl in $(find /Users/sac/process-intelligence -name "*.ttl"); do
  echo "Validating: $ttl"
  rapper -c "$ttl" 2>&1 | head -5
done
```

**Expected Output:** List of TTL files with parse status (PASS/FAIL)

**Time:** 30 minutes

#### 1.3 Document TTL Failures
```bash
# Create TTL validation report
cat > /Users/sac/process-intelligence/research/pi-program/emitted/ggen-unified-run/ttl-validation-report.txt << 'EOF'
# TTL Validation Report — PI GGEN Audit 2026-06-01
# Generated: 2026-06-01
# Tool: rapper (librdf-utils)

EOF

# For each failed TTL, append error details
```

**Time:** 30 minutes

### Gaps Closed
- GAP_GGEN_001_TTL_SYNTAX_VALIDATION (gate_03_ttl_graphs_parse)

### Blockers Resolved
None (establishes baseline for Phase 2)

### Gate Status After Phase 1
- Gate 3: FAIL → PENDING (depends on fix of failures)

---

## Phase 2: Query & Template Validation (2 hours)

### Objective
Validate SPARQL RQ query files using rapper, identify syntax errors.

### Steps

#### 2.1 Validate RQ Files
```bash
# Find all RQ files
find /Users/sac/process-intelligence -name "*.rq" -type f | wc -l
# Expected: 61 files

# Validate each RQ file
for rq in $(find /Users/sac/process-intelligence -name "*.rq"); do
  echo "Validating: $rq"
  rapper -q "$rq" 2>&1 | head -5
done
```

**Expected Output:** List of RQ files with SPARQL parse status

**Time:** 45 minutes

#### 2.2 Document RQ Failures
Create RQ validation report listing parse errors by pipeline (ggen-001, ggen-002, ggen-003).

**Time:** 30 minutes

#### 2.3 Fix ggen-003 Missing Queries
If ggen-003 is expected to have 7 queries but only 2 are present, investigate:
- Are the 5 missing queries documented in requirements?
- Should they be created or are they optional?

**Time:** 15 minutes

### Gaps Closed
- GAP_GGEN_002_RQ_QUERY_VALIDATION (gate_04_rq_queries_parse)

### Gate Status After Phase 2
- Gate 4: FAIL → PENDING (depends on fix of failures)

---

## Phase 3: Legacy .ggen Classification (1 hour)

### Objective
Create formal MANIFEST.md classifying all 13 legacy .ggen files.

### Steps

#### 3.1 Inventory Legacy .ggen Files
```bash
find /Users/sac/process-intelligence/ggen -name "*.ggen" -type f | sort
```

**Expected output:** List of 13 files

**Time:** 10 minutes

#### 3.2 Create ggen/MANIFEST.md
```markdown
# GGEN Manifest — Process Intelligence Legacy Files

**Date:** 2026-06-01  
**Authority:** Process Intelligence GGEN Repository

## Legacy .ggen Files Classification

| File | Type | Status | Created | LastModified | Purpose |
|------|------|--------|---------|---------------|---------|
| audit-component-boundary.sh.ggen | Audit Rule | ACTIVE | 2026-01-15 | 2026-01-20 | Validate component boundary types |
| audit-feature-law.sh.ggen | Audit Rule | ACTIVE | 2026-01-15 | 2026-02-01 | Enforce feature law compliance |
| ... (11 more) | | | | | |
```

**Time:** 40 minutes

#### 3.3 Commit ggen/MANIFEST.md
```bash
git add ggen/MANIFEST.md
git commit -m "docs-law(ggen): classify legacy .ggen files in MANIFEST"
```

**Time:** 5 minutes

### Gaps Closed
- GAP_GGEN_003_LEGACY_CLASSIFICATION (gate_09_legacy_ggen_classified)

### Gate Status After Phase 3
- Gate 9: FAIL → PASS

---

## Phase 4: Doctrine Warrant Audit (2 hours)

### Objective
Audit doctrine files for unsupported claims; move to gaps/ or provide evidence.

### Steps

#### 4.1 Identify Unsupported Claims
```bash
# Search for suspicious language patterns in doctrine files
grep -r "assume\|we believe\|should be\|infer.*without\|likely that" \
  /Users/sac/process-intelligence/doctrine/ \
  --include="*.md" 2>/dev/null
```

**Time:** 20 minutes

#### 4.2 Evidence Review
For each unsupported claim found:
1. Identify the claim
2. Search for evidence:
   - Paper citations (sources/papers/)
   - Experiment results (experiments/)
   - Prior checkpoint (checkpoints/)
3. If no evidence: create gap document
4. If evidence found: update doctrine with citation

**Time:** 60 minutes

#### 4.3 Remediation
- Remove unsupported warrants from doctrine files
- Move claims to gaps/ with owner and timeline
- Add citations to evidenced claims

**Time:** 40 minutes

### Gaps Closed
- GAP_GGEN_005_HAND_WRITTEN_WARRANTS (gate_12_no_hand_written_warrant)

### Gate Status After Phase 4
- Gate 12: FAIL → PASS

---

## Phase 5: Parent Checkpoint Audit (2 hours)

### Objective
Audit parent checkpoints (PI_RESEARCH_PROGRAM_ALIVE_001) to verify no file-count or commit-count gates used.

### Steps

#### 5.1 Audit File-Count Gates
```bash
# Search for file-count language in parent checkpoint
grep -n "file.*count\|files\|documents\|therefore" \
  /Users/sac/process-intelligence/checkpoints/PI_RESEARCH_PROGRAM_ALIVE_001.md \
  2>/dev/null | head -20
```

**Time:** 20 minutes

#### 5.2 Audit Commit-Count Gates
```bash
# Search for commit-count language
grep -n "commit.*count\|pull request\|frequency\|velocity" \
  /Users/sac/process-intelligence/checkpoints/PI_RESEARCH_PROGRAM_ALIVE_001.md \
  2>/dev/null | head -20
```

**Time:** 20 minutes

#### 5.3 Decision Tree
- **If NO file-count or commit-count gates found:** Gates 10 & 14 PASS
- **If violations found:** 
  1. Document findings
  2. Revert parent checkpoint (PI_RESEARCH_PROGRAM_ALIVE_001) from ALIVE to PARTIAL
  3. Reissue with corrective evidence
  4. Escalate to program authority

**Time:** 60 minutes

### Gaps Closed
- GAP_GGEN_004_FILE_COUNT_GATE (gate_10_no_file_count_alive)
- GAP_GGEN_006_COMMIT_COUNT_GATE (gate_14_commit_count_gaming_rejected)

### Gate Status After Phase 5
- Gate 10: FAIL → PENDING (depends on audit outcome)
- Gate 14: FAIL → PENDING (depends on audit outcome)

---

## Execution Timeline

### Day 1 (Today, 2026-06-01)

| Time | Phase | Task | Duration | Owner |
|------|-------|------|----------|-------|
| 14:00 | 1 | Install librdf/rapper | 20 min | PI |
| 14:20 | 1 | Validate TTL files | 30 min | PI |
| 14:50 | 1 | Document TTL failures | 30 min | PI |
| 15:20 | 2 | Validate RQ files | 45 min | PI |
| 16:05 | 2 | Document RQ failures | 30 min | PI |
| 16:35 | 2 | Investigate missing ggen-003 queries | 15 min | PI |
| 16:50 | 3 | Inventory legacy .ggen files | 10 min | PI |
| 17:00 | 3 | Create ggen/MANIFEST.md | 40 min | PI |
| 17:40 | 3 | Commit MANIFEST | 5 min | PI |

**Daily Effort:** 5.5 hours (stops at 17:45)

### Day 2 (2026-06-02)

| Time | Phase | Task | Duration | Owner |
|------|-------|------|----------|-------|
| 09:00 | 4 | Identify unsupported claims | 20 min | PI |
| 09:20 | 4 | Evidence review | 60 min | PI |
| 10:20 | 4 | Remediation | 40 min | PI |
| 11:00 | 5 | Audit file-count gates | 20 min | PI |
| 11:20 | 5 | Audit commit-count gates | 20 min | PI |
| 11:40 | 5 | Decision & escalation | 60 min | PI |

**Daily Effort:** 3.5 hours

**Total Effort:** 9 hours (1 FTE, 1.1 days)

---

## Success Criteria

### Phase 1 Success
- [ ] rapper installed and verified
- [ ] 23 TTL files validated (pass/fail documented)
- [ ] TTL validation report generated

### Phase 2 Success
- [ ] 61 RQ files validated (pass/fail documented)
- [ ] RQ validation report generated
- [ ] Missing ggen-003 queries investigated

### Phase 3 Success
- [ ] ggen/MANIFEST.md created with 13 legacy files classified
- [ ] Status assigned to each: ACTIVE | DEPRECATED | LEGACY
- [ ] Commit merged (docs-law: classify legacy .ggen files)

### Phase 4 Success
- [ ] All unsupported warrants identified
- [ ] Evidence collected for each claim
- [ ] Doctrine files updated or gaps created
- [ ] 2 violations resolved

### Phase 5 Success
- [ ] File-count gates audited (result: PASS or escalation)
- [ ] Commit-count gates audited (result: PASS or escalation)
- [ ] Parent checkpoint verified or flagged

---

## Expected Gate Status After Remediation

| Gate | Current | After Phase | Expected Final |
|------|---------|-------------|-----------------|
| Gate 1 | PASS | — | PASS |
| Gate 2 | PASS | — | PASS |
| Gate 3 | FAIL | 1 | PENDING (fix syntax) |
| Gate 4 | FAIL | 2 | PENDING (fix syntax) |
| Gate 5 | PASS | — | PASS |
| Gate 6 | PASS | — | PASS |
| Gate 7 | PASS | — | PASS |
| Gate 8 | PASS | — | PASS |
| Gate 9 | FAIL | 3 | PASS |
| Gate 10 | FAIL | 5 | PASS (if clean) or ESCALATE |
| Gate 11 | PASS | — | PASS |
| Gate 12 | FAIL | 4 | PASS |
| Gate 13 | PASS | — | PASS |
| Gate 14 | FAIL | 5 | PASS (if clean) or ESCALATE |
| Gate 15 | PASS | — | PASS |

---

## Risk Assessment

### Low Risk
- Phase 1 (Tool installation): Standard package installation
- Phase 3 (MANIFEST): Simple documentation task
- Phase 2 (Validation): Syntax checking (no code changes)

### Medium Risk
- Phase 4 (Doctrine audit): May require evidence collection or gap creation
- Phase 5 (Parent checkpoint audit): May require escalation if violations found

### Critical Path
1. Phase 1: Tool installation (blocks Phases 2)
2. Phases 2-4: Can run in parallel after Phase 1
3. Phase 5: Parallel with Phases 2-4, may require escalation

### Escalation Scenario
If parent checkpoint (PI_RESEARCH_PROGRAM_ALIVE_001) contains file-count or commit-count gates:
1. Revert parent checkpoint to PARTIAL
2. Document violations
3. Notify program authority
4. Reissue parent checkpoint with corrective evidence
5. Reissue GGEN audit checkpoint after parent is corrected

---

## Post-Remediation Audit

After all phases complete, re-run the 15-gate conformance audit:

```bash
# Re-run audit
bash /tmp/conformance-audit-full.sh

# Expected result: 9 gates PASS + 6 gates PASS (if remediation successful) = 15/15 PASS
# Or escalation flag if parent checkpoint violations found
```

---

## Next Workflow After ALIVE

Once all 15 gates PASS and ALIVE verdict issued, execute:

1. **GGEN_ECOSYSTEM_MANUFACTURING_001** (FFI projections for wasm4pm-compat)
2. **M&A_DECK_MANUFACTURING_PIPELINE_001** (Board artifacts)
3. **BLUE_RIVER_AUTONOMIC_LOOP_MANUFACTURING_001** (Governance orchestrator)

See final checkpoint document for detailed workflow definitions.

---

## Owner Assignments

| Phase | Responsible | Authority |
|-------|-------------|-----------|
| Phase 1 | Process Intelligence | PI Research Directorate |
| Phase 2 | Process Intelligence | PI Research Directorate |
| Phase 3 | Process Intelligence | PI Research Directorate |
| Phase 4 | Process Intelligence | PI Research Directorate |
| Phase 5 | Process Intelligence | PI Research Directorate (escalation if needed) |

---

## Document Authority

**Remediation Plan:** PI_GGEN_REMEDIATION_PLAN_001  
**Issued:** 2026-06-01 20:30 UTC  
**Authority:** Process Intelligence GGEN Research Program  
**Base Audit:** PI_GGEN_CONFORMANCE_AUDIT_2026_06_01

