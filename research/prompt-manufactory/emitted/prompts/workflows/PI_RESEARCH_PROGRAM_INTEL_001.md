# Workflow Warrant: PI_RESEARCH_PROGRAM_INTEL_001

**Program:** PI_RESEARCH_PROGRAM_INTEL_001  
**Workflow URI:** https://pi-research.dev/workflows#INTEL_WORKFLOW  
**Authority Layer:** Research Program Law  
**Manufacturing Timestamp:** 2026-06-01T00:00:00Z  
**Manufacture Method:** Manual Warrant (Fallback from ggen v5 blocker)

---

## Workflow Identity

| Property | Value |
|----------|-------|
| **Program ID** | PI_RESEARCH_PROGRAM_INTEL_001 |
| **Canonical Name** | PI Research Program Reconciliation Workflow |
| **Description** | Full research-program reconciliation: census all projects, classify into RDF ontology, emit ggen surfaces, audit conformance, produce program map |
| **Scope** | All Referenced Projects (wasm4pm, wasm4pm-compat, ggen, Blue River, ZOEapp, OTel Weaver, Expo/Supabase, Claude Code) |
| **Prompt Class** | INTEL (Census, classify, map existing work) |
| **Authority Reference** | research-program-law.ttl § PI_RESEARCH_PROGRAM_INTEL_001 |

---

## Program Mission

```
Full research-program reconciliation: census all projects, classify into 
RDF ontology, emit ggen surfaces, audit conformance, produce program map
```

**Scope Definition:** Discovers program topology across:
- wasm4pm — Process mining core library
- wasm4pm-compat — Type law and compatibility layer
- ggen — Governance generation engine
- Blue River Dam — Autonomic orchestrator
- ZOEapp — Proof cell (Expo/Supabase)
- OTel Weaver — Telemetry registry and feedstock
- Expo/Supabase — Mobile framework extraction
- Claude Code — Dynamic workflow orchestration

---

## Authorized Workflow Stages

The workflow is authorized to progress through the following stages in order:

1. **CENSUS** - Enumerate and document all projects and artifacts
2. **CLASSIFY** - Map artifacts into RDF ontology surface
3. **EMIT_GGEN_SURFACES** - Generate ggen query, template, and manifest rules
4. **AUDIT_CONFORMANCE** - Validate program completeness and correctness
5. **PRODUCE_PROGRAM_MAP** - Generate comprehensive program documentation
6. **EMIT_VERDICT** - Issue ALIVE/PARTIAL/FAILED verdict

---

## Transition Rules

Transitions are allowed in the following sequences:

- **CENSUS** → **CLASSIFY**  
  - Condition: All projects enumerated (census completeness ≥ 95%)
  - Warrant: Census audit gate passed

- **CLASSIFY** → **EMIT_GGEN_SURFACES**  
  - Condition: All artifacts classified into ontology (classification completeness ≥ 100%)
  - Warrant: Classification audit gate passed

- **EMIT_GGEN_SURFACES** → **AUDIT_CONFORMANCE**  
  - Condition: All ggen rules validated and syntactically correct
  - Warrant: ggen manifest schema validation passed

- **AUDIT_CONFORMANCE** → **PRODUCE_PROGRAM_MAP**  
  - Condition: All audit gates passed (fitness ≥ 95%, precision ≥ 90%)
  - Warrant: Conformance audit completion certificate

- **PRODUCE_PROGRAM_MAP** → **EMIT_VERDICT**  
  - Condition: Program documentation complete and receipted
  - Warrant: Documentation completeness audit

---

## Forbidden Transitions

The following transitions are explicitly forbidden and will trigger immediate failure:

- ~~CENSUS → EMIT_GGEN_SURFACES~~ (FORBIDDEN: skips classification)  
  Reason: Unclassified artifacts cannot be safely generated

- ~~CLASSIFY → PRODUCE_PROGRAM_MAP~~ (FORBIDDEN: skips ggen and audit)  
  Reason: Unmapped surfaces and unaudited claims are not board-admissible

- ~~Any Stage → EMIT_VERDICT (unless AUDIT_CONFORMANCE passed)~~ (FORBIDDEN: verdict without evidence)  
  Reason: COVENANT violation; verdicts require proof gates

- ~~EMIT_GGEN_SURFACES → CENSUS~~ (FORBIDDEN: backward transition)  
  Reason: Irreversible commitment; cannot un-emit surfaces

---

## Artifact Lifecycle

### Artifact Type 1: Census Report
- **Proof Gate(s):** Completeness audit (all projects enumerated)
- **Receipt Required:** YES (census-receipt.json)
- **Immutability:** YES (evidence artifact)
- **Authority:** Research program checkpoint

### Artifact Type 2: Ontology Classification
- **Proof Gate(s):** Classification audit (100% of artifacts mapped)
- **Receipt Required:** YES (classification-receipt.json)
- **Immutability:** YES (evidence artifact)
- **Authority:** Research program checkpoint

### Artifact Type 3: GGEN Manifests
- **Proof Gate(s):** Manifest schema validation (ggen v5 compliant)
- **Receipt Required:** YES (manifest-receipt.yaml)
- **Immutability:** YES (code artifact)
- **Authority:** ggen synchronization record

### Artifact Type 4: Program Map
- **Proof Gate(s):** Audit completeness (all gates passed)
- **Receipt Required:** YES (program-map-receipt.md)
- **Immutability:** YES (evidence artifact)
- **Authority:** Research program final verdict

### Artifact Type 5: ALIVE/PARTIAL Verdict
- **Proof Gate(s):** All preceding gates passed; summary evidence complete
- **Receipt Required:** YES (verdict-receipt.json)
- **Immutability:** PERMANENT (foundational checkpoint)
- **Authority:** PROCESS_INTELLIGENCE_ALIVE_001 or PROCESS_INTELLIGENCE_PARTIAL_001

---

## Manufacturing Authorization

| Property | Value |
|----------|-------|
| **Authorized by** | Research Program Authority (Sean Chatman) |
| **Authority Layer** | Prompt Manufactory (research-program-law.ttl) |
| **Binding Doctrine** | Van der Aalst Constitution (Process Mining Chicago TDD) |
| **COVENANT Status** | AUTHORIZED; all claims require event log evidence |
| **Manufacturing Date** | 2026-06-01 |
| **Signature** | Process Intelligence Research Foundry |

**Binding Declaration:**
> The product is CodeManufactory; RevOps is merely proof that CodeManufactory works.

This workflow warrant authorizes the manufacture of research artifacts under the Process Intelligence Research Foundry governance model. All claims produced must satisfy the Van der Aalst Constitution: if the code says it worked but the event log cannot prove a lawful process happened, then it did not work.

---

## Warrant Proof

```
receipt_hash: sha256(PI_RESEARCH_PROGRAM_INTEL_001.md)
manufacture_chain: research-program-law.ttl → warrant-path-proof.yaml → PI_RESEARCH_PROGRAM_INTEL_001.md
proof_timestamp: 2026-06-01T00:00:00Z
manufacture_method: Fallback (ggen v5 template rendering blocked; manual warrant issued)
proof_status: PARTIAL (warrant designed; execution blocked by ggen; fallback successful)
warrant_authority: https://pi-research.dev/programs#PI_RESEARCH_PROGRAM_INTEL_001
warranty_chain: research-program-law.ttl (RDF source) → select-workflow-prompts.rq (query) → workflow-prompt.md.tera (template) → PI_RESEARCH_PROGRAM_INTEL_001.md (output) → prompt-receipt-ledger.md (receipt)
```

---

## Manufacturing Notes

**Note 1: Fallback Status**
This warrant was issued via **fallback manual manufacture** because:
- ggen v5 pipeline blocked on template rendering
- SPARQL query returned empty result set (ontology data incomplete)
- Manual warrant issued to unblock Phase 6 delivery
- Full automated manufacturing path documented for Phase 7 recovery

**Note 2: Phase 7 Recovery**
The automated warrant path (`research-program-law.ttl` → `select-workflow-prompts.rq` → `workflow-prompt.md.tera`) is fully designed and partially validated. Phase 7 must:
1. Populate missing workflow/phase ontology data, OR
2. Debug ggen v5 context binding for empty SPARQL results

**Note 3: Proof Commitment**
This warrant proves the warrant path design and authority chain, even though ggen execution was deferred. The path is manufacturable and will be automated in Phase 7.

---

**End of Warrant Document**

Manufacturing receipt: See `prompt-receipt-ledger.md`  
Authority record: `research-program-law.ttl`  
Warranty chain source: `warrant-path-proof.yaml`
