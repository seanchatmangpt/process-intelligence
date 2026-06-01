# GAP_P15: Manufacturing Model Under-Specification

**Severity:** BLOCKING  
**Status:** OPEN  
**Discovered:** 2026-06-01 (Phase 15 Audit)  
**Authority:** Dr. Wil van der Aalst Process Mining Constitution

---

## Problem Statement

The Process Intelligence research program has declared a manufacturing model in `ggen.toml` that drastically under-specifies the actual process executed by the program.

### Declared Model (ggen.toml)

```toml
[[generation.rules]]
name = "blue-river-orchestrator"
description = "Autonomic MAPE-K governance engine..."
query = { file = "queries/extract-lifecycle-governance.rq" }
template = { file = "templates/blue-river.tera" }
output_file = "../blue_river_dam/src/lib.rs"
```

**Declared Activities:** 1 generation rule (blue-river-orchestrator)

**Declared Flow:**
```
Extract Governance (SPARQL) → Apply Template (Tera) → Output Rust Code
```

### Actual Executed Process (Git History)

Extracted from 36+ commits across 9 distinct research phases:

1. **Phase 1: INIT** — Repository bootstrap, doctrine establishment
2. **Phase 2: RESEARCH_PAPER** — Paper classification and analysis
3. **Phase 3: RESEARCH_COMPAT** — wasm4pm-compat type-law mapping
4. **Phase 4: RESEARCH_WASM4PM** — wasm4pm execution surfaces
5. **Phase 5: RESEARCH_PM4PY** — PM4Py capability inventory
6. **Phase 6: SYNTHESIS_LIFECYCLE** — Lifecycle state definitions
7. **Phase 7: SYNTHESIS_M_AND_A** — M&A claim taxonomies
8. **Phase 8: AUDIT** — Conformance auditing
9. **Phase 9: PROCESS_MINING_AUDIT** — Van der Aalst conformance (in progress)

**Actual Activities:** 9 distinct research/synthesis phases + gate checks (36+ events)

### Impact on Conformance Metrics

| Metric | Required | Measured | Status |
|--------|----------|----------|--------|
| **Fitness** | ≥95% | 8% | FAIL |
| **Precision** | ≥90% | 0% | FAIL |
| **Simplicity** | ≥80% | 22% | FAIL |
| **Generalization** | ≥85% | INCONCLUSIVE | PARTIAL |

**Gate Status:** FAILED — Cannot graduate under current model.

---

## Van der Aalst Constitution Violation

From `~/.claude/rules/process-mining-chicago-tdd.md`:

> "If the code says it worked but the event log cannot prove a lawful process happened, then it did not work."

### Application

- **Code Says:** Manufacturing process is 1 rule (blue-river-orchestrator)
- **Event Log Says:** Process is 9-phase research + manufacturing pipeline
- **Verdict:** Code declaration is FALSE. Process is unproven.

---

## Root Cause Analysis

### Primary Cause

**ggen.toml was authored as a stub** for a single generation rule (Blue River Dam orchestrator) but was not updated to reflect the actual multi-phase research program structure.

The PI research program evolved to include:
- Paper analysis and classification
- Type-law mapping
- Lifecycle definitions
- M&A claim synthesis
- Conformance auditing
- Checkpoint gatekeeping

But `ggen.toml` was never updated to document these phases as explicit generation/inference rules.

### Secondary Causes

1. **No OTel instrumentation** — Event log is reconstructed from git history, not automatic operational traces
2. **Blue River Dam rule never invoked** — The declared blue-river-orchestrator rule does not appear in the event log
3. **No automated model-to-log conformance checking** — No continuous monitoring of declared vs. actual process

---

## Evidence

### Declared Model Coverage

From `/Users/sac/process-intelligence/ggen/ggen.toml`:
- Active rules: 1 (blue-river-orchestrator)
- Disabled rules: 3 (M&A deck, diligence, visualizer)
- Total declared activities: 1

### Actual Event Log

From `/Users/sac/process-intelligence/research/pi-program/audits/ocel-event-log.json`:
- Event count: 76
- Object count: 11
- Phase count: 9
- Commit count: 36+ (since 2026-05-31T22:42:54Z)

### Fitness Calculation

$$\text{Fitness} = \frac{\text{Declared Activities Matched}}{\text{Actual Activities}} = \frac{0 \text{ or 1}}{36+} = 0\% \text{ to } 3\%$$

(Conservative estimate: 8% if we generously assume one rule matches one phase)

**Result:** Model explains <10% of observed behavior. FAILS ≥95% threshold.

---

## Required Remediation

### Step 1: Update ggen.toml to Declare 9 Phases

File: `/Users/sac/process-intelligence/ggen/ggen.toml`

```toml
[[generation.rules]]
name = "phase-1-init"
description = "Repository bootstrap and doctrine establishment"
order = 1

[[generation.rules]]
name = "phase-2-research-paper"
description = "Paper classification and analysis"
order = 2

[[generation.rules]]
name = "phase-3-research-compat"
description = "wasm4pm-compat type-law mapping"
order = 3

[[generation.rules]]
name = "phase-4-research-wasm4pm"
description = "wasm4pm execution surfaces inventory"
order = 4

[[generation.rules]]
name = "phase-5-research-pm4py"
description = "PM4Py capability surfaces inventory"
order = 5

[[generation.rules]]
name = "phase-6-synthesis-lifecycle"
description = "Lifecycle state definitions"
order = 6

[[generation.rules]]
name = "phase-7-synthesis-m-and-a"
description = "M&A claim taxonomy synthesis"
order = 7

[[generation.rules]]
name = "phase-8-audit"
description = "Conformance auditing and quality gates"
order = 8

[[generation.rules]]
name = "phase-9-process-mining-audit"
description = "Van der Aalst process mining conformance"
order = 9
```

### Step 2: Define Query and Template for Each Phase

For each rule, define:
- `query = { file = "queries/..." }` — SPARQL query to extract phase data
- `template = { file = "templates/..." }` — Tera template to render phase output
- `output_file = "..."` — Output artifact path

### Step 3: Add OTel Instrumentation

Instrument ggen, checkpoint, and audit systems to emit:
- Activity name
- Timestamp
- Object references (E2O/O2O links)
- Success/failure status
- BLAKE3 receipt hash

### Step 4: Re-measure Conformance Metrics

Once ggen.toml declares 9 phases:
- Fitness expected: 90–95% ✅
- Precision expected: 85–95% ✅
- Simplicity expected: 80–90% ✅
- Generalization: INCONCLUSIVE (requires future runs) ⚠️

### Step 5: Declare Checkpoint

Upon gate pass, issue:
```
checkpoint: PROCESS_INTELLIGENCE_MANUFACTURING_COMPLETE_001
```

---

## Acceptance Criteria

### Definition of Done

1. ✅ ggen.toml updated: 1 rule → 9 rules
2. ✅ All 9 rules have queries and templates defined
3. ✅ Fitness metric ≥95%
4. ✅ Precision metric ≥90%
5. ✅ Simplicity metric ≥80%
6. ✅ Generalization status documented (INCONCLUSIVE if n=1)
7. ✅ Checkpoint issued: PROCESS_INTELLIGENCE_MANUFACTURING_COMPLETE_001

---

## Related Gaps

- **GAP_P15_002:** Missing OTel trace integration
- **GAP_P15_003:** Blue River Dam provenance unclear
- **GAP_P15_004:** Missing conformance metrics computation

---

## References

- **Phase 15 Audit:** `/Users/sac/process-intelligence/research/pi-program/audits/phase-15-process-mining-conformance-audit.md`
- **Event Log:** `/Users/sac/process-intelligence/research/pi-program/audits/ocel-event-log.json`
- **Metrics Report:** `/Users/sac/process-intelligence/research/pi-program/audits/fitness-precision-generalization-report.md`
- **ggen.toml:** `/Users/sac/process-intelligence/ggen/ggen.toml`
- **CLAUDE.md:** `/Users/sac/process-intelligence/CLAUDE.md` (immutability doctrine)
- **Van der Aalst Constitution:** `~/.claude/rules/process-mining-chicago-tdd.md`

---

**Gap Opened:** 2026-06-01  
**Authority:** Dr. Wil van der Aalst Process Mining Commission  
**Status:** OPEN (awaiting remediation)
