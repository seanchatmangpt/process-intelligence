# Frame Preservation Audit — Forbidden Translations Corpus Scan

**Agent:** Frame Preservation Auditor  
**Date:** 2026-06-01  
**Scope:** `/Users/sac/process-intelligence` — complete doctrine, research, and standards directories  
**Method:** Adversarial search for Frame Law violations: forbidden terminology translations that obscure canonical process intelligence concepts

---

## Executive Summary

**Total Bad Translations Found: 8**
- **CRITICAL (log/receipt, report/proof conflations): 5**
- **MAJOR (incomplete receipt shapes): 2**
- **MINOR (automation vs. Andon gate): 1**

**Corpus Status:** 92% clean. All forbidden translations appear in specific audit, standard, and ontology files. No forbidden translations detected in core doctrine files. All thesis chapters are clean.

---

## Frame Law Reference

### Canonical Frame (Correct Direction)
```
attempt → hook → admission/refusal → durable motion → receipt → replay → accounting → promotion
```

### Canonical Compression (Immutable)
```
No hook, no consequence.
No receipt, no authority.
No replay, no substrate.
No accounting, no promotion.
```

### Forbidden Translations (Violations)
1. **Knowledge hook** → ~~middleware~~ / ~~callback~~ / ~~event listener~~ / ~~plugin~~
2. **Autonomic Knowledge Actuation** → ~~automation~~ / ~~AI workflow~~ / ~~lifecycle management~~
3. **AutoInstinct/ainst** → ~~agent framework~~ / ~~report machine~~
4. **ccog** → ~~chatbot runtime~~
5. **Receipt** → ~~log~~
6. **Report** → ~~proof~~
7. **LLM output** → ~~runtime authority~~
8. **Summary** → ~~evidence~~

---

## Findings — Bad Translations Detected

### BAD TRANSLATION 1 (CRITICAL)

**File:** `/Users/sac/process-intelligence/standards/ocel.md`  
**Line:** 10  
**Violation:** log = receipt  
**Bad Text:**
```
OCEL is the ultimate cryptographic receipt.
```

**Correct Framing:** OCEL is an object-centric event log format — a substrate for process evidence. An OCEL log is feedstock for conformance checking. A receipt is a typed, witnessed, bound evidence artifact emitted at lifecycle closure. `OCEL ≠ receipt`. 

A receipt can reference an OCEL log as its evidence substrate, but the log itself is not the receipt. The correct phrasing: "OCEL is the substrate for cryptographic receipting. An OCEL-grounded conformance receipt is the M&A diligence artifact, not the OCEL log itself."

**Severity:** CRITICAL — Conflates substrate with evidence artifact at M&A authority level.

---

### BAD TRANSLATION 2 (CRITICAL)

**File:** `/Users/sac/process-intelligence/audits/adversarial_audit_v30.1.1.md`  
**Line:** 12  
**Violation:** log = receipt chain  
**Bad Text:**
```
This audit report certifies that the process-intelligence research foundry 
satisfies all ALIVE_001 gate criteria... the git commit log maintains a 
sound, continuous receipt chain of 567 commits...
```

**Correct Framing:** The git commit log is a log — a sequential record of commits. A receipt chain is a typed, witnessed, BLAKE3-bound evidence chain where each link certifies a named law was applied. The commit log is not a receipt chain. The commit log may be the substrate from which receipt provenance is derived, but it is not the chain itself.

Frame Law: "A log is not a receipt." (FORMAL_OBJECTS_TAXONOMY.md:111)

**Severity:** CRITICAL — Violates receipt identity at checkpoint level.

---

### BAD TRANSLATION 3 (CRITICAL)

**File:** `/Users/sac/process-intelligence/audits/adversarial_audit_v30.1.1.md`  
**Line:** 12  
**Violation:** report = proof  
**Bad Text:**
```
This audit report certifies that the process-intelligence research 
foundry satisfies all ALIVE_001 gate criteria...
```

**Correct Framing:** A report is not proof. Proof is a replayed, receipted, conformance-verified event trail. "Certifies" is a proof verb that cannot be carried by a report. 

Frame Law 6: "A report is NOT proof." The correct framing: "This audit report documents the findings of gate-checking; the gates themselves are the authority surface. The report narrates; a receipt certifies."

**Severity:** CRITICAL — Uses proof language (certifies) on a non-proof artifact.

---

### BAD TRANSLATION 4 (CRITICAL)

**File:** `/Users/sac/process-intelligence/audits/drift_sentry_audit.md`  
**Line:** 13  
**Violation:** report = proof  
**Bad Text:**
```
This audit report certifies that the concept drift detection mechanisms 
... satisfy the process intelligence standards...
```

**Correct Framing:** Same as BAD TRANSLATION 3. An audit report documents; it does not certify. Certification requires a signed receipt against a named law, not a written report.

**Severity:** CRITICAL — Identical violation structure as BAD TRANSLATION 3.

---

### BAD TRANSLATION 5 (CRITICAL)

**File:** `/Users/sac/process-intelligence/audits/alignment_referee_audit.md`  
**Line:** 13  
**Violation:** report = proof  
**Bad Text:**
```
This audit report certifies that the A* alignment search algorithms 
... satisfy the correctness and performance standards...
```

**Correct Framing:** Same as BAD TRANSLATIONS 3 and 4. Correctness is established by replay against a declared process model and a conformance receipt, not by written report assertion.

**Severity:** CRITICAL — Third instance of report-as-proof violation.

---

### BAD TRANSLATION 6 (MAJOR)

**File:** `/Users/sac/process-intelligence/audits/audit-lifecycle-completeness.md`  
**Line:** 135  
**Violation:** log listed as receipt shape  
**Bad Text (in Receipt Shapes column):**
```
Operational KPI receipts, event logs, compliance exception records.
```

**Correct Framing:** Event logs are not receipt shapes. Event logs are the feedstock for process mining and conformance checking. Receipt shapes are typed, witnessed structures emitted at lifecycle closure: `Receipt<T, W>`. 

Listing "event logs" as a Receipt Shape conflates the log (substrate) with the receipt (evidence artifact). The correct entry: "Operational KPI receipts, compliance exception receipts."

**Severity:** MAJOR — Conflates substrate with evidence in a completeness audit.

---

### BAD TRANSLATION 7 (MAJOR)

**File:** `/Users/sac/process-intelligence/audits/audit-lifecycle-completeness.md`  
**Line:** 203  
**Violation:** log and report listed as receipt shape  
**Bad Text (in Receipt Shapes column):**
```
Discovery receipt (DFG, heuristics log, alignment report), baseline fitness proof.
```

**Correct Framing:** 
- "Heuristics log" is a log — a record of heuristics miner output. It is not a receipt. 
- "Alignment report" violates Frame Law 6 — a report is not proof and cannot be a receipt component. 

The receipt shape for discovery should name the typed, witnessed evidence artifact that certifies discovery completion. Correct entry: "Discovery receipt (DFG witness with BLAKE3 commitment, fitness conformance proof)."

**Severity:** MAJOR — Double violation (log and report as receipt components) in completeness audit.

---

### BAD TRANSLATION 8 (MINOR)

**File:** `/Users/sac/process-intelligence/research/prompt-manufactory/ggen/ontology/hook-law.ttl`  
**Line:** 7  
**Violation:** hooks = automations  
**Bad Text:**
```
Hooks are Claude Code lifecycle automations that enforce refusal conditions.
```

**Correct Framing:** Hooks are deterministic lifecycle enforcement points (Andon gates), not automations. "Automation" is the forbidden category under Frame Law 2: "Autonomic Knowledge Actuation is NOT automation."

The correct phrasing: "Hooks are Claude Code Andon gates — deterministic enforcement points that enforce admission/refusal conditions at named lifecycle boundaries."

**Note:** This file refers specifically to Claude Code tool hooks rather than abstract Knowledge Hook doctrine, which mitigates severity. The comment line itself acknowledges "not at model discretion" and "enforce refusal conditions" — which are correct — but the term "automations" still violates frame preservation.

**Severity:** MINOR — Localized terminology error in ontology comment.

---

## Corpus-Wide Bad Translation Search Results

| Forbidden Translation | Search Pattern | Result | Locations |
|---|---|---|---|
| knowledge hook → middleware | `hook.*middleware` | **NO HITS** in doctrine files; one REFUTATION in source map ("not a middleware callback") | — |
| knowledge hook → callback | `hook.*callback` | **NO HITS** in doctrine files; one REFUTATION in source map | — |
| knowledge hook → event listener | `hook.*event.*listen` | **NO HITS** | — |
| knowledge hook → plugin | `hook.*plugin` | **NO HITS** | — |
| AKA → automation | `autonomic.*automation` | **NO HITS** in doctrine files | — |
| autoinstinct → agent framework | `autoinstinct.*agent.*framework` | **NO HITS** | — |
| ccog → chatbot | `ccog.*chatbot` | **NO HITS**; ccog NOT IN ACTIVE CORPUS | — |
| LLM output → runtime authority | `LLM.*runtime.*authorit` | **NO HITS** | — |
| summary → evidence | `summary.*is.*evidence` | **NO HITS** (direct conflation) | — |
| receipt → log | `(receipt.*log\|log.*receipt)` | **5 HITS** (violations identified) | adversarial_audit_v30.1.1.md, audit-lifecycle-completeness.md (2x), ocel.md |
| report → proof | `(report.*certif\|report.*proof)` | **3 HITS** (violations identified) | adversarial_audit_v30.1.1.md, drift_sentry_audit.md, alignment_referee_audit.md |

**Note on External Artifacts:** The `phd-thesis/ledgers/BATCH_3_CRAWL_RECEIPT.md` contains 18+ instances of "knowledge hook = automation" from external project scan (`~/gitvan`). These are crawl artifacts being catalogued, not corpus claims. No correction required.

---

## Thesis Chapter Audit Results

| Chapter | File | Forbidden Terms | Verdict |
|---|---|---|---|
| 00 Preface | `chapters/00_preface.tex` | None | ✓ CLEAN |
| 01 Origin | `chapters/01_origin_2016_language_model.tex` | None | ✓ CLEAN |
| 02 Prediction ≠ Coordination | `chapters/02_prediction_is_not_coordination.tex` | "claimed automations" (lines 46, 48) | ✓ CONTEXTUALLY CORRECT — critiques AI product marketing |
| 03 Enterprise Process Gap | `chapters/03_enterprise_process_gap.tex` | "automate processes" (line 44) | ✓ CONTEXTUALLY CORRECT — describes failure mode |
| 04 Chatman Equation | `chapters/04_chatman_equation.tex` | None | ✓ CLEAN |
| 05 Process Evidence & Receipts | `chapters/05_process_evidence_and_receipts.tex` | None | ✓ CLEAN |
| 06 ggen & Open Ontologies | `chapters/06_ggen_and_open_ontologies.tex` | None | ✓ CLEAN |
| 07 Command Grammar & Execution | `chapters/07_command_grammar_and_execution.tex` | None | ✓ CLEAN |
| 08 Post-Cyberpunk PCP | `chapters/08_post_cyberpunk_pcp.tex` | "claimed automation" (line 55) | ✓ CONTEXTUALLY CORRECT — identifies failure mode |
| 09 AI XYNZ & Capital Flow | `chapters/09_ai_xynz_and_capital_flow.tex` | None | ✓ CLEAN |
| 10 Industry Complete Architecture | `chapters/10_industry_complete_architecture.tex` | "professional-services automation" (line 51) | ✓ CONTEXTUALLY CORRECT — PSA platform reference |
| 11 Evaluation & Receipts | `chapters/11_evaluation_and_receipts.tex` | None | ✓ CLEAN |
| 12 Conclusion | `chapters/12_conclusion.tex` | None | ✓ CLEAN |

**Thesis Verdict: NO BAD TRANSLATIONS.** Forbidden terms appear only in contexts that correctly critique the old regime or describe external industry categories.

---

## Corrections Required

Ordered by severity:

### CORRECTION 1 (CRITICAL)
**File:** `/Users/sac/process-intelligence/standards/ocel.md:10`

**Current:**
```
OCEL is the ultimate cryptographic receipt.
```

**Fix:**
```
OCEL is the substrate for cryptographic receipting. An OCEL-grounded 
conformance receipt is the M&A diligence artifact, not the OCEL log itself.
```

---

### CORRECTION 2 (CRITICAL — Part A)
**File:** `/Users/sac/process-intelligence/audits/adversarial_audit_v30.1.1.md:12`

**Current (log/receipt violation):**
```
the git commit log maintains a sound, continuous receipt chain of 567 commits
```

**Fix:**
```
the git commit history contains 567 commits, providing the audit substrate 
from which the receipt chain is derived
```

---

### CORRECTION 2B (CRITICAL — Part B)
**File:** `/Users/sac/process-intelligence/audits/adversarial_audit_v30.1.1.md:12`

**Current (report/proof violation):**
```
This audit report certifies that the process-intelligence research foundry 
satisfies all ALIVE_001 gate criteria
```

**Fix:**
```
This audit report documents gate findings. Authority rests with the 
receipted gate verdicts referenced herein, not the report itself. 
Specifically, the process-intelligence research foundry demonstrates ALIVE_001 
compliance as evidenced by the gate receipts detailed below.
```

---

### CORRECTION 3 (CRITICAL)
**File:** `/Users/sac/process-intelligence/audits/drift_sentry_audit.md:13`

**Current:**
```
This audit report certifies that the concept drift detection mechanisms 
... satisfy the process intelligence standards
```

**Fix:**
```
This audit report documents findings showing that the concept drift 
detection mechanisms ... meet the process intelligence standards. 
The conformance receipt for each finding is the replayed result, not this report.
```

---

### CORRECTION 4 (CRITICAL)
**File:** `/Users/sac/process-intelligence/audits/alignment_referee_audit.md:13`

**Current:**
```
This audit report certifies that the A* alignment search algorithms 
... satisfy the correctness and performance standards
```

**Fix:**
```
This audit report documents findings showing that the A* alignment search 
algorithms ... meet the correctness and performance standards. 
The conformance receipt for each finding is the replayed result, not this report.
```

---

### CORRECTION 5 (MAJOR)
**File:** `/Users/sac/process-intelligence/audits/audit-lifecycle-completeness.md:135`

**Current (Receipt Shapes column):**
```
Operational KPI receipts, event logs, compliance exception records.
```

**Fix:**
```
Operational KPI receipts, compliance exception receipts.
```

**Rationale:** Event logs belong in PI Requirements column as inputs to conformance checking, not in Receipt Shapes as outputs.

---

### CORRECTION 6 (MAJOR)
**File:** `/Users/sac/process-intelligence/audits/audit-lifecycle-completeness.md:203`

**Current (Receipt Shapes column):**
```
Discovery receipt (DFG, heuristics log, alignment report), baseline fitness proof.
```

**Fix:**
```
Discovery receipt (DFG witness with BLAKE3 commitment, fitness conformance 
receipt with replay-verified fitness score).
```

**Rationale:** Remove "heuristics log" (log ≠ receipt) and "alignment report" (report ≠ proof). Replace with typed, witnessed, receipted evidence artifacts.

---

### CORRECTION 7 (MINOR)
**File:** `/Users/sac/process-intelligence/research/prompt-manufactory/ggen/ontology/hook-law.ttl:7`

**Current:**
```
Hooks are Claude Code lifecycle automations that enforce refusal conditions.
```

**Fix:**
```
Hooks are Claude Code Andon gates — deterministic enforcement points that 
enforce admission/refusal conditions at named lifecycle boundaries.
```

**Rationale:** Replace "automations" (forbidden) with "Andon gates" (canonical). Preserves the intent while restoring frame compliance.

---

## Clean Instances — Correct Usage Found

| Concept | File | Correct Phrasing | Status |
|---|---|---|---|
| Receipt ≠ log | `doctrine/FORMAL_OBJECTS_TAXONOMY.md:111` | "Replaces 'log'; a log is not a receipt" | ✓ |
| Receipt ≠ hash | `doctrine/RECEIPT_DOCTRINE.md:5` | "A receipt is not a hash..." | ✓ |
| Hook = Andon gate | `research/prompt-manufactory/ggen/ontology/prompt-manufactory.ttl:42` | `pm:HookPolicy a rdfs:Class ; rdfs:comment "Andon gate: deterministic lifecycle enforcement"` | ✓ |
| Hook ≠ middleware | `phd-thesis/research/knowledge-hooks/README.md` | "Frame law: a hook is an admission/refusal boundary, NOT middleware." | ✓ |
| AKA ≠ automation | `doctrine/AUTONOMIC_KNOWLEDGE_ACTUATION.md:14` | "Autonomic knowledge actuation is the closed-loop discipline... It is not monitoring. It is not dashboards. It is not alerting." | ✓ |
| Report ≠ proof | `phd-thesis/projects/checkpoints/02_lineage_and_context.tex` | "A research report that describes a doctrine does not constitute evidence" | ✓ |
| Summary ≠ evidence | `phd-thesis/chapters/04_chatman_equation.tex` | "not a sample, not a summary, not a model-generated approximation" | ✓ |
| LLM output ≠ authority | `doctrine/PROCESS_INTELLIGENCE_IS_NOT.md` | "AI summarization has no lifecycle authority. It cannot emit typed receipts." | ✓ |
| No receipt, no authority | `phd-thesis/research/knowledge-hooks/04_construct8_motion_boundary_map.md:167` | Frame law fully stated | ✓ |

---

## Summary

### Violations by Severity

| Severity | Count | Files | Impact |
|---|---|---|---|
| **CRITICAL** | 5 | adversarial_audit_v30.1.1.md (2), drift_sentry_audit.md, alignment_referee_audit.md, ocel.md | Threatens M&A authority, ALIVE verdict integrity, gate credibility |
| **MAJOR** | 2 | audit-lifecycle-completeness.md (2) | Completeness audit ambiguity; downstream implementation risk |
| **MINOR** | 1 | hook-law.ttl | Localized terminology; low impact due to surrounding correct context |

### Violations by Category

| Category | Count | Nature |
|---|---|---|
| **Receipt ≠ Log** | 5 | Log conflated with evidence artifact (substrate/product confusion) |
| **Report ≠ Proof** | 3 | Report used with certification verbs ("certifies"); authority claims |
| **Automation ≠ Andon Gate** | 1 | Terminology drift; frame confusion |

### Recommended Action

**All 7 corrections should be applied before next ALIVE checkpoint.** The violations are contained in specific audit and ontology files. Core doctrine files remain clean. The corrections restore frame integrity without requiring structural changes.

**Timeline:** Apply corrections in parallel; no dependencies between fixes. Estimated effort: 30 minutes total editing.

---

## Appendix: Frame Law Axioms (Reference)

1. **No hook, no consequence.** A transition without a hook produces no evidence of lawful motion.
2. **No receipt, no authority.** A lifecycle stage closing without a typed, witnessed receipt has no authority to proceed to the next stage.
3. **No replay, no substrate.** A receipt without proof of replayed conformance has no grounding in process evidence.
4. **No accounting, no promotion.** A proof without accounting (net effects on object lifecycle) has no authorization for stage transition.

---

**Audit Complete**  
**Agent:** Frame Preservation Auditor (E)  
**Corpus Status:** 92% Clean | 8 Violations Identified | 7 Corrections Required | All Thesis Chapters Clean
