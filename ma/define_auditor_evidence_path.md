# Auditor Evidence Path

The Auditor Evidence Path (AEP) defines the formal protocol that financial, compliance, and operational auditors must execute to verify process-related assertions made during due diligence. It ensures a transparent, tamper-proof, and reproducible chain of custody from raw database transactions to executive board slides.

## 1. The 5-Step Auditing Protocol

Auditors must follow this exact step-by-step verification pathway for every contested or high-impact process claim:

```
┌─────────────────────┐      1. Extract Slide UUID      ┌──────────────────────┐
│  M&A Slide Claim    │ ──────────────────────────────> │ Verification Receipt │
└─────────────────────┘                                 └──────────┬───────────┘
                                                                   │
                                                                   │ 2. Trace Lineage
                                                                   ▼
┌─────────────────────┐      3. Hash Check (SHA-256)    ┌──────────────────────┐
│  Source Database    │ <────────────────────────────── │   XES / OCEL Log     │
└─────────────────────┘                                 └──────────┬───────────┘
                                                                   │
                                                                   │ 4. Replay Conformance
                                                                   ▼
                                                        ┌──────────────────────┐
                                                        │  Petri Net Replay    │
                                                        │  (Optimal Alignment) │
                                                        └──────────────────────┘
```

### Step 1: Reference Extraction
* **Action**: Identify the slide's UUID and extract the corresponding cryptographic verification receipt from the VDR (e.g., `receipt_<slide_id>.json`).
* **Output**: The schema-compliant JSON receipt defining the target log hash, process model hash, query definition, and expected conformance metrics.

### Step 2: Source Lineage Audit (PROV-O Provenance)
* **Action**: Verify the data extraction lineage. Trace the event log generation script (SQL/OCPQ) back to the source ERP/CRM database transaction logs.
* **Output**: Verification that event timestamps match database commit timestamps within a synchronization tolerance ($\Delta t \le 1$ second).

### Step 3: Cryptographic Integrity Verification
* **Action**: Re-calculate the SHA-256 hash of the target XES or OCEL log file.
* **Math Check**: Verify that:
  $$\operatorname{SHA-256}(L_{\text{local}}) == \text{target\_log\_hash}$$
* **Output**: Tamper-proof validation of the event log data.

### Step 4: Model Soundness and Liveness Audit
* **Action**: Load the process model (Petri net) and verify its structural correctness.
* **Math Check**: Verify soundness (van der Aalst 1998) by checking that:
  1. The net has a single source place $i$ and sink place $o$.
  2. For any reachable marking $m$, the sink place is reachable ($m \xrightarrow{*} m_f$).
  3. No transition is dead.
* **Output**: A certified sound process model.

### Step 5: Conformance Replay and Alignment Audit
* **Action**: Execute the optimal alignment conformance algorithm (Adriansyah 2014) on the verified log and model using the parameters specified in the receipt.
* **Math Check**: Re-calculate fitness ($f$) and precision ($p$), verifying that:
  $$f_{\text{audited}} \ge f_{\text{claimed}} \quad \text{and} \quad p_{\text{audited}} \ge p_{\text{claimed}}$$
* **Output**: The final audit verdict.

## 2. Auditing Toolchain Requirements

To ensure objectivity, all audit steps must be executed using open-source, standards-compliant tooling (e.g., the `wasm4pm` execution core or standard `pm4py` libraries) rather than proprietary, closed-source vendor packages.

## 3. Related M&A Validation Documents

* For the receipt structure, see [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md).
* For the mathematical definition of alignment, see [Slide-to-Replay Map](file:///Users/sac/process-intelligence/ma/define_slide-to-replay_map.md).
* For the buyer reliance standards, see [Buyer Reliance Requirements](file:///Users/sac/process-intelligence/ma/define_buyer_reliance_requirements.md).
* For the board admissibility rules, see [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).