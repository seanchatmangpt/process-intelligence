# SPARQL Query Surface Report

**Generated:** 2026-06-01  
**Program:** ggen-unified-run  
**Authority:** research/pi-program/ggen/queries/  
**Total Queries:** 20 (12 selection + 8 audit)

## Overview

This report documents the complete SPARQL query surface for the ggen program's unified run execution and validation pipeline. All queries are constructed to support:

- **Selection Queries (12)**: Enumerate artifacts, rules, gates, checkpoints, and provenance
- **Audit Queries (8)**: Assert invariants required for ALIVE/PARTIAL verdict gates

## Selection Queries (12)

### 1. select-all-ggen-manifests.rq
**Purpose:** Enumerate all registered ggen manifest documents  
**Returns:** Manifest URIs with metadata (label, created, modified, status)  
**Use Case:** Global manifest inventory and version tracking  
**Lines:** 16

### 2. select-all-generation-rules.rq
**Purpose:** Enumerate all generation rule definitions  
**Returns:** Rule URIs with inputs, outputs, conditions, and rule types  
**Use Case:** Rule graph construction and dependency analysis  
**Lines:** 16

### 3. select-all-ontology-graphs.rq
**Purpose:** Enumerate all ontology definitions with namespace bindings  
**Returns:** Ontology URIs with namespace, version, and label  
**Use Case:** Semantic context resolution and type hierarchy inspection  
**Lines:** 15

### 4. select-all-query-surfaces.rq
**Purpose:** Enumerate all SPARQL query definitions  
**Returns:** Query surface URIs with purpose, pattern, result format  
**Use Case:** Query registry and capability self-documentation  
**Lines:** 16

### 5. select-all-template-surfaces.rq
**Purpose:** Enumerate all template definitions for artifact manufacturing  
**Returns:** Template URIs with engine, type, and parameter bindings  
**Use Case:** Template registry and render capability auditing  
**Lines:** 15

### 6. select-all-rendered-artifacts.rq
**Purpose:** Enumerate all manufactured artifacts with source tracing  
**Returns:** Artifact URIs with type, source rule, status, receipt binding  
**Use Case:** Artifact inventory, receipt chain validation, status tracking  
**Lines:** 18

### 7. select-all-checkpoints.rq
**Purpose:** Enumerate all lifecycle checkpoint verdicts (ALIVE/PARTIAL/FAILED)  
**Returns:** Checkpoint URIs with verdict, timestamp, criteria, evidence bindings  
**Use Case:** Milestone tracking, gate history, verdict audit trail  
**Lines:** 17

### 8. select-failed-gates.rq
**Purpose:** Enumerate all proof gates that failed validation  
**Returns:** Gate URIs with artifact, failure reason, failed assertion, evaluation timestamp  
**Use Case:** Failure analysis, remediation candidate identification  
**Lines:** 17

### 9. select-remediation-candidates.rq
**Purpose:** Identify artifacts and rules requiring remediation  
**Returns:** Candidate URIs (artifact or rule) with failed gate, remedy path, priority  
**Use Case:** Failure response planning, priority-ordered remediation queue  
**Lines:** 29

### 10. select-invalid-ggen-files.rq
**Purpose:** Enumerate all ggen configuration files with errors  
**Returns:** File URIs with error type, message, location, severity  
**Use Case:** Configuration audit, error classification, severity triage  
**Lines:** 16

### 11. select-unified-run-plan.rq
**Purpose:** Return the complete execution plan with rule ordering and dependencies  
**Returns:** Step URIs with precedence, dependencies, parallelization hints, duration  
**Use Case:** Execution scheduling, parallelization analysis, duration estimation  
**Lines:** 17

### 12. select-warrant-paths.rq
**Purpose:** Return provenance chains from checkpoints through receipts to evidence  
**Returns:** Paths from checkpoint → artifact → receipt → source rule → evidence  
**Use Case:** Warrant validation, ground-truth verification, causality tracing  
**Lines:** 16

---

## Audit Queries (8)

All audit queries are constructed as `ASK` queries asserting invariants required for valid verdicts. A return value of `true` indicates a **violation**.

### 1. audit-no-invalid-new-ggen-source.rq
**Invariant:** All newly created ggen files pass validation gates before registration  
**Failure Pattern:** New file exists with errors and no validation timestamp  
**Verdict Impact:** Blocks ALIVE transition if violated  
**Lines:** 13

### 2. audit-all-legacy-ggen-classified.rq
**Invariant:** Every ggen file from prior checkpoints has been reviewed and classified  
**Failure Pattern:** File exists without reviewedAt timestamp or classification  
**Verdict Impact:** Blocks ALIVE transition if violated  
**Lines:** 14

### 3. audit-no-file-count-alive.rq
**Invariant:** ALIVE verdicts are NOT based solely on artifact count thresholds  
**Failure Pattern:** ALIVE checkpoint with only file-count-threshold criteria  
**Verdict Impact:** Demotes verdict to PARTIAL if violated  
**Lines:** 16

### 4. audit-no-forced-alive.rq
**Invariant:** ALIVE checkpoints require all gate criteria to be met with evidence  
**Failure Pattern:** ALIVE checkpoint without validated criteria and evidence  
**Verdict Impact:** Demotes verdict to PARTIAL if violated  
**Lines:** 14

### 5. audit-every-generation-rule-has-query-template-output.rq
**Invariant:** All generation rules have associated query surfaces and template surfaces  
**Failure Pattern:** Rule exists without output query or outputTemplate binding  
**Verdict Impact:** Blocks rule execution if violated  
**Lines:** 16

### 6. audit-every-rendered-artifact-has-source-trace.rq
**Invariant:** All manufactured artifacts can be traced to their source rules and queries  
**Failure Pattern:** Artifact exists without sourceRule or sourceQuery binding  
**Verdict Impact:** Fails artifact proof gate if violated  
**Lines:** 16

### 7. audit-checkpoints-have-receipts-or-explicit-missing.rq
**Invariant:** ALIVE/PARTIAL checkpoints have cryptographic receipts or documented gaps  
**Failure Pattern:** ALIVE or PARTIAL checkpoint without receipt and without declaredGaps  
**Verdict Impact:** Blocks verdict issuance if violated  
**Lines:** 18

### 8. audit-warrant-path-exists.rq
**Invariant:** Every ALIVE verdict has a complete provenance chain from checkpoint to evidence  
**Failure Pattern:** ALIVE checkpoint without complete receipt trace chain  
**Verdict Impact:** Revokes ALIVE status if violated  
**Lines:** 15

---

## Query Organization

### Directory Structure
```
research/pi-program/ggen/queries/
├── select-all-ggen-manifests.rq
├── select-all-generation-rules.rq
├── select-all-ontology-graphs.rq
├── select-all-query-surfaces.rq
├── select-all-template-surfaces.rq
├── select-all-rendered-artifacts.rq
├── select-all-checkpoints.rq
├── select-failed-gates.rq
├── select-remediation-candidates.rq
├── select-invalid-ggen-files.rq
├── select-unified-run-plan.rq
├── select-warrant-paths.rq
├── audit-no-invalid-new-ggen-source.rq
├── audit-all-legacy-ggen-classified.rq
├── audit-no-file-count-alive.rq
├── audit-no-forced-alive.rq
├── audit-every-generation-rule-has-query-template-output.rq
├── audit-every-rendered-artifact-has-source-trace.rq
├── audit-checkpoints-have-receipts-or-explicit-missing.rq
└── audit-warrant-path-exists.rq
```

## Execution Model

### Selection Query Pipeline
1. Execute all 12 selection queries to populate the artifact graph
2. Order results by precedence/timestamp for chronological verification
3. Cross-reference artifact URIs across queries for completeness

### Audit Query Pipeline
1. Execute all 8 audit queries sequentially
2. Each query returns `true` if an invariant is violated
3. Any violation blocks advancement to the next checkpoint verdict
4. Violations must be logged with reference to specific failed query results

### Unified Run Plan Execution
1. Query `select-unified-run-plan.rq` to retrieve execution order
2. Group steps by `precedence` for dependency resolution
3. Mark steps as parallelizable if `?parallelizable = true`
4. Execute in precedence order, respecting `?dependsOn` edges

### Warrant Path Validation
1. Query `select-warrant-paths.rq` for each ALIVE checkpoint
2. Verify complete chain: checkpoint → artifact → receipt → rule → evidence
3. Validate receipt cryptographic signature
4. Confirm source evidence exists and is traceable to doctrine

---

## Semantic Prefixes

All queries use these standard prefixes:

| Prefix | Namespace |
|--------|-----------|
| `ggen` | `http://purl.org/ggen/core#` |
| `rdfs` | `http://www.w3.org/2000/01/rdf-schema#` |
| `owl` | `http://www.w3.org/2002/07/owl#` |
| `dc` | `http://purl.org/dc/elements/1.1/` |

---

## Query Execution Notes

- **SPARQL Compliance:** All queries conform to SPARQL 1.1 specification
- **Result Format:** Selection queries return SPARQL result sets; Audit queries return boolean
- **Performance:** Selection queries are optimized for enumeration; Audit queries are optimized for falsification
- **Timeout:** Recommend 30-second timeout for large graph queries
- **Graph Format:** Queries assume NTriples format for RDF store serialization

---

## Validation Gates

The audit query surface is the authority for checkpoint verdict advancement:

- **PARTIAL → ALIVE:** Requires all 8 audit queries to return `false` (no violations)
- **ANY STATE → FAILED:** Any audit query returns `true` triggers failure analysis
- **Forced Audit:** Re-run audit surface after remediation before verdict re-issuance

---

**Authority:** Process Intelligence Research Foundry  
**Last Updated:** 2026-06-01  
**Status:** EMISSION COMPLETE
