# GGEN Manifest Census

**Authority:** PI Research Program Unified Intel  
**Date:** 2026-06-01  
**Scope:** Complete enumeration of all ggen.toml files and their generation rules

---

## Executive Summary

The process-intelligence repository contains 3 ggen.toml files representing 3 distinct generation programs:

1. **process-intelligence-ggen** — M&A asset and orchestrator generation (2 active rules)
2. **PI_RESEARCH_PROGRAM_INTEL_001** — Research reconciliation analyzer (query-only, no generation rules)
3. **prompt-manufactory** — Research warrant manufacturing (8 rules defined, all incomplete)

**Critical Finding:** The prompt-manufactory program is non-functional. All 8 generation rules are blocked by missing templates; 6 rules are also missing queries. No prompts can be manufactured until assets are implemented.

---

## File 1: process-intelligence-ggen

**Path:** `/Users/sac/process-intelligence/ggen/ggen.toml`

### Project Metadata
| Property | Value |
|----------|-------|
| Name | process-intelligence-ggen |
| Version | 0.1.0 |
| Description | Generate board-admissible M&A assets and autonomic governance engines from process intelligence |
| Hand Coding | false |
| Evidence Sources | ../receipts, ../checkpoints |

### Ontology Inputs
| Input | Path | Format |
|-------|------|--------|
| Primary Ontology | ontology-extensions.ttl | TTL |

**Count:** 1 file

### Query Directory
| Query File | Status |
|------------|--------|
| extract-lifecycle-governance.rq | ✓ Exists |
| extract-visualizer-data.rq | ✓ Exists |
| extract-board-claims.rq | Declared in comments (deactivated) |
| extract-diligence-claims.rq | Declared in comments (deactivated) |

**Count:** 2 active queries, 2 deactivated

### Template Directory
| Template File | Status | Referenced in Generation Rule |
|---------------|--------|-------------------------------|
| blue-river.tera | ✓ Exists | Yes (active) |
| visualizer-dashboard.tsx.tera | ✓ Exists | Yes (active) |
| ma-deck.tera | ✓ Exists | No (rule deactivated) |
| ma-diligence.tera | ✓ Exists | No (rule deactivated) |

**Count:** 4 templates total, 2 active

### Generation Rules: Detailed Manifest

#### Rule 1: blue-river-orchestrator

| Property | Value |
|----------|-------|
| Name | blue-river-orchestrator |
| Description | Autonomic MAPE-K governance engine with lifecycle state transitions |
| Query | queries/extract-lifecycle-governance.rq |
| Template | templates/blue-river.tera |
| Output File | ../blue_river_dam/src/lib.rs |
| Output Mode | Overwrite |
| Target Format | Rust |
| Audience | Process Intelligence Runtime |
| Compliance | MAPE-K autonomic loop |
| Evidence Backing | lifecycle state machine + governance rule enforcement |
| Status | **READY** ✓ |

**Rendered Target Path:** `/Users/sac/blue_river_dam/src/lib.rs`

#### Rule 2: visualizer-dashboard-nextjs

| Property | Value |
|----------|-------|
| Name | visualizer-dashboard-nextjs |
| Description | Generate NextJS visualizer dashboard pages with conformance-backed claims |
| Query | queries/extract-visualizer-data.rq |
| Template | templates/visualizer-dashboard.tsx.tera |
| Output File | ../experiments/visualizer-nextjs/src/app/page.tsx |
| Output Mode | Overwrite |
| Status | **READY** ✓ |

**Rendered Target Path:** `/Users/sac/process-intelligence/experiments/visualizer-nextjs/src/app/page.tsx`

#### Rule 3: ma-deck (DEACTIVATED)

**Status:** ⚠️ Rule commented out in configuration

| Property | Value |
|----------|-------|
| Name | ma-deck (declared in comment) |
| Query | extract-board-claims.rq (would be declared, not in config) |
| Template | ma-deck.tera (✓ exists) |
| Note | Template artifact exists but generation rule is disabled |

#### Rule 4: ma-diligence (DEACTIVATED)

**Status:** ⚠️ Rule commented out in configuration

| Property | Value |
|----------|-------|
| Name | ma-diligence (declared in comment) |
| Query | extract-diligence-claims.rq (would be declared, not in config) |
| Template | ma-diligence.tera (✓ exists) |
| Note | Template artifact exists but generation rule is disabled |

### Configuration: Query Endpoint & Ontology Namespace

```toml
[query]
endpoint = "in-memory"
format = "sparql-11"
ontology_namespace = {
  ma = "https://process.intelligence/ma/",
  lifecycle = "https://process.intelligence/lifecycle/",
  wasm4pm = "https://process.intelligence/wasm4pm/",
  compat = "https://process.intelligence/compat/"
}
```

### Configuration: Template Engine

```toml
[template]
engine = "tera"
base_dir = "templates"
autoescape = true
strict_variables = true
```

### Configuration: Output & Receipt Format

```toml
[output]
checksum_algorithm = "blake3"
receipt_format = "json"
proof_format = "cryptographic-chain"
```

### Summary for File 1

| Metric | Count |
|--------|-------|
| Active Generation Rules | 2 |
| Deactivated Rules | 2 |
| Ontology Files | 1 |
| Query Files | 2 active, 2 deactivated |
| Template Files | 4 total, 2 active |
| Output Targets | 2 |

---

## File 2: PI_RESEARCH_PROGRAM_INTEL_001

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/ggen.toml`

### Program Metadata
| Property | Value |
|----------|-------|
| Name | PI_RESEARCH_PROGRAM_INTEL_001 |
| Mode | research_program_reconciliation |
| Hand Coding | false |
| Scope | all_referenced_projects |

### Input Project Registry

| Project | Path |
|---------|------|
| wasm4pm | /Users/sac/process-intelligence/sources/wasm4pm |
| wasm4pm_compat | /Users/sac/wasm4pm-compat |
| zoeapp | /Users/sac/zoeapp |
| blue_river_dam | /Users/sac/blue_river_dam |
| process_intelligence | /Users/sac/process-intelligence |

### Ontology Inputs (Research Authority Layer)

| Ontology File | Purpose |
|---------------|---------|
| ontology/pi-program.ttl | Core research program definition |
| ontology/project-registry.ttl | Project ecosystem registry |
| ontology/checkpoint-ledger.ttl | Checkpoint state ledger |
| ontology/research-artifact-ledger.ttl | Research artifact tracking |
| ontology/conformance-ledger.ttl | Conformance state tracking |
| ontology/graduation-boundary.ttl | Graduation gate definitions |
| ontology/forbidden-collapse-law.ttl | Forbidden state law |

**Count:** 7 files

### Query Directory

**Total Query Files:** 37

#### Audit Queries (17 files)

These queries enforce conformance constraints and validate research integrity:

| Query File | Purpose |
|------------|---------|
| audit-checkpoint-has-receipts.rq | Verify checkpoints have cryptographic receipts |
| audit-closure-invariant.rq | Verify closure invariant holds across programs |
| audit-commitment-integrity.rq | Verify commitment integrity across claims |
| audit-compliance-ledger.rq | Validate compliance ledger state |
| audit-evidence-traceability.rq | Trace evidence back to source |
| audit-gates-complete.rq | Verify all proof gates completed |
| audit-no-client-only-auth.rq | Reject client-only auth patterns |
| audit-no-dashboard-truth.rq | Reject dashboard as source of truth |
| audit-no-dto-flattening.rq | Reject DTO flattening patterns |
| audit-no-forced-alive.rq | Reject forced ALIVE verdicts |
| audit-no-invalid-ggen-extension.rq | Reject invalid ggen type extensions |
| audit-no-realtime-as-evidence.rq | Reject realtime data as evidence |
| audit-no-telemetry-as-receipt.rq | Reject telemetry as cryptographic receipt |
| audit-no-tool-smuggling.rq | Reject tool smuggling patterns |
| audit-no-unsigned-verdicts.rq | Reject unsigned checkpoint verdicts |
| audit-partial-has-gaps.rq | Verify PARTIAL verdicts have gap documentation |
| audit-source-court-citations.rq | Verify source citations are complete |

#### Selection Queries (20 files)

These queries extract program state for further analysis:

| Query File | Purpose |
|------------|---------|
| select-alive-claims.rq | Extract all ALIVE claims |
| select-all-projects.rq | Extract all registered projects |
| select-checkpoints.rq | Extract all checkpoints |
| select-compatibility-surfaces.rq | Extract compatibility type surfaces |
| select-engine-surfaces.rq | Extract engine type surfaces |
| select-failed-gates.rq | Extract failed proof gates |
| select-forbidden-collapses.rq | Extract forbidden state collapses |
| select-manufacturing-surfaces.rq | Extract manufacturing type surfaces |
| select-mobile-substrate-surfaces.rq | Extract mobile substrate surfaces |
| select-next-workflows.rq | Extract next workflow candidates |
| select-partial-claims.rq | Extract all PARTIAL claims |
| select-proof-cells.rq | Extract proof cells |
| select-remediation-candidates.rq | Extract remediation candidates |
| select-telemetry-feedstock-surfaces.rq | Extract telemetry feedstock surfaces |
| select-workflow-substrate-surfaces.rq | Extract workflow substrate surfaces |

(Note: 5 additional selection queries beyond those listed)

### Template Directory

| Template File | Status | Referenced in Generation Rule |
|---------------|--------|-------------------------------|
| pi-program-walkthrough.md.tera | ✓ Exists | **No** (orphaned) |

**Count:** 1 template (orphaned)

### Output & Checkpoint Configuration

| Property | Value |
|----------|-------|
| Emitted Output Directory | ../emitted/ |
| Checkpoint Target (PARTIAL) | ../checkpoints/PI_RESEARCH_PROGRAM_PARTIAL_001.md |
| Checkpoint Target (ALIVE) | ../checkpoints/PI_RESEARCH_PROGRAM_ALIVE_001.md |

### Generation Rules Status

**Critical Finding:** This program is a **query-only reconciliation engine**. 

- **Active Generation Rules:** 0
- **Templates Defined:** 0 (the one template is orphaned)
- **Purpose:** Execute 37 audit and selection queries to validate research program integrity
- **Output:** Query results feed directly to checkpoint validation logic, not artifact generation

### Summary for File 2

| Metric | Count |
|--------|-------|
| Active Generation Rules | 0 |
| Ontology Files | 7 |
| Query Files | 37 (17 audit, 20 selection) |
| Template Files | 1 (orphaned) |
| Input Projects | 5 |
| Checkpoint Targets | 2 |

**Role in Ecosystem:** Authority layer for PI research program validation. Not a generation engine.

---

## File 3: prompt-manufactory

**Path:** `/Users/sac/process-intelligence/research/prompt-manufactory/ggen/ggen.toml`

### Project Metadata
| Property | Value |
|----------|-------|
| Name | prompt-manufactory |
| Version | 0.1.0 |
| Description | Post-cyberpunk layer: research warrants manufactured from law |
| Authors | Sean Chatman |
| Hand Coding | false |

### Ontology Inputs

| Ontology File | Classification | Status |
|---------------|-----------------|--------|
| ontology/prompt-manufactory.ttl | Primary | ✓ Exists |
| ontology/research-program-law.ttl | Additional | ✓ Exists |
| ontology/workflow-law.ttl | Additional | ✓ Exists |
| ontology/subagent-role-law.ttl | Additional | ✓ Exists |
| ontology/skill-law.ttl | Additional | ✓ Exists |
| ontology/hook-law.ttl | Additional | ✓ Exists |
| ontology/checkpoint-law.ttl | Additional | ✓ Exists |
| ontology/forbidden-collapse-law.ttl | Additional | ✓ Exists |

**Count:** 8 files (1 primary, 7 additional)

### Query Directory

| Query File | Required By | Status |
|------------|------------|--------|
| select-workflow-prompts.rq | workflow-prompts rule | ✓ Exists |
| select-research-programs.rq | program-index rule | ✓ Exists |
| select-subagent-prompts.rq | subagent-prompts rule | ✗ **MISSING** |
| select-skill-prompts.rq | skill-docs rule | ✗ **MISSING** |
| select-hook-policies.rq | hook-policies rule | ✗ **MISSING** |
| select-checkpoint-prompts.rq | checkpoint-prompts rule | ✗ **MISSING** |
| select-legacy-ggen-files.rq | invalid-ggen-ledger rule | ✗ **MISSING** |
| select-rendered-prompts.rq | receipt-ledger rule | ✗ **MISSING** |

**Count:** 2 exist, 6 missing

### Template Directory

**Status:** All templates missing. Directory is empty.

| Template File | Required By | Status |
|---------------|------------|--------|
| workflow-prompt.md.tera | workflow-prompts rule | ✗ **MISSING** |
| subagent-prompt.md.tera | subagent-prompts rule | ✗ **MISSING** |
| skill.md.tera | skill-docs rule | ✗ **MISSING** |
| hook-policy.md.tera | hook-policies rule | ✗ **MISSING** |
| checkpoint-prompt.md.tera | checkpoint-prompts rule | ✗ **MISSING** |
| research-program-index.md.tera | program-index rule | ✗ **MISSING** |
| invalid-ggen-classification-ledger.md.tera | invalid-ggen-ledger rule | ✗ **MISSING** |
| prompt-receipt.md.tera | receipt-ledger rule | ✗ **MISSING** |

**Count:** 0 exist, 8 missing

### Generation Rules: Detailed Manifest

#### Rule 1: workflow-prompts

| Property | Value |
|----------|-------|
| Name | workflow-prompts |
| Description | Render one .md workflow warrant per ResearchProgram instance |
| Query | queries/select-workflow-prompts.rq |
| Query Status | ✓ Exists |
| Template | templates/workflow-prompt.md.tera |
| Template Status | ✗ **MISSING** |
| Output Directory | emitted/prompts/workflows/ |
| Output Mode | Overwrite |
| **RULE STATUS** | **BLOCKED** ✗ |

**Blocking Issue:** Template missing

**Rendered Target Path Pattern:** `emitted/prompts/workflows/{generated_name}.md`

#### Rule 2: subagent-prompts

| Property | Value |
|----------|-------|
| Name | subagent-prompts |
| Description | Render one .md subagent role prompt per SubagentRole instance |
| Query | queries/select-subagent-prompts.rq |
| Query Status | ✗ **MISSING** |
| Template | templates/subagent-prompt.md.tera |
| Template Status | ✗ **MISSING** |
| Output Directory | emitted/prompts/subagents/ |
| Output Mode | Overwrite |
| **RULE STATUS** | **BLOCKED** ✗ |

**Blocking Issues:** Query missing, Template missing

**Rendered Target Path Pattern:** `emitted/prompts/subagents/{generated_name}.md`

#### Rule 3: skill-docs

| Property | Value |
|----------|-------|
| Name | skill-docs |
| Description | Render one .md skill doc per Skill instance |
| Query | queries/select-skill-prompts.rq |
| Query Status | ✗ **MISSING** |
| Template | templates/skill.md.tera |
| Template Status | ✗ **MISSING** |
| Output Directory | emitted/prompts/skills/ |
| Output Mode | Overwrite |
| **RULE STATUS** | **BLOCKED** ✗ |

**Blocking Issues:** Query missing, Template missing

**Rendered Target Path Pattern:** `emitted/prompts/skills/{generated_name}.md`

#### Rule 4: hook-policies

| Property | Value |
|----------|-------|
| Name | hook-policies |
| Description | Render one .md hook policy per HookPolicy instance |
| Query | queries/select-hook-policies.rq |
| Query Status | ✗ **MISSING** |
| Template | templates/hook-policy.md.tera |
| Template Status | ✗ **MISSING** |
| Output Directory | emitted/prompts/hooks/ |
| Output Mode | Overwrite |
| **RULE STATUS** | **BLOCKED** ✗ |

**Blocking Issues:** Query missing, Template missing

**Rendered Target Path Pattern:** `emitted/prompts/hooks/{generated_name}.md`

#### Rule 5: checkpoint-prompts

| Property | Value |
|----------|-------|
| Name | checkpoint-prompts |
| Description | Render one .md checkpoint prompt per program |
| Query | queries/select-checkpoint-prompts.rq |
| Query Status | ✗ **MISSING** |
| Template | templates/checkpoint-prompt.md.tera |
| Template Status | ✗ **MISSING** |
| Output Directory | emitted/prompts/checkpoints/ |
| Output Mode | Overwrite |
| **RULE STATUS** | **BLOCKED** ✗ |

**Blocking Issues:** Query missing, Template missing

**Rendered Target Path Pattern:** `emitted/prompts/checkpoints/{generated_name}.md`

#### Rule 6: program-index

| Property | Value |
|----------|-------|
| Name | program-index |
| Description | Render single comprehensive program index |
| Query | queries/select-research-programs.rq |
| Query Status | ✓ Exists |
| Template | templates/research-program-index.md.tera |
| Template Status | ✗ **MISSING** |
| Output File | emitted/indexes/research-program-prompt-index.md |
| Output Mode | Overwrite |
| **RULE STATUS** | **BLOCKED** ✗ |

**Blocking Issue:** Template missing

**Rendered Target Path:** `/Users/sac/process-intelligence/research/prompt-manufactory/emitted/indexes/research-program-prompt-index.md`

#### Rule 7: invalid-ggen-ledger

| Property | Value |
|----------|-------|
| Name | invalid-ggen-ledger |
| Description | Render ledger of all 22 legacy .ggen files with classification |
| Query | queries/select-legacy-ggen-files.rq |
| Query Status | ✗ **MISSING** |
| Template | templates/invalid-ggen-classification-ledger.md.tera |
| Template Status | ✗ **MISSING** |
| Output File | emitted/indexes/invalid-ggen-classification-ledger.md |
| Output Mode | Overwrite |
| **RULE STATUS** | **BLOCKED** ✗ |

**Blocking Issues:** Query missing, Template missing

**Rendered Target Path:** `/Users/sac/process-intelligence/research/prompt-manufactory/emitted/indexes/invalid-ggen-classification-ledger.md`

#### Rule 8: receipt-ledger

| Property | Value |
|----------|-------|
| Name | receipt-ledger |
| Description | Render ledger of all rendered prompts with proof-of-manufacture receipts |
| Query | queries/select-rendered-prompts.rq |
| Query Status | ✗ **MISSING** |
| Template | templates/prompt-receipt.md.tera |
| Template Status | ✗ **MISSING** |
| Output File | emitted/indexes/prompt-receipt-ledger.md |
| Output Mode | Overwrite |
| **RULE STATUS** | **BLOCKED** ✗ |

**Blocking Issues:** Query missing, Template missing

**Rendered Target Path:** `/Users/sac/process-intelligence/research/prompt-manufactory/emitted/indexes/prompt-receipt-ledger.md`

### Configuration: Sync & RDF

```toml
[sync]
enabled = true
on_change = "manual"
validate_after = true
conflict_mode = "fail"

[rdf]
formats = ["turtle"]
default_format = "turtle"
strict_validation = false

[templates]
enable_caching = true
auto_reload = true

[output]
formatting = "default"
line_length = 100
indent = 4
```

### Summary for File 3

| Metric | Count |
|--------|-------|
| **Active Generation Rules** | **0** |
| **Defined Generation Rules** | **8** |
| **Blocked Rules (missing template)** | **8** |
| **Blocked Rules (missing query)** | **6** |
| **Ontology Files** | **8** |
| **Query Files (exist)** | **2** |
| **Query Files (missing)** | **6** |
| **Template Files (exist)** | **0** |
| **Template Files (missing)** | **8** |
| **Output Targets Defined** | **8** |

**Critical Status:** ⚠️ **100% NON-FUNCTIONAL** — All generation rules are blocked.

---

## Cross-Program Manifest Comparison

### Generation Rule Summary

| Program | Active Rules | Blocked Rules | Query-Only | Total Rules |
|---------|--------------|---------------|-----------|------------|
| process-intelligence-ggen | 2 | 2 (deactivated) | — | 4 |
| PI_RESEARCH_PROGRAM_INTEL_001 | — | — | Yes (37 queries) | 0 |
| prompt-manufactory | 0 | 8 | — | 8 |

### Asset Inventory

| Asset Type | File 1 | File 2 | File 3 | Total |
|------------|--------|--------|--------|-------|
| Ontology Files | 1 | 7 | 8 | 16 |
| Query Files | 4 | 37 | 2 | 43 |
| Template Files | 4 | 1 | 0 | 5 |
| **Missing** | **0** | **0** | **14** | **14** |

### Functional Status

| Program | Status | Notes |
|---------|--------|-------|
| process-intelligence-ggen | ✓ **OPERATIONAL** | 2 active rules. 2 deactivated rules have template artifacts. |
| PI_RESEARCH_PROGRAM_INTEL_001 | ✓ **OPERATIONAL** | Query-only validator. Functions as audit layer. |
| prompt-manufactory | ✗ **NON-FUNCTIONAL** | No queries can be executed. No templates can be rendered. No warrants can be manufactured. |

---

## Critical Findings

### Finding 1: Prompt Manufactory is 100% Blocked

**Evidence:**
- All 8 generation rules declare template dependencies
- 0 of 8 required templates exist
- 6 of 8 required queries do not exist
- Output directory structure exists but is empty

**Impact:** The prompt-manufactory program cannot manufacture research warrants from law. All downstream work depending on workflow prompts, subagent role prompts, skill documentation, hook policies, and checkpoint prompts is stalled.

**Resolution Required:**
1. Implement all 8 missing Tera templates
2. Implement all 6 missing SPARQL queries
3. Test each rule independently before enabling

### Finding 2: Process Intelligence ggen Has Stale Artifacts

**Evidence:**
- Rules for ma-deck and ma-diligence are commented out
- Templates ma-deck.tera and ma-diligence.tera still exist
- No indication whether rules should be reactivated or templates deleted

**Impact:** Unclear intent. Either:
- Rules are temporarily disabled pending implementation (keep templates)
- Rules are deprecated (delete templates)

**Resolution Required:** Decide: reactivate or deprecate.

### Finding 3: PI Research Program Template is Orphaned

**Evidence:**
- pi-program-walkthrough.md.tera exists
- No generation rule references it
- No output target declared

**Impact:** Asset exists but has no use case defined.

**Resolution Required:** Either connect to a generation rule or remove.

---

## Rendered Target Paths Summary

### File 1 Targets (process-intelligence-ggen)

| Rule | Target Path | Format | Status |
|------|------------|--------|--------|
| blue-river-orchestrator | `/Users/sac/blue_river_dam/src/lib.rs` | Rust | Ready |
| visualizer-dashboard-nextjs | `/Users/sac/process-intelligence/experiments/visualizer-nextjs/src/app/page.tsx` | TypeScript | Ready |

### File 3 Targets (prompt-manufactory) - BLOCKED

| Rule | Target Path | Status |
|------|------------|--------|
| workflow-prompts | `emitted/prompts/workflows/{generated}.md` | Blocked |
| subagent-prompts | `emitted/prompts/subagents/{generated}.md` | Blocked |
| skill-docs | `emitted/prompts/skills/{generated}.md` | Blocked |
| hook-policies | `emitted/prompts/hooks/{generated}.md` | Blocked |
| checkpoint-prompts | `emitted/prompts/checkpoints/{generated}.md` | Blocked |
| program-index | `emitted/indexes/research-program-prompt-index.md` | Blocked |
| invalid-ggen-ledger | `emitted/indexes/invalid-ggen-classification-ledger.md` | Blocked |
| receipt-ledger | `emitted/indexes/prompt-receipt-ledger.md` | Blocked |

---

## Recommendations

### Immediate Actions

1. **Unblock prompt-manufactory**: Implement all missing templates and queries. This is a prerequisite for warrant manufacturing.

2. **Clarify process-intelligence-ggen intent**: Decide on M&A deck and diligence workbook rules (activate or remove).

3. **Connect orphaned template**: Either create generation rule for pi-program-walkthrough.md.tera or remove it.

### Long-term Governance

1. **Enforce ggen.toml completeness**: Require all declared queries and templates to exist before configuration is accepted.

2. **Track rule health**: Monitor active rules for coverage, ensuring no orphaned templates or queries remain.

3. **Document intention**: Each deactivated rule must have explicit rationale and activation plan.

---

## Census Metadata

| Field | Value |
|-------|-------|
| Census Date | 2026-06-01 |
| Repository | /Users/sac/process-intelligence |
| Total ggen.toml Files | 3 |
| Total Queries | 43 |
| Total Templates | 5 |
| Total Ontologies | 16 |
| Functional Programs | 2 |
| Non-Functional Programs | 1 |
| Generation Rules (Active) | 2 |
| Generation Rules (Blocked) | 8 |
| Generation Rules (Query-Only) | 0 |

