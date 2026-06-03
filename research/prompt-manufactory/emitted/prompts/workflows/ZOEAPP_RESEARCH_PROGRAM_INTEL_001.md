# Workflow Warrant: ZOEAPP_RESEARCH_PROGRAM_INTEL_001

**Program ID:** ZOEAPP_RESEARCH_PROGRAM_INTEL_001
**Mission:** ZOEapp proof cell: Expo, Supabase, livestream, on-device inference, Jest receipts
**Authority Layer:** Research Program Law
**Workflow URI:** https://pi-research.dev/workflows#INTEL_WORKFLOW
**Warrant Type:** Workflow Permit
**Status:** AUTHORIZED
**Issued:** 2026-06-03
**derivedFrom:** research-program-law.ttl#ZOEAPP_RESEARCH_PROGRAM_INTEL_001

---

## Program Details

| Property | Value |
|----------|-------|
| **Program ID** | ZOEAPP_RESEARCH_PROGRAM_INTEL_001 |
| **Mission** | ZOEapp proof cell: Expo, Supabase, livestream, on-device inference, Jest receipts |
| **Workflow** | https://pi-research.dev/workflows#INTEL_WORKFLOW |
| **Prompt Class** | INTEL |
| **Scope** | Expo Router, Supabase, on-device inference, Jest test receipts |
| **Classification** | PROOF CELL — ZOEapp proves that CodeManufactory works in a mobile context |

---

## Workflow Phases

| Phase | Label | Mission | Subagent Roles |
|-------|-------|---------|----------------|
| 1 | Phase 1: Census | Inventory ZOEapp surfaces: Expo, Supabase, inference, Jest | Proof Cell Census (primary) |
| 2 | Phase 2: Classify | Classify each surface as proof-cell evidence vs framework extraction | Classification Agent |
| 3 | Phase 3: Manifest | Create proof-cell projection manifests | Manifest Agent |
| 4 | Phase 4: Queries | Emit SPARQL queries for proof-cell classification | Query Agent |
| 5 | Phase 5: Templates | Emit Tera templates for proof-cell artifacts | Template Agent |
| 6 | Phase 6: Conformance | Van der Aalst audit: verify proof-cell event streams | Audit Agent |
| 7 | Phase 7: Reconciliation | Emit proof-cell map with Jest receipt coverage | Reconciliation Agent |
| 8 | Phase 8: Checkpoint | Emit ALIVE/PARTIAL verdict | Checkpoint Agent |

### Phase Transitions (Lawful Order)

```
Census → Classify → Manifest → Queries → Templates → Conformance → Reconciliation → Checkpoint
```

### Phase Entry Conditions

| Transition | Entry Condition |
|------------|-----------------|
| Census → Classify | All ZOEapp surfaces inventoried: Expo Router files, Supabase schema, Jest test files |
| Classify → Manifest | Each surface classified as: proof-cell evidence OR framework extraction candidate |
| Manifest → Queries | Projection manifests enumerate event stream boundaries for on-device inference |
| Queries → Templates | SPARQL validates proof-cell receipts (Jest output → OCEL event log) |
| Templates → Conformance | Templates render proof-cell event log from Jest receipts |
| Conformance → Reconciliation | Van der Aalst audit confirms Jest receipts constitute lawful process evidence |
| Reconciliation → Checkpoint | Proof-cell map confirms ZOEapp as CodeManufactory proof case |
| Checkpoint → EMIT | ALIVE only if Jest receipts constitute verifiable process evidence |

---

## Proof Cell Classification Rules

**ZOEapp surfaces classified as PROOF CELL EVIDENCE:**
- Jest test results (receipted test passes prove manufacturing works)
- Expo Router navigation events (constitute process events)
- Supabase row-level security policy executions (constitute lifecycle events)
- On-device inference invocations (constitute AI operator events)

**ZOEapp surfaces classified as FRAMEWORK EXTRACTION:**
- Expo Router file layout (→ GGEN_EXPO_SUPABASE_FRAMEWORK_INTEL_001)
- Supabase RLS policy schemas (→ GGEN_EXPO_SUPABASE_FRAMEWORK_INTEL_001)
- Realtime subscription contracts (→ GGEN_EXPO_SUPABASE_FRAMEWORK_INTEL_001)

---

## Forbidden Paths

- Treating ZOEapp as a production product rather than a proof cell
- Publishing ZOEapp implementation details as CodeManufactory documentation
- Claiming ZOEapp event streams as OCEL-conformant without transformation proof
- Declaring proof-cell ALIVE before Jest receipts are verified as process evidence
- Collapsing `Evidence<T, State, W>` into untyped JSON strings without boundary classification
- Hand-coding proof-cell classification instead of rendering from templates

**Authority:** `forbidden-collapse-law.ttl` — three collapse bans enforced at all lifecycle phases.

---

## Artifact Lifecycle

| Stage | Artifact | Gate | Verdict |
|-------|----------|------|---------|
| Census | ZOEapp surface inventory | Ontology Availability | PASS/FAIL |
| Classify | Proof-cell vs framework classification | Query Syntax + Template Syntax | PASS/FAIL |
| Manifest | Proof-cell projection manifests | Manifest Schema Validation | PASS/FAIL |
| Queries | Proof-cell SPARQL queries | SPARQL Validation | PASS/FAIL |
| Templates | Proof-cell Tera templates | Template Validation | PASS/FAIL |
| Conformance | Jest receipt → process evidence audit | Van der Aalst Conformance | PASS/FAIL |
| Reconciliation | Proof-cell map | Warrant Completeness | PASS/FAIL |
| Checkpoint | ALIVE/PARTIAL verdict | All 8 gates | ALIVE/PARTIAL |

---

## Manufacturing Authorization

- **Authority Layer:** Research Program Law
- **Warrant Type:** Workflow Permit
- **Status:** AUTHORIZED
- **Binding Doctrine:** Process Intelligence Lifecycle Law
- **Upstream Receipt:** research-program-law.ttl (instance: ZOEAPP_RESEARCH_PROGRAM_INTEL_001)
- **Manufacture Method:** Manual warrant from ontology instance (ggen SPARQL execution blocked — see audit.json)

*Warrant manufactured: 2026-06-03 | Authority: Process Intelligence Research Foundry*
