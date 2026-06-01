# Process Intelligence Gap Remediation Plan

**Date Issued:** 2026-06-01  
**Authority:** Process Intelligence Research Foundry  
**Program Authority:** PI_RESEARCH_PROGRAM_ALIVE_001  
**Repository:** /Users/sac/process-intelligence  
**Total Gaps:** 11 (4 CRITICAL, 5 MAJOR, 2 MINOR)  
**ALIVE Blockers:** 5 gaps must close before ALIVE reissue  
**Total Remediation Effort:** 56-64 hours across all projects

---

## Executive Summary

The Process Intelligence Research Foundry has issued **PROCESS_INTELLIGENCE_ALIVE_001** (2026-06-01) based on 12 audit gates. Subsequent conformance audits have identified **11 structural gaps** that must be remediated to sustain ALIVE status and support downstream manufacturing.

### Critical Findings

1. **5 gaps block ALIVE reissue** (GAP_001, GAP_002, GAP_003, GAP_004, GAP_005)
2. **6 gaps are major/minor but require remediation** for full capability (GAP_006 through GAP_011)
3. **Two projects have critical blockers:** wasm4pm-compat (graduation boundary not implemented) and wasm4pm (named refusals missing)
4. **Behavioral gate replacement needed:** Current ALIVE verdict uses file-count gates (forbidden by Covenant)

### Constraints

- **Fix Forward Only:** No `git reset --hard` or destructive operations. All fixes applied via targeted commits.
- **No Partial Authorization:** No downstream manufacturing until all CRITICAL gaps remediated.
- **Receipt Chain Required:** Every gap close must emit cryptographic receipt (BLAKE3) linking remediation to original gap.

---

## Remediation Workflow Phases

### Phase 1: IMMEDIATE Fixes (0-4 hours)
**Scope:** Quick wins that unblock later phases  
**Parallelizable:** All fixes in this phase are independent

#### GAP_010: Remove Forgeable WfNet Attestation
**Owner:** wasm4pm-compat team  
**Time:** 1 hour  
**Change Type:** SOURCE_SURFACE_RENAME

**What to do:**
```rust
// BEFORE: ~/wasm4pm-compat/src/petri_net/wf_net.rs
impl WfNet {
    pub fn attest_witnessed(self) -> WfNetConst<SOUNDNESS> { /* forgeable */ }
}

// AFTER: Mark as deprecated or hide
impl WfNet {
    #[deprecated(since = "0.2.0", note = "Use WfNetConst<SOUNDNESS> instead")]
    pub fn attest_witnessed(self) -> WfNetConst<SOUNDNESS> { /* deprecated */ }
}
```

**Checklist:**
- [ ] Add `#[deprecated]` annotation to `WfNet::attest_witnessed()`
- [ ] Update function documentation to reference `WfNetConst<SOUNDNESS>`
- [ ] Update all tests to use `WfNetConst` instead
- [ ] Commit with message: `fix(wasm4pm-compat): deprecate forgeable WfNet::attest_witnessed()`
- [ ] Verify: `cargo check` passes in wasm4pm-compat
- [ ] Emit receipt: `BLAKE3(GAP_010_remediated)`

**Expected Receipt:**
```
BLAKE3: [hash of commit containing deprecation]
Date: 2026-06-01
Authority: wasm4pm-compat maintainer
```

---

### Phase 2: URGENT Fixes (4-12 hours)
**Scope:** Unblock ggen manufacturing and compat boundary  
**Parallelizable:** All fixes in this phase are independent

#### GAP_004: Fix TTL Ontology Parse Violations
**Owner:** process-intelligence/ggen team  
**Time:** 4 hours  
**Change Type:** QUERY_MISSING

**What to do:**
1. Locate and parse `ggen/ontology-extensions.ttl`
2. Enumerate all 8 parse violations
3. Fix each violation (syntax, prefix, triple format)
4. Validate with Apache Jena or Turtle parser

**Checklist:**
- [ ] Run `rapper -c ggen/ontology-extensions.ttl` to identify violations
- [ ] Document each violation (line number, error message)
- [ ] Fix violation 1 (likely missing prefix or invalid IRI)
- [ ] Fix violations 2-8
- [ ] Verify: `rapper -c` with no errors
- [ ] Commit with message: `fix(ggen): correct TTL parse violations in ontology-extensions`
- [ ] Emit receipt: `BLAKE3(GAP_004_remediated)`

**Expected Violations (sample):**
- Missing @prefix declarations
- Invalid IRI syntax (< > or special chars)
- Incomplete triple (missing . terminator)
- Undefined namespace prefixes

**Expected Receipt:**
```
BLAKE3: [hash of commit fixing TTL]
Date: 2026-06-01
Authority: ggen ontology maintainer
```

#### GAP_005: Classify Legacy ggen Files
**Owner:** process-intelligence/ggen team  
**Time:** 3 hours  
**Change Type:** LEGACY_EXTENSION_CLASSIFICATION

**What to do:**
1. Enumerate 12 .ggen files (8 audits + 4 templates)
2. Review each for purpose (active vs deprecated)
3. Classify in ggen/MANIFEST.md or move to ggen/legacy/
4. Document deprecation dates and reasons

**Checklist:**
- [ ] List all `ggen/audits/audit-*.sh.ggen` files (8)
- [ ] List all `ggen/templates/*.ggen` files (4)
- [ ] Review each for purpose
- [ ] Classify as active/deprecated/deleted
- [ ] Update ggen/MANIFEST.md with classification
- [ ] Move deprecated to ggen/legacy/ with reason
- [ ] Commit with message: `fix(ggen): classify legacy .ggen files`
- [ ] Emit receipt: `BLAKE3(GAP_005_remediated)`

**Manifest Entry Format:**
```yaml
legacy_files:
  - path: "ggen/legacy/audit-old-feature.sh.ggen"
    status: "deprecated"
    deprecation_date: "2026-06-01"
    reason: "Replaced by newer audit-*.sh in ggen/audits/"
    notes: "Kept for reference; do not use in manufacturing"
```

**Expected Receipt:**
```
BLAKE3: [hash of MANIFEST update + legacy/ reorganization]
Date: 2026-06-01
Authority: ggen maintainer
```

#### GAP_008: Remove DTO Flattening from wasm4pm-compat
**Owner:** wasm4pm-compat team  
**Time:** 2 hours  
**Change Type:** SOURCE_SURFACE_RENAME

**What to do:**
1. Remove `to_json_string()` from `compat/src/manufacturing/mod.rs:735`
2. Remove `receipt_json()` from `compat/src/manufacturing/traits.rs:34`
3. Move JSON serialization logic to wasm4pm engine
4. Update any API consumers

**Checklist:**
- [ ] Locate `to_json_string()` in mod.rs line 735
- [ ] Locate `receipt_json()` in traits.rs line 34
- [ ] Identify all callers (grep -r "to_json_string\|receipt_json")
- [ ] Move JSON serialization logic to wasm4pm/src/manufacturing/
- [ ] Remove method definitions from compat public API
- [ ] Update calling code to use wasm4pm serialization
- [ ] Commit with message: `fix(wasm4pm-compat): remove DTO flattening from public API`
- [ ] Re-run audit_1: `bash audits/audit-no-dto-flattening.sh`
- [ ] Emit receipt: `BLAKE3(GAP_008_remediated)`

**Forbidden Changes:**
- Do NOT keep *_json methods in compat
- Do NOT move *_json methods to a feature gate
- Do NOT rename to disguise the pattern

**Expected Receipt:**
```
BLAKE3: [hash of DTO removal commit]
Date: 2026-06-01
Authority: wasm4pm-compat maintainer
Audit Result: audit_1 PASS (no DTO flattening violations)
```

---

### Phase 3: HIGH Priority (12-24 hours)
**Scope:** Close CRITICAL gaps required for ALIVE reissue  
**Parallelizable:** Gaps must close in order (GAP_001 before GAP_002)

#### GAP_001: Implement wasm4pm-compat Graduation Boundary
**Owner:** wasm4pm team  
**Time:** 8 hours  
**Prerequisite:** GAP_010, GAP_004, GAP_005, GAP_008 (all PASS)  
**Change Type:** SOURCE_SURFACE_RENAME

**Architecture:**
```
wasm4pm-compat                          wasm4pm
─────────────────────────────────       ──────────────────────
Evidence<T, Raw, W>                     (not accepted)
    ↓ admission gate
Evidence<T, Admitted, W>  ─→ GraduateToWasm4pm trait ─→  algorithm_input
    ↓                                                         ↓
WfNetReceipt<Ocel20>     ─link─────────────────────→  algorithm_receipt
```

**What to do:**

1. **Add wasm4pm-compat dependency to wasm4pm/Cargo.toml**
   ```toml
   [dependencies]
   wasm4pm-compat = { workspace = true }  # ADD THIS
   wasm4pm-types = { workspace = true }
   wasm4pm-algos = { workspace = true }
   ```

2. **Update algorithm function signatures** (all 6 algorithm entry points)
   ```rust
   // BEFORE: ~/wasm4pm/src/discovery/ilp.rs
   pub fn discover_ilp_petri_net_from_log(
       log: &EventLog,
       activity_key: &str
   ) -> (PetriNet, f64, f64)
   
   // AFTER:
   use wasm4pm_compat::Evidence;
   pub fn discover_ilp_petri_net_from_log(
       log: Evidence<EventLog, Admitted, W>,  // Admitted evidence required
       activity_key: &str
   ) -> Result<(PetriNet, f64, f64), Refusal<DiscoveryFailure, W>>
   ```

3. **Implement GraduateToWasm4pm trait**
   ```rust
   // ~/wasm4pm/src/graduation.rs
   use wasm4pm_compat::GraduateToWasm4pm;
   
   impl GraduateToWasm4pm for DiscoveryResult {
       fn graduate(&self) -> WfNetReceipt { /* ... */ }
   }
   ```

4. **Add admission gate at algorithm entry**
   ```rust
   // Verify evidence is actually admitted before execution
   if log.state != LifecycleState::Admitted {
       return Err(Refusal::InvalidState(NotAdmitted));
   }
   ```

5. **Link receipts**
   - Input receipt (from compat admission)
   - Output receipt (algorithm execution)
   - Link output → input via receipt_parent field

**Checklist:**
- [ ] Add wasm4pm-compat to Cargo.toml
- [ ] Update function signatures for all 6 algorithms
- [ ] Implement GraduateToWasm4pm trait
- [ ] Add admission gate in algorithm entry points
- [ ] Update error handling to use compat Refusal types
- [ ] Write tests using admitted evidence as input
- [ ] Commit with message: `feat(wasm4pm): implement compat graduation boundary`
- [ ] Verify: `cargo test` passes in wasm4pm
- [ ] Emit receipt: `BLAKE3(GAP_001_remediated)`

**Affected Algorithm Functions:**
1. `discover_ilp_petri_net_from_log()`
2. `discover_simple_process_tree_from_log()`
3. `discover_performance_dfg_from_log()`
4. `replay_with_token_game()`
5. `conformance_check_fitness()`
6. (and any others that accept EventLog)

**Expected Receipt:**
```
BLAKE3: [hash of graduation boundary implementation commit]
Date: 2026-06-01
Authority: wasm4pm maintainer
Dependencies: wasm4pm-compat v[version]
Conformance: All 6 algorithm functions accept Evidence<_, Admitted, _>
```

**Verification Proof:**
- [ ] `cargo check` passes
- [ ] `cargo test` passes
- [ ] At least one algorithm accepts `Evidence<EventLog, Admitted, W>`
- [ ] `GraduateToWasm4pm` is implemented for at least one type
- [ ] Receipt linking works end-to-end

#### GAP_002: Implement Named Law Refusals in wasm4pm
**Owner:** wasm4pm team  
**Time:** 6 hours  
**Prerequisite:** GAP_001 (PASS)  
**Change Type:** ONTOLOGY_MISSING

**What to do:**

1. **Enumerate all validation failure reasons in wasm4pm**
   - Parse errors (invalid XML/JSON)
   - Validation errors (missing event objects, dangling links)
   - Execution errors (state machine violations)
   - Conformance errors (fitness < threshold)

2. **Create named refusal types** (or reuse from compat)
   ```rust
   // ~/wasm4pm-compat/src/refusal.rs (may already exist)
   pub enum DiscoveryFailure {
       InsufficientEvents,
       NoActivityKey,
       InvalidEventStructure,
       AlgorithmDivergence,
   }
   
   pub enum ConformanceFailure {
       LogNotAdmitted,
       ModelNotSound,
       TraceSequenceInvalid,
   }
   ```

3. **Replace Error::ValidationError(String) everywhere**
   ```rust
   // BEFORE: ~/wasm4pm/src/error.rs
   pub enum Error {
       ValidationError(String),  // FORBIDDEN
       ExecutionError(String),
   }
   
   // AFTER:
   pub enum Error<R, W> {
       ValidationFailed(Refusal<R, W>),
       ExecutionFailed(Refusal<R, W>),
   }
   ```

4. **Update all error construction sites**
   - Find all `ValidationError("...")` constructions
   - Replace with `ValidationFailed(Refusal::NamedLaw(...))`

**Checklist:**
- [ ] Define named refusal types for each failure mode
- [ ] Replace Error::ValidationError(String) with Error::ValidationFailed(Refusal<R, W>)
- [ ] Update all error construction sites (grep for "ValidationError\|ExecutionError")
- [ ] Add witness type to each refusal
- [ ] Update tests to use named refusals
- [ ] Commit with message: `feat(wasm4pm): implement named law refusals`
- [ ] Verify: `cargo test` passes
- [ ] Emit receipt: `BLAKE3(GAP_002_remediated)`

**Named Refusal Categories (sample):**
```
ParsingFailure::
  - InvalidXmlStructure
  - MissingRequiredField
  
ValidationFailure::
  - DanglingEventObjectLink
  - MissingFinalMarking
  - LogNotAdmitted
  
ExecutionFailure::
  - StateTransitionInvalid
  - EventSequenceInvalid
  
ConformanceFailure::
  - ModelNotSound
  - FitnessBelowThreshold
  - AlignmentFailed
```

**Expected Receipt:**
```
BLAKE3: [hash of named refusal implementation commit]
Date: 2026-06-01
Authority: wasm4pm maintainer
Refusal Types Defined: [count]
Error Enum Replaced: ALL String variants removed
Witness Type: Applied to all refusals
```

---

### Phase 4: MAJOR Priority (24-48 hours)
**Scope:** Reissue ALIVE verdict with behavioral gates  
**Prerequisite:** GAP_001, GAP_002, GAP_003, GAP_004, GAP_005 (all PASS)

#### GAP_003: Replace File-Count Gates with Behavioral Gates
**Owner:** process-intelligence research team  
**Time:** 8 hours  
**Change Type:** FORCED_ALIVE_RISK

**What to do:**

1. **Audit Current ALIVE Verdict**
   - Current: 12 file-count gates (e.g., "doctrine >= 15 files")
   - Problem: File counts ≠ process evidence
   - Solution: Replace with behavioral gates that execute and produce evidence

2. **Define New Behavioral Gates**

   **Gate 1: Parse All Doctrine Files**
   ```
   Test: Load and parse all doctrine/*.md files
   Evidence: md_parse_success.log (lists all files parsed without error)
   Pass Criteria: 0 parse errors
   Expected Count: 33 files parse successfully
   ```

   **Gate 2: Validate Standards Mappings**
   ```
   Test: For each standard (XES, OCEL, BPMN), validate that standards/*.md
         has correct specification citations
   Evidence: standards_validation_results.yaml (standard: pass/fail)
   Pass Criteria: All 51 standards files cite correct specification
   ```

   **Gate 3: Run PM4Py on Paper Test Cases**
   ```
   Test: Execute PM4Py on test logs from each paper
   Evidence: pm4py_execution_results.json (fitness, precision, generalization)
   Pass Criteria: All test cases produce fitness >= 0.8
   ```

   **Gate 4-12: Similar behavioral proofs**
   ```
   Gate 4: Lifecycle Phase Transitions
     Test: Verify state machine transitions for all 12 phases
     Evidence: transition_validation.txt
   
   Gate 5: M&A Claim Verification
     Test: Load and validate all 42 M&A claims
     Evidence: claim_validation_results.yaml
   
   Gate 6: Type-Law Soundness
     Test: Run compat type checks on all refusal types
     Evidence: type_check_results.log
   
   ... (6 more)
   ```

3. **Implement Test Harness**
   ```bash
   # gates/run_behavioral_gates.sh
   cd /Users/sac/process-intelligence
   
   echo "Gate 1: Parse all doctrine files..."
   python3 gates/test_doctrine_parse.py
   
   echo "Gate 2: Validate standards mappings..."
   python3 gates/test_standards_mapping.py
   
   # ... (10 more gates)
   
   echo "All gates complete. Summary:"
   cat gates/results/gate_summary.yaml
   ```

4. **Update Checkpoint File**
   - Replace PROCESS_INTELLIGENCE_ALIVE_001 (file-count gates)
   - Create PROCESS_INTELLIGENCE_ALIVE_002 (behavioral gates)
   - Cite execution evidence instead of file counts

**Checklist:**
- [ ] Design 12 behavioral gates with execution tests
- [ ] Implement test harness in Python or Rust
- [ ] Execute all gates and collect evidence
- [ ] Create gates/results/ directory with all evidence files
- [ ] Update checkpoint template to cite behavioral gate results
- [ ] Issue PROCESS_INTELLIGENCE_ALIVE_002 checkpoint
- [ ] Commit with message: `checkpoint: PROCESS_INTELLIGENCE_ALIVE_002 (behavioral gates)`
- [ ] Emit receipt: `BLAKE3(GAP_003_remediated)`

**Example Behavioral Gate Test (Python):**
```python
# gates/test_doctrine_parse.py
import os
import yaml

doctrine_dir = "/Users/sac/process-intelligence/doctrine"
parse_results = []

for file in os.listdir(doctrine_dir):
    if file.endswith('.md'):
        try:
            with open(os.path.join(doctrine_dir, file), 'r') as f:
                content = f.read()
                # Validate basic markdown structure
                assert content.startswith('#'), f"{file}: missing title"
                parse_results.append({'file': file, 'status': 'PASS'})
        except Exception as e:
            parse_results.append({'file': file, 'status': 'FAIL', 'error': str(e)})

# Write results
with open('gates/results/gate_1_doctrine_parse.yaml', 'w') as f:
    yaml.dump({'gate': 'parse-doctrine', 'results': parse_results}, f)
```

**Expected Receipt:**
```
BLAKE3: [hash of behavioral gates implementation + results]
Date: 2026-06-01
Authority: Process Intelligence Research Foundry
Gates Passed: 12 / 12
Evidence Location: checkpoints/PROCESS_INTELLIGENCE_ALIVE_002.md
```

---

### Phase 5: EXPANSION (48+ hours)
**Scope:** Implement missing algorithms in wasm4pm  
**Prerequisite:** GAP_001, GAP_002 (all PASS)  
**Can proceed in parallel**

#### GAP_006: Implement Inductive Miner
**Owner:** wasm4pm team  
**Time:** 12 hours  
**Change Type:** GENERATION_RULE_MISSING

**What to do:**
1. Implement Inductive Miner algorithm in `wasm4pm/src/discovery/inductive.rs`
2. Accept `Evidence<EventLog, Admitted, W>` as input
3. Return `TypedProcessTree` using compat types
4. Include soundness proof in documentation
5. Add property-based tests verifying output is sound

**Checklist:**
- [ ] Implement InductiveMiner struct
- [ ] Implement mine() method
- [ ] Add soundness validation logic
- [ ] Write documentation with algorithm proof
- [ ] Add property tests (e.g., generated tree always sound)
- [ ] Commit with message: `feat(wasm4pm): implement Inductive Miner algorithm`
- [ ] Emit receipt: `BLAKE3(GAP_006_remediated)`

#### GAP_007: Implement Alignment-Based Conformance
**Owner:** wasm4pm team  
**Time:** 10 hours  
**Change Type:** GENERATION_RULE_MISSING

**What to do:**
1. Implement `AlignmentConformance` in `wasm4pm/src/conformance/alignment.rs`
2. Accept `Evidence<EventLog, Admitted, W>` and model as input
3. Compute optimal alignment and return `Metric<FITNESS, N, D>`
4. Calculate precision in addition to fitness
5. Add conformance test cases with known expected values

**Checklist:**
- [ ] Implement AlignmentConformance struct
- [ ] Implement conformance_check() method
- [ ] Add precision calculation
- [ ] Write algorithm documentation
- [ ] Add test cases with known fitness/precision values
- [ ] Commit with message: `feat(wasm4pm): implement alignment-based conformance`
- [ ] Emit receipt: `BLAKE3(GAP_007_remediated)`

#### GAP_009: Implement OCPQ Evaluator
**Owner:** wasm4pm team  
**Time:** 14 hours  
**Change Type:** RUNNER_MISSING

**What to do:**
1. Implement `OcpqEvaluator` in `wasm4pm/src/query/ocpq.rs`
2. Implement evaluate(&query, &log) method
3. Support all OCPQ operators (eventually, path, next, parallel, etc.)
4. Return QueryResult with matched events and conformance info
5. Add comprehensive test cases

**Checklist:**
- [ ] Implement OcpqEvaluator struct
- [ ] Implement operator handlers (eventually, path, next, parallel)
- [ ] Implement conformance tracing (which events matched)
- [ ] Add performance benchmarks
- [ ] Write test cases for each operator
- [ ] Commit with message: `feat(wasm4pm): implement OCPQ evaluator`
- [ ] Emit receipt: `BLAKE3(GAP_009_remediated)`

#### GAP_011: Implement POWL Discovery Miner
**Owner:** wasm4pm team  
**Time:** 8 hours  
**Change Type:** GENERATION_RULE_MISSING

**What to do:**
1. Implement `PowerMiner` in `wasm4pm/src/discovery/powl.rs`
2. Accept `Evidence<EventLog, Admitted, W>` as input
3. Return `TypedPowl` using compat types
4. Use TreeProjectable trait for output projection
5. Add test cases

**Checklist:**
- [ ] Implement PowerMiner struct
- [ ] Implement mine() method
- [ ] Use TreeProjectable for projection
- [ ] Write algorithm documentation
- [ ] Add property-based tests
- [ ] Commit with message: `feat(wasm4pm): implement POWL miner`
- [ ] Emit receipt: `BLAKE3(GAP_011_remediated)`

---

## Remediation Timeline & Sequencing

### Timeline Overview

```
2026-06-01 — 2026-06-08 (ONE WEEK)

Phase 1 (IMMEDIATE):  GAP_010              [1h]
                      ├─ Day 1, 9:00 AM

Phase 2 (URGENT):     GAP_004, GAP_005,    [4+3+2 = 9h]
                      GAP_008              ├─ Days 1-2 (parallel)

Phase 3 (HIGH):       GAP_001, GAP_002     [8+6 = 14h]
                      ├─ Days 2-3 (serial: GAP_001 → GAP_002)

Phase 4 (MAJOR):      GAP_003              [8h]
                      ├─ Days 3-4

Phase 5 (EXPANSION):  GAP_006, GAP_007,    [12+10+14+8 = 44h]
                      GAP_009, GAP_011     ├─ Days 4-8 (parallel)
```

### Serial Dependencies (MUST complete in order)

1. **GAP_001 MUST complete before:**
   - GAP_002 (needs compat types imported)
   - GAP_006, GAP_007, GAP_009, GAP_011 (all need compat types)

2. **GAP_004 MUST complete before:**
   - GAP_005 (TTL must parse before ggen rules can be classified)

3. **Phases 1-3 MUST complete before:**
   - GAP_003 reissue (ALIVE verdict)
   - Downstream manufacturing can proceed

4. **GAP_003 reissue MUST complete before:**
   - Phases 5 algorithms

### Parallel Execution Strategy

```
Day 1-2:  Phase 1 + Phase 2 (4 teams parallel)
          └─ Gap_010 (wasm4pm-compat) + Gap_004/005/008 (ggen + compat)

Day 2-3:  Phase 3 (serial, 1 team)
          └─ Gap_001 → Gap_002 (must be sequential)

Day 3-4:  Phase 4 (1 team)
          └─ Gap_003 (behavioral gates)

Day 4-8:  Phase 5 (4 teams parallel)
          └─ Gap_006 + Gap_007 + Gap_009 + Gap_011
```

### Effort Breakdown

| Phase | Gaps | Hours | Days | FTE | Status |
|-------|------|-------|------|-----|--------|
| 1 | GAP_010 | 1 | 0.125 | 1 | Ready |
| 2 | GAP_004, 005, 008 | 9 | 1.125 | 3 | Ready |
| 3 | GAP_001, 002 | 14 | 1.75 | 2 | Blocked on Phase 2 |
| 4 | GAP_003 | 8 | 1 | 1 | Blocked on Phase 3 |
| 5 | GAP_006, 007, 009, 011 | 44 | 5.5 | 4 | Blocked on Phase 3 |
| **TOTAL** | **11 gaps** | **56-64** | **7-8** | **3-4 FTE** | |

---

## Receipt Issuance Protocol

Every gap closure must emit a cryptographic receipt (BLAKE3) linking:
1. Original gap ID
2. Remediation commit hash
3. Verification timestamp
4. Behavioral test results
5. Downstream impact assessment

### Receipt Format

```yaml
gap_closure_receipt:
  gap_id: "GAP_001"
  title: "wasm4pm-compat graduation boundary signal implementation"
  issued_date: "2026-06-01T14:30:00Z"
  
  remediation:
    commit_hash: "a3f7d8b2c9e1f4a6d8c0e2a4b6d8f0a2"
    commit_message: "feat(wasm4pm): implement compat graduation boundary"
    author: "wasm4pm maintainer"
    branch: "main"
  
  verification:
    test_suite: "cargo test"
    test_result: "PASS (42/42 tests)"
    behavioral_gate: "gate-9-compat-graduation-boundary-complete"
    gate_result: "PASS"
  
  evidence:
    - "cargo check passes"
    - "All 6 algorithm functions accept Evidence<_, Admitted, _>"
    - "GraduateToWasm4pm trait implemented"
    - "Receipt linking verified end-to-end"
  
  content_hash: "BLAKE3:e7c8f2d94a71b5c3e9f1d6a4b2c8e5f7a1d3c5b7e9f2d4a6c8e0f1a3b5c7d9"
  
  next_workflow: "GGEN_ECOSYSTEM_MANUFACTURING_001"
  authorization: "Process Intelligence Research Foundry"
```

### Receipt Emission Checklist

For each gap closure:
- [ ] Commit includes ONLY gap remediation (no unrelated changes)
- [ ] Commit message references gap ID (e.g., "fix(project): remediate GAP_001")
- [ ] All tests PASS
- [ ] Behavioral gate PASS (or new behavioral gate added)
- [ ] Receipt YAML file created in `receipts/GAP_XXX_closure_receipt.yaml`
- [ ] Receipt BLAKE3 hash computed and verified
- [ ] Receipt added to `receipts/RECEIPT_REGISTRY.md`

---

## Authorization Gate

**IMPORTANT:** This remediation plan is issued by PI_RESEARCH_PROGRAM_ALIVE_001 checkpoint authority.

### Authorization Prerequisites Met

- [x] Process Intelligence Research Foundry has completed 12 audit gates
- [x] All gaps have been formally identified with evidence
- [x] Remediation paths have been defined
- [x] Prerequisites and dependencies have been analyzed

### Authorization Blocked Until

- [ ] GAP_001, GAP_002, GAP_003, GAP_004, GAP_005 are CLOSED
- [ ] All receipts are issued and registered
- [ ] ALIVE reissue checkpoint is ready

### Downstream Workflows

**CANNOT START** until all CRITICAL gaps are closed:
- GGEN_ECOSYSTEM_MANUFACTURING_001
- M&A_DECK_MANUFACTURING_PIPELINE_001
- BOARD-LEVEL CLAIM DELIVERY

**CAN START** after Phase 1 completes (minor fixes):
- Continue research analysis
- Prepare manufacturing templates
- Validate existing doctrine

---

## Governance & Escalation

### Decision Authority

| Decision | Authority | Timeline |
|----------|-----------|----------|
| Remediation approach | Project maintainer | Real-time |
| Phase sequencing | Process Intelligence PDCA | Real-time |
| Blocker escalation | Research Directorate | 4 hours |
| ALIVE reissue | Research Directorate | After Phase 3 PASS |
| Downstream manufacturing auth | Research Directorate | After Phase 3 PASS |

### Escalation Criteria

Escalate to Research Directorate if:
- [ ] Any gap cannot be remediated within estimated hours
- [ ] Remediation reveals additional undiscovered gaps
- [ ] File-count gates cannot be replaced with behavioral gates
- [ ] More than 2 tests fail after remediation

### Remediation Blocker Report

If a gap cannot be remediated:

```yaml
blocker_report:
  gap_id: "GAP_XXX"
  blocker_type: "ARCHITECTURAL | DEPENDENCY | UNKNOWN"
  description: "..."
  impact: "Blocks downstream: ..."
  recommended_action: "..."
  escalation_to: "Research Directorate"
  timestamp: "2026-06-01T..."
```

---

## Appendices

### A. File Locations Reference

| Gap | File Paths |
|-----|-----------|
| GAP_001 | ~/wasm4pm/Cargo.toml, ~/wasm4pm/src/*.rs, ~/wasm4pm-compat/src/graduation.rs |
| GAP_002 | ~/wasm4pm-types/src/error.rs, ~/wasm4pm-compat/src/admission.rs |
| GAP_003 | checkpoints/PROCESS_INTELLIGENCE_ALIVE_001.md, checkpoints/PI_RESEARCH_PROGRAM_ALIVE_001.md |
| GAP_004 | ggen/ontology-extensions.ttl |
| GAP_005 | ggen/audits/audit-*.sh.ggen, ggen/templates/*.ggen, ggen/MANIFEST.md |
| GAP_006 | ~/wasm4pm/src/discovery/inductive.rs (new) |
| GAP_007 | ~/wasm4pm/src/conformance/alignment.rs (new) |
| GAP_008 | compat/src/manufacturing/mod.rs:735, compat/src/manufacturing/traits.rs:34 |
| GAP_009 | ~/wasm4pm/src/query/ocpq.rs (new) |
| GAP_010 | ~/wasm4pm-compat/src/petri_net/wf_net.rs |
| GAP_011 | ~/wasm4pm/src/discovery/powl.rs (new) |

### B. Tools & Commands

**RDF Turtle Parsing Validation:**
```bash
# Install rapper (Raptor RDF parser)
brew install raptor2

# Validate TTL syntax
rapper -c ggen/ontology-extensions.ttl

# List all triples
rapper ggen/ontology-extensions.ttl
```

**Rust Testing:**
```bash
# In wasm4pm or wasm4pm-compat directory
cargo test                          # All tests
cargo test gap_001                  # Tests matching "gap_001"
cargo check                         # Compile check only
cargo doc --open                    # Generate and open docs
```

**Git Workflow:**
```bash
cd /Users/sac/process-intelligence

# Check current status
git status

# Commit with gap reference
git add <files>
git commit -m "fix(project): remediate GAP_001"

# Verify receipt
git log --oneline -1
```

### C. Conformance Testing

```python
# gates/test_gap_remediation.py
import subprocess
import yaml

def verify_gap_001():
    """Verify GAP_001 remediation (graduation boundary)"""
    result = {
        'gap_id': 'GAP_001',
        'checks': []
    }
    
    # Check 1: compat in Cargo.toml
    with open('/Users/sac/wasm4pm/Cargo.toml') as f:
        if 'wasm4pm-compat' in f.read():
            result['checks'].append({
                'name': 'compat_dependency',
                'status': 'PASS'
            })
    
    # Check 2: cargo test passes
    proc = subprocess.run(
        ['cargo', 'test'],
        cwd='/Users/sac/wasm4pm',
        capture_output=True
    )
    result['checks'].append({
        'name': 'cargo_test',
        'status': 'PASS' if proc.returncode == 0 else 'FAIL'
    })
    
    # Check 3: At least one algorithm accepts Evidence<_, Admitted, _>
    # (Manual code review)
    
    return result
```

---

## Sign-Off

**Program Authority:** Process Intelligence Research Foundry  
**Issued By:** PI_RESEARCH_PROGRAM_ALIVE_001 Checkpoint Authority  
**Date:** 2026-06-01  
**Repository:** /Users/sac/process-intelligence  
**Authority Certificate:** [BLAKE3_HASH]

This remediation plan supersedes all prior gap documentation and provides the authoritative path to full ALIVE status and downstream manufacturing authorization.

**Next Action:** Execute Phase 1 (GAP_010 remediation, 1 hour).

