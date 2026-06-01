# Template Validation Report

**Timestamp:** 2026-06-01T13:22:14.181415

## Summary

| Status | Count |
|--------|-------|
| CONTEXT_MISSING | 23 |
| RENDER_PASS | 1 |

## Status Definitions

- **PARSE_PASS**: Tera syntax is valid
- **RENDER_PASS**: Valid syntax and all variables in sample context
- **CONTEXT_MISSING**: Valid syntax but missing variables (acceptable if not in active rules)
- **PARSE_FAIL**: Invalid Tera syntax (must be fixed)
- **RENDER_FAIL**: Valid syntax but render error
- **OUT_OF_SCOPE**: Could not validate (e.g., file not readable)

## Acceptable: Context Missing

Templates with valid syntax but missing variables in sample context:

*(These are acceptable unless the template is actively used in generation rules)*

### research/pi-program/ggen/templates/checkpoint.md.tera
**Missing Variables (2):**
  - `critical_defect`
  - `failing_gate`

### research/pi-program/ggen/templates/failed-gate-ledger.yaml.tera
**Missing Variables (9):**
  - `blocking_checkpoint`
  - `criterion`
  - `critical_path_gates`
  - `cumulative_fitness_delta`
  - `cumulative_precision_delta`
  - `failed_gates`
  - `gate_failure`
  - `gate_type_summary`
  - `gates_blocking_release`

### research/pi-program/ggen/templates/generation-rule-ledger.yaml.tera
**Missing Variables (6):**
  - `artifact`
  - `category`
  - `cumulative_duration_seconds`
  - `cumulative_success_rate`
  - `rule`
  - `rules`

### research/pi-program/ggen/templates/ggen-source-ledger.yaml.tera
**Missing Variables (4):**
  - `provenance_chain`
  - `source`
  - `sources`
  - `use`

### research/pi-program/ggen/templates/ggen-unified-run-report.md.tera
**Missing Variables (3):**
  - `action`
  - `artifact`
  - `failure`

### research/pi-program/ggen/templates/invalid-ggen-classification-ledger.md.tera
**Missing Variables (6):**
  - `action`
  - `classification`
  - `factor`
  - `invalid_classifications`
  - `summary`
  - `violation`

### research/pi-program/ggen/templates/pi-program-walkthrough.md.tera
**Missing Variables (19):**
  - `claim_cat`
  - `claims`
  - `doctrine_item`
  - `doctrines`
  - `experiment`
  - `gap`
  - `gaps`
  - `lifecycle_state`
  - `lifecycle_states`
  - `now`
  - ... and 9 more

### research/pi-program/ggen/templates/project-registry.yaml.tera
**Missing Variables (3):**
  - `artifact`
  - `project`
  - `projects`

### research/pi-program/ggen/templates/remediation-plan.md.tera
**Missing Variables (15):**
  - `criterion`
  - `critical_path_duration`
  - `critical_path_gates`
  - `current_state`
  - `dep`
  - `factor`
  - `milestone`
  - `prepared_by`
  - `remediation`
  - `remediations`
  - ... and 5 more

### research/pi-program/ggen/templates/rendered-artifact-ledger.yaml.tera
**Missing Variables (3):**
  - `artifact`
  - `artifacts`
  - `validation`

### research/pi-program/ggen/templates/warrant-path-proof.md.tera
**Missing Variables (29):**
  - `all_artifacts_valid`
  - `all_stages_lawful`
  - `artifact`
  - `chain_status`
  - `chain_verified`
  - `conformance_boundary`
  - `declared_model_diagram`
  - `derivation_chain_diagram`
  - `event_log`
  - `fitness_status`
  - ... and 19 more

### research/prompt-manufactory/ggen/templates/checkpoint-prompt.md.tera
**Missing Variables (21):**
  - `alive_consequence`
  - `authority_layer`
  - `authority_ref`
  - `authorized_by`
  - `binding_doctrine`
  - `checkpoint_uri`
  - `condition`
  - `covenant_status`
  - `criterion`
  - `evidence`
  - ... and 11 more

### research/prompt-manufactory/ggen/templates/hook-policy.md.tera
**Missing Variables (23):**
  - `action`
  - `activation_rule`
  - `authority_layer`
  - `authority_ref`
  - `authorized_by`
  - `binding_doctrine`
  - `covenant_status`
  - `escalation`
  - `event_pattern`
  - `forbidden`
  - ... and 13 more

### research/prompt-manufactory/ggen/templates/invalid-ggen-classification-ledger.md.tera
**Missing Variables (20):**
  - `authority_layer`
  - `binding_doctrine`
  - `classified_count`
  - `covenant_compliance`
  - `deprecated_count`
  - `file`
  - `ggen_file`
  - `immutability_chain`
  - `manufacture_chain`
  - `manufacture_timestamp`
  - ... and 10 more

### research/prompt-manufactory/ggen/templates/prompt-receipt.md.tera
**Missing Variables (26):**
  - `artifact`
  - `authority_layer`
  - `authorized_by`
  - `binding_doctrine`
  - `covenant_binding`
  - `error`
  - `failed_renders`
  - `failure`
  - `final_manufacture_chain`
  - `final_receipt_hash`
  - ... and 16 more

### research/prompt-manufactory/ggen/templates/research-program-index.md.tera
**Missing Variables (23):**
  - `conformance_status`
  - `layer`
  - `manufacture_chain`
  - `manufacture_timestamp`
  - `manufacturing_root`
  - `mfg_timestamp`
  - `ontology_version`
  - `program`
  - `program_ref`
  - `proof_ledger`
  - ... and 13 more

### research/prompt-manufactory/ggen/templates/skill.md.tera
**Missing Variables (24):**
  - `authority_layer`
  - `authority_ref`
  - `authorized_by`
  - `binding_doctrine`
  - `boundary`
  - `capability`
  - `covenant_status`
  - `knowledge`
  - `manufacture_chain`
  - `metric`
  - ... and 14 more

### research/prompt-manufactory/ggen/templates/subagent-prompt.md.tera
**Missing Variables (18):**
  - `authority`
  - `authority_layer`
  - `authority_ref`
  - `authorized_by`
  - `binding_doctrine`
  - `covenant_status`
  - `escalation`
  - `forbidden`
  - `manufacture_chain`
  - `mfg_timestamp`
  - ... and 8 more

### research/prompt-manufactory/ggen/templates/workflow-prompt.md.tera
**Missing Variables (17):**
  - `artifact_type`
  - `authority_layer`
  - `authority_ref`
  - `authorized_by`
  - `binding_doctrine`
  - `covenant_status`
  - `forbidden`
  - `manufacture_chain`
  - `mfg_timestamp`
  - `proof_timestamp`
  - ... and 7 more

### ggen/templates/blue-river.tera
**Missing Variables (8):**
  - `action`
  - `analyze`
  - `knowledge`
  - `monitor`
  - `now`
  - `plan`
  - `state`
  - `transition`

### ggen/templates/ma-deck.tera
**Missing Variables (8):**
  - `claim`
  - `claims`
  - `conformance_summary`
  - `debt_items`
  - `debt_slide`
  - `signoff_slide`
  - `synergy_items`
  - `synergy_waterfall`

### ggen/templates/ma-diligence.tera
**Missing Variables (1):**
  - `claims`

### ggen/templates/visualizer-dashboard.tsx.tera
**Missing Variables (3):**
  - `claim`
  - `height`
  - `transition`

## Success: Fully Validated

- research/pi-program/ggen/templates/checkpoint-ledger.md.tera


## All Results

| Template | Status | Details |
|----------|--------|---------|
| research/pi-program/ggen/templates/checkpoint-ledger.md.tera | RENDER_PASS |  |
| research/pi-program/ggen/templates/checkpoint.md.tera | CONTEXT_MISSING | 2 missing vars |
| research/pi-program/ggen/templates/failed-gate-ledger.yaml.tera | CONTEXT_MISSING | 9 missing vars |
| research/pi-program/ggen/templates/generation-rule-ledger.yaml.tera | CONTEXT_MISSING | 6 missing vars |
| research/pi-program/ggen/templates/ggen-source-ledger.yaml.tera | CONTEXT_MISSING | 4 missing vars |
| research/pi-program/ggen/templates/ggen-unified-run-report.md.tera | CONTEXT_MISSING | 3 missing vars |
| research/pi-program/ggen/templates/invalid-ggen-classification-ledger.md.tera | CONTEXT_MISSING | 6 missing vars |
| research/pi-program/ggen/templates/pi-program-walkthrough.md.tera | CONTEXT_MISSING | 19 missing vars |
| research/pi-program/ggen/templates/project-registry.yaml.tera | CONTEXT_MISSING | 3 missing vars |
| research/pi-program/ggen/templates/remediation-plan.md.tera | CONTEXT_MISSING | 15 missing vars |
| research/pi-program/ggen/templates/rendered-artifact-ledger.yaml.tera | CONTEXT_MISSING | 3 missing vars |
| research/pi-program/ggen/templates/warrant-path-proof.md.tera | CONTEXT_MISSING | 29 missing vars |
| research/prompt-manufactory/ggen/templates/checkpoint-prompt.md.tera | CONTEXT_MISSING | 21 missing vars |
| research/prompt-manufactory/ggen/templates/hook-policy.md.tera | CONTEXT_MISSING | 23 missing vars |
| research/prompt-manufactory/ggen/templates/invalid-ggen-classification-ledger.md.tera | CONTEXT_MISSING | 20 missing vars |
| research/prompt-manufactory/ggen/templates/prompt-receipt.md.tera | CONTEXT_MISSING | 26 missing vars |
| research/prompt-manufactory/ggen/templates/research-program-index.md.tera | CONTEXT_MISSING | 23 missing vars |
| research/prompt-manufactory/ggen/templates/skill.md.tera | CONTEXT_MISSING | 24 missing vars |
| research/prompt-manufactory/ggen/templates/subagent-prompt.md.tera | CONTEXT_MISSING | 18 missing vars |
| research/prompt-manufactory/ggen/templates/workflow-prompt.md.tera | CONTEXT_MISSING | 17 missing vars |
| ggen/templates/blue-river.tera | CONTEXT_MISSING | 8 missing vars |
| ggen/templates/ma-deck.tera | CONTEXT_MISSING | 8 missing vars |
| ggen/templates/ma-diligence.tera | CONTEXT_MISSING | 1 missing vars |
| ggen/templates/visualizer-dashboard.tsx.tera | CONTEXT_MISSING | 3 missing vars |