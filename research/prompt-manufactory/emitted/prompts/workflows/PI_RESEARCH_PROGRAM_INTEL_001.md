# Workflow Warrant: PI_RESEARCH_PROGRAM_INTEL_001

**Program ID:** PI_RESEARCH_PROGRAM_INTEL_001
**Mission:** Full research-program reconciliation: census all projects, classify into RDF ontology, emit ggen surfaces, audit conformance, produce program map
**Authority Layer:** Research Program Law
**Workflow URI:** https://pi-research.dev/workflows#INTEL_WORKFLOW
**Warrant Type:** Workflow Permit
**Status:** AUTHORIZED
**Issued:** 2026-06-03
**derivedFrom:** research-program-law.ttl#PI_RESEARCH_PROGRAM_INTEL_001

---

## Program Details

| Property | Value |
|----------|-------|
| **Program ID** | PI_RESEARCH_PROGRAM_INTEL_001 |
| **Mission** | Full research-program reconciliation: census all projects, classify into RDF ontology, emit ggen surfaces, audit conformance, produce program map |
| **Workflow** | https://pi-research.dev/workflows#INTEL_WORKFLOW |
| **Prompt Class** | INTEL |
| **Scope** | wasm4pm, wasm4pm-compat, ggen, Blue River Dam, ZOEapp, OTel Weaver, Expo/Supabase, Claude Code |

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
| Census → Classify | Census report emitted to research/pi-program/intel/ for each surface |
| Classify → Manifest | All surfaces mapped to ontology categories; zero unmapped surfaces |
| Manifest → Queries | projection manifests emitted; manifest schema validation PASS |
| Queries → Templates | SPARQL .rq files valid and return non-empty result sets |
| Templates → Conformance | Tera .tera files syntactically valid and render without error |
| Conformance → Reconciliation | Van der Aalst audit completed; fitness/precision/generalization reported |
| Reconciliation → Checkpoint | Unified program map emitted; all outputs present |
| Checkpoint → EMIT | All 8 gates evaluated; ALIVE or PARTIAL verdict justified |

---

## Subagent Role Assignments

### Phase 1 Roles

| Role | Owned Surface | Forbidden Surface | Output Contract |
|------|--------------|-------------------|-----------------|
| wasm4pm Engine Census | src/mining, src/replay, src/conformance, src/ocpq, src/receipt, tests/ | ../wasm4pm-compat/, ../process-intelligence/ | research/pi-program/intel/wasm4pm-engine-census.md |
| wasm4pm-compat Census | src/, Cargo.toml, ggen/ | ../wasm4pm/ | research/pi-program/intel/wasm4pm-compat-census.md |
| ggen Census | ggen/ | .ggen files as source surfaces | research/pi-program/intel/ggen-census.md |
| ZOEapp Proof Cell Census | src/ | other repositories | research/pi-program/intel/zoeapp-census.md |
| OTel Weaver Feedstock Census | otel-weaver/ | other repositories | research/pi-program/intel/otel-weaver-census.md |
| Expo/Supabase Framework Census | expo-supabase-framework/ | app implementation details | research/pi-program/intel/expo-supabase-framework-census.md |
| Claude Code Workflow Census | workflow artifacts | other repositories | research/pi-program/intel/claude-workflow-census.md |

### Phase 2–8 Roles

| Phase | Role | Output Contract |
|-------|------|-----------------|
| 2 | Classification Agent | research/pi-program/intel/classification-map.md |
| 3 | Manifest Agent | research/pi-program/intel/projection-manifest.yaml |
| 4 | Query Agent | research/pi-program/queries/*.rq |
| 5 | Template Agent | research/pi-program/templates/*.tera |
| 6 | Audit Agent | research/pi-program/conformance/audit-report.md |
| 7 | Reconciliation Agent | research/pi-program/intel/program-map.md |
| 8 | Checkpoint Agent | checkpoints/PI_RESEARCH_PROGRAM_ALIVE_001.md |

---

## Forbidden Paths

The following execution paths are absolutely forbidden under this warrant:

- Declaring ALIVE verdict when any required audit gate has failed
- Emitting prompts without a `pm:derivedFrom` provenance triple
- Introducing `.ggen` extension files as source surfaces
- Collapsing `Evidence<T, State, W>` into untyped JSON strings without boundary classification
- Skipping phases or bypassing subagent role assignments
- Hand-coding program artifacts instead of rendering from templates
- Claiming conformance without running Van der Aalst audit (Phase 6)
- Emitting Checkpoint verdict before Reconciliation output exists

**Authority:** `forbidden-collapse-law.ttl` — three collapse bans enforced at all lifecycle phases.

---

## Artifact Lifecycle

Artifacts manufactured under this warrant follow the lawful lifecycle:

| Stage | Artifact | Gate | Verdict |
|-------|----------|------|---------|
| Census | Census report | Ontology Availability | PASS/FAIL |
| Classify | TTL ontology instances | Query Syntax + Template Syntax | PASS/FAIL |
| Manifest | Projection manifests | Manifest Schema Validation | PASS/FAIL |
| Queries | SPARQL `.rq` files | SPARQL Validation | PASS/FAIL |
| Templates | Tera `.tera` files | Template Validation | PASS/FAIL |
| Conformance | Audit result files | Van der Aalst Conformance | PASS/FAIL |
| Reconciliation | Unified program map | Warrant Completeness | PASS/FAIL |
| Checkpoint | ALIVE/PARTIAL verdict | All 8 gates | ALIVE/PARTIAL |

**Lifecycle Rule:** No artifact may advance to the next stage unless its gate verdict is PASS.
**Fallback Rule:** PARTIAL is the lawful verdict when any gate is FAIL. No forced ALIVE.

---

## Manufacturing Authorization

- **Authority Layer:** Research Program Law
- **Warrant Type:** Workflow Permit
- **Status:** AUTHORIZED
- **Binding Doctrine:** Process Intelligence Lifecycle Law
- **Upstream Receipt:** research-program-law.ttl (instance: PI_RESEARCH_PROGRAM_INTEL_001)
- **Manufacture Method:** Manual warrant from ontology instance (ggen SPARQL execution blocked — see audit.json)

*Warrant manufactured: 2026-06-03 | Authority: Process Intelligence Research Foundry*
