---
gap: FIRMAMENT_002_PROMPT_MANUFACTORY
project: prompt-manufactory
date: 2026-06-02
status: OPEN
severity: BLOCKING
gate: Water Gate
---

# Gap: prompt-manufactory

## Summary

The prompt-manufactory project contains seven interlocking structural defects that prevent it from reaching an ALIVE verdict. The firmament authority layer evaluates the wrong directory path, the only existing checkpoint is explicitly PARTIAL (9/11 gates), a manufactured template artifact has been misclassified as an ALIVE receipt in the PhD evidence ledger, no Water Gate certification artifact exists, the ggen audit.json is empty despite 41 manufactured artifacts, the project receipt ledger contains a literal placeholder hash, and the emitted workflow warrants are minimal stubs missing phase structure. Until these gaps are closed the Water Gate — which governs lawful downstream passage of prompts as authorized doctrine — cannot be satisfied, and any downstream agent that relies on these prompts as warranted artifacts is operating without a valid provenance chain.

## Gap Register

### GAP_PROMPT_MANUFACTORY_001 — Firmament authority layer evaluates wrong canonical path

- **Severity:** BLOCKING
- **Category:** WRONG_PATH
- **Specific Blocker:** `PROJECT_GATE_ASSIGNMENT_MATRIX_002.md` records `UNKNOWN @ /Users/sac/process-intelligence/prompts (subdirectory only, no ALIVE receipts, no Water Gate artifacts found)`. The actual manufacturing substrate — 8 TTL ontologies, 8 SPARQL queries, 8 Tera templates, ggen.toml, and 41 emitted artifacts — lives entirely at `/Users/sac/process-intelligence/research/prompt-manufactory`. The `/prompts` directory contains only legacy downstream directive files with no manufacturing substrate. All firmament ALIVE gate evaluations are therefore evaluating the wrong location.
- **Remediation:** Update `PROJECT_GATE_ASSIGNMENT_MATRIX_002.md`, `FIRMAMENT_PROJECT_SPR_LEDGER_002.md`, and `PUBLIC_PRIVATE_CONNECTION_MAP_002.md` to use `/Users/sac/process-intelligence/research/prompt-manufactory` as the canonical path. Re-evaluate ALIVE status against the corrected location.
- **Effort:** < 1 hour

### GAP_PROMPT_MANUFACTORY_002 — No ALIVE receipt exists; only a PARTIAL checkpoint

- **Severity:** BLOCKING
- **Category:** MISSING_RECEIPTS
- **Specific Blocker:** The SPR Ledger requires `prompts/PROMPT_MANUFACTORY_ALIVE_001.yaml` linking to at least one upstream doctrine receipt. This file does not exist at either `/Users/sac/process-intelligence/prompts/PROMPT_MANUFACTORY_ALIVE_001.yaml` or `/Users/sac/process-intelligence/research/prompt-manufactory/`. The only existing checkpoint is `GGEN_PROMPT_MANUFACTORY_PARTIAL_001.md` (9/11 gates, explicitly PARTIAL). No `GGEN_PROMPT_MANUFACTORY_ALIVE_001` receipt has been issued.
- **Remediation:** Close the 2 pending gates from the PARTIAL checkpoint (PI_INTEL topology complete and remaining templates implemented), then emit `GGEN_PROMPT_MANUFACTORY_ALIVE_001.md` at `/Users/sac/process-intelligence/research/prompt-manufactory/checkpoints/` and create `PROMPT_MANUFACTORY_ALIVE_001.yaml` at the canonical path linking to upstream doctrine receipts.
- **Effort:** 1-4 hours

### GAP_PROMPT_MANUFACTORY_003 — Manufactured template misclassified as project ALIVE receipt

- **Severity:** BLOCKING
- **Category:** FALSE_ALIVE
- **Specific Blocker:** `/Users/sac/process-intelligence/phd-thesis/ledgers/EVIDENCE_LEDGER.yaml` entry for `CHECKPOINT_ALIVE.md` carries `verdict: ALIVE, description: Manufactured alive checkpoint artifact from prompt-manufactory`. This file is a manufactured prompt template that authorizes downstream agents to emit ALIVE verdicts — it is not the project's own ALIVE receipt. The BLAKE3 hash (`089f0c259fff3b67aead9cbde8293b34d08c7bddf38441e4c4293bbc8ff6a9cf`) is receipted in `RECEIPT_LEDGER_20260601.yaml` as a manufactured artifact of rule `checkpoint-prompts`, confirming it is a template output. The file's own content says `Warrant Type: Checkpoint Verdict / Status: AUTHORIZED` with no gate evaluation results.
- **Remediation:** Correct `EVIDENCE_LEDGER.yaml` to classify `CHECKPOINT_ALIVE.md` as `type: manufactured-template, verdict: NONE`. Add a note that `GGEN_PROMPT_MANUFACTORY_ALIVE_001` has not yet been issued.
- **Effort:** < 1 hour

### GAP_PROMPT_MANUFACTORY_004 — Water Gate certification artifact undefined and absent

- **Severity:** MAJOR
- **Category:** MISSING_ARTIFACTS
- **Specific Blocker:** No file matching a Water Gate certification artifact exists at `/Users/sac/process-intelligence/prompts/` or `/Users/sac/process-intelligence/research/prompt-manufactory/`. The SPR Ledger ALIVE condition states `Water Gate artifact (formal public-reading document) present in directory` — this artifact type is undefined and unimplemented. The emitted prompts exist but none carries explicit Water Gate certification.
- **Remediation:** Define the Water Gate artifact schema (a YAML or Markdown file listing warranted prompts, their upstream doctrine citations, and a governance statement certifying they are approved for downstream passage). Manufacture it via ggen or create it as a doctrine document. Place it at `/Users/sac/process-intelligence/research/prompt-manufactory/emitted/` or the canonical prompts directory.
- **Effort:** 1-4 hours

### GAP_PROMPT_MANUFACTORY_005 — ggen audit.json is empty despite 41 manufactured artifacts

- **Severity:** MAJOR
- **Category:** MISSING_ARTIFACTS
- **Specific Blocker:** `/Users/sac/process-intelligence/research/prompt-manufactory/audit.json` has `generated_at: 2026-06-01T20:40:40` but `pipeline:[], outputs:[], manifest_hash:'', ontology_hashes:{}, template_hashes:{}`. The ggen sync state at `.ggen/receipts/latest.json` records an ontology hash but `output_hashes:[]` is also empty. The manufactured artifact chain cannot be independently audited from the audit.json alone.
- **Remediation:** Determine why ggen v26.5.21 does not write to audit.json during manufacturing runs. Either fix the ggen audit output path configuration in `ggen.toml`, or regenerate audit.json by running `ggen sync`/`ggen manufacture` and verifying the audit file captures pipeline execution. The audit.json must bind input hashes to output hashes for the receipt chain to be complete.
- **Effort:** 1-4 hours

### GAP_PROMPT_MANUFACTORY_006 — Project receipt ledger contains a literal placeholder hash

- **Severity:** MAJOR
- **Category:** MISSING_RECEIPTS
- **Specific Blocker:** `/Users/sac/process-intelligence/research/prompt-manufactory/emitted/indexes/prompt-receipt-ledger.md` records the only receipt as `MANUFACTURABLE_FALLBACK` with hash `sha256(...)` — a literal placeholder string, not a computed hash. Manufacturing status is `PARTIAL (ggen v5 blocked; fallback manual used)`. Only 1 of 41 claimed artifacts has a provenance chain in this ledger, and that chain uses a fallback method. The pi-program `RECEIPT_LEDGER_20260601.yaml` does contain BLAKE3 hashes for some artifacts, but the project's own Water Gate accountability surface contains a placeholder.
- **Remediation:** Regenerate `prompt-receipt-ledger.md` via ggen so that all 41 emitted artifacts have real computed hashes (BLAKE3 or SHA256) bound to their source queries and templates. Replace the fallback method with verified ggen execution, or update the receipt ledger to reflect the actual BLAKE3 hashes from `RECEIPT_LEDGER_20260601.yaml` for each artifact.
- **Effort:** 1-4 hours

### GAP_PROMPT_MANUFACTORY_007 — Emitted workflow warrants are minimal stubs missing required structure

- **Severity:** MINOR
- **Category:** MISSING_DOCTRINE
- **Specific Blocker:** The 7 emitted workflow warrants at `/Users/sac/process-intelligence/research/prompt-manufactory/emitted/prompts/workflows/` are 712-767 bytes each, containing only program ID, mission, authority layer, workflow URI, and a minimal `Manufacturing Authorization` block. The PARTIAL checkpoint claims the proof specimen `PI_RESEARCH_PROGRAM_INTEL_001.md` contains `Complete workflow warrant with program identity, mission, transitions, forbidden paths, artifact lifecycle` — but the actual file at that path contains none of these fields. The `workflow-prompt.md.tera` template does not emit phase transitions, forbidden paths, or artifact lifecycle data.
- **Remediation:** Enrich `workflow-prompt.md.tera` to include phase structure from `workflow-law.ttl`, forbidden paths from `forbidden-collapse-law.ttl`, and artifact lifecycle data. Re-run ggen to regenerate all 7 workflow warrants with complete structure. Verify against the proof specimen specification in the PARTIAL checkpoint.
- **Effort:** 1-4 hours

## ALIVE Conditions Blocked

The following ALIVE conditions from the SPR Ledger and Water Gate cannot be met until the gaps above are closed:

1. **Canonical path resolution** (GAP_001) — No firmament evaluation is valid until the authority layer points to the correct directory. All other ALIVE conditions are evaluated against the wrong path.
2. **ALIVE receipt present** (GAP_002) — `PROMPT_MANUFACTORY_ALIVE_001.yaml` does not exist. The ALIVE gate cannot pass without it.
3. **Honest evidence classification** (GAP_003) — `EVIDENCE_LEDGER.yaml` carries a false ALIVE verdict for a manufactured template. The PhD evidence chain is corrupt at this entry.
4. **Water Gate artifact present** (GAP_004) — The Water Gate ALIVE condition explicitly requires a formal public-reading document certifying prompt governance. This artifact class does not exist anywhere in the project.
5. **Auditable receipt chain** (GAP_005, GAP_006) — The audit.json is empty and the receipt ledger contains a placeholder. No independent audit can verify that the 41 manufactured artifacts derive lawfully from the declared ontology and template inputs.
6. **Proof specimen conformance** (GAP_007) — The PARTIAL checkpoint's 9/11 gate count relied on a proof specimen claim that does not match current file content. The gate count may be overstated.

## Resolution Path

Steps are ordered by dependency. Each step unblocks the next.

1. **Correct canonical path in firmament authority layer** (GAP_001). Update the three firmament files to use `/Users/sac/process-intelligence/research/prompt-manufactory`. Commit as `gap: update firmament path for prompt-manufactory to research/ location`.

2. **Correct EVIDENCE_LEDGER.yaml false ALIVE entry** (GAP_003). Reclassify `CHECKPOINT_ALIVE.md` as a manufactured template with no gate verdict. Commit as `gap: correct false ALIVE classification for CHECKPOINT_ALIVE.md in evidence ledger`.

3. **Fix ggen audit.json capture** (GAP_005). Diagnose why ggen v26.5.21 writes empty pipeline and output fields. Fix `ggen.toml` or the ggen invocation. Re-run manufacture and verify audit.json contains non-empty pipeline and output hashes.

4. **Regenerate receipt ledger with real hashes** (GAP_006). After ggen audit is fixed, re-run `ggen manufacture` and confirm `prompt-receipt-ledger.md` contains computed BLAKE3/SHA256 hashes for all 41 artifacts. Remove the `MANUFACTURABLE_FALLBACK` placeholder entry.

5. **Define and manufacture the Water Gate certification artifact** (GAP_004). Author the Water Gate artifact schema. Add a ggen rule or doctrine document that manufactures or defines the formal public-reading document. Place it at the correct emitted path.

6. **Enrich workflow warrant template and regenerate** (GAP_007). Update `workflow-prompt.md.tera` to emit phase transitions, forbidden paths, and artifact lifecycle from the TTL ontologies. Re-run ggen. Verify `PI_RESEARCH_PROGRAM_INTEL_001.md` and the other 6 workflow warrants match the proof specimen specification.

7. **Close the 2 remaining PARTIAL gates** (GAP_002). Complete the PI_INTEL topology and any remaining template implementations identified in `GGEN_PROMPT_MANUFACTORY_PARTIAL_001.md`. Verify 11/11 gates pass.

8. **Emit GGEN_PROMPT_MANUFACTORY_ALIVE_001** (GAP_002). Create the ALIVE checkpoint at `/Users/sac/process-intelligence/research/prompt-manufactory/checkpoints/GGEN_PROMPT_MANUFACTORY_ALIVE_001.md` with gate results, receipt hashes, and upstream doctrine citations.

9. **Create PROMPT_MANUFACTORY_ALIVE_001.yaml**. Place the ALIVE receipt at the firmament canonical path, linking to upstream doctrine receipts and the ALIVE checkpoint. Commit as `checkpoint: GGEN_PROMPT_MANUFACTORY_ALIVE_001`.

10. **Re-run firmament ALIVE evaluation**. After steps 1-9 are complete, re-run the firmament gate scan against the corrected canonical path and verify the Water Gate returns ALIVE.

## Doctrine Note

The Van der Aalst Constitution requires that if the event log cannot prove a lawful process happened then it did not happen — and a receipt ledger containing placeholder hashes, an audit.json with empty pipeline fields, and an evidence ledger misclassifying template outputs as ALIVE verdicts collectively constitute proof that no lawful manufacturing process can be demonstrated for this project.
