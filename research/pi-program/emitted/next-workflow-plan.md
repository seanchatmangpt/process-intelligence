# Next Workflow Plan
**Process Intelligence Program — Phased Remediation and Authorization**

Authority: Research Foundry (PROCESS_INTELLIGENCE_ALIVE_001)
Generated: 2026-06-01
Source: failed-gate-ledger.yaml + audit-results.yaml

---

## Current Status

**Program Verdict:** ALIVE_001 ✓  
**Gates Passed:** 11/13  
**Blocking Violations:** 1 (audit_005: DTO flattening in wasm4pm-compat)  
**Non-Blocking Issues:** 1 (audit_012: meta-audit routing clarity)  
**Next Checkpoint:** PROCESS_INTELLIGENCE_ALIVE_002 (upon remediation completion)  

---

## Phase 1: Immediate Remediation (2026-06-02)

### 1.1 Remediate audit_005 (DTO Flattening)

**Owner:** wasm4pm-compat maintenance team  
**Duration:** 4 hours  
**Blocking:** Yes (prevents ALIVE_002 certification)  

**Workflow:**

```
audit_005_remediation
├─ Step 1: Code Analysis (1 hour)
│  ├─ Task: Identify all callers of receipt_json() and to_json_string()
│  ├─ Tool: grep, LSP findReferences
│  ├─ Location: sources/wasm4pm-compat/
│  └─ Deliverable: Call dependency graph
│
├─ Step 2: Wasm4PM Engine Migration (2 hours)
│  ├─ Task: Create wasm4pm/src/manufacturing/json.rs
│  ├─ Spec: New JSON serialization functions accepting Evidence<T, State, Witness>
│  ├─ Constraints: Backward-compatible non-public APIs
│  └─ Deliverable: JSON layer in wasm4pm (not compat)
│
├─ Step 3: Compat API Replacement (1 hour)
│  ├─ Task: Remove receipt_json() and to_json_string() from compat public API
│  ├─ Pattern: Sealed trait pattern (prevent public String leakage)
│  ├─ Update Files: compat/src/manufacturing/traits.rs, mod.rs
│  └─ Deliverable: Zero String-returning methods in manufacturing module
│
└─ Step 4: Audit Script Creation (0.5 hours)
   ├─ Task: Create audit-no-json-in-compat.sh.ggen
   ├─ Location: sources/wasm4pm-compat/ggen/audits/
   ├─ Rule: Grep for "to_json" and "json()" in compat public API
   └─ Deliverable: Audit passes (0 violations)
```

**Verification Checklist:**
- [ ] All callers of removed methods identified and updated
- [ ] New JSON functions in wasm4pm pass all tests
- [ ] compat public API contains no String-returning receipt/manufacturing methods
- [ ] audit-no-json-in-compat.sh.ggen passes (0 violations)
- [ ] wasm4pm-compat compile-pass/fail fixtures all pass
- [ ] ALIVE_002 audit passes (audit_005 now PASS)

**Unblocks:**
- PROCESS_INTELLIGENCE_ALIVE_002 certification
- Full board-admissible claim manufacturing
- Production wasm4pm-compat deployments

---

### 1.2 Remediate audit_012 (Meta-Audit Routing)

**Owner:** Process Intelligence Program Authority  
**Duration:** 2 hours  
**Blocking:** No (advisory; does not prevent ALIVE_002)  

**Workflow:**

```
audit_012_routing_documentation
├─ Step 1: Create Routing Law (1 hour)
│  ├─ Task: Create gaps/audit-routing-law.yaml
│  ├─ Content: Explicit routing rules for meta-audit failures
│  ├─ Schema: meta_audit_name, remediation_class, blocking, owner
│  └─ Deliverable: audit-routing-law.yaml (defines non-blocking policy)
│
├─ Step 2: Update GAP Register (0.5 hours)
│  ├─ Task: Add GAP_003_META_AUDIT_ROUTING to gaps/GAP_REGISTER.md
│  ├─ Status: RESOLVED (via audit-routing-law.yaml)
│  ├─ Severity: LOW
│  └─ Deliverable: GAP_003 entry in register
│
└─ Step 3: Checkpoint Addendum (0.5 hours)
   ├─ Task: Add addendum to PROCESS_INTELLIGENCE_ALIVE_001
   ├─ Content: Explain audit_012 is meta-audit only, non-blocking
   ├─ Signature: Research Foundry authority
   └─ Deliverable: Immutable addendum (no modification of original checkpoint)
```

**Verification Checklist:**
- [ ] audit-routing-law.yaml created and syntactically valid
- [ ] GAP_003 added to GAP_REGISTER.md
- [ ] Checkpoint addendum documents non-blocking nature
- [ ] No new blocking violations introduced

**Impact:** Clarifies audit process; does not block any workflows

---

## Phase 2: ALIVE_002 Recertification (2026-06-02)

**Duration:** 2 hours  
**Authority:** Van der Aalst Chicago TDD (13-gate framework)  

### 2.1 Re-run Full Audit Suite

```
alive_002_recertification
├─ Gate 1: Project Registry Complete → VERIFY (unchanged)
├─ Gate 2: Checkpoint Ledger Complete → VERIFY (unchanged)
├─ Gate 3: No Forced ALIVE → VERIFY (unchanged)
├─ Gate 4: No Invalid .ggen → VERIFY (unchanged)
├─ Gate 5: No DTO Flattening → RE-TEST (should now PASS)
├─ Gate 6: No Tool Smuggling → VERIFY (unchanged)
├─ Gate 7: No Telemetry as Receipt → VERIFY (unchanged)
├─ Gate 8: No Realtime as Evidence → VERIFY (unchanged)
├─ Gate 9: No Dashboard Truth → VERIFY (unchanged)
├─ Gate 10: No Client-Only Auth → VERIFY (unchanged)
├─ Gate 11: Receipts Present → VERIFY (unchanged)
├─ Gate 12: Remediation Routed → VERIFY (clarified via audit-routing-law)
└─ Gate 13: (Final authorization) → PASS if all above PASS
```

### 2.2 Issue PROCESS_INTELLIGENCE_ALIVE_002

**Checkpoint:** PROCESS_INTELLIGENCE_ALIVE_002  
**Date:** 2026-06-02  
**Authority:** Dr. Wil van der Aalst AGI Swarm Court  
**Gates:** 13/13 passed  
**Status:** IMMUTABLE  

**Checkpoint Content:**
- All 13 gates passed
- audit_005 remediated (DTO flattening fixed)
- audit_012 clarified (meta-audit routing documented)
- Full authorization for downstream workflows
- Receipt chain continuity verified

**Authorized Downstream Workflows:**
1. ✓ ggen manufacturing pipeline scaling (board-admissible artifacts)
2. ✓ OTel Weaver production integration (feedstock validation)
3. ✓ Blue River Dam orchestration (autonomic governance)
4. ✓ ZOEapp full-lifecycle process intelligence rollout

---

## Phase 3: Downstream Authorization Wave (2026-06-03+)

### 3.1 GGEN_DTO_REMEDIATION_AUTHORITY

**Name:** GGEN DTO Remediation Authority Checkpoint  
**Duration:** 0.5 hours (checkpoint issuance only)  
**Purpose:** Formally authorize ggen manufacturing with fixed wasm4pm-compat boundary  

**Checkpoint Content:**
- audit_005 remediation verified
- compat DTO boundary sealed (no String leakage)
- Manufacturing ontology fully specified (592 lines)
- Board-admissible gate (fitness ≥ 0.95) enforceable

**Unblocks:**
- ggen scaling to 100+ board-admissible templates
- M&A deck manufacturing at enterprise scale

---

### 3.2 GGEN_TS_PROJECTION_MANUFACTURING_001

**Name:** ggen TypeScript Projection Manufacturing  
**Duration:** 2-3 days (engineering work)  
**Purpose:** Extend ggen templates to generate TypeScript DTO bindings with Evidence<T, State, Witness> enforcement  

**Workflow:**
```
ggen_ts_projection_manufacturing
├─ Template Design (8 hours)
│  ├─ ts-dto-evidence.tera (generates TypeScript Evidence<T,S,W> types)
│  ├─ ts-witness-lattice.tera (generates witness enumeration)
│  └─ ts-refusal-codex.tera (generates refusal code enums)
│
├─ SPARQL Query Design (4 hours)
│  ├─ extract-ts-evidence-requirements.rq
│  ├─ extract-ts-witness-bindings.rq
│  └─ extract-ts-refusal-codes.rq
│
├─ Ontology Extensions (4 hours)
│  └─ ts-projection-ontology.ttl (Evidence properties in TS domain)
│
└─ Testing (8 hours)
   ├─ Compile-pass fixtures (Evidence types generated correctly)
   ├─ Compile-fail fixtures (invalid state transitions caught)
   └─ Type soundness (no escape to plain objects)
```

**Deliverables:**
- 3 new ggen templates
- 3 SPARQL queries
- Ontology extensions
- 40+ test fixtures
- ts-evidence-generation.md documentation

**Unblocks:** ZOEapp type-safe frontend component generation

---

### 3.3 BLUE_RIVER_INTAKE_BOUNDARY_001

**Name:** Blue River Dam Intake Boundary Checkpoint  
**Duration:** 1 week (engineering + integration testing)  
**Purpose:** Formalize 11-pathway admission law and integrate with wasm4pm output streams  

**Workflow:**
```
blue_river_intake_boundary
├─ Formal Specification (16 hours)
│  ├─ Temporal monotonicity proofs (LTL invariants)
│  ├─ Schema compatibility automaton (state machine for 11 pathways)
│  ├─ Causal consistency checker (happens-before ordering)
│  └─ Cryptographic signature verification (Ed25519 threshold)
│
├─ Integration Testing (24 hours)
│  ├─ Admit wasm4pm discovery output (InductiveWitness, HeuristicsWitness, etc.)
│  ├─ Admit Blue River governance receipts (backward chain to BLAKE3)
│  ├─ Admit ZOEapp replay fixtures (516 OCEL 2.0 traces)
│  └─ Refusal testing (trigger each 11 refusal pathways)
│
├─ Performance Validation (8 hours)
│  ├─ Throughput: 1000+ events/second
│  ├─ Latency: <100ms admission decision
│  └─ Memory: <100MB governance state
│
└─ Safety Verification (8 hours)
   ├─ LTL invariants (no impossible state sequences)
   ├─ Deadlock freedom (no admission loops)
   └─ Liveness (all valid traces eventually admitted)
```

**Deliverables:**
- Formalized 11-pathway admission law (TLA+ specification)
- Integrated wasm4pm→Blue River intake pipeline
- 50+ integration test cases
- Performance benchmark report
- Safety proof document

**Unblocks:** Autonomous governance orchestration (Blue River in production)

---

### 3.4 GGEN_OTEL_WEAVER_PRODUCTION_001

**Name:** OTel Weaver Production Integration Checkpoint  
**Duration:** 2 weeks (engineering + integration + observability)  
**Purpose:** Deploy OTel Weaver feedstock validation at enterprise scale  

**Workflow:**
```
otel_weaver_production
├─ Schema Registry Scaling (16 hours)
│  ├─ Multi-tenant schema management
│  ├─ Version negotiation (client vs. server schema)
│  ├─ Drift detection and reporting
│  └─ Auto-remediation suggestions (via Blue River)
│
├─ Manufacturing Scale-Out (24 hours)
│  ├─ OTel collector configuration templates (100+ variants)
│  ├─ Witness projection templates (OTel→OCEL mappings)
│  ├─ Refusal routing (finding→law binding)
│  └─ Loss report templates (evidence quality metrics)
│
├─ Observability Pipeline (16 hours)
│  ├─ Metrics: admission rate, refusal rate, loss rate
│  ├─ Traces: end-to-end feedstock flow (OTel Weaver→Blue River)
│  ├─ Logs: admission decisions with decision path
│  └─ Dashboards: (projection only; not process evidence)
│
└─ Production Hardening (12 hours)
   ├─ Rate limiting (prevent DOS)
   ├─ Batch processing (1000+ events/batch)
   ├─ Fallback strategies (queue, retry, dead-letter)
   └─ Disaster recovery (state machine replay)
```

**Deliverables:**
- 100+ OTel collector configuration templates
- Witness projection ontology extensions
- Metrics/traces/logs observability pipeline
- Production runbook (operations guide)
- Capacity planning report (scalability to 1B+ events/day)

**Unblocks:** Enterprise OTel telemetry pipeline with Process Intelligence integration

---

## Phase 4: Continuous Operations (2026-06-15+)

### 4.1 Ongoing Receipt Emission

**Workflow:**
- Blue River Dam emits receipt per action (success + failure)
- wasm4pm emits receipt per discovery/conformance/replay execution
- ZOEapp emits receipt per route admission decision
- All receipts chained via BLAKE3 (cryptographic ledger)

**Metrics:**
- Receipt count: 100+ per hour (target steady-state)
- Chain integrity: 100% (no forked ledgers)
- Verification latency: <1ms per receipt check

### 4.2 Continuous Audit Loop

**Frequency:** Weekly  
**Authority:** Van der Aalst Chicago TDD framework  
**Scope:** 13-gate system (gates 1-13)

**Workflow:**
```
continuous_audit_loop
├─ Weekly Runs (Mondays 09:00 UTC)
│  ├─ All 13 gates executed
│  ├─ New violations trigger escalation alerts
│  ├─ Pass/fail verdicts logged to audit ledger
│  └─ Remediation plans routed to owners
│
├─ Monthly Reviews (1st of month)
│  ├─ Aggregate audit results (4 weeks)
│  ├─ Trend analysis (gate health over time)
│  ├─ Remediation success rate (closed gaps)
│  └─ Checkpoint milestone assessment
│
└─ Annual Graduation Review (June 1)
   ├─ Full ALIVE recertification (13-gate verification)
   ├─ New checkpoint issuance (ALIVE_003, etc.)
   └─ Program health summary (public report)
```

### 4.3 Evidence-Driven Improvements

**Sources of Evidence:**
- Receipt ledger (continuous operation data)
- Audit verdicts (gate health)
- Gap register (known defects)
- Experiment results (new capabilities)

**Improvement Cycles:**
- Weekly: Performance tuning (receipt throughput, admission latency)
- Monthly: Capability expansion (new discovery algorithms, manufacturing templates)
- Quarterly: Major refactoring (wasm4pm optimizations, compat API cleanups)
- Annually: Version bumps (OCEL 2.1, new PM4Py functions, emerging standards)

---

## Timeline Summary

| Date | Phase | Deliverables | Status |
|------|-------|--------------|--------|
| **2026-06-02** | Phase 1 + Phase 2 | audit_005 remediation + ALIVE_002 checkpoint | IMMEDIATE (4-6 hrs) |
| **2026-06-02** | Phase 2 | GGEN_DTO_REMEDIATION_AUTHORITY | SAME DAY |
| **2026-06-03 to 06-05** | Phase 3.2 | GGEN_TS_PROJECTION_MANUFACTURING_001 | 2-3 DAYS |
| **2026-06-08 to 06-15** | Phase 3.3 | BLUE_RIVER_INTAKE_BOUNDARY_001 | 1 WEEK |
| **2026-06-16 to 06-30** | Phase 3.4 | GGEN_OTEL_WEAVER_PRODUCTION_001 | 2 WEEKS |
| **2026-07-01+** | Phase 4 | Continuous operations & evidence-driven improvements | ONGOING |

---

## Resource Allocation

### Phase 1 Remediation (4-6 hours)
- Primary: wasm4pm-compat maintenance team (4 hours)
- Secondary: Process Intelligence Program Authority (2 hours)
- Tools: Rust compiler, cargo, LSP, grep

### Phase 2 Recertification (2 hours)
- Primary: Van der Aalst Chicago TDD audit framework
- Tool: 13-gate audit harness (automated)

### Phase 3 Downstream Authorization (6+ weeks)
- **3.2 TypeScript Projection:** 1 frontend engineer (2-3 days)
- **3.3 Blue River Integration:** 2 systems engineers (1 week)
- **3.4 OTel Weaver Production:** 1 telemetry engineer + 1 SRE (2 weeks)

### Phase 4 Continuous Operations (Ongoing)
- **Audit Loop:** Automated (weekly runs)
- **Receipt Emission:** Distributed (all systems)
- **Observability:** 0.5 SRE FTE (monitoring/dashboards)
- **Improvements:** Quarterly planning (program team)

---

## Success Criteria

### Phase 1 (Remediation)
- ✓ audit_005 passes (JSON moved to wasm4pm engine)
- ✓ audit_012 clarified (meta-audit routing documented)
- ✓ No new violations introduced

### Phase 2 (ALIVE_002)
- ✓ All 13 gates pass simultaneously
- ✓ PROCESS_INTELLIGENCE_ALIVE_002 checkpoint issued
- ✓ Authorization for Phase 3 workflows granted

### Phase 3 (Downstream Workflows)
- ✓ TypeScript evidence generation produces sound types
- ✓ Blue River admits 100% of valid traces (0 false rejections)
- ✓ OTel Weaver processes 1000+ events/second (enterprise scale)

### Phase 4 (Continuous Operations)
- ✓ Receipt ledger grows continuously (100+ per hour)
- ✓ Weekly audits pass (zero new violations)
- ✓ Mean time to remediation (MTTR) < 24 hours for gate failures

---

## Authority and Immutability

This workflow plan is **definitive** as of **2026-06-01**.

- **Authority:** Research Foundry (PROCESS_INTELLIGENCE_ALIVE_001 checkpoint)
- **Immutability:** Amendments only (no retroactive modification)
- **Version Control:** All phases tracked in git commits with receipt chain
- **Checkpoint Ledger:** Each phase completion triggers immutable checkpoint

**Next Plan Review:** 2026-07-01 (after Phase 3 completion)
