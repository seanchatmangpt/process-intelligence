# Workflow Warrant: GGEN_OTEL_WEAVER_PI_INTEL_001

**Program ID:** GGEN_OTEL_WEAVER_PI_INTEL_001
**Mission:** OTel Weaver research: telemetry feedstock, registry model, witness mappings (NOT evidence)
**Authority Layer:** Research Program Law
**Workflow URI:** https://pi-research.dev/workflows#INTEL_WORKFLOW
**Warrant Type:** Workflow Permit
**Status:** AUTHORIZED
**Issued:** 2026-06-03
**derivedFrom:** research-program-law.ttl#GGEN_OTEL_WEAVER_PI_INTEL_001

---

## Program Details

| Property | Value |
|----------|-------|
| **Program ID** | GGEN_OTEL_WEAVER_PI_INTEL_001 |
| **Mission** | OTel Weaver research: telemetry feedstock, registry model, witness mappings (NOT evidence) |
| **Workflow** | https://pi-research.dev/workflows#INTEL_WORKFLOW |
| **Prompt Class** | INTEL |
| **Scope** | OTel Weaver registry model, telemetry schemas, witness boundary classification |
| **Critical Constraint** | OTel Weaver artifacts are TELEMETRY FEEDSTOCK — they are NOT process-mining evidence |

---

## Workflow Phases

| Phase | Label | Mission | Subagent Roles |
|-------|-------|---------|----------------|
| 1 | Phase 1: Census | Comprehensive inventory of all OTel Weaver surfaces | Feedstock Census (primary), Orchestration Census (secondary) |
| 2 | Phase 2: Classify | Classify each Weaver artifact as feedstock vs evidence | Classification Agent |
| 3 | Phase 3: Manifest | Create feedstock projection manifests | Manifest Agent |
| 4 | Phase 4: Queries | Emit SPARQL queries for feedstock classification | Query Agent |
| 5 | Phase 5: Templates | Emit Tera templates for feedstock boundary artifacts | Template Agent |
| 6 | Phase 6: Conformance | Verify feedstock/evidence boundary integrity | Audit Agent |
| 7 | Phase 7: Reconciliation | Emit feedstock boundary map | Reconciliation Agent |
| 8 | Phase 8: Checkpoint | Emit ALIVE/PARTIAL verdict | Checkpoint Agent |

### Phase Transitions (Lawful Order)

```
Census → Classify → Manifest → Queries → Templates → Conformance → Reconciliation → Checkpoint
```

### Phase Entry Conditions

| Transition | Entry Condition |
|------------|-----------------|
| Census → Classify | Weaver registry model inventoried; all semantic convention schemas enumerated |
| Classify → Manifest | EVERY Weaver artifact classified as FEEDSTOCK or EVIDENCE_CANDIDATE with justification |
| Manifest → Queries | No EVIDENCE_CANDIDATE without explicit doctrine authorization |
| Queries → Templates | SPARQL confirms feedstock/evidence boundary is exhaustive |
| Templates → Conformance | Templates enforce the feedstock-only classification rule |
| Conformance → Reconciliation | Van der Aalst audit: zero boundary violations detected |
| Reconciliation → Checkpoint | Witness boundary map complete with all Weaver artifacts classified |
| Checkpoint → EMIT | ALIVE only if feedstock/evidence boundary is 100% certified |

---

## Critical Boundary Rule

**OTel Weaver artifacts ARE:**
- Telemetry schema definitions (feedstock for PI evidence derivation)
- Semantic convention registries (feedstock)
- Signal type definitions (feedstock)
- Span attribute schemas (feedstock)

**OTel Weaver artifacts are NOT:**
- Process-mining evidence (evidence requires event log derivation)
- OCEL objects (those require object-centric log transformation)
- Conformance witnesses (those require pm4py replay)
- ALIVE proof gates (those require Van der Aalst audit)

---

## Forbidden Paths

- Classifying any OTel Weaver artifact as process-mining evidence without event log derivation
- Treating OTel spans as OCEL events without XES/OCEL transformation
- Declaring conformance based on OTel trace data alone (traces ≠ conformance)
- Emitting evidence claims without pm4py replay backing
- Hand-coding feedstock boundary mappings instead of rendering from templates
- Collapsing `Evidence<T, State, W>` into untyped JSON strings without boundary classification

**Authority:** `forbidden-collapse-law.ttl` — three collapse bans enforced at all lifecycle phases.

---

## Artifact Lifecycle

| Stage | Artifact | Gate | Verdict |
|-------|----------|------|---------|
| Census | Weaver surface inventory | Ontology Availability | PASS/FAIL |
| Classify | Feedstock classification map | Query Syntax + Template Syntax | PASS/FAIL |
| Manifest | Feedstock projection manifests | Manifest Schema Validation | PASS/FAIL |
| Queries | Feedstock SPARQL queries | SPARQL Validation | PASS/FAIL |
| Templates | Feedstock boundary Tera templates | Template Validation | PASS/FAIL |
| Conformance | Boundary integrity audit | Van der Aalst Conformance | PASS/FAIL |
| Reconciliation | Weaver witness boundary map | Warrant Completeness | PASS/FAIL |
| Checkpoint | ALIVE/PARTIAL verdict | All 8 gates | ALIVE/PARTIAL |

---

## Manufacturing Authorization

- **Authority Layer:** Research Program Law
- **Warrant Type:** Workflow Permit
- **Status:** AUTHORIZED
- **Binding Doctrine:** Process Intelligence Lifecycle Law
- **Upstream Receipt:** research-program-law.ttl (instance: GGEN_OTEL_WEAVER_PI_INTEL_001)
- **Manufacture Method:** Manual warrant from ontology instance (ggen SPARQL execution blocked — see audit.json)

*Warrant manufactured: 2026-06-03 | Authority: Process Intelligence Research Foundry*
