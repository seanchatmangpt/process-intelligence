# PI_RESEARCH_PROGRAM_INTEL_001

## Mission
Full research-program reconciliation: census all projects, classify into RDF ontology, emit ggen surfaces, audit conformance, produce program map.

---

## Preamble

This is not an implementation sprint.
This is a research-program manufacturing workflow.

**Valid ggen source surfaces:**
- `.ttl` (Turtle/RDF ontologies)
- `.rq` (SPARQL queries)
- `.tera` (Tera templates)
- `ggen.toml` (generation configuration)
- `.md` (documentation)

**Forbidden:**
- `.ggen` files as source surfaces
- Hand-written program prompts
- Forced ALIVE declarations

---

## Phases

### Phase 1 — Census

Spawn 8 parallel subagents.

**Required outputs:**
- `research/pi-program/intel/wasm4pm-engine-census.md`
- `research/pi-program/intel/wasm4pm-compat-census.md`
- `research/pi-program/intel/ggen-census.md`
- `research/pi-program/intel/blue-river-dam-census.md`
- `research/pi-program/intel/zoeapp-census.md`
- `research/pi-program/intel/otel-weaver-census.md`
- `research/pi-program/intel/expo-supabase-framework-census.md`
- `research/pi-program/intel/claude-workflow-census.md`

### Phase 2 — Classify

Classify all census data into program ontology.

**Required outputs:**
- `ggen/ontology/pi-program.ttl`
- `ggen/ontology/project-registry.ttl`
- `ggen/ontology/checkpoint-ledger.ttl`

### Phase 3 — Manifest

Create projection manifests and program declarations.

**Required outputs:**
- `ggen/manifests/*.yaml` files

### Phase 4 — Queries

Emit SPARQL selection and audit queries.

**Required outputs:**
- `ggen/queries/*.rq` files

### Phase 5 — Templates

Emit Tera templates for artifact rendering.

**Required outputs:**
- `ggen/templates/*.tera` files

### Phase 6 — Conformance

Van der Aalst audit: verify program structure against law.

**Required outputs:**
- `audits/audit-results.yaml`
- `audits/audit-report.md`

### Phase 7 — Reconciliation

Emit unified program map outputs.

**Required outputs:**
- `emitted/pi-program-walkthrough.md`
- `emitted/project-registry.yaml`
- `emitted/checkpoint-ledger.md`
- `emitted/alive-partial-matrix.md`
- `emitted/failed-gate-ledger.yaml`
- `emitted/remediation-plan.md`
- `emitted/program-surface-map.yaml`
- `emitted/next-workflow-plan.md`

### Phase 8 — Checkpoint

Emit ALIVE/PARTIAL verdict.

**Required output:**
- `checkpoints/PI_RESEARCH_PROGRAM_INTEL_001_ALIVE_001.md` (if all gates pass)
- OR `checkpoints/PI_RESEARCH_PROGRAM_INTEL_001_PARTIAL_001.md` (if any gate fails)

---

## Audit Gates

| Gate | Pass Condition |
|------|---|
| Project registry complete | Every referenced project found or marked MISSING_REFERENCED_PROJECT |
| Checkpoint ledger complete | Every ALIVE/PARTIAL checkpoint classified |
| No forced ALIVE | Failed gate prevents ALIVE promotion |
| No invalid ggen extension | Zero .ggen source files |
| No DTO flattening | Compat/projected surfaces don't collapse law into JSON/string |
| No tool smuggling | Compat surfaces don't contain execution engines |
| No telemetry-as-receipt | Weaver/OTel findings never classified as receipts |
| No realtime-as-evidence | ZOEapp Realtime stays feedstock unless admitted |
| No dashboard truth | Dashboards/reports are projections, not courts |
| No client-only auth | ZOEapp paths don't rely only on client state |
| Receipts present | Claimed tests cite receipts or mark RECEIPT_MISSING |
| Remediation routed | Every failed gate has owner + remediation class |

---

## Checkpoint

Emit PARTIAL if any gate fails or prerequisites incomplete.

Emit ALIVE only when **all** gates pass **and** this workflow has successfully proven the end-to-end manufacturing path.

**ALIVE requires:**
- ✓ All 8 ontologies emitted and parse without error
- ✓ All SPARQL queries return valid results
- ✓ All Tera templates produce valid outputs
- ✓ At least one program instance renders to a complete workflow warrant
- ✓ The warrant carries receipted proof-of-manufacture
- ✓ All 12 audit gates pass
- ✓ No unresolved blockers

**PARTIAL is correct when:**
- ⚠️ End-to-end warrant path is unproven
- ⚠️ Legacy .ggen files are unclassified
- ⚠️ PI_RESEARCH_PROGRAM_INTEL_001 topology incomplete
- ⚠️ Any audit gate returns FAIL

---

## Seal

This warrant was manufactured from graph law via SPARQL and Tera.
Not hand-written. Not improvised.

**Derivation Source:** `<https://pi-research.dev/programs#PI_RESEARCH_PROGRAM_INTEL_001>`
**Query Used:** `select-workflow-prompts.rq`
**Template Used:** `workflow-prompt.md.tera`
**Date Rendered:** 2026-06-01
