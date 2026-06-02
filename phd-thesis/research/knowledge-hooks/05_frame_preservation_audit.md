# Frame Preservation Audit

**Agent:** E — Frame Preservation Auditor
**Date:** 2026-06-01
**Scope:** /Users/sac/process-intelligence, /Users/sac/ostar
**Method:** Adversarial search for bad translations of Frame Law canonical terms

---

## Frame Law Reference

```
THE CORRECT DIRECTION:
  attempt → hook → admission/refusal → durable motion → receipt → replay → accounting → promotion

THE CORRECT COMPRESSION:
  No hook, no consequence.
  No receipt, no authority.
  No replay, no substrate.
  No accounting, no promotion.
```

**Forbidden translations:**
1. Knowledge hook → middleware / callback / event listener / plugin
2. Autonomic Knowledge Actuation → automation / AI workflow / lifecycle management
3. AutoInstinct/ainst → agent framework / report machine
4. ccog → chatbot runtime
5. Receipt → log
6. Report → proof
7. LLM output → runtime authority
8. Summary → evidence

---

## Agent E Findings — Bad Translations Found

---

### BAD TRANSLATION 1

**File:** `/Users/sac/process-intelligence/standards/ocel.md`
**Line:** 10
**Bad text:** `"OCEL is the ultimate cryptographic receipt."`
**Forbidden category:** log = receipt
**Correct framing:** OCEL is an object-centric event log format — a substrate for process evidence. An OCEL log is feedstock for conformance checking. A receipt is a typed, witnessed, bound evidence artifact emitted at lifecycle closure. `OCEL ≠ receipt`. A receipt can reference an OCEL log as its evidence substrate, but the log itself is not the receipt.
**Severity:** CRITICAL

---

### BAD TRANSLATION 2

**File:** `/Users/sac/process-intelligence/audits/adversarial_audit_v30.1.1.md`
**Line:** 12
**Bad text:** `"the git commit log maintains a sound, continuous receipt chain of 567 commits"`
**Forbidden category:** log = receipt
**Correct framing:** The git commit log is a log — a sequential record of commits. A receipt chain is a typed, witnessed, BLAKE3-bound evidence chain where each link certifies a named law was applied. The commit log is not a receipt chain. The commit log may be the substrate from which receipt provenance is derived, but it is not the chain itself. `FORMAL_OBJECTS_TAXONOMY.md` is explicit: "a log is not a receipt."
**Severity:** CRITICAL

---

### BAD TRANSLATION 3

**File:** `/Users/sac/process-intelligence/audits/adversarial_audit_v30.1.1.md`
**Line:** 12
**Bad text:** `"This audit report certifies that the process-intelligence research foundry satisfies all ALIVE_001 gate criteria"`
**Forbidden category:** report = proof
**Correct framing:** A report is not proof. Proof is a replayed, receipted, conformance-verified event trail. "Certifies" is a proof verb that cannot be carried by a report. The correct framing: this audit report documents the findings of gate-checking; the gates themselves are the authority surface. The report narrates; a receipt certifies. Frame Law 6: "A report is NOT proof."
**Severity:** CRITICAL

---

### BAD TRANSLATION 4

**File:** `/Users/sac/process-intelligence/audits/drift_sentry_audit.md`
**Line:** 13
**Bad text:** `"This audit report certifies that the concept drift detection mechanisms ... satisfy the process intelligence standards"`
**Forbidden category:** report = proof
**Correct framing:** Same violation as BAD TRANSLATION 3. An audit report documents; it does not certify. Certification requires a signed receipt against a named law, not a written report. Frame Law 6: "A report is NOT proof."
**Severity:** CRITICAL

---

### BAD TRANSLATION 5

**File:** `/Users/sac/process-intelligence/audits/alignment_referee_audit.md`
**Line:** 13
**Bad text:** `"This audit report certifies that the A* alignment search algorithms ... satisfy the correctness and performance standards"`
**Forbidden category:** report = proof
**Correct framing:** Same violation as BAD TRANSLATIONS 3 and 4. Correctness is established by replay against a declared process model and a conformance receipt. A written report asserting correctness is not proof. Frame Law 6: "A report is NOT proof."
**Severity:** CRITICAL

---

### BAD TRANSLATION 6

**File:** `/Users/sac/process-intelligence/audits/audit-lifecycle-completeness.md`
**Line:** 135
**Bad text:** `"Receipt Shapes | COMPLETE | Operational KPI receipts, event logs, compliance exception records."`
**Forbidden category:** log = receipt (event logs listed as instances of receipt shapes)
**Correct framing:** Event logs are not receipt shapes. Event logs are the feedstock for process mining and conformance checking. Receipt shapes are typed, witnessed structures emitted at lifecycle closure: `Receipt<T, W>`. Listing "event logs" as a Receipt Shape conflates the log (substrate) with the receipt (evidence artifact). The correct entry would list only the receipt types and their witness types.
**Severity:** MAJOR

---

### BAD TRANSLATION 7

**File:** `/Users/sac/process-intelligence/audits/audit-lifecycle-completeness.md`
**Line:** 203
**Bad text:** `"Receipt Shapes | COMPLETE | Discovery receipt (DFG, heuristics log, alignment report), baseline fitness proof."`
**Forbidden category:** log = receipt; report = proof (heuristics log and alignment report listed as content of receipt shapes)
**Correct framing:** A "heuristics log" is a log — a record of heuristics miner output. It is not a receipt. The receipt shape for discovery should name the typed, witnessed evidence artifact that certifies discovery completion, not the log produced during discovery. "Alignment report" also violates Frame Law 6 — a report is not proof; it cannot be a receipt component. The receipt should carry the conformance score, the witness type, and the BLAKE3 commitment, not a log or a report.
**Severity:** MAJOR

---

### BAD TRANSLATION 8

**File:** `/Users/sac/process-intelligence/research/prompt-manufactory/ggen/ontology/hook-law.ttl`
**Line:** 7
**Bad text:** `"# Hooks are Claude Code lifecycle automations that enforce refusal conditions."`
**Forbidden category:** hooks = automations
**Correct framing:** Hooks are deterministic lifecycle enforcement points (Andon gates), not automations. "Automation" is the forbidden category under Frame Law 2 (Autonomic Knowledge Actuation is NOT automation). The line partially redeems itself with "not at model discretion" and "enforce refusal conditions" — but calling hooks "automations" introduces the forbidden category. The correct framing: "Hooks are deterministic Andon gates that enforce admission/refusal conditions at named lifecycle boundaries. They run at specific events, not at model discretion."
**Note:** This file refers specifically to Claude Code tool hooks rather than the abstract Knowledge Hook doctrine, which mitigates severity. However the word "automations" still violates the frame.
**Severity:** MINOR

---

## Corpus-Wide Bad Translation Search Results

| Forbidden Pair | Search Pattern | Result |
|---|---|---|
| knowledge hook → middleware | `knowledge hook.*middleware` | NO HITS in doctrine files; one REFUTATION in HOOK_AKA_SOURCE_MAP.yaml ("not a middleware callback") |
| knowledge hook → callback | `knowledge hook.*callback` | NO HITS in doctrine files; one REFUTATION in HOOK_AKA_SOURCE_MAP.yaml |
| knowledge hook → event listener | `knowledge hook.*event.*listen` | NO HITS |
| knowledge hook → plugin | `knowledge hook.*plugin` | NO HITS |
| AKA → automation | `autonomic.*automation` | NO HITS in doctrine files |
| autoinstinct → agent framework | `autoinstinct.*agent.*framework` | NO HITS |
| ccog → chatbot | `ccog.*chatbot` | NO HITS; ccog NOT IN CORPUS (confirmed by search index) |
| LLM output → runtime authority | `LLM.*runtime.*authorit` | NO HITS |
| summary → evidence | `summary.*is.*evidence` | NO HITS (direct conflation) |

**External project excerpts** (GitVan, `~/gitvan`) in `BATCH_2_CRAWL_REPORT.txt` contain 18+ instances of "knowledge hook = automation." These are crawl artifacts from an external project being catalogued, not corpus claims. They are evidence hits from the external `~/gitvan` directory scan. No correction required for those excerpts.

---

## Thesis Chapter Audit Results

| Chapter | File | Forbidden Terms Found | Verdict |
|---|---|---|---|
| 00 Preface | `chapters/00_preface.tex` | None | CLEAN |
| 01 Origin | `chapters/01_origin_2016_language_model.tex` | None | CLEAN |
| 02 Prediction is Not Coordination | `chapters/02_prediction_is_not_coordination.tex` | "claimed automations" (lines 46, 48) | CONTEXTUALLY CORRECT — critiques AI product marketing using the forbidden term correctly; these are what the old regime falsely claims |
| 03 Enterprise Process Gap | `chapters/03_enterprise_process_gap.tex` | "automate processes" (line 44) | CONTEXTUALLY CORRECT — describes the failure mode of systems that claim to automate |
| 04 Chatman Equation | `chapters/04_chatman_equation.tex` | None | CLEAN |
| 05 Process Evidence and Receipts | `chapters/05_process_evidence_and_receipts.tex` | None | CLEAN |
| 06 ggen and Open Ontologies | `chapters/06_ggen_and_open_ontologies.tex` | None | CLEAN |
| 07 Command Grammar and Execution | `chapters/07_command_grammar_and_execution.tex` | None | CLEAN |
| 08 Post-Cyberpunk PCP | `chapters/08_post_cyberpunk_pcp.tex` | "claimed automation" (line 55) | CONTEXTUALLY CORRECT — identifies the failure mode |
| 09 AI XYNZ | `chapters/09_ai_xynz_and_capital_flow.tex` | None | CLEAN |
| 10 Industry Architecture | `chapters/10_industry_complete_architecture.tex` | "professional-services automation" (line 51) | CONTEXTUALLY CORRECT — industry integration reference to PSA platforms, not a frame law assertion |
| 11 Evaluation and Receipts | `chapters/11_evaluation_and_receipts.tex` | None | CLEAN |
| 12 Conclusion | `chapters/12_conclusion.tex` | None | CLEAN |

**Thesis chapter verdict: NO BAD TRANSLATIONS in any thesis chapter.** The forbidden terms appear only in contexts that correctly critique the old regime or describe external industry integration categories.

---

## Corrections Required

Ordered by severity:

### CORRECTION 1 (CRITICAL)
**File:** `/Users/sac/process-intelligence/standards/ocel.md:10`
**Fix:** Replace "OCEL is the ultimate cryptographic receipt" with "OCEL is the substrate for cryptographic receipting. An OCEL-grounded conformance receipt is the M&A diligence artifact, not the OCEL log itself."

### CORRECTION 2 (CRITICAL)
**File:** `/Users/sac/process-intelligence/audits/adversarial_audit_v30.1.1.md:12`
**Fix (part 1 — log/receipt):** Replace "the git commit log maintains a sound, continuous receipt chain of 567 commits" with "the git commit history contains 567 commits, providing the audit substrate from which the receipt chain is derived."
**Fix (part 2 — report/proof):** Replace "This audit report certifies" with "This audit report documents gate findings. Authority rests with the receipted gate verdicts referenced herein, not the report itself."

### CORRECTION 3 (CRITICAL)
**File:** `/Users/sac/process-intelligence/audits/drift_sentry_audit.md:13`
**Fix:** Replace "This audit report certifies that" with "This audit report documents findings showing that" — with an addendum: the conformance receipt for each finding is the replayed result, not this report.

### CORRECTION 4 (CRITICAL)
**File:** `/Users/sac/process-intelligence/audits/alignment_referee_audit.md:13`
**Fix:** Replace "This audit report certifies that" with "This audit report documents findings showing that" — same addendum as CORRECTION 3.

### CORRECTION 5 (MAJOR)
**File:** `/Users/sac/process-intelligence/audits/audit-lifecycle-completeness.md:135`
**Fix:** Remove "event logs" from the Receipt Shapes column. Replace with only the receipt types: "Operational KPI receipts, compliance exception receipts." Event logs belong in the PI Requirements column as inputs, not in Receipt Shapes as outputs.

### CORRECTION 6 (MAJOR)
**File:** `/Users/sac/process-intelligence/audits/audit-lifecycle-completeness.md:203`
**Fix:** Replace "Discovery receipt (DFG, heuristics log, alignment report)" with "Discovery receipt (typed DFG witness, conformance fitness receipt with BLAKE3 commitment)." Remove "heuristics log" and "alignment report" — these are logs and reports, not receipt components.

### CORRECTION 7 (MINOR)
**File:** `/Users/sac/process-intelligence/research/prompt-manufactory/ggen/ontology/hook-law.ttl:7`
**Fix:** Replace "Hooks are Claude Code lifecycle automations that enforce refusal conditions." with "Hooks are Claude Code Andon gates — deterministic enforcement points that enforce admission/refusal conditions at named lifecycle boundaries."

---

## Clean Instances — Correct Usage Found

| Term | File | Correct Usage |
|---|---|---|
| Receipt is not a log | `doctrine/FORMAL_OBJECTS_TAXONOMY.md:111` | "Replaces 'log'; a log is not a receipt" |
| Receipt vs hash distinction | `doctrine/RECEIPT_DOCTRINE.md:5` | "A receipt is not a hash. A hash proves integrity of bytes. A receipt proves something richer..." |
| Hook as Andon gate | `research/prompt-manufactory/ggen/ontology/prompt-manufactory.ttl:42` | `pm:HookPolicy a rdfs:Class ; rdfs:comment "Andon gate: deterministic lifecycle enforcement"` |
| Hook not middleware | `phd-thesis/ledgers/HOOK_AKA_SOURCE_MAP.yaml:179` | "HookPolicy is an Andon gate — a deterministic enforcement point, not a middleware callback" |
| Hook not middleware | `phd-thesis/research/knowledge-hooks/README.md` | "Frame law: a hook is an admission/refusal boundary, NOT middleware." |
| AKA is not automation | `doctrine/AUTONOMIC_KNOWLEDGE_ACTUATION.md:14` | "Autonomic knowledge actuation is the closed-loop discipline of self-managing process execution. It is not monitoring. It is not dashboards. It is not alerting." |
| Report is not proof | `phd-thesis/projects/checkpoints/02_lineage_and_context.tex` | "A research report that describes a doctrine does not constitute evidence" |
| Summary is not evidence | `phd-thesis/chapters/04_chatman_equation.tex` | "not a sample, not a summary, not a model-generated approximation" |
| LLM output not authority | `doctrine/PROCESS_INTELLIGENCE_IS_NOT.md` | "AI summarization has no lifecycle authority. It cannot emit typed receipts." |
| Telemetry is not evidence | `otel-weaver/doctrine/telemetry-is-not-process-evidence.md` | "Telemetry is a signal; evidence is a fact." |
| No receipt, no authority | `phd-thesis/research/knowledge-hooks/04_construct8_motion_boundary_map.md:167` | "No receipt, no authority" mapped to canonical corpus source |
| Frame Law direction | `phd-thesis/ledgers/HOOK_AKA_SOURCE_MAP.yaml:7` | Full Frame Law direction stated verbatim |

---

## Summary Table

| # | File | Line | Category | Severity |
|---|---|---|---|---|
| 1 | `standards/ocel.md` | 10 | log = receipt | CRITICAL |
| 2 | `audits/adversarial_audit_v30.1.1.md` | 12 | log = receipt chain | CRITICAL |
| 3 | `audits/adversarial_audit_v30.1.1.md` | 12 | report = proof | CRITICAL |
| 4 | `audits/drift_sentry_audit.md` | 13 | report = proof | CRITICAL |
| 5 | `audits/alignment_referee_audit.md` | 13 | report = proof | CRITICAL |
| 6 | `audits/audit-lifecycle-completeness.md` | 135 | log listed as receipt shape | MAJOR |
| 7 | `audits/audit-lifecycle-completeness.md` | 203 | log and report listed as receipt shape | MAJOR |
| 8 | `research/prompt-manufactory/ggen/ontology/hook-law.ttl` | 7 | hooks = automations | MINOR |

**Total bad translations found: 8**
**CRITICAL: 5 | MAJOR: 2 | MINOR: 1**
**Corrections required: 7** (findings 2 and 3 share one file/line and are addressed by two fixes on that line)
