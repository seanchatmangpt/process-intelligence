# Workflow Warrant: GGEN_ECOSYSTEM_INTEL_001

**Program ID:** GGEN_ECOSYSTEM_INTEL_001
**Mission:** Gather ecosystem intelligence for wasm4pm-compat projections; Van der Aalst conformance-driven ggen manufacturing
**Authority Layer:** Research Program Law
**Workflow URI:** https://pi-research.dev/workflows#INTEL_WORKFLOW
**Warrant Type:** Workflow Permit
**Status:** AUTHORIZED
**Issued:** 2026-06-03
**derivedFrom:** research-program-law.ttl#GGEN_ECOSYSTEM_INTEL_001

---

## Program Details

| Property | Value |
|----------|-------|
| **Program ID** | GGEN_ECOSYSTEM_INTEL_001 |
| **Mission** | Gather ecosystem intelligence for wasm4pm-compat projections; Van der Aalst conformance-driven ggen manufacturing |
| **Workflow** | https://pi-research.dev/workflows#INTEL_WORKFLOW |
| **Prompt Class** | INTEL |
| **Scope** | Cargo, rustdoc, Specta, tsify, wasm-bindgen, WIT, Component Model |

---

## Workflow Phases

| Phase | Label | Mission | Subagent Roles |
|-------|-------|---------|----------------|
| 1 | Phase 1: Census | Comprehensive inventory of all surfaces in referenced projects | Engine Census, Compat Census, ggen Census, Proof Cell Census, Feedstock Census, Framework Census, Orchestration Census |
| 2 | Phase 2: Classify | Map each surface to program ontology categories | Classification Agent |
| 3 | Phase 3: Manifest | Create projection manifests and program declarations | Manifest Agent |
| 4 | Phase 4: Queries | Emit SPARQL selection and audit queries | Query Agent |
| 5 | Phase 5: Templates | Emit Tera templates for artifact rendering | Template Agent |
| 6 | Phase 6: Conformance | Van der Aalst audit: verify program structure against declared law | Audit Agent |
| 7 | Phase 7: Reconciliation | Emit unified program map outputs | Reconciliation Agent |
| 8 | Phase 8: Checkpoint | Emit ALIVE/PARTIAL verdict | Checkpoint Agent |

### Phase Transitions (Lawful Order)

```
Census → Classify → Manifest → Queries → Templates → Conformance → Reconciliation → Checkpoint
```

Each phase gate must PASS before the next phase begins. No phase skipping permitted.

### Phase Entry Conditions

| Transition | Entry Condition |
|------------|-----------------|
| Census → Classify | Ecosystem census covers Cargo.toml, Specta, tsify, wasm-bindgen, WIT surfaces |
| Classify → Manifest | All type-binding surfaces classified against wasm4pm-compat type law |
| Manifest → Queries | Projection manifests enumerate all compat boundary crossings |
| Queries → Templates | SPARQL queries validate type compatibility for each compat rule |
| Templates → Conformance | Tera templates generate compat witness artifacts |
| Conformance → Reconciliation | Van der Aalst audit: compat witness lattice fitness measured |
| Reconciliation → Checkpoint | Ecosystem map emitted with ggen rule coverage assessment |
| Checkpoint → EMIT | ALIVE/PARTIAL based on: type coverage, compat fitness, ggen rules valid |

---

## Subagent Role Assignments

### Phase 1 Roles (same INTEL topology)

| Role | Owned Surface | Forbidden Surface | Output Contract |
|------|--------------|-------------------|-----------------|
| wasm4pm Engine Census | src/mining, src/replay, src/conformance | ../wasm4pm-compat/, ../process-intelligence/ | research/pi-program/intel/wasm4pm-engine-census.md |
| wasm4pm-compat Census | src/, Cargo.toml, ggen/ | ../wasm4pm/ | research/pi-program/intel/wasm4pm-compat-census.md |
| ggen Census | ggen/ | .ggen files as source surfaces | research/pi-program/intel/ggen-census.md |
| ZOEapp Proof Cell Census | src/ | other repositories | research/pi-program/intel/zoeapp-census.md |
| OTel Weaver Feedstock Census | otel-weaver/ | other repositories | research/pi-program/intel/otel-weaver-census.md |
| Expo/Supabase Framework Census | expo-supabase-framework/ | app implementation details | research/pi-program/intel/expo-supabase-framework-census.md |
| Claude Code Workflow Census | workflow artifacts | other repositories | research/pi-program/intel/claude-workflow-census.md |

---

## Forbidden Paths

- Declaring ALIVE when any compat type boundary violation remains unresolved
- Treating Cargo.toml type declarations as conformant without boundary validation
- Classifying wasm-bindgen types as process-mining evidence (they are feedstock, not evidence)
- Emitting compat witness lattice entries without SPARQL validation
- Hand-coding compat bridge types instead of rendering from ggen templates
- Collapsing `Evidence<T, State, W>` into untyped JSON strings without boundary classification

**Authority:** `forbidden-collapse-law.ttl` — three collapse bans enforced at all lifecycle phases.

---

## Artifact Lifecycle

| Stage | Artifact | Gate | Verdict |
|-------|----------|------|---------|
| Census | Ecosystem census report | Ontology Availability | PASS/FAIL |
| Classify | Type-law classification map | Query Syntax + Template Syntax | PASS/FAIL |
| Manifest | Compat projection manifests | Manifest Schema Validation | PASS/FAIL |
| Queries | Type-compatibility SPARQL queries | SPARQL Validation | PASS/FAIL |
| Templates | Compat witness Tera templates | Template Validation | PASS/FAIL |
| Conformance | Van der Aalst compat audit | Conformance Fitness | PASS/FAIL |
| Reconciliation | Ecosystem map with ggen rules | Warrant Completeness | PASS/FAIL |
| Checkpoint | ALIVE/PARTIAL verdict | All 8 gates | ALIVE/PARTIAL |

---

## Manufacturing Authorization

- **Authority Layer:** Research Program Law
- **Warrant Type:** Workflow Permit
- **Status:** AUTHORIZED
- **Binding Doctrine:** Process Intelligence Lifecycle Law
- **Upstream Receipt:** research-program-law.ttl (instance: GGEN_ECOSYSTEM_INTEL_001)
- **Manufacture Method:** Manual warrant from ontology instance (ggen SPARQL execution blocked — see audit.json)

*Warrant manufactured: 2026-06-03 | Authority: Process Intelligence Research Foundry*
