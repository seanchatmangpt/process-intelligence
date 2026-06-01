# Process Intelligence Gap Ledger & Remediation Plan

**Generated:** 2026-06-01  
**Authority:** Process Intelligence Research Foundry  
**Program:** PI_RESEARCH_PROGRAM_ALIVE_001

---

## Files in This Directory

### 1. gap-ledger.yaml (528 lines)
**Purpose:** Comprehensive inventory of all 11 structural gaps

**Contents:**
- **Metadata:** Program authority, emission timestamp, gap counts
- **Gap Definitions (11 total):**
  - 4 CRITICAL gaps (GAP_001 to GAP_005) that block ALIVE reissue
  - 5 MAJOR gaps (GAP_006 to GAP_009) requiring remediation
  - 2 MINOR gaps (GAP_010 to GAP_011) requiring remediation
- **For Each Gap:**
  - gap_id, title, owner project, severity, blocks_alive status
  - affected_files list
  - expected_capability vs observed_capability (with evidence sources)
  - remediation_class, allowed/forbidden change types
  - audit_gate and expected_receipt
  - remediation_effort_hours
  - remediation_steps (ordered checklist)
  - prerequisite_gaps and dependent_gaps (dependency graph)
  - recommended_next_workflow
- **Dependency Chains:** Shows which gaps must close before others
- **Audit Paths:** Maps audit gates to gaps they verify

**Key Insight:** Every gap includes structured evidence of both expected and observed capability, plus specific remediation steps.

### 2. remediation-plan.md (899 lines)
**Purpose:** Actionable execution plan for closing all gaps

**Contents:**
- **Executive Summary:** Critical findings, constraints, ownership
- **5 Remediation Phases:**
  - Phase 1: IMMEDIATE (0-4 hours) — GAP_010 (quick win)
  - Phase 2: URGENT (4-12 hours) — GAP_004, GAP_005, GAP_008 (unblock ggen)
  - Phase 3: HIGH (12-24 hours) — GAP_001, GAP_002 (graduation + named refusals)
  - Phase 4: MAJOR (24-48 hours) — GAP_003 (behavioral gates)
  - Phase 5: EXPANSION (48+ hours) — GAP_006 through GAP_011 (algorithms)
- **For Each Gap in the Plan:**
  - Executive summary (what, why, how long)
  - Architecture diagrams (for complex gaps like GAP_001)
  - Detailed step-by-step remediation instructions
  - Code snippets (before/after)
  - Checklist (copy-pasteable task list)
  - Expected receipt format
- **Timeline & Sequencing:**
  - Serial dependencies (which gaps must finish before others)
  - Parallel execution strategy (which can run simultaneously)
  - Effort breakdown by phase (hours, days, FTE)
  - Gantt-style timeline visualization
- **Receipt Issuance Protocol:**
  - Standard receipt format (YAML)
  - Receipt emission checklist (6 items)
  - How to register receipts in receipt registry
- **Governance & Escalation:**
  - Decision authority table
  - Escalation criteria (when to alert Research Directorate)
  - Blocker report format
- **Appendices:**
  - A. File locations reference
  - B. Tools & commands (rapper for TTL, cargo for Rust)
  - C. Conformance testing (Python test harness example)

**Key Insight:** Every remediation step is executable and includes a checklist; no ambiguity about what to do.

---

## Quick Navigation

### By Severity

**CRITICAL (blocks ALIVE reissue):**
- GAP_001: wasm4pm-compat graduation boundary
- GAP_002: Named law refusals in wasm4pm
- GAP_003: Replace file-count gates with behavioral gates
- GAP_004: Fix TTL ontology parse violations
- GAP_005: Classify legacy ggen files

**MAJOR (high priority):**
- GAP_006: Inductive Miner implementation
- GAP_007: Alignment-based conformance
- GAP_008: Remove DTO flattening from compat
- GAP_009: OCPQ evaluator implementation

**MINOR (important but not blocking):**
- GAP_010: Remove forgeable WfNet attestation
- GAP_011: POWL discovery miner

### By Owner Project

**wasm4pm** (4 gaps):
- GAP_001: Add compat dependency + graduation impl
- GAP_002: Implement named law refusals
- GAP_006: Inductive Miner algorithm
- GAP_007: Alignment-based conformance
- GAP_009: OCPQ evaluator

**wasm4pm-compat** (3 gaps):
- GAP_002: Define refusal law types
- GAP_008: Remove *_json methods
- GAP_010: Deprecate forgeable attest_witnessed()

**process-intelligence** (2 gaps):
- GAP_003: Replace file-count gates with behavioral gates
- GAP_004: Fix TTL parse violations
- GAP_005: Classify ggen legacy files

### By Timeline

**Execute Day 1 (8 hours):**
- Phase 1: GAP_010 (1h) — Deprecate WfNet::attest_witnessed()
- Phase 2: GAP_004, GAP_005, GAP_008 (9h) — TTL, ggen legacy, DTO removal

**Execute Days 2-3 (14 hours):**
- Phase 3: GAP_001, GAP_002 (14h) — Graduation + named refusals (serial)

**Execute Days 3-4 (8 hours):**
- Phase 4: GAP_003 (8h) — Behavioral gates

**Execute Days 4-8 (44 hours, parallel):**
- Phase 5: GAP_006, 007, 009, 011 — Algorithms

---

## Remediation Checklist (Copy-Paste Ready)

### Phase 1: IMMEDIATE (1h)
```
- [ ] GAP_010: Mark WfNet::attest_witnessed() as #[deprecated]
  - [ ] Edit: ~/wasm4pm-compat/src/petri_net/wf_net.rs
  - [ ] Commit: fix(wasm4pm-compat): deprecate forgeable WfNet::attest_witnessed()
  - [ ] Verify: cargo check passes
  - [ ] Emit receipt: BLAKE3(GAP_010_remediated)
```

### Phase 2: URGENT (9h)
```
- [ ] GAP_004: Fix TTL parse violations (4h)
  - [ ] Run: rapper -c ggen/ontology-extensions.ttl
  - [ ] Fix: All 8 RDF/Turtle syntax errors
  - [ ] Verify: rapper -c with no errors
  - [ ] Commit: fix(ggen): correct TTL parse violations

- [ ] GAP_005: Classify legacy .ggen files (3h)
  - [ ] List all 12 .ggen files
  - [ ] Review each (active vs deprecated)
  - [ ] Update ggen/MANIFEST.md
  - [ ] Move deprecated to ggen/legacy/
  - [ ] Commit: fix(ggen): classify legacy .ggen files

- [ ] GAP_008: Remove DTO flattening (2h)
  - [ ] Remove: to_json_string() from mod.rs:735
  - [ ] Remove: receipt_json() from traits.rs:34
  - [ ] Move JSON logic to wasm4pm engine
  - [ ] Commit: fix(wasm4pm-compat): remove DTO flattening
  - [ ] Re-run: bash audits/audit-no-dto-flattening.sh (PASS)
```

### Phase 3: HIGH (14h, serial)
```
- [ ] GAP_001: Implement graduation boundary (8h)
  - [ ] Add wasm4pm-compat to Cargo.toml
  - [ ] Update 6 algorithm signatures to accept Evidence<_, Admitted, _>
  - [ ] Implement GraduateToWasm4pm trait
  - [ ] Add admission gates
  - [ ] Link receipts end-to-end
  - [ ] Commit: feat(wasm4pm): implement compat graduation boundary
  - [ ] Verify: cargo test passes (all 42 tests)

- [ ] GAP_002: Implement named law refusals (6h) [DEPENDS ON GAP_001]
  - [ ] Define named refusal types (or reuse from compat)
  - [ ] Replace Error::ValidationError(String) everywhere
  - [ ] Update all error construction sites
  - [ ] Commit: feat(wasm4pm): implement named law refusals
  - [ ] Verify: cargo test passes
```

### Phase 4: MAJOR (8h)
```
- [ ] GAP_003: Replace file-count gates with behavioral gates (8h)
  - [ ] Design 12 behavioral gates with execution tests
  - [ ] Implement test harness (Python/Rust)
  - [ ] Execute all gates and collect evidence
  - [ ] Update checkpoint to cite behavioral results
  - [ ] Issue PROCESS_INTELLIGENCE_ALIVE_002
  - [ ] Commit: checkpoint: PROCESS_INTELLIGENCE_ALIVE_002
```

### Phase 5: EXPANSION (44h, parallel)
```
- [ ] GAP_006: Implement Inductive Miner (12h)
  - [ ] wasm4pm/src/discovery/inductive.rs
  - [ ] Accept Evidence<EventLog, Admitted, W>
  - [ ] Return TypedProcessTree
  - [ ] Include soundness proof

- [ ] GAP_007: Implement Alignment Conformance (10h)
  - [ ] wasm4pm/src/conformance/alignment.rs
  - [ ] Accept Evidence<EventLog, Admitted, W>
  - [ ] Return Metric<FITNESS, N, D>
  - [ ] Calculate precision + fitness

- [ ] GAP_009: Implement OCPQ Evaluator (14h)
  - [ ] wasm4pm/src/query/ocpq.rs
  - [ ] evaluate(&query, &log) -> QueryResult
  - [ ] Support all OCPQ operators

- [ ] GAP_011: Implement POWL Miner (8h)
  - [ ] wasm4pm/src/discovery/powl.rs
  - [ ] Accept Evidence<EventLog, Admitted, W>
  - [ ] Return TypedPowl
```

---

## Key Decisions Already Made

1. **Fix Forward Only:** No destructive git operations. All fixes applied as new commits.

2. **Behavioral Gates Required:** File-count gates (e.g., "33 doctrine files") are forbidden for ALIVE. Must replace with execution evidence.

3. **Receipt Chain Mandatory:** Every gap close must emit BLAKE3 receipt linking remediation to original gap.

4. **Serial Dependencies Enforced:**
   - GAP_001 must close before GAP_002, 006, 007, 009, 011
   - GAP_004 must close before GAP_005
   - Phases 1-3 must close before downstream manufacturing

5. **No Downstream Manufacturing Until Phase 3 Complete:**
   - GGEN_ECOSYSTEM_MANUFACTURING_001 blocked
   - M&A_DECK_MANUFACTURING_PIPELINE_001 blocked
   - Board-level claims blocked

---

## Authority

**Issued By:** PI_RESEARCH_PROGRAM_ALIVE_001 Checkpoint  
**Authority:** Process Intelligence Research Foundry  
**Date:** 2026-06-01  
**Status:** ACTIVE (remediation plan in effect)

This ledger and plan supersede all prior gap documentation and provide the authoritative path to full ALIVE reissue and downstream manufacturing authorization.

---

## Next Steps

1. **Immediately:** Execute Phase 1 (GAP_010, 1 hour)
2. **Today:** Execute Phase 2 (GAP_004, 005, 008, 9 hours)
3. **Days 2-3:** Execute Phase 3 (GAP_001, 002, 14 hours — serial)
4. **Days 3-4:** Execute Phase 4 (GAP_003, 8 hours)
5. **Days 4-8:** Execute Phase 5 (GAP_006-011 parallel, 44 hours)

**Timeline:** 7-8 calendar days, 3-4 FTE, 56-64 total hours

