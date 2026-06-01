# RED TEAM FINDINGS 001 — Initial Pass

**Status:** Active  
**Findings:** 001 through 007  
**Scope:** Per RED_TEAM_SCOPE.md, categories 1–7

---

## Finding 001 — CRITICAL — Graduation bridge is one-sided

**Category:** Board claims without evidence paths  
**Severity:** CRITICAL

`GraduationCandidate` is defined in wasm4pm-compat (`src/graduation.rs` or equivalent) but wasm4pm has no intake function that accepts it. The graduation bridge is structurally one-sided: compat can declare a candidate for graduation, but wasm4pm cannot receive it.

This means the claim "process evidence admitted through compat can be graduated to wasm4pm execution" has no manufacturing path. The evidence chain terminates at the compat boundary.

**Echoes:** GAP_001 in the gap register.

**Remediation:** Implement wasm4pm intake path for `GraduationCandidate`. Until GAP_001 is closed, all graduation claims must carry the qualifier `[BLOCKED: GAP_001]`.

**Status:** UNRESOLVABLE PRE-GAP_001-CLOSE

---

## Finding 002 — EXPECTED — M&A claims are research claims, not manufacturing receipts

**Category:** Board claims without evidence paths  
**Severity:** EXPECTED (not a defect in the research program)

The M&A documents in `ma/` define what would be board-admissible evidence if the manufacturing pipeline were complete. They do not claim to be manufacturing receipts. They are research claims correctly situated as research claims.

The distinction is:
- Research claim: "A conformance report derived from an acquisition target's OCEL log would justify a premium."
- Manufacturing receipt: The actual conformance report, derived from the actual log, with a seal.

The M&A documents are research claims. This is appropriate for the current stage of the research program.

**Remediation:** No change required. Ensure all M&A documents carry the explicit qualifier: "This is a research claim. Evidence path: [target OCEL log] → [conformance report] → [board projection]. Not yet manufactured."

**Status:** SELF-CORRECTING (research program is aware of its own stage)

---

## Finding 003 — MINOR — Standards docs describe ideal placement but some not yet in src/

**Category:** Inflated paper classifications  
**Severity:** MINOR

Standards documents in `standards/` reference module paths (e.g. `ocpq.rs`) as if the module provides execution semantics. The structural types in `wasm4pm-compat/src/ocpq.rs` exist and provide the correct structural shapes. However, the execution semantics (query evaluation) reside in wasm4pm — and OCPQ evaluation does not yet exist in wasm4pm (GAP_005).

The standards documents are not wrong: they correctly identify where the execution engine should live. The gap is in the absence of that engine.

**Remediation:** Add a note to the OCPQ standards document: "Execution engine target: wasm4pm OcpqEvaluator. Status: [BLOCKED: GAP_005]."

**Status:** MINOR — notation fix, not a structural defect

---

## Finding 004 — MAJOR — Lifecycle coverage phases 9–12 have minimal wasm4pm support

**Category:** Lifecycle phases that stop at observation  
**Severity:** MAJOR

The process intelligence lifecycle maturity model defines phases through level 5 (predictive / autonomous). Phases 9–12 (prediction, control actuation, repair recommendation, autonomous adaptation) require wasm4pm to provide streaming conformance, prediction surfaces, and control outputs.

Current wasm4pm support for these phases consists of token replay fitness only. No prediction surface, no repair actuation, no control output exists.

**Gap:** Lifecycle phases 9–12 are observation claims without actuation backing.

**Remediation:** Mark lifecycle phases 9–12 in `lifecycle/` documents as `[OBSERVATION ONLY — actuation not yet manufactured]`. Link to GAP_004 (alignment conformance) and GAP_005 (OCPQ evaluation) as prerequisites for actuation.

**Status:** OPEN — requires lifecycle document annotation

---

## Finding 005 — CRITICAL — No research document proves PM4Py algorithm count empirically

**Category:** Comparison claims without matrix evidence  
**Severity:** CRITICAL

All PM4Py capability claims in the corpus assert a number of algorithms or capabilities available in PM4Py. These claims have not been verified against the actual `pm4py/` source tree. The claims may be correct but they are asserted, not derived.

**Challenge:** For every PM4Py algorithm count claim, show the specific Python file or function in the pm4py source that implements it. If the count was derived from documentation rather than source, mark it as `[DOCUMENTATION-DERIVED — not source-verified]`.

**Remediation:** Run a source-level inventory of pm4py algorithms. Compare against the comparison matrix entries. Update any count that does not match.

**Status:** OPEN — requires pm4py source audit

---

## Finding 006 — MAJOR — bibliography.json may not match the workflow papers directory

**Category:** Unsupported claims in doctrine  
**Severity:** MAJOR

The `bibliography.json` file and the `sources/` or equivalent papers directory may have drifted. Papers referenced in the bibliography may not have corresponding fixture files; papers with fixtures may not be in the bibliography.

**Challenge:** Verify the paper corpus inventory file-by-file, not just from the bibliography. Every paper must have: a bibliography entry, a classification label, and a fixture (compile-pass or compile-fail) or an explicit `[NO FIXTURE — reason]` annotation.

**Remediation:** Run a file-level reconciliation between `bibliography.json` and the papers directory. Document any mismatches.

**Status:** OPEN — requires file-level reconciliation

---

## Finding 007 — MINOR — Evidence chain trace is design, not proof

**Category:** Board claims without evidence paths  
**Severity:** MINOR

The evidence chain trace in `experiments/EVIDENCE_CHAIN_TRACE.md` describes the intended flow from raw event data through compat admission to wasm4pm execution to conformance output. This document is a design document, not a proof of execution.

The chain works in theory. GAP_001 means it does not work in practice yet: the compat-wasm bridge does not exist, so the chain cannot be manufactured end-to-end.

**Remediation:** Add a header to `experiments/EVIDENCE_CHAIN_TRACE.md`: "Status: DESIGN DOCUMENT. This chain is not yet manufacturable end-to-end. Blocked by GAP_001."

**Status:** MINOR — notation fix; no structural defect

---

## Findings Status Summary

| Finding | Severity | Status |
|---------|----------|--------|
| 001 | CRITICAL | UNRESOLVABLE PRE-GAP_001-CLOSE |
| 002 | EXPECTED | SELF-CORRECTING |
| 003 | MINOR | Open — notation fix |
| 004 | MAJOR | Open — lifecycle annotation needed |
| 005 | CRITICAL | Open — pm4py source audit needed |
| 006 | MAJOR | Open — bibliography reconciliation needed |
| 007 | MINOR | Open — notation fix |
