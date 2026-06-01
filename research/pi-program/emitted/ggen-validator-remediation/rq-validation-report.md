# SPARQL Query Validation Report
**Generated:** 2026-06-01T13:39:40.927849
**Total Queries:** 67

## Summary
- **ERROR:** 0
- **OK:** 58
- **PARSE_ERROR:** 9

## Active Generation Queries (Referenced by ggen.toml)

### ggen/queries/extract-lifecycle-governance.rq
**Status:** OK 🟢
**Query Type:** SELECT
**Lines:** 119
**Features:** FILTER, OPTIONAL, ORDER BY

### research/prompt-manufactory/ggen/queries/select-checkpoint-prompts.rq
**Status:** OK 🟢
**Query Type:** SELECT
**Lines:** 18
**Features:** ORDER BY

### research/prompt-manufactory/ggen/queries/select-hook-policies.rq
**Status:** OK 🟢
**Query Type:** SELECT
**Lines:** 18
**Features:** OPTIONAL, ORDER BY

### research/prompt-manufactory/ggen/queries/select-legacy-ggen-files.rq
**Status:** OK 🟢
**Query Type:** SELECT
**Lines:** 15
**Features:** ORDER BY

### research/prompt-manufactory/ggen/queries/select-rendered-prompts.rq
**Status:** OK 🟢
**Query Type:** SELECT
**Lines:** 17
**Features:** ORDER BY

### research/prompt-manufactory/ggen/queries/select-research-programs.rq
**Status:** OK 🟢
**Query Type:** SELECT
**Lines:** 16
**Select Variables:** `mission, program, programId, promptClass`
**Features:** ORDER BY

### research/prompt-manufactory/ggen/queries/select-skill-prompts.rq
**Status:** OK 🟢
**Query Type:** SELECT
**Lines:** 18
**Features:** OPTIONAL, ORDER BY

### research/prompt-manufactory/ggen/queries/select-subagent-prompts.rq
**Status:** OK 🟢
**Query Type:** SELECT
**Lines:** 19
**Features:** OPTIONAL, ORDER BY

### research/prompt-manufactory/ggen/queries/select-workflow-prompts.rq
**Status:** OK 🟢
**Query Type:** SELECT
**Lines:** 29
**Features:** OPTIONAL, ORDER BY

## Blocking Queries

- research/pi-program/ggen/queries/audit-checkpoints-have-receipts-or-explicit-missing.rq
- research/pi-program/ggen/queries/audit-every-generation-rule-has-query-template-output.rq
- research/pi-program/ggen/queries/audit-every-rendered-artifact-has-source-trace.rq
- research/pi-program/ggen/queries/audit-no-commit-count-alive.rq
- research/pi-program/ggen/queries/audit-no-forced-alive.rq
- research/pi-program/ggen/queries/audit-no-hand-written-research-warrant.rq
- research/pi-program/ggen/queries/audit-no-invalid-new-ggen-source.rq
- research/pi-program/ggen/queries/audit-no-manual-prompt-writing.rq
- research/pi-program/ggen/queries/audit-partial-checkpoint-possible.rq

## All Queries by Type

### ASK Queries (3)

- `research/pi-program/ggen/queries/audit-validator-passes-valid-tera.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-validator-rejects-invalid-tera.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-warrant-path-exists.rq` — OK 🟢

### ERROR Queries (9)

- `research/pi-program/ggen/queries/audit-checkpoints-have-receipts-or-explicit-missing.rq` — PARSE_ERROR 🔴
- `research/pi-program/ggen/queries/audit-every-generation-rule-has-query-template-output.rq` — PARSE_ERROR 🔴
- `research/pi-program/ggen/queries/audit-every-rendered-artifact-has-source-trace.rq` — PARSE_ERROR 🔴
- `research/pi-program/ggen/queries/audit-no-commit-count-alive.rq` — PARSE_ERROR 🔴
- `research/pi-program/ggen/queries/audit-no-forced-alive.rq` — PARSE_ERROR 🔴
- `research/pi-program/ggen/queries/audit-no-hand-written-research-warrant.rq` — PARSE_ERROR 🔴
- `research/pi-program/ggen/queries/audit-no-invalid-new-ggen-source.rq` — PARSE_ERROR 🔴
- `research/pi-program/ggen/queries/audit-no-manual-prompt-writing.rq` — PARSE_ERROR 🔴
- `research/pi-program/ggen/queries/audit-partial-checkpoint-possible.rq` — PARSE_ERROR 🔴

### SELECT Queries (55)

- `ggen/queries/extract-board-claims.rq` — OK 🟢
- `ggen/queries/extract-diligence-claims.rq` — OK 🟢
- `ggen/queries/extract-lifecycle-governance.rq` — OK 🟢
- `ggen/queries/extract-visualizer-data.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-all-legacy-ggen-classified.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-checkpoint-has-receipts.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-closure-invariant.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-commitment-integrity.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-compliance-ledger.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-evidence-traceability.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-gates-complete.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-no-client-only-auth.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-no-dashboard-truth.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-no-dto-flattening.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-no-file-count-alive.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-no-invalid-ggen-extension.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-no-realtime-as-evidence.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-no-telemetry-as-receipt.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-no-tool-smuggling.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-no-unsigned-verdicts.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-partial-has-gaps.rq` — OK 🟢
- `research/pi-program/ggen/queries/audit-source-court-citations.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-alive-claims.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-all-checkpoints.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-all-generation-rules.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-all-ggen-manifests.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-all-ontology-graphs.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-all-projects.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-all-query-surfaces.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-all-rendered-artifacts.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-all-template-surfaces.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-checkpoints.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-compatibility-surfaces.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-engine-surfaces.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-failed-gates.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-forbidden-collapses.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-invalid-ggen-files.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-manufacturing-surfaces.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-mobile-substrate-surfaces.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-next-workflows.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-partial-claims.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-proof-cells.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-remediation-candidates.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-telemetry-feedstock-surfaces.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-unified-run-plan.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-warrant-paths.rq` — OK 🟢
- `research/pi-program/ggen/queries/select-workflow-substrate-surfaces.rq` — OK 🟢
- `research/prompt-manufactory/ggen/queries/select-checkpoint-prompts.rq` — OK 🟢
- `research/prompt-manufactory/ggen/queries/select-hook-policies.rq` — OK 🟢
- `research/prompt-manufactory/ggen/queries/select-legacy-ggen-files.rq` — OK 🟢
- `research/prompt-manufactory/ggen/queries/select-rendered-prompts.rq` — OK 🟢
- `research/prompt-manufactory/ggen/queries/select-research-programs.rq` — OK 🟢
- `research/prompt-manufactory/ggen/queries/select-skill-prompts.rq` — OK 🟢
- `research/prompt-manufactory/ggen/queries/select-subagent-prompts.rq` — OK 🟢
- `research/prompt-manufactory/ggen/queries/select-workflow-prompts.rq` — OK 🟢

## Gate 3 Criteria Verification

### Criterion 1: Every active query parses
**Result:** ✓ PASS

### Criterion 2: Every generation query returns required bindings
**Result:** ⚠ REQUIRES_DATASET — Cannot verify without live SPARQL endpoint

## Remediation Summary

**Blocking Status:** GATE_3_READY
All active queries parse successfully.