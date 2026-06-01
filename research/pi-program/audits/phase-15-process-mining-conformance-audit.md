# Phase 15: Process Mining Audit — Van der Aalst Conformance

**Date:** 2026-06-01  
**Program:** Process Intelligence Research Foundry  
**Authority:** Dr. Wil van der Aalst Process Mining Constitution  
**Auditor:** Claude Code (Haiku 4.5)

---

## Executive Summary

This audit examines whether the PI research program's declared manufacturing process (as declared in `ggen.toml` and git commit history) conforms to Van der Aalst process mining law: **"If code says it worked but event log cannot prove a lawful process happened, it did not work."**

**Verdict: PARTIAL**

- **Declared Process:** 1 active generation rule (blue-river-orchestrator) documented in ggen.toml
- **Actual Event Log:** 36+ commits representing 9 research phases (init, paper, pm4py, compat, wasm4pm, lifecycle, m-and-a, audit, checkpoint)
- **Evidence Available:** Git log (commits), checkpoint files, receipt JSON
- **Evidence Missing:** OTel traces, explicit object-centric event logs (OCEL format), conformance metrics (fitness/precision/generalization)

### Critical Finding

The PI program has:
- ✅ Declared manufacturing process (ggen.toml)
- ✅ Produced checkpoints (ALIVE_001, partial audits)
- ❌ **No extractable OTel traces for event log mining**
- ❌ **No OCEL event logs proving lawful object lifecycles**
- ❌ **No conformance metrics (fitness/precision/simplicity/generalization)**
- ❌ **No negative test evidence (impossible traces rejected)**

**Result:** Process is DECLARED but UNPROVEN per Van der Aalst standards.

---

## 1. Declared Process Definition

### ggen.toml Manufacturing Pipeline

From `/Users/sac/process-intelligence/ggen/ggen.toml`:

```toml
[project]
name = "process-intelligence-ggen"
version = "0.1.0"
description = "Generate board-admissible M&A assets and autonomic governance engines"
evidence_sources = ["../receipts", "../checkpoints"]

[[generation.rules]]
name = "blue-river-orchestrator"
description = "Autonomic MAPE-K governance engine with lifecycle state transitions"
query = { file = "queries/extract-lifecycle-governance.rq" }
template = { file = "templates/blue-river.tera" }
output_file = "../blue_river_dam/src/lib.rs"
mode = "Overwrite"
```

**Declared Activities:**
1. **Normalize** (inference rule, pass-through)
2. **Generate Blue River Orchestrator** (main production rule)
   - Input: Lifecycle governance SPARQL query
   - Transform: Tera template
   - Output: Rust source code

**Declared Phases:** 3 pipelines (disabled):
- M&A deck manufacturing (DISABLED)
- Diligence workbook (DISABLED)
- Visualizer dashboard (DISABLED)

---

## 2. Event Log Extraction

### Methodology

Event log extracted from:
1. **Git commit history** (36+ commits, 5/31–6/1/2026)
2. **Checkpoint files** (ALIVE_001, partial verdicts)
3. **Receipt JSON** (5 slide receipts, 6 ggen operation receipts)
4. **Audit files** (5 Van der Aalst conformance audits)

### Extracted Event Log (Chronological)

**Objects (Artifact Lifecycle):**

| Object ID | Object Type | Created | Status |
|-----------|-------------|---------|--------|
| PI_001 | ResearchProgram | 2026-05-31T22:42:54Z | ALIVE_001 |
| GGEN_001 | ManufacturingEngine | 2026-05-31T22:42:54Z | ACTIVE |
| BR_DAM_001 | GovernanceRuntime | 2026-05-31T23:04:00Z | ACTIVE |
| WASM4PM_COMPAT_001 | ProjectionLayer | 2026-05-31T22:55:00Z | PARTIAL |

**Events (Commit Sequence):**

```
E-001: research-init (PI_001 created)
       timestamp: 2026-05-31T22:42:54Z
       subject: create ~/process-intelligence root
       objects: [PI_001]

E-002: doctrine (PI_001 state → DOCTRINE_DEFINED)
       timestamp: 2026-05-31T22:42:54Z
       subject: define process intelligence research program
       objects: [PI_001]

E-003: doctrine (PI_001 state → DOCTRINE_COMPLETE)
       timestamp: 2026-05-31T22:42:56Z
       subject: publish research foundry README
       objects: [PI_001]

E-004–E-009: research-paper
       timestamp: 2026-05-31T22:42:55–2026-05-31T22:42:56Z
       subject: paper analysis (6 papers classified)
       objects: [PI_001]

E-010–E-014: research-compat
       timestamp: 2026-05-31T22:42:55Z
       subject: wasm4pm-compat type-law inventory
       objects: [WASM4PM_COMPAT_001]

E-015–E-019: research-wasm4pm
       timestamp: 2026-05-31T22:42:55Z
       subject: wasm4pm execution surfaces inventory
       objects: [PI_001]

E-020–E-024: research-pm4py
       timestamp: 2026-05-31T22:42:55Z
       subject: PM4Py capability inventory
       objects: [PI_001]

E-025–E-040+: lifecycle, m-and-a, audit, checkpoint
       timestamp: 2026-05-31T23:00:00Z → 2026-06-01T10:19:00Z
       subjects: [lifecycle definitions, M&A claims, audits, checkpoints]
       objects: [PI_001, GGEN_001, BR_DAM_001, WASM4PM_COMPAT_001]
```

### Object Lifecycle Analysis

**Object: PI_001 (ResearchProgram)**

```
INIT (2026-05-31T22:42:54Z)
  ├─ doctrine: define process intelligence research program
  ├─ doctrine: define Blue River Dam research role
  ├─ doctrine: define autonomic knowledge actuation
  ├─ doctrine: define full lifecycle process scope
  └─ doctrine: define public standards gravity
  
RESEARCH (2026-05-31T22:42:55Z)
  ├─ research-paper: 6 papers classified
  ├─ research-compat: wasm4pm-compat type-law surfaces
  ├─ research-wasm4pm: wasm4pm execution surfaces
  └─ research-pm4py: PM4Py capability surfaces

SYNTHESIS (2026-05-31T23:00:00Z)
  ├─ lifecycle: define 9 state definitions
  ├─ m-and-a: 7 claim taxonomies
  └─ audit: 5 conformance audits

CHECKPOINT (2026-06-01T10:19:00Z)
  └─ checkpoint: PROCESS_INTELLIGENCE_ALIVE_001
     (ALIVE verdict issued)
```

**Object: GGEN_001 (ManufacturingEngine)**

```
DECLARED (ggen.toml)
  └─ name: "process-intelligence-ggen"
     version: "0.1.0"
     rules: 1 active (blue-river-orchestrator)
     rules: 3 disabled (M&A, diligence, visualizer)

EXECUTION (git commits)
  ├─ 2026-06-01T00:30:00Z: blue_river_generation.md receipt
  ├─ 2026-06-01T00:32:00Z: ma_deck_rendering_authority_assessment.md receipt
  └─ 2026-06-01T00:38:00Z: 6 ggen operation receipts (BLAKE3 cryptographic chain)
```

---

## 3. Declared vs. Actual Process Conformance

### Declared Model (ggen.toml)

```
Query (SPARQL)
  ↓
Template (Tera)
  ↓
Output (Rust)
```

**Declared Activities:**
1. Execute `queries/extract-lifecycle-governance.rq` (SPARQL)
2. Template output with `templates/blue-river.tera` (Tera)
3. Write to `../blue_river_dam/src/lib.rs` (Overwrite)

### Actual Model (Git History + Receipts)

```
research-paper → research-compat → research-wasm4pm → research-pm4py
  ↓
doctrine (process law)
  ↓
lifecycle (state definitions)
  ↓
m-and-a (synergy/debt claims)
  ↓
audit (conformance audits)
  ↓
checkpoint (ALIVE_001 verdict)
```

### Conformance Analysis

**MISMATCH DETECTED:**

1. **Declared:** SPARQL query → Tera template → Rust output (3 steps)
   **Actual:** Git commits represent 36+ distinct events across 9 research phases

2. **Declared:** Single generation rule (blue-river-orchestrator)
   **Actual:** Multi-phase research pipeline with paper analysis, type-law mapping, lifecycle definitions, and M&A claims

3. **Declared:** evidence_sources = ["../receipts", "../checkpoints"]
   **Actual:** Evidence generated from git history (commits), manual doctrine writing, and audit execution

**Verdict:** The PI research program's **actual process is NOT conforming to the declared ggen.toml process model.** The ggen.toml describes a single generation pipeline, but the actual process is a multi-phase research program with discovery, synthesis, and checkpoint gates.

---

## 4. Quality Metrics (Conformance Testing)

Van der Aalst standards require:
- **Fitness** ≥95% (actual events match declared flow)
- **Precision** ≥90% (declared model doesn't over-generalize)
- **Simplicity** ≥80% (model not overfitted)
- **Generalization** ≥85% (model explains behavior beyond observed traces)

### Calculated Metrics

#### Fitness (Actual Events ÷ Declared Activities)

**Declared activities:** 3 (normalize, query, template)
**Actual events:** 36+ distinct commits

```
Fitness = 3 / 36+ = 0.08 (8%)
```

**Finding:** FAIL. The declared ggen.toml process describes <10% of the actual event sequence.

**Rationale:** ggen.toml is NOT the authority model for the PI research program. The git commit history reveals the actual process:
- 5 research phases (paper, compat, wasm4pm, pm4py, docs)
- 3 synthesis phases (lifecycle, m-and-a, audit)
- 1 checkpoint phase

The correct declared model should enumerate these 9 phases, not a single SPARQL→Tera→Rust pipeline.

#### Precision (False Positives ÷ Declared Activities)

The ggen.toml model does not accommodate the observed phases (research-paper, lifecycle, audit, etc.). These are **not false positives**—they are **required activities omitted from the declared model**.

```
Precision = (declared ∩ actual) / declared
          = 0 / 3 = 0% (declared model explains 0% of observed behavior)
```

**Finding:** FAIL. The declared model over-simplifies the process.

#### Simplicity (Model Complexity)

The true model (9 research phases) is more complex than the declared model (1 generation rule), but the declared model is **incompletely specified**.

```
Simplicity Score: Declared model is too simple (incomplete)
                  True model is appropriately complex (complete)
```

**Finding:** The declared model fails simplicity test by being **under-specified** rather than over-specified.

#### Generalization (Model Stability)

No replication data available. Cannot assess whether the process is generalizable to future research phases.

```
Generalization Score: INCONCLUSIVE (n=1 instance)
```

---

## 5. Artifact Lifecycle Verification

### Requirement: Every artifact must have source, receipt, and valid history

**Analyzed Artifacts:**

| Artifact | Source | Receipt | History | Status |
|----------|--------|---------|---------|--------|
| doctrine/ files | git commit | RECEIPT_REGISTRY.md | ✓ git history | ✓ VALID |
| sources/papers/ | git commit | audit-paper-coverage.md | ✓ git history | ✓ VALID |
| sources/wasm4pm/ | git commit | audit-type-law-coverage.md | ✓ git history | ✓ VALID |
| sources/wasm4pm-compat/ | git commit | CONFORMANCE_REPORT.md | ✓ git history | ✓ VALID |
| checkpoints/ | git commit | checkpoint files | ✓ git history | ✓ VALID |
| blue_river_dam/src/lib.rs | ggen rule | ggen receipt | ? (needs verification) | ? PARTIAL |

**Issue:** The Blue River Dam orchestrator source code (blue_river_dam/src/lib.rs) should be produced by the blue-river-orchestrator ggen rule. However:
- No trace of SPARQL query execution
- No trace of Tera template execution
- File is manually written or auto-generated but not documented

**Finding:** PARTIAL. Most artifacts have clear provenance; Blue River Dam orchestrator source lacks ggen receipt chain.

---

## 6. Negative Testing (Impossible Traces)

**Requirement:** System must reject impossible process sequences.

**Test Case 1: Out-of-order phases**

Suppose we injected a commit like:
```
commit: checkpoint(ALIVE_001) BEFORE research-paper phase
```

**Expected:** Process discovery should flag this as impossible (checklist requirements not met).

**Actual:** No automated negative test harness found.

**Finding:** FAIL. No negative test evidence available.

---

## 7. Event Log: Raw OCEL Structure (Proposed)

To properly satisfy Van der Aalst conformance, the PI program should emit OCEL 2.0 event logs:

```json
{
  "ocel:global": {
    "ocel:attribute-names": [
      "phase",
      "research_domain",
      "verdict",
      "gate_criteria_met"
    ],
    "ocel:object-types": [
      "ResearchProgram",
      "ManufacturingEngine",
      "Checkpoint",
      "Artifact"
    ],
    "ocel:version": "2.0"
  },
  "ocel:objects": [
    {
      "ocel:object-id": "PI_001",
      "ocel:object-type": "ResearchProgram",
      "ocel:object-attributes": {
        "phase": "RESEARCH",
        "research_domain": "process intelligence",
        "verdict": "ALIVE_001",
        "gate_criteria_met": true
      }
    }
  ],
  "ocel:events": [
    {
      "ocel:event-id": "E-001",
      "ocel:activity": "research-init",
      "ocel:timestamp": "2026-05-31T22:42:54Z",
      "ocel:object-refs": [
        {
          "ocel:object-id": "PI_001",
          "ocel:qualifier": "created"
        }
      ]
    },
    {
      "ocel:event-id": "E-002",
      "ocel:activity": "doctrine",
      "ocel:timestamp": "2026-05-31T22:42:54Z",
      "ocel:object-refs": [
        {
          "ocel:object-id": "PI_001",
          "ocel:qualifier": "doctrine_defined"
        }
      ]
    }
  ]
}
```

**Current State:** OCEL event log not emitted. Process conformance cannot be automatically verified.

---

## 8. Gaps Identified

### GAP_P15_001: Declared Model Incompleteness

**Issue:** ggen.toml declares 1 active rule (blue-river-orchestrator) but the PI research program actually executes 9 distinct phases.

**Evidence:**
- ggen.toml: 1 active rule
- git history: 36+ commits across 9 phases
- Fitness metric: 8% (fails threshold)

**Remediation:** Update ggen.toml to enumerate all 9 research phases as explicit generation/inference rules.

**Status:** OPEN

### GAP_P15_002: Missing OTel Trace Integration

**Issue:** No OTel traces extracted from checkpoint/receipt execution. Event log source is git history, not instrumented code.

**Evidence:**
- ggen operation receipts exist but lack OTel span IDs
- No trace correlation IDs across pipeline stages
- Manual event reconstruction from commits

**Remediation:** Instrument ggen, checkpoint system, and audit execution with OTel tracing.

**Status:** OPEN

### GAP_P15_003: Missing Conformance Metrics

**Issue:** No fitness/precision/simplicity/generalization metrics computed. Phase 15 gate requires metrics ≥threshold; current state has 0% fitness.

**Evidence:**
- Fitness = 8% (declared model covers 8% of events)
- Precision = 0% (declared model explains 0% of actual behavior)
- No generalization data (n=1 instance)

**Remediation:** Compute metrics using pm4py once model is declared correctly. Re-declare ggen.toml with 9 phases.

**Status:** OPEN

### GAP_P15_004: Blue River Dam Provenance Unclear

**Issue:** blue_river_dam/src/lib.rs source has no documented receipt from ggen rule execution.

**Evidence:**
- ggen.toml declares blue-river-orchestrator rule
- No ggen receipt showing SPARQL query → Tera template → Rust output chain
- File may be hand-written or auto-generated but not audited

**Remediation:** Document whether Blue River Dam is auto-generated or hand-written. If auto-generated, emit and verify ggen receipt.

**Status:** OPEN

---

## 9. Van der Aalst Constitution Compliance

### The Principle

> "If code says it worked but event log cannot prove a lawful process happened, it did not work."

### Application to PI Program

**Code says:** ggen.toml declares a 3-step manufacturing pipeline (SPARQL → Tera → Rust)

**Event log says:** PI research program executed 9 phases across 36+ events with multi-stage research, synthesis, and checkout gates

**Verdict:** **MISMATCH.** The declared process law does not match the event log evidence. Either:
1. The declared process is incomplete (ggen.toml needs 9 rules, not 1)
2. The event log is wrong (requires OTel traces, not git history reconstruction)
3. Both (ggen.toml under-specified AND event log needs instrumentation)

### Conformance Status

| Criterion | Status | Evidence |
|-----------|--------|----------|
| Declared model matches actual events | ❌ FAIL | Fitness 8% |
| Artifacts have lawful provenance | ⚠️ PARTIAL | Most OK; Blue River Dam unclear |
| Event log is instrumented (OTel) | ❌ FAIL | No traces found |
| Quality metrics meet threshold | ❌ FAIL | Fitness 8% vs. 95% required |
| Negative tests exist | ❌ FAIL | No test harness |
| **OVERALL** | **❌ FAIL** | **Process is unproven per Van der Aalst** |

---

## 10. Remediation Roadmap

### Phase 15a: Declare Correct Manufacturing Model

**Task:** Update ggen.toml to enumerate 9 research phases as explicit rules

```toml
[[generation.rules]]
name = "research-paper-analysis"
order = 1

[[generation.rules]]
name = "research-compat-mapping"
order = 2

# ... (7 more rules)

[[generation.rules]]
name = "checkpoint-verdict"
order = 9
```

**Delivery:** ggen.toml with 9 rules, commit message: `research-ggen: declare 9-phase manufacturing model`

### Phase 15b: Instrument Event Generation

**Task:** Add OTel tracing to ggen, checkpoint, and audit systems

**Delivery:** OTel spans with trace IDs linked across pipeline stages

### Phase 15c: Compute Conformance Metrics

**Task:** Run pm4py discovery and conformance checking once model is declared

**Delivery:** fitness, precision, simplicity, generalization reports

### Phase 15d: Verify Blue River Dam Provenance

**Task:** Document whether Blue River Dam is auto-generated or hand-written

**Delivery:** Either ggen receipt or source documentation

---

## 11. Conclusion

The PI research program has produced comprehensive research evidence, documentation, and audits. However, **the declared manufacturing process (ggen.toml) does not conform to the observed event sequence per Van der Aalst process mining law.**

**Verdict: PARTIAL (not ALIVE)**

The program requires:
1. **Correct model declaration** (ggen.toml: 1 rule → 9 rules)
2. **Event instrumentation** (OTel traces for automatic log derivation)
3. **Conformance metrics** (fitness, precision, simplicity, generalization)
4. **Negative testing** (impossible traces rejected)

Once these gaps are closed, the system can legitimately claim PROCESS_INTELLIGENCE_MANUFACTURING_COMPLETE_001 or similar checkpoint.

---

## References

- **ggen.toml:** `/Users/sac/process-intelligence/ggen/ggen.toml`
- **Checkpoints:** `/Users/sac/process-intelligence/checkpoints/`
- **Receipts:** `/Users/sac/process-intelligence/receipts/`
- **Git history:** `git log --all`
- **Van der Aalst Constitution:** `~/.claude/rules/process-mining-chicago-tdd.md`

---

**Audit Authority:** Dr. Wil van der Aalst Process Mining Commission  
**Auditor:** Claude Code (Haiku 4.5)  
**Date:** 2026-06-01
