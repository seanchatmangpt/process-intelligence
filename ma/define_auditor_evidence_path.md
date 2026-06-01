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
* **Action**: Re-calculate the SHA-256 hash and Merkle root of the target event log.
* **Math Check**: Verify that the file hash and Merkle root of the event hash chains match the closing agreement:
  $$\operatorname{SHA-256}(L_{\text{local}}) == \text{target\_log\_hash}$$
  $$M_{\text{root, local}} == M_{\text{root, receipt}}$$
* **Output**: Tamper-proof validation of the event log data.

### Step 4: Model Soundness and Liveness Audit
* **Action**: Load the process model (Petri net Workflow Net) and verify structural soundness.
* **Math Check**: Formally verify soundness (van der Aalst 1998):
  1. **Option to Complete**: $\forall M \in [M_0\rangle, \quad [o] \in [M\rangle$
  2. **Liveness**: $\forall M \in [M_0\rangle, \forall t \in T, \exists M', M'' \in [P\rangle : M \xrightarrow{*} M' \xrightarrow{t} M''$
  3. **Boundedness**: $\exists k \in \mathbb{N}^+ : \forall M \in [M_0\rangle, \forall p \in P, \quad M(p) \le k$
* **Output**: A certified sound process model.

### Step 5: Conformance Replay and Alignment Audit
* **Action**: Execute the optimal alignment conformance algorithm (Adriansyah 2014) on the verified log and model using the parameters specified in the receipt.
* **Math Check**: Re-calculate fitness ($f$) and precision ($p$) using alignment-driven state space analysis, verifying that the difference lies within the replication tolerance:
  $$\left| f_{\text{audited}} - f_{\text{claimed}} \right| < 10^{-6} \quad \text{and} \quad \left| p_{\text{audited}} - p_{\text{claimed}} \right| < 10^{-6}$$
* **Output**: The final audit verdict.

## 2. Auditing Toolchain Requirements

To ensure objectivity, all audit steps must be executed using open-source, standards-compliant tooling (e.g., the `wasm4pm` execution core or standard `pm4py` libraries) rather than proprietary, closed-source vendor packages.

## 3. Related M&A Validation Documents

* For the receipt structure, see [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md).
* For the mathematical definition of alignment, see [Slide-to-Replay Map](file:///Users/sac/process-intelligence/ma/define_slide-to-replay_map.md).
* For the buyer reliance standards, see [Buyer Reliance Requirements](file:///Users/sac/process-intelligence/ma/define_buyer_reliance_requirements.md).
* For the board admissibility rules, see [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).