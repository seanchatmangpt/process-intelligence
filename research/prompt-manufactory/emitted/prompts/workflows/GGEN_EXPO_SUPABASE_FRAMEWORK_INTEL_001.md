# Workflow Warrant: GGEN_EXPO_SUPABASE_FRAMEWORK_INTEL_001

**Program ID:** GGEN_EXPO_SUPABASE_FRAMEWORK_INTEL_001
**Mission:** Framework extraction: Expo Router law, RLS policies, Realtime contracts
**Authority Layer:** Research Program Law
**Workflow URI:** https://pi-research.dev/workflows#INTEL_WORKFLOW
**Warrant Type:** Workflow Permit
**Status:** AUTHORIZED
**Issued:** 2026-06-03
**derivedFrom:** research-program-law.ttl#GGEN_EXPO_SUPABASE_FRAMEWORK_INTEL_001

---

## Program Details

| Property | Value |
|----------|-------|
| **Program ID** | GGEN_EXPO_SUPABASE_FRAMEWORK_INTEL_001 |
| **Mission** | Framework extraction: Expo Router law, RLS policies, Realtime contracts |
| **Workflow** | https://pi-research.dev/workflows#INTEL_WORKFLOW |
| **Prompt Class** | INTEL |
| **Scope** | Expo Router, Supabase RLS, Realtime subscription contracts |
| **Goal** | Extract reusable mobile substrate from ZOEapp; publish as separate research artifact |

---

## Workflow Phases

| Phase | Label | Mission | Subagent Roles |
|-------|-------|---------|----------------|
| 1 | Phase 1: Census | Inventory Expo Router, RLS, Realtime surfaces | Framework Census (primary) |
| 2 | Phase 2: Classify | Classify extracted framework vs ZOEapp-specific code | Classification Agent |
| 3 | Phase 3: Manifest | Create framework extraction manifests | Manifest Agent |
| 4 | Phase 4: Queries | Emit SPARQL queries for framework surface selection | Query Agent |
| 5 | Phase 5: Templates | Emit Tera templates for framework extraction artifacts | Template Agent |
| 6 | Phase 6: Conformance | Verify framework extraction completeness and reusability | Audit Agent |
| 7 | Phase 7: Reconciliation | Emit framework extraction map as separate research artifact | Reconciliation Agent |
| 8 | Phase 8: Checkpoint | Emit ALIVE/PARTIAL verdict | Checkpoint Agent |

### Phase Transitions (Lawful Order)

```
Census → Classify → Manifest → Queries → Templates → Conformance → Reconciliation → Checkpoint
```

### Phase Entry Conditions

| Transition | Entry Condition |
|------------|-----------------|
| Census → Classify | Expo Router file-system layout inventoried; Supabase schema enumerated; Realtime contracts documented |
| Classify → Manifest | Framework-generic vs ZOEapp-specific code separated with justification |
| Manifest → Queries | Extraction manifests enumerate which patterns are reusable mobile substrate |
| Queries → Templates | SPARQL confirms framework patterns are app-agnostic |
| Templates → Conformance | Tera templates generate reusable framework extraction artifacts |
| Conformance → Reconciliation | Reusability audit confirms extracted patterns have no ZOEapp-specific dependencies |
| Reconciliation → Checkpoint | Framework extraction map published as standalone research artifact |
| Checkpoint → EMIT | ALIVE only if extracted framework is demonstrably app-agnostic |

---

## Framework Extraction Taxonomy

**Expo Router Law (extractable):**
- File-system routing convention (tab layout, stack layout, modal patterns)
- Deep link URL scheme patterns
- Navigation type definitions
- Auth guard patterns

**Supabase RLS Policy Patterns (extractable):**
- Row-level security policy templates
- Role-based access patterns
- JWT claim verification patterns

**Realtime Contracts (extractable):**
- Channel subscription lifecycle
- Presence tracking patterns
- Broadcast message schemas

---

## Forbidden Paths

- Extracting ZOEapp-specific business logic as "framework" (it is proof-cell code)
- Publishing extracted framework before separation from proof-cell evidence
- Declaring framework patterns as reusable without conformance audit
- Hand-coding framework taxonomy instead of rendering from templates
- Collapsing `Evidence<T, State, W>` into untyped JSON strings without boundary classification

**Authority:** `forbidden-collapse-law.ttl` — three collapse bans enforced at all lifecycle phases.

---

## Artifact Lifecycle

| Stage | Artifact | Gate | Verdict |
|-------|----------|------|---------|
| Census | Framework surface inventory | Ontology Availability | PASS/FAIL |
| Classify | Framework vs proof-cell classification | Query Syntax + Template Syntax | PASS/FAIL |
| Manifest | Framework extraction manifests | Manifest Schema Validation | PASS/FAIL |
| Queries | Framework selection SPARQL queries | SPARQL Validation | PASS/FAIL |
| Templates | Framework extraction Tera templates | Template Validation | PASS/FAIL |
| Conformance | Reusability audit | Van der Aalst Conformance | PASS/FAIL |
| Reconciliation | Framework extraction research artifact | Warrant Completeness | PASS/FAIL |
| Checkpoint | ALIVE/PARTIAL verdict | All 8 gates | ALIVE/PARTIAL |

---

## Manufacturing Authorization

- **Authority Layer:** Research Program Law
- **Warrant Type:** Workflow Permit
- **Status:** AUTHORIZED
- **Binding Doctrine:** Process Intelligence Lifecycle Law
- **Upstream Receipt:** research-program-law.ttl (instance: GGEN_EXPO_SUPABASE_FRAMEWORK_INTEL_001)
- **Manufacture Method:** Manual warrant from ontology instance (ggen SPARQL execution blocked — see audit.json)

*Warrant manufactured: 2026-06-03 | Authority: Process Intelligence Research Foundry*
