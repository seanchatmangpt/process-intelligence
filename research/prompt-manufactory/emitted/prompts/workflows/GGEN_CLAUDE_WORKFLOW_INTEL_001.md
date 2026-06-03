# Workflow Warrant: GGEN_CLAUDE_WORKFLOW_INTEL_001

**Program ID:** GGEN_CLAUDE_WORKFLOW_INTEL_001
**Mission:** Claude Code dynamic workflow orchestration: phases, subagent topology, audit gates, checkpoints
**Authority Layer:** Research Program Law
**Workflow URI:** https://pi-research.dev/workflows#INTEL_WORKFLOW
**Warrant Type:** Workflow Permit
**Status:** AUTHORIZED
**Issued:** 2026-06-03
**derivedFrom:** research-program-law.ttl#GGEN_CLAUDE_WORKFLOW_INTEL_001

---

## Program Details

| Property | Value |
|----------|-------|
| **Program ID** | GGEN_CLAUDE_WORKFLOW_INTEL_001 |
| **Mission** | Claude Code dynamic workflow orchestration: phases, subagent topology, audit gates, checkpoints |
| **Workflow** | https://pi-research.dev/workflows#INTEL_WORKFLOW |
| **Prompt Class** | INTEL |
| **Scope** | Claude Code orchestration substrate, subagent coordination, audit gate patterns |

---

## Workflow Phases

| Phase | Label | Mission | Subagent Roles |
|-------|-------|---------|----------------|
| 1 | Phase 1: Census | Inventory Claude Code workflow artifacts: phases, subagent specs, audit gates | Orchestration Census (primary) |
| 2 | Phase 2: Classify | Classify orchestration patterns into orchestration ontology categories | Classification Agent |
| 3 | Phase 3: Manifest | Create orchestration projection manifests | Manifest Agent |
| 4 | Phase 4: Queries | Emit SPARQL queries for orchestration surface selection | Query Agent |
| 5 | Phase 5: Templates | Emit Tera templates for orchestration artifacts | Template Agent |
| 6 | Phase 6: Conformance | Verify subagent coordination patterns against declared law | Audit Agent |
| 7 | Phase 7: Reconciliation | Emit orchestration pattern map | Reconciliation Agent |
| 8 | Phase 8: Checkpoint | Emit ALIVE/PARTIAL verdict | Checkpoint Agent |

### Phase Transitions (Lawful Order)

```
Census → Classify → Manifest → Queries → Templates → Conformance → Reconciliation → Checkpoint
```

### Phase Entry Conditions

| Transition | Entry Condition |
|------------|-----------------|
| Census → Classify | All Claude Code workflow documents inventoried; phase structures enumerated |
| Classify → Manifest | Each workflow pattern classified against subagent-role-law.ttl taxonomy |
| Manifest → Queries | Orchestration manifests cover all TaskCreate/TaskUpdate coordination patterns |
| Queries → Templates | SPARQL confirms orchestration patterns are law-grounded |
| Templates → Conformance | Tera templates generate orchestration artifacts with law citations |
| Conformance → Reconciliation | Van der Aalst audit: orchestration event log matches declared phase topology |
| Reconciliation → Checkpoint | Orchestration pattern map complete with all coordination primitives classified |
| Checkpoint → EMIT | ALIVE only if orchestration patterns are fully backed by ontology law |

---

## Orchestration Pattern Taxonomy

**Lawful Claude Code Orchestration Patterns:**
- Phase-gated subagent dispatch (each phase has a declared subagent role)
- Audit-gate verdict collection (each gate returns PASS/FAIL/PARTIAL)
- Receipt-chain propagation (each subagent output receipted before next dispatch)
- Checkpoint emission (ALIVE/PARTIAL verdict with gate evidence)

**Forbidden Orchestration Patterns:**
- Undeclared phase execution (subagent without a registered role)
- Gate-skipping (advancing phase without PASS verdict)
- Receipt fabrication (emitting PASS without computational evidence)
- Forced ALIVE (declaring ALIVE when any gate is FAIL)

---

## Forbidden Paths

- Classifying undeclared orchestration patterns as lawful
- Treating Claude Code subagent output as process evidence without receipting
- Hand-coding orchestration topology instead of rendering from templates
- Emitting ALIVE checkpoint before orchestration audit is complete
- Collapsing `Evidence<T, State, W>` into untyped JSON strings without boundary classification
- Documenting coordination patterns without law citations

**Authority:** `forbidden-collapse-law.ttl` — three collapse bans enforced at all lifecycle phases.

---

## Artifact Lifecycle

| Stage | Artifact | Gate | Verdict |
|-------|----------|------|---------|
| Census | Orchestration surface inventory | Ontology Availability | PASS/FAIL |
| Classify | Orchestration pattern classification | Query Syntax + Template Syntax | PASS/FAIL |
| Manifest | Orchestration projection manifests | Manifest Schema Validation | PASS/FAIL |
| Queries | Orchestration SPARQL queries | SPARQL Validation | PASS/FAIL |
| Templates | Orchestration Tera templates | Template Validation | PASS/FAIL |
| Conformance | Orchestration pattern audit | Van der Aalst Conformance | PASS/FAIL |
| Reconciliation | Orchestration pattern map | Warrant Completeness | PASS/FAIL |
| Checkpoint | ALIVE/PARTIAL verdict | All 8 gates | ALIVE/PARTIAL |

---

## Manufacturing Authorization

- **Authority Layer:** Research Program Law
- **Warrant Type:** Workflow Permit
- **Status:** AUTHORIZED
- **Binding Doctrine:** Process Intelligence Lifecycle Law
- **Upstream Receipt:** research-program-law.ttl (instance: GGEN_CLAUDE_WORKFLOW_INTEL_001)
- **Manufacture Method:** Manual warrant from ontology instance (ggen SPARQL execution blocked — see audit.json)

*Warrant manufactured: 2026-06-03 | Authority: Process Intelligence Research Foundry*
