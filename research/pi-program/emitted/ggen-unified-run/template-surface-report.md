# GGen Tera Template Surface Report

**Generated:** 2026-06-01
**Location:** `/Users/sac/process-intelligence/research/pi-program/ggen/templates/`
**Output Directory:** `/Users/sac/process-intelligence/research/pi-program/emitted/ggen-unified-run/`

---

## Overview

Eleven (11) Tera template files have been created to surface the complete execution evidence from GGen unified runs. These templates render to the ggen-unified-run emitted directory and implement the Van der Aalst Constitution requirement: *event logs must prove lawful process execution*.

---

## Template Inventory

### 1. ggen-unified-run-report.md.tera

**Format:** Markdown  
**Output:** `ggen-unified-run-report.md`

Executive summary of entire unified run. Aggregates metrics from all ledgers.

**Renders:**
- Total artifacts generated
- Pipeline stage status
- Gate assessment summary
- Conformance metrics (fitness, precision, generalization, simplicity)
- Next actions and remediation requirements

**Key Variables:**
- `run.timestamp`, `run.run_id`, `run.program_name`
- `run.total_artifacts`, `run.successful_renders`, `run.failed_gates`
- `run.conformance_score`, `run.duration_seconds`
- `run.pipeline_stages`, `run.artifacts`, `run.gates`

---

### 2. project-registry.yaml.tera

**Format:** YAML  
**Output:** `project-registry.yaml`

Comprehensive project index. Maps all projects to generated artifacts.

**Renders:**
- Project ID, name, description, status
- Authority level and repository
- Artifacts per project (id, name, type, path, checksum, size)
- Conformance metrics per project
- Proof gate verdicts

**Key Variables:**
- `projects[]` (id, name, description, status, owner, repository)
- `projects[].artifacts[]` (id, name, type, output_path, checksum, size_bytes)
- `projects[].conformance` (declared_model_type, fitness, precision, generalization)
- `projects[].proof_gates[]` (name, status, verdict)

**Aggregations:**
- `projects_successful`, `projects_partial`, `projects_failed`

---

### 3. ggen-source-ledger.yaml.tera

**Format:** YAML  
**Output:** `ggen-source-ledger.yaml`

Authority and provenance chain. Every source used in generation is indexed with confidence.

**Renders:**
- Source type (paper, specification, ontology, prior_research)
- Authority level and confidence score
- Generation rule bindings
- Materialization status
- Metadata (line count, section count, reference count)

**Key Variables:**
- `sources[]` (id, type, name, authority_level, location, content_hash)
- `sources[].generation_uses[]` (rule_id, artifact_id, binding_type)
- `provenance_chain` (depth, root_source, derivation_steps, evidence_hash)

**Aggregations:**
- Source count by type
- Average confidence score across all sources

---

### 4. generation-rule-ledger.yaml.tera

**Format:** YAML  
**Output:** `generation-rule-ledger.yaml`

Catalog of all transformation rules applied. Traces rule invocations and artifacts produced.

**Renders:**
- Rule ID, name, description, category
- Input/output signatures
- Transformation logic method and complexity class
- Application statistics (invocations, success/failure counts, duration)
- Artifacts produced per rule
- Gate coverage and pass rates

**Key Variables:**
- `rules[]` (rule_id, name, description, category, source_authority, confidence_level)
- `rules[].input_signature`, `rules[].output_signature`
- `rules[].applications` (total_invocations, successful_renders, failed_renders, average_duration_ms)
- `rules[].artifacts_produced[]` (artifact_id, artifact_name, status)
- `rules[].gate_coverage[]` (gate_name, coverage_type, pass_rate)

**Aggregations:**
- Total rules and rules by category
- Cumulative success rate and duration

---

### 5. rendered-artifact-ledger.yaml.tera

**Format:** YAML  
**Output:** `rendered-artifact-ledger.yaml`

Complete inventory of all generated artifacts. The source of truth for what was produced.

**Renders:**
- Artifact ID, name, type, format
- Template and rule that generated it
- Output path, size, line count, SHA256 checksum
- Render status, gate status, conformance status, final verdict
- Validation results per artifact
- Dependency relationships
- Authority level and immutability status

**Key Variables:**
- `artifacts[]` (artifact_id, name, type, format)
- `artifacts[].generation` (template, rule_applied, source_binding, generated_at, duration_ms)
- `artifacts[].output` (path, absolute_path, size_bytes, line_count, checksum_sha256)
- `artifacts[].status` (render_status, gate_status, conformance_status, final_verdict)
- `artifacts[].validation[]` (validator, check_type, result, timestamp)
- `artifacts[].dependencies` (upstream_artifacts, downstream_artifacts)

**Aggregations:**
- Artifact count by render status
- Total output bytes and total lines generated

---

### 6. invalid-ggen-classification-ledger.md.tera

**Format:** Markdown  
**Output:** `invalid-ggen-classification-ledger.md`

Documents all invalid artifact classifications. A classification violation means declared type does not match actual rendered output.

**Renders Per Violation:**
- Artifact name and ID
- Declared vs. actual type
- Primary and secondary violations
- Content sample showing mismatch
- Impact assessment (conformance breach, gate failure, affected artifacts)
- Root cause analysis
- Remediation actions required

**Key Variables:**
- `invalid_classifications[]` (artifact_id, artifact_name, declared_type, actual_type, discovered_at)
- `invalid_classifications[].primary_violation`, `secondary_violations[]`
- `invalid_classifications[].impact` (conformance_breach, gate_failure, downstream_effects, remediation_level)
- `invalid_classifications[].root_cause` (primary, contributing_factors[])
- `invalid_classifications[].remediation_actions[]`

**Aggregations:**
- Violation type summary table
- Total invalid classifications count

---

### 7. checkpoint-ledger.md.tera

**Format:** Markdown  
**Output:** `checkpoint-ledger.md`

Immutable record of execution checkpoints. Every stage transition is recorded with state snapshot.

**Renders Per Checkpoint:**
- Sequence number and name
- Timestamp and delta from previous
- State summary (artifacts, rules, gates evaluated, conformance score)
- Pipeline stage breakdown
- Artifact status distribution
- Gate status
- Notable events
- Conformance evidence (fitness, precision, generalization, simplicity)
- Decisions made

**Key Variables:**
- `checkpoints[]` (sequence_number, name, timestamp, delta_ms, status)
- `checkpoints[].pipeline_stages[]` (name, status, processed, total, duration_ms)
- `checkpoints[].artifact_distribution` (success, partial, failed, pending)
- `checkpoints[].gates[]` (name, status, verdict, criteria_met, total_criteria)
- `checkpoints[].conformance` (fitness, precision, generalization, simplicity)

**Additional:**
- `execution_flow_diagram` — ASCII diagram of checkpoint progression
- Checkpoint integrity hashes for immutability verification

---

### 8. failed-gate-ledger.yaml.tera

**Format:** YAML  
**Output:** `failed-gate-ledger.yaml`

Ledger of all failed proof gates. Required for understanding what prevented ALIVE verdict.

**Renders Per Failed Gate:**
- Gate ID, name, type
- Associated artifact ID
- Evaluation timestamp and duration
- Verdict and severity
- Failed criteria (criterion ID, name, expected vs. actual, failure reason)
- Conformance impact (fitness/precision delta, makes_model_unsound flag)
- Dependency chain (blocking/blocked gates, propagation risk)
- Evidence (event log excerpt, model-vs-log violation, discovered variant)
- Required remediation (action type, priority, estimated effort, downstream impact)

**Key Variables:**
- `failed_gates[]` (gate_id, gate_name, gate_type, artifact_id, artifact_name)
- `failed_gates[].evaluation` (timestamp, evaluator, duration_ms)
- `failed_gates[].failed_criteria[]` (criterion_id, criterion_name, expected, actual, delta, failure_reason)
- `failed_gates[].conformance_impact` (fitness_delta, precision_delta, makes_model_unsound)
- `failed_gates[].required_remediation` (action_type, priority, estimated_effort, downstream_impact)

**Aggregations:**
- Failure count by severity
- Failure count by type
- Cumulative fitness/precision delta
- Blocking checkpoint status
- Critical path gates and estimated remediation hours

---

### 9. remediation-plan.md.tera

**Format:** Markdown  
**Output:** `remediation-plan.md`

Detailed corrective action plan. Binding specification of all work required to achieve ALIVE.

**Renders:**
- Overview (total remediations, estimated effort, critical path)
- Critical path analysis (blocking gates, impact, duration)
- Per-remediation breakdown:
  - Title, priority, status, estimated hours
  - Scope (artifacts, gates, rules affected)
  - Problem statement and root cause
  - Solution steps (file changes, validation, time estimate per step)
  - Success criteria (checkbox list)
  - Verification evidence and event log checks
  - Rollback procedure
  - Dependencies
  - Post-remediation validation tests
- Summary by priority
- Execution order diagram (Mermaid)
- Checkpoint milestones
- Sign-off section

**Key Variables:**
- `remediations[]` (title, priority, status, estimated_hours)
- `remediations[].problem_statement`, `root_cause`
- `remediations[].solution_steps[]` (description, files, validation_method, estimated_time)
- `remediations[].success_criteria[]`
- `remediations[].verification` (gate, conformance_check, event_log_check)
- `remediations[].validation_tests[]`
- `remediation_depends_on[]` — upstream dependencies
- `critical_path_gates`, `milestones`

---

### 10. warrant-path-proof.md.tera

**Format:** Markdown  
**Output:** `warrant-path-proof.md`

Proof document that execution was lawful according to declared process model. Implements Van der Aalst Constitution.

**Renders:**
- Process model declaration (diagram, type, stages, conformance boundary)
- Event log evidence (tracer, format, span/event count, OCEL transformation)
- Stage-by-stage execution proof:
  - Entry/exit conditions with evidence
  - Execution log excerpts
  - Intermediate events
  - Duration and artifacts
- Artifact lifecycle proof (birth, intermediate, validation, delivery events)
- Rule application audit (total invocations, coverage percentage)
- Conformance metrics table
- Negative testing results (impossible variant injection, rejection proof)
- Cryptographic receipt chain (root hash, depth, verification status)
- Authority lineage (sources with levels and usage)
- Derivation chain diagram
- Final verdict with warrant (fitness ≥ 85%, precision ≥ 80%, lawfulness status)

**Key Variables:**
- `declared_model_diagram`, `model_type`, `pipeline_stages`, `conformance_boundary`
- `event_log` (tracer, format, span_count, duration_seconds)
- `transformed_events`, `object_types`, `relations_discovered`
- `pipeline_stages[]` (name, status, entry_condition_met, entry_evidence, execution_log_excerpt, exit_condition_met, exit_evidence, duration_ms, artifacts_produced)
- `artifacts_with_lifecycle[]` (name, birth_event, birth_timestamp, rule_applied, intermediate_events[], validations[], delivery_event, delivery_timestamp, final_status, lifecycle_conformance)
- `rule_invocations`, `rule_invocations_with_proof`
- `impossible_tests[]` (name, variant_description, result, status)
- `receipt_chain` (root_hash, depth, all_verified)
- `sources[]` (name, type, authority_level, reference, used_in_rules, derived_artifacts)
- `conformance` (fitness, precision, generalization, simplicity with pass/fail status)

---

### 11. checkpoint.md.tera

**Format:** Markdown  
**Output:** `checkpoint-{{ checkpoint_id }}.md`

Final verdict document. Declares ALIVE, PARTIAL, or FAILED status. Immutable once issued.

**Renders:**
- Checkpoint ID, name, timestamp, run ID
- **Verdict:** ALIVE | PARTIAL | FAILED
- Metadata table (checkpoint ID, phase, execution duration, artifact count, gate count, conformance score)
- Verdict-specific details:
  - **ALIVE:** All criteria met, production ready, next actions, release authorized
  - **PARTIAL:** Failing gates listed, remediation count, estimated effort, blocker status, next actions
  - **FAILED:** Critical defects listed, recovery path, architecture review required
- Pipeline stages table
- Proof gates evaluated table
- Conformance evidence metrics
- Artifact distribution summary
- Reference table to all generated ledgers
- Authorization section (approved/reviewed by, timestamp, signature/chain)
- Immutability declaration

**Key Variables:**
- `checkpoint_name`, `checkpoint_id`, `checkpoint_timestamp`, `run_id`, `program_name`
- `verdict` (ALIVE | PARTIAL | FAILED)
- `phase`, `execution_duration_seconds`, `total_artifacts`, `gates_evaluated`, `conformance_score`
- `pipeline_stages[]` (name, status, duration_ms, artifacts_count)
- `proof_gates[]` (name, type, result, criteria_met, total_criteria)
- `conformance` (fitness, precision, generalization, simplicity)
- `artifact_counts` (success, partial, failed, total)
- For PARTIAL: `failing_gates[]` (gate_name, reason), `remediation_count`, `estimated_remediation_hours`, `blocker_status`
- For FAILED: `critical_defects[]` (name, description, impact, recovery), `previous_checkpoint_valid`
- `approved_by`, `authorization_timestamp`, `checkpoint_signature`

---

## Template Architecture

### Rendering Pipeline

```
GGen Unified Run
    ↓
[Execution Trace / OCEL Log]
    ↓
[Template Variable Extraction]
    ↓
[Tera Template Rendering]
    ↓
emitted/ggen-unified-run/
    ├── ggen-unified-run-report.md
    ├── project-registry.yaml
    ├── rendered-artifact-ledger.yaml
    ├── generation-rule-ledger.yaml
    ├── ggen-source-ledger.yaml
    ├── checkpoint-ledger.md
    ├── failed-gate-ledger.yaml
    ├── invalid-ggen-classification-ledger.md
    ├── remediation-plan.md
    ├── warrant-path-proof.md
    └── checkpoint-<id>.md
```

### Immutability

- All templates render to `emitted/ggen-unified-run/` (NOT to source directories)
- Checkpoints are immutable once rendered
- Ledgers accumulate evidence — never delete prior runs
- ALIVE verdicts cannot be revoked (only new checkpoints can be issued)

### Data Flow

Each template declares its input contract in the Tera context:

| Template | Primary Data Source | Secondary Sources |
|----------|---------------------|-------------------|
| ggen-unified-run-report.md | run metadata | All ledgers |
| project-registry.yaml | projects[] | All artifacts, gates |
| ggen-source-ledger.yaml | sources[], provenance_chain | Generation rule usage |
| generation-rule-ledger.yaml | rules[] | Artifacts produced, gates covered |
| rendered-artifact-ledger.yaml | artifacts[] | Validation events, dependencies |
| invalid-ggen-classification-ledger.md | invalid_classifications[] | Root cause analysis, remediation |
| checkpoint-ledger.md | checkpoints[] | Pipeline stages, gates, events |
| failed-gate-ledger.yaml | failed_gates[] | Conformance impact, dependencies |
| remediation-plan.md | remediations[] | Critical path, milestones |
| warrant-path-proof.md | Model + event log | All sources, rules, lifecycles |
| checkpoint.md | checkpoint metadata | All ledgers (summary) |

---

## Van der Aalst Constitution Alignment

These templates implement the doctrine: *"If the code says it worked but the event log cannot prove a lawful process happened, then it did not work."*

**Key Implementation:**

1. **warrant-path-proof.md** — Proves execution is lawful
   - Declares model-vs-log conformance
   - Proves artifact lifecycles are valid
   - Rejects impossible variants
   - Signs proof with receipt chain

2. **checkpoint-ledger.md** — Records all execution steps
   - Every stage transition is timestamped
   - Conformance metrics at each point
   - Decisions documented
   - Integrity hashes for immutability

3. **failed-gate-ledger.yaml** — Documents all non-conformances
   - Model-vs-log violations
   - Missing/unexpected execution patterns
   - Prevents blind acceptance of flawed processes

4. **checkpoint.md** — Declares final verdict
   - ALIVE only if event log proves lawfulness
   - PARTIAL if gates fail
   - FAILED if process violated

---

## Output Directory

All template renders go to:

```
/Users/sac/process-intelligence/research/pi-program/emitted/ggen-unified-run/
```

This directory is the authority for all ggen-unified run evidence. It is NOT a source directory and should never be committed to doctrine/ or sources/.

---

## Integration Points

- **GGen System:** Query execution traces → extract variables → render templates
- **OTel Tracer:** Convert spans to OCEL event log → bind to `event_log` variable
- **Process Miner:** Compute fitness/precision/generalization → populate conformance
- **Receipt Chain:** Hash each artifact → bind to `receipt_chain` variable
- **Checkpoint System:** Issue verdict → render checkpoint.md with immutable proof

---

## Next Steps

1. **Implement GGen Variable Extractor** — Parse execution artifacts and generate Tera context
2. **Bind OCEL Log** — Transform OTel traces to OCEL format for warrant proof
3. **Compute Conformance** — Run pm4py discovery/conformance to populate metrics
4. **Execute Negative Tests** — Inject impossible variants, verify rejection
5. **Issue First Checkpoint** — Render all templates and declare verdict (ALIVE/PARTIAL/FAILED)

---

**Template Surface Report Generated:** 2026-06-01  
**Templates Location:** `/Users/sac/process-intelligence/research/pi-program/ggen/templates/`  
**Output Location:** `/Users/sac/process-intelligence/research/pi-program/emitted/ggen-unified-run/`
