# M&A Board Projection Renderer — Specification & Execution Plan

**Version:** 1.0  
**Status:** Authority stack complete, ready for execution  
**Date:** 2026-06-01

---

## Overview

The M&A Board Projection Renderer is a seven-step pipeline that transforms board-admissible process mining claims into cryptographically-sealed PowerPoint presentations and Excel workbooks suitable for executive acquisition review and board-level decision-making.

**Key Doctrine:** Every slide claim must reference a cryptographic receipt. No claim without a receipt is admissible. No receipt without underlying event log evidence is valid.

---

## Inputs Required for Execution

### 1. Claims Array (JSON)

A JSON array of board-admissible claims, each with:

```json
{
  "claim": "https://process.intelligence/ma/<claim-id>",
  "claimLabel": "Descriptive claim text",
  "claimType": "ma:SynergyProjection|ma:OperationalDebtClaim|ma:IntegrationRiskAssertion|...",
  "metricValue": 1250000,
  "metricThreshold": 95,
  "verdictFitness": 0.982,
  "verdictPrecision": 0.945,
  "verdictGeneralization": 0.91,
  "verdictSimplicity": 0.85,
  "receiptHash": "8a3e811c22904e2aa0f3215903b41fa",
  "receiptTimestamp": "2026-05-31T23:14:00Z",
  "receipt": "rec_ebitda_rework_001.json",
  "logFormat": "ocel:OCEL2.0",
  "verdict": "https://process.intelligence/wasm4pm/conformance-verdict-001",
  "replayTrace": "https://process.intelligence/wasm4pm/replay-trace-001",
  "traceDeviations": 0,
  "traceGasToReturn": 0,
  "relatedActivity": ["Purchase Order Rework", "Invoice Review"],
  "remediationEffortHours": 240,
  "operationalDebtIfApplicable": null,
  "synergyCategoryIfApplicable": "Labor efficiency",
  "riskSeverity": null,
  "activityBottleneck": false,
  "statePhase": "Optimization"
}
```

**Minimum Valid Claim:**
- `claim` (URI)
- `claimLabel` (text)
- `claimType` (enum)
- `metricValue` (number)
- `verdictFitness` (0.0–1.0, must be ≥ 0.95 for board admission)
- `verdictPrecision` (0.0–1.0, must be ≥ 0.90)
- `receiptHash` (BLAKE3 hash)
- `receipt` (filename)

### 2. Receipt Files (JSON, JCS-canonical)

One JSON file per claim, stored in `/Users/sac/process-intelligence/receipts/`:

**Sample:** `/Users/sac/process-intelligence/receipts/rec_ebitda_rework_001.json`

```json
{
  "slide_id": "8a3e811c-2290-482a-a5f1-3215903b41fa",
  "slide_title": "Slide 1: EBITDA Optimization via Process Rework Reduction",
  "assertion_text": "Annual EBITDA will increase by $1,250,000 by reducing...",
  "target_log_hash": "ee9ab7234bcaf5e1613ab7d5f45af28229552e9327880cc2d4b97f193df4971a",
  "process_model_hash": "81f7dca25ba3594074888c74547b0e70796a2082f9cda3b2c12a843e620581ba9",
  "query_definition": {
    "engine": "wasm4pm",
    "query_uri": "file:///Users/sac/process-intelligence/sources/wasm4pm/queries/ebitda_rework_reduction.wasm",
    "parameters": {
      "rework_activities": ["Change Price", "Modify Order", "Re-enter Order"],
      "projected_annual_volume": 500000,
      "labor_cost_per_event_usd": 2.00,
      "baseline_rework_intensity": 1.45,
      "target_rework_intensity": 0.20
    }
  },
  "verification_results": {
    "fitness": 0.982,
    "precision": 0.945,
    "throughput_days": 4.12,
    "ebitda_impact_usd": 1250000.0
  },
  "validator_signature": "ed25519_sig_8a3e811c_..."
}
```

**Required Receipt Fields:**
- `slide_id` (UUIDv4)
- `assertion_text` (claim text)
- `target_log_hash` (BLAKE3)
- `process_model_hash` (BLAKE3)
- `query_definition.engine` (wasm4pm or pm4py)
- `verification_results.fitness` (≥ 0.95)
- `verification_results.precision` (≥ 0.90)
- `validator_signature` (Ed25519)

### 3. Templates (Tera2)

Already in place:
- `/Users/sac/process-intelligence/ggen/templates/ma-deck.tera` (202 lines)
- `/Users/sac/process-intelligence/ggen/templates/ma-diligence.tera` (283 lines)

---

## Seven-Step Execution Pipeline

### Step 1: Query Board-Admissible Claims

**Input:** `/Users/sac/process-intelligence/ma/` (40 claim definition files)

**Operations:**
1. Read all claim definition files
2. Extract structured claim metadata (taxonomy type, admissibility rules, financial formulas)
3. Build RDF graph or JSON-LD ontology of claim relationships
4. Filter claims that meet board admissibility gates:
   - Fitness ≥ 0.95
   - Precision ≥ 0.90
   - Generalization ≥ 0.85 (optional)
   - Simplicity ≥ 0.80 (optional)
   - Receipt hash present
   - logFormat ∈ {XES, OCEL2.0}

**Output:** Validated claims array (JSON)

**Authority Reference:**
- `/Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md`
- `/Users/sac/process-intelligence/ma/define_board_claim_taxonomy.md`

---

### Step 2: Cross-Reference wasm4pm Rendered Modules

**Input:** Claims array + `/Users/sac/process-intelligence/sources/wasm4pm/` (authority maps)

**Operations:**
1. For each claim, locate corresponding wasm4pm authority:
   - `verdictFitness` → `wasm4pm_conformance_authority_generation.md` (v30.1.2, A* alignment)
   - `verdict` (conformance verdict) → Conformance Authority Module
   - `replayTrace` (token game replay) → Replay Authority Module
   - `logFormat` (OCEL2.0 vs XES) → Mining Authority Module

2. Verify authority signatures and timestamps:
   - Check `validator_signature` in each receipt against pinned public key
   - Verify receipt timestamp is recent (within N days of rendering)

3. Confirm all metrics are calculated using board-admissible algorithms:
   - Fitness: Adriansyah 2014 optimal alignments (A* search)
   - Precision: Escaping Transitions Cardinality (ETC) or alignment-driven state space
   - Generalization: transition coverage
   - Simplicity: inverse model complexity

**Output:** Augmented claims array with verified wasm4pm metadata

**Authority References:**
- `/Users/sac/process-intelligence/receipts/wasm4pm_conformance_authority_generation.md`
- `/Users/sac/process-intelligence/receipts/wasm4pm_mining_generation.md`
- `/Users/sac/process-intelligence/receipts/wasm4pm_replay_generation.md`
- `/Users/sac/process-intelligence/receipts/wasm4pm_lifecycle_generation.md`

---

### Step 3: Cross-Reference Compat Evidence Ledger

**Input:** Augmented claims array + `/Users/sac/process-intelligence/sources/wasm4pm-compat/`

**Operations:**
1. For each claim, verify type-safety across compat boundary:
   - Claim type (ma:SynergyProjection, ma:OperationalDebtClaim, etc.) maps to Evidence<T, State, Witness> type
   - T = ConformanceVerdicts | DiscoveryReceipt | ReplayReceipt | LifecyclePhaseTransition
   - State = AlignmentState | DiscoveryState | ReplayState | PhaseState
   - Witness = ConformanceWitness | MiningWitness | ReplayWitness | PhaseWitness

2. Verify cryptographic integrity:
   - BLAKE3 hashes of event logs match `target_log_hash` in receipt
   - Ed25519 signatures on receipts verify against auditor public key
   - JCS (RFC 8785) canonical serialization used for signature verification

3. Confirm all receipts follow immutable naming convention:
   - `rec_<claim-id>.json` or `rec_<metric-type>_<sequence>.json`

**Output:** Compat-verified claims array with type-safe assurances

**Authority References:**
- `/Users/sac/process-intelligence/receipts/RECEIPT_REGISTRY.md`
- `/Users/sac/process-intelligence/sources/wasm4pm-compat/` (type boundaries)

---

### Step 4: Apply Templates (Tera2)

**Input:** Compat-verified claims array

**Operations:**

#### 4.1: Render PowerPoint Structure (ma-deck.tera)

Template iterates over claims and generates:

1. **Title Slide**
   - Title: "Process Intelligence Acquisition Overview"
   - Subtitle: "Conformance-Backed Operational Claims"
   - Timestamp: Render date
   - Confidentiality: "Board of Directors Only"

2. **Executive Summary**
   - Total claims: {{ claims | length }}
   - Total quantified value: ${{ claims | map(attribute='metricValue') | sum }}
   - Average fitness: {{ (claims | map(attribute='verdictFitness') | sum / claims | length * 100) | round(precision=1) }}%
   - Average precision: {{ (claims | map(attribute='verdictPrecision') | sum / claims | length * 100) | round(precision=1) }}%
   - Synergy projections: {{ claims | filter(attribute='claimType', value='ma:SynergyProjection') | length }}
   - Operational debt items: {{ claims | filter(attribute='claimType', value='ma:OperationalDebtClaim') | length }}

3. **Claim Detail Slides** (1 per claim)
   - Claim title and description
   - Category (Synergy, Debt, Risk, etc.)
   - Quantified impact: ${{ metricValue }}
   - Fitness: {{ verdictFitness * 100 | round(precision=2) }}% [ADMISSIBLE if ≥ 95%]
   - Precision: {{ verdictPrecision * 100 | round(precision=2) }}% [ADMISSIBLE if ≥ 90%]
   - Receipt hash: {{ receiptHash | truncate(length=16) }}... (full hash linked in data room)
   - Event log format: {{ logFormat }}
   - Conformance verdict URI: {{ verdict }}

4. **Operational Debt Slide** (if debt items present)
   - Table of debt categories, descriptions, remediation effort, fitness impact
   - Total remediation hours
   - Post-remediation fitness projection: 0.98

5. **Synergy Waterfall Slide** (if synergy items present)
   - Waterfall breakdown by category
   - Confidence level (derived from fitness percentage)
   - Timeline (Year 1 if value > $500k, else Year 2)
   - Total synergy value
   - Weighted probability

6. **Conformance Model Summary**
   - Model soundness: PROVEN (van der Aalst 1998 WF-net framework)
   - Event log provenance: CRYPTOGRAPHICALLY_CHAINED (BLAKE3 + system signatures)
   - Alignment algorithm: OPTIMAL_ALIGNMENTS (Adriansyah 2014, A* search)
   - Aggregate statistics: Avg fitness, avg precision

7. **Board Sign-Off Slide**
   - "All claims are backed by optimal alignment-based conformance metrics"
   - "Event logs conform to IEEE 1849-2016 (XES) or OCEL 2.0 standards"
   - "Each receipt is cryptographically verifiable and audit-repeatable"
   - "Process models are proven sound under van der Aalst (1998) WF-net framework"
   - "This presentation is valid only when accompanied by receipts in the data room"
   - Presentation UUID
   - Generation timestamp

#### 4.2: Render Excel Workbook Structure (ma-diligence.tera)

Template generates 7 worksheets:

1. **Executive_Summary**
   - Total claims
   - Total value
   - Avg fitness
   - Avg precision
   - Claims meeting board threshold

2. **Synergy_Claims**
   - Columns: Claim ID, Description, Category, Annual Value, Phase, Fitness, Precision, Verdict, Receipt, Verify URL, Activity Count
   - One row per synergy claim

3. **Operational_Debt**
   - Columns: Debt ID, Category, Description, Remediation Hours, Est. Cost, Activities, Current Fitness, Post Fitness, Deviations, Priority, Owner
   - One row per debt item

4. **Integration_Risks**
   - Columns: Risk ID, Description, Severity, Fitness, Probability, Impact, Mitigation, Timeline, Owner, Evidence, Approval Required
   - One row per risk claim

5. **Replay_Traces**
   - Columns: Claim ID, Trace ID, Log Format, Deviations, Gas, Fitness, Precision, Alignment Cost, Worst Case, Receipt, Timestamp, Repeatable
   - One row per claim

6. **Activity_Impact**
   - Columns: Activity Name, Activity ID, Related Claims, Bottleneck, Total Impact, Remediation Hours, Affected Claims, Opportunity, Phase
   - Aggregated impact analysis by activity

7. **Governance**
   - Requirement, Status (VERIFIED), Evidence, Signed By, Date
   - 5 rows covering: log format compliance, event chaining, fitness thresholds, model soundness, receipt availability

**Output:** Two Tera2-rendered JSON intermediate structures ready for PowerPoint and Excel generation

---

### Step 5: Generate Target Artifacts (pptx-rs + calamine)

**Input:** Rendered JSON structures from ma-deck.tera and ma-diligence.tera

**Operations:**

#### 5.1: Render PowerPoint (ma/acquisition_ready_deck_FINAL.pptx)

1. Create new presentation metadata:
   - Title: "Process Intelligence Acquisition Package"
   - Subject: "M&A Transaction Support"
   - Keywords: ["process-mining", "conformance", "board-admissible", "m&a"]
   - Created: {{ now() }}
   - Modified: {{ now() }}

2. Add each slide from rendered JSON:
   - Title slide (no receipt)
   - Executive summary (aggregate metrics)
   - N claim detail slides (each with receipt reference)
   - Operational debt slide (if present)
   - Synergy waterfall (if present)
   - Conformance summary
   - Board sign-off

3. For each slide, embed:
   - Visible: claim label, metric value, fitness/precision percentages, category
   - Metadata: slide_id (UUIDv4), receipt_hash, receipt_filename, verification_url

4. Add speaker notes to each claim slide:
   - Claim definition text
   - Calculation method (e.g., "Adriansyah 2014 optimal alignments")
   - Related activity list
   - Remediation path (if applicable)

**Output File:** `/Users/sac/process-intelligence/ma/acquisition_ready_deck_FINAL.pptx`

#### 5.2: Render Excel Workbook (ma/diligence_workbook_FINAL.xlsx)

1. Create workbook metadata:
   - Title: "M&A Due Diligence - Process Intelligence Claims"
   - Created: {{ now() }}
   - Author: "Process Intelligence Team"

2. Add each worksheet from rendered JSON:
   - Executive_Summary (summary metrics)
   - Synergy_Claims (synergy detail, 1 row per claim)
   - Operational_Debt (debt inventory)
   - Integration_Risks (risk assessment)
   - Replay_Traces (conformance evidence detail)
   - Activity_Impact (process activity analysis)
   - Governance (compliance checklist)

3. Format each worksheet:
   - Header row (bold, colored background)
   - Data rows with cell types: text, number (USD or %), percentage, date, URL
   - Conditional formatting: Fitness/Precision cells colored red (<90%), yellow (90–95%), green (≥95%)
   - Freeze panes on header row

4. Add data validation:
   - Claim ID column: links to VDR receipt files (hyperlinks)
   - Receipt column: clickable links to `/receipts/rec_*.json`
   - Verdict column: clickable links to wasm4pm verdicts

**Output File:** `/Users/sac/process-intelligence/ma/diligence_workbook_FINAL.xlsx`

---

### Step 6: Validate Artifacts

**Input:** Generated PowerPoint and Excel files

**Operations:**

#### 6.1: Receipt Cross-Reference Validation

For each claim slide in PowerPoint:
1. Extract `slide_id` from slide metadata
2. Lookup `/Users/sac/process-intelligence/receipts/rec_<slide_id>.json`
3. Verify file exists and is readable
4. Parse receipt JSON
5. Verify receipt structure:
   ```
   Receipt has:
   ✓ slide_id (matches slide)
   ✓ assertion_text (non-empty)
   ✓ target_log_hash (64-char hex, BLAKE3)
   ✓ process_model_hash (64-char hex, BLAKE3)
   ✓ query_definition.engine (wasm4pm or pm4py)
   ✓ query_definition.query_uri (non-empty)
   ✓ verification_results.fitness (0.0–1.0, ≥ 0.95 for board admission)
   ✓ verification_results.precision (0.0–1.0, ≥ 0.90)
   ✓ validator_signature (Ed25519, 128 hex chars)
   ```

6. Verify cryptographic signature:
   ```
   1. Remove validator_signature field from receipt JSON
   2. Serialize remaining JSON using JCS (RFC 8785)
   3. Compute Ed25519-Verify(pinned_auditor_pk, serialized_json, signature)
   4. Assert result == True
   ```

**Validation Result:** 
- **PASS** if all receipts present, structure valid, signatures verify
- **FAIL** if any receipt missing, invalid, or signature fails

#### 6.2: Witness Marker Verification

For each claim in Excel workbook:
1. Verify `verdictFitness` is populated (not null)
2. Verify `verdictFitness` ≥ 0.95
3. Verify `verdictPrecision` is populated (not null)
4. Verify `verdictPrecision` ≥ 0.90
5. Verify `verdict` URI references wasm4pm conformance verdict
6. Verify `replayTrace` URI references wasm4pm replay trace
7. Verify `logFormat` ∈ {"XES", "OCEL 2.0"}

**Validation Result:**
- **PASS** if all claims have witnesses and meet thresholds
- **FAIL** if any claim missing witnesses or below thresholds

#### 6.3: Board Compliance Justification

For each call-out in PowerPoint:
1. Verify claim type matches board claim taxonomy:
   - ma:SynergyProjection
   - ma:OperationalDebtClaim
   - ma:IntegrationRiskAssertion
   - ma:ControlClaim
   - ma:ProcessAssetClaim
   - ma:ProcessLiabilityClaim
   - ma:ScalabilityClaim

2. Verify compliance with board admissibility rules:
   - **Rule 1 — Event Log Integrity:** Log format declared (XES or OCEL2.0), BLAKE3 hash in receipt ✓
   - **Rule 2 — Mathematical Conformance:** Fitness and precision from Adriansyah optimal alignments ✓
   - **Rule 3 — Structural Soundness:** Model is WF-net (soundness certified) or process tree (valid arity) ✓
   - **Rule 4 — Cryptographic Mapping:** Slide UUID maps to receipt, signature verifiable ✓

**Validation Result:**
- **PASS** if all rules satisfied
- **FAIL** if any rule violated

**Output File:** Validation report (markdown or JSON) detailing results per slide/claim

---

### Step 7: Seal with Receipt Ledger

**Input:** Validated PowerPoint, Excel, and all receipt JSON files

**Operations:**

#### 7.1: Assemble Receipt Ledger Markdown

Create `/Users/sac/process-intelligence/receipts/ma_deck_rendering.md`:

```markdown
# M&A Board Projection Rendering — Receipt Ledger

**Presentation UUID:** ma-deck-{{ now() | date(format="%s") }}
**Generated:** {{ now() | date(format="%Y-%m-%dT%H:%M:%SZ") }}
**Validator:** Process Intelligence Team
**Authority:** Board-Admissible Claim Requirements v1.0

---

## Board Admissibility Declaration

This presentation is valid only when accompanied by cryptographic receipts and event logs. All claims have been validated using:

- **Fitness Metric:** van der Aalst token-based replay with optimal alignment (Adriansyah 2014)
- **Precision Metric:** Alignment-driven state space analysis (Escaping Transitions Cardinality)
- **Model Soundness:** Petri net WF-net soundness proofs (van der Aalst 1998)
- **Event Log Integrity:** BLAKE3 hash chains with system signatures at transaction commit
- **Receipt Cryptography:** Ed25519 signatures with JCS (RFC 8785) canonical serialization

---

## Slide-to-Receipt Registry

[Table with columns: Slide #, Slide Title, Claim ID, Metric Value, Fitness %, Precision %, Receipt File, Receipt Hash, Validator Signature, Status]

| # | Title | Claim ID | Value | Fitness | Precision | Receipt | Hash | Signature | Status |
|---|-------|----------|-------|---------|-----------|---------|------|-----------|--------|
| 1 | Title Slide | - | - | - | - | - | - | - | N/A |
| 2 | Executive Summary | - | - | - | - | - | - | - | N/A |
| 3 | Claim: EBITDA via Rework | rec_ebitda_rework_001 | $1.25M | 98.2% | 94.5% | rec_ebitda_rework_001.json | 8a3e811c...22904e2a | ed25519_sig_8a3e811c_... | PASS ✓ |
| ... | ... | ... | ... | ... | ... | ... | ... | ... | ... |

---

## Aggregate Statistics

- **Total Slides:** N
- **Slides Requiring Receipts:** N-2 (excluding title and summary)
- **Receipts Present:** N-2
- **Receipts Verified:** N-2
- **Average Fitness:** {{ avg_fitness }}%
- **Average Precision:** {{ avg_precision }}%
- **Claims Meeting Board Threshold (F≥95%, P≥90%):** N-2
- **Synergy Claims:** M
- **Operational Debt Items:** K
- **Integration Risks:** R
- **Total Quantified Value:** ${{ total_value }}

---

## Ledger Integrity Seal

**Ledger Content Hash (BLAKE3):** `<hash of all content above>`
**Ledger Signature (Ed25519):** `<signature of ledger hash>`
**Pinned Auditor Public Key:** `<pk for verification>`
**Signature Verification URL:** `https://proof.intelligence/verify/ma-deck-<uuid>`

This ledger is immutable. Any modification of this file or the receipts it references will be detected through hash verification.

---

## Evidence Chain Traceability

Each claim's evidence can be independently verified by following this chain:

1. PowerPoint slide → extract slide_id
2. Look up receipt file: /receipts/rec_<slide_id>.json
3. Extract from receipt:
   - target_log_hash: BLAKE3 hash of event log
   - process_model_hash: BLAKE3 hash of process model
   - query_definition.query_uri: path to wasm4pm query WASM module
   - validator_signature: Ed25519 signature
4. Obtain event log and process model from data room
5. Verify log hash: BLAKE3(obtained_log) == target_log_hash
6. Execute wasm4pm query: Load WASM module, provide event log and process model
7. Compare results: Computed fitness/precision should match receipt values (within 1e-6 tolerance)
8. Verify signature: Ed25519-Verify(auditor_pk, JCS(receipt), validator_signature) == true

---

*Ledger sealed on 2026-06-01*
*Authority: Board-Admissible Claim Requirements, wasm4pm Conformance Authority v30.1.2*
```

#### 7.2: Compute Ledger Hash and Signature

1. Serialize ledger markdown to UTF-8
2. Compute BLAKE3 hash of serialized content
3. Sign hash using Ed25519 private key (stored securely)
4. Append signature to ledger header

#### 7.3: Store Ledger Immutably

1. Write ledger markdown to `/Users/sac/process-intelligence/receipts/ma_deck_rendering.md`
2. Set file permissions to read-only (no modification after seal)
3. Store reference to ledger hash in transaction smart contract or signed closing agreement (if applicable)

**Output File:** `/Users/sac/process-intelligence/receipts/ma_deck_rendering.md`

---

## Final Outputs

Upon successful completion of all seven steps:

### 1. PowerPoint Presentation
**File:** `/Users/sac/process-intelligence/ma/acquisition_ready_deck_FINAL.pptx`

**Contents:**
- Title slide
- Executive summary (aggregate metrics)
- N claim detail slides (one per board-admissible claim)
- Operational debt slide (if applicable)
- Synergy waterfall slide (if applicable)
- Conformance model summary
- Board sign-off slide

**Properties:**
- All slides reference receipts in metadata
- All metrics are board-admissible (fitness ≥ 95%, precision ≥ 90%)
- All claims are mathematically justified (van der Aalst, Adriansyah, IEEE standards)
- Board-ready format (professional colors, clear typography, speaker notes)

### 2. Excel Workbook
**File:** `/Users/sac/process-intelligence/ma/diligence_workbook_FINAL.xlsx`

**Sheets:**
1. Executive_Summary — High-level metrics
2. Synergy_Claims — Synergy detail (one row per claim)
3. Operational_Debt — Debt inventory
4. Integration_Risks — Risk assessment
5. Replay_Traces — Conformance evidence (one row per claim)
6. Activity_Impact — Process activity analysis
7. Governance — Compliance checklist

**Properties:**
- Hyperlinked receipts in VDR
- Formatted fitness/precision columns (conditional coloring)
- Data validation on claim IDs and receipt files
- Suitable for syndication to legal, finance, operations teams

### 3. Receipt Ledger
**File:** `/Users/sac/process-intelligence/receipts/ma_deck_rendering.md`

**Contents:**
- Board admissibility declaration
- Slide-to-receipt registry (table with slide #, claim ID, metrics, receipt file, signature, status)
- Aggregate statistics
- Ledger integrity seal (BLAKE3 hash + Ed25519 signature)
- Evidence chain traceability instructions

**Properties:**
- Cryptographically sealed
- Audit-repeatable
- Links to all receipts and wasm4pm authorities
- Legally defensible for M&A due diligence

---

## Authority Stack Summary

| Component | Location | Status | Authority |
|-----------|----------|--------|-----------|
| Board Claim Taxonomy | ma/define_board_claim_taxonomy.md | ✓ Complete | Board-admissible requirements |
| Admissibility Rules | ma/define_board-admissible_claim_requirements.md | ✓ Complete | van der Aalst, Adriansyah, IEEE standards |
| Conformance Authority | receipts/wasm4pm_conformance_authority_generation.md | ✓ Generated | v30.1.2, A* alignment |
| Mining Authority | receipts/wasm4pm_mining_generation.md | ✓ Generated | Inductive Miner, Split Miner, Heuristics Miner |
| Replay Authority | receipts/wasm4pm_replay_generation.md | ✓ Generated | Token-based replay, fitness calculation |
| Lifecycle Authority | receipts/wasm4pm_lifecycle_generation.md | ✓ Generated | 12+ lifecycle phases |
| PowerPoint Template | ggen/templates/ma-deck.tera | ✓ Available | 202 lines, Tera2 syntax |
| Excel Template | ggen/templates/ma-diligence.tera | ✓ Available | 283 lines, Tera2 syntax |
| Receipt Schema | ma/define_slide-to-receipt_map.md | ✓ Complete | JCS (RFC 8785), Ed25519 |
| Validation Protocol | (this document, Step 6) | ✓ Defined | Cryptographic verification |
| Receipt Ledger | (this document, Step 7) | ✓ Defined | BLAKE3 + Ed25519 sealing |

---

## Execution Checklist

- [ ] **Step 1:** Query claims from ma/ directory → Validated claims array
- [ ] **Step 2:** Cross-reference wasm4pm authorities → Augmented claims array
- [ ] **Step 3:** Cross-reference compat evidence → Compat-verified claims array
- [ ] **Step 4:** Apply ma-deck.tera and ma-diligence.tera → Rendered JSON structures
- [ ] **Step 5:** Generate PowerPoint and Excel → acquisition_ready_deck_FINAL.pptx + diligence_workbook_FINAL.xlsx
- [ ] **Step 6:** Validate receipts, witness markers, compliance → Validation report (PASS)
- [ ] **Step 7:** Seal receipt ledger → ma_deck_rendering.md with BLAKE3 + Ed25519 seal
- [ ] **Board Sign-Off:** Executives review and approve deck before acquirer presentation

---

*Specification completed 2026-06-01*
*Authority: M&A Process Intelligence Module v1.0, Blue River Dam doctrine*
