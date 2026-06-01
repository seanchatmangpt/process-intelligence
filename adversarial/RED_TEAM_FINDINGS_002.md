# RED TEAM FINDINGS 002 — Extended Pass

**Status:** Active  
**Findings:** 008 through 010  
**Scope:** Per RED_TEAM_SCOPE.md, categories 1–7 (continued)

---

## Finding 008 — MAJOR — Reverse Porter Five Forces makes strategic claims without case study evidence

**Category:** Board claims without evidence paths  
**Severity:** MAJOR

`ma/REVERSE_PORTER_FIVE.md` (or equivalent) makes strategic claims about how process intelligence inverts the Porter Five Forces framework. For example: "Supplier power is reduced because process mining reveals inefficiencies the supplier cannot hide." These are strategic claims, not manufacturing receipts.

**Challenge:** Every inversion claim asserts that a specific process intelligence artifact (a conformance report, a discovery output, a fitness metric) changes the power dynamic in the claimed direction. No case study evidence, no manufactured artifact, and no event log trace backs these assertions.

**Remediation:** Add an explicit qualifier to each inversion claim:

> "This is a research claim. Evidence path: [company X OCEL log] → [conformance report] → [diligence presentation]. Not yet manufactured."

This qualifier must appear adjacent to each strategic claim, not just in a document header. The document can remain as a research artifact; the qualifier distinguishes it from a manufactured board artifact.

**Status:** OPEN — document annotation required

---

## Finding 009 — MINOR — MAPE-K map assumes wasm4pm has streaming conformance

**Category:** Lifecycle phases that stop at observation  
**Severity:** MINOR

`lifecycle/MAPE_K_MAP.md` maps the MAPE-K autonomic control loop (Monitor, Analyze, Plan, Execute) to process intelligence capabilities. The Monitor phase references streaming conformance as the sensing mechanism.

**Check:** Does `wasm4pm-compat/src/streaming.rs` exist and provide `EventStream`?

**Result:** `/Users/sac/wasm4pm-compat/src/streaming.rs` EXISTS.

`EventStream` is available in wasm4pm-compat as an append-only buffer (per CLAUDE.md: `EventStream` append-only buffer in `basic_eventlog` example). This provides the structural sensing surface for the Monitor phase.

**However:** The Analyze phase requires conformance checking. wasm4pm currently provides token replay only (token replay fitness). Alignment-based conformance (GAP_004) is not yet available. The MAPE-K Analyze phase is therefore limited to token replay fitness until GAP_004 is closed.

**Remediation:** Annotate the MAPE-K Analyze phase in `lifecycle/MAPE_K_MAP.md`:

> "Current wasm4pm support: token replay fitness only. Alignment-based conformance (superior method) blocked by GAP_004."

`streaming.rs` existence is noted — the Monitor phase has structural backing. The gap is in the Analyze phase, not the Monitor phase.

**Status:** MINOR — notation fix; streaming.rs confirmed present

---

## Finding 010 — MAJOR — No single document ties all 8 gaps to a specific downstream workflow authorization sequence

**Category:** Board claims without evidence paths  
**Severity:** MAJOR

The gap register (GAP_REGISTER.md) and priority matrix (GAP_PRIORITY_MATRIX.md) describe the gaps in full. However, no single document ties the 8 gaps to a specific ordered sequence of downstream workflow authorizations that a program manager or workflow orchestrator would follow.

`prompts/DOWNSTREAM_WASM4PM_REFACTOR.md` exists but may not list the gaps in priority order or make the authorization sequence explicit.

**Challenge:** A downstream workflow agent receiving the refactor mandate needs to know: (1) which gap to close first, (2) what completion looks like, (3) what becomes unblocked after each closure.

**Remediation:** Add the gap sequence to `prompts/DOWNSTREAM_WASM4PM_REFACTOR.md` in explicit priority order:

1. GAP_007 — fix WfNet split-brain (independent, low effort)
2. GAP_001 — import compat, thread Admitted types (critical path unlock)
3. GAP_002 — replace ValidationError(String) with named Refusal types
4. GAP_003, GAP_004, GAP_008 — parallel execution after step 2
5. GAP_005, GAP_006 — parallel execution after step 3

Each step must specify the completion criterion: what type, function, or fixture change constitutes closure.

**Status:** OPEN — `DOWNSTREAM_WASM4PM_REFACTOR.md` update required

---

## Findings Status Summary (Findings 008–010)

| Finding | Severity | Status |
|---------|----------|--------|
| 008 | MAJOR | Open — document annotation required |
| 009 | MINOR | Open — notation fix; streaming.rs confirmed present |
| 010 | MAJOR | Open — DOWNSTREAM_WASM4PM_REFACTOR.md update required |

---

## All Findings Status Summary (001–010)

| Finding | Severity | Status |
|---------|----------|--------|
| 001 | CRITICAL | UNRESOLVABLE PRE-GAP_001-CLOSE |
| 002 | EXPECTED | SELF-CORRECTING |
| 003 | MINOR | Open — notation fix |
| 004 | MAJOR | Open — lifecycle annotation needed |
| 005 | CRITICAL | Open — pm4py source audit needed |
| 006 | MAJOR | Open — bibliography reconciliation needed |
| 007 | MINOR | Open — notation fix |
| 008 | MAJOR | Open — document annotation required |
| 009 | MINOR | Open — notation fix; streaming.rs confirmed present |
| 010 | MAJOR | Open — DOWNSTREAM_WASM4PM_REFACTOR.md update required |

**3 CRITICAL, 3 MAJOR (open), 2 MAJOR (pre-ALIVE unresolvable or self-correcting), 3 MINOR**

Critical findings 001 and 005 are unresolvable until downstream workflows execute (GAP_001 closure and pm4py source audit respectively). All remaining findings are research program self-corrections, appropriately noted and actionable.
