# RED TEAM SCOPE — Adversarial Review Charter

**Status:** Active  
**Purpose:** Define what the adversarial reviewer is authorized and expected to challenge  
**Mandate:** Every claim in this research corpus must survive adversarial scrutiny before it becomes a board artifact or a manufacturing receipt

---

## What the Adversarial Reviewer Challenges

### 1. Unsupported Claims in doctrine/

Every claim in `doctrine/` that asserts a structural property of the type system, the process-evidence covenant, or the graduation law must be backed by:
- A specific file path in `wasm4pm-compat/src/` or `wasm4pm/src/` where the implementing type or function lives
- OR an explicit marker: `[RESEARCH CLAIM — not yet manufactured]`

Challenge form: "Show me the type. Show me the module. If you cannot, mark this as a research claim."

### 2. Inflated Paper Classifications (COVERED_BY_TYPE without actual types)

The paper corpus classifies papers against a set of coverage labels. `COVERED_BY_TYPE` is the strongest claim — it asserts that a specific Rust type in the codebase structurally encodes the paper's contribution.

Challenge form: "For every `COVERED_BY_TYPE` label, name the type, name the module, and paste the relevant field or const bound. If you cannot, downgrade to `COVERED_BY_STRUCTURE` or `COVERED_BY_WITNESS`."

### 3. Board Claims Without Evidence Paths

Board-level claims (M&A diligence claims, Porter Five Forces inversions, acquisition premium claims) must trace to a manufacturing receipt, not to a research document.

Challenge form: "What is the event log path from this claim to a conformance report? If the path requires GAP_001 to be closed first, mark the claim as `[BLOCKED: GAP_001]`."

### 4. Lifecycle Phases That Stop at Observation (No Repair Actuation)

The process intelligence lifecycle includes both observation phases (discover, mine, conform) and actuation phases (repair, predict, control). Claims that assert a full lifecycle must demonstrate actuation, not just observation.

Challenge form: "Does this lifecycle phase produce a repair mandate or a control action? If it only produces a report, it is an observation phase, not a full lifecycle phase."

### 5. Comparison Claims Without Matrix Evidence

Claims that assert wasm4pm is superior to, equivalent to, or complementary with pm4py must be backed by a comparison matrix with specific algorithm-level entries.

Challenge form: "For every comparison claim, show the matrix row. If the algorithm is not yet implemented in wasm4pm (see GAP_003, GAP_004), mark the comparison entry as `[NOT YET IMPLEMENTED]`."

### 6. Crosswalk Claims Without Loss Policy Specification

Crosswalk documents that map between two process evidence formats (e.g. OCEL → XES, BPMN → Petri net) must specify:
- The loss policy in use (`RefuseLoss`, `AllowNamedProjection`, `AllowLossWithReport`)
- The named projection and its `LossReport` type
- What is lost and why it is acceptable or not

Challenge form: "What is the loss policy? What is lost? Is it named? If the crosswalk document does not answer these questions, it is incomplete."

### 7. M&A Claims Without Diligence Path

M&A documents that assert acquisition premium justification or integration risk assessment must specify:
- The specific OCEL log artifact that would be manufactured from the target company's process data
- The conformance report that would be presented in the diligence room
- The board projection that follows from the conformance report

Challenge form: "Show me the diligence path: [target OCEL log] → [conformance report] → [board projection]. If the path is hypothetical, mark it as `[RESEARCH CLAIM — diligence path not yet manufactured]`."

---

## What the Adversarial Reviewer Does NOT Challenge

- Research claims that are explicitly marked as research claims
- Gap register entries (gaps are intended findings, not defects in the research program)
- Fixture quality issues already captured in GAP_008
- The overall architecture of the type-law covenant (the ALIVE gate is the arbiter, not the adversarial reviewer)

---

## Output Format for Findings

Each finding must be filed as a numbered entry in `RED_TEAM_FINDINGS_NNN.md` with:
- Finding number
- Severity: CRITICAL / MAJOR / MINOR / EXPECTED
- Challenge category (from the 7 categories above)
- Specific document and claim challenged
- Remediation path or `[UNRESOLVABLE PRE-ALIVE]`
