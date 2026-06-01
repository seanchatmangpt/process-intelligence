# Public Standards to M&A Claims Mapping

During corporate mergers and acquisitions (M&A), operational and compliance claims (e.g., "annual cost savings of $1.5M due to process optimization," "zero compliance drift in procurement") must be backed by empirical evidence. This document establishes how process standards-compliant metrics map to board-admissible M&A assertions.

For the core criteria of board admissibility, see [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).

---

## 1. M&A Claims Mapped to Standards Metrics

Every high-level business assertion presented to transaction partners or executive boards must resolve to a specific query on a standard-compliant process log or model:

| Board-Level Assertion | Underlying Metric | Standard Format | Verification Query |
| :--- | :--- | :--- | :--- |
| **Operational Synergy** | Cycle Time Latency | **XES** | Replay latencies across key milestones (Adriansyah 2014). |
| **Process Compliance** | Replay Fitness ($f$) | **Petri Net** | Token-based replay fitness ($f \ge 0.95$) (van der Aalst 2016). |
| **Process Simplicity** | Structural Complexity | **Process Tree**| Node count and hierarchy depth calculations. |
| **Data Room Integrity** | Source Lineage | **PROV-O** | Lineage path check back to raw database extracts. |
| **Regulatory Risk** | Constraint Violations | **Declare** | FSA violation counts on compliance constraint templates. |

---

## 2. The Slide-to-Receipt Bridge

To guarantee the defensibility of transaction materials:

1.  **Slide UUID Registration**: Every slide in the investment presentation containing an operational claim is assigned a unique UUID.
2.  **Cryptographic Receipt Binding**: The slide UUID is bound to a `wasm4pm` execution receipt stored on the ledger:
    $$\text{SlideBinding} = \operatorname{BLAKE3}\left( \text{SlideUUID} \parallel \text{ReceiptHash} \parallel \text{ClaimValue} \right)$$
3.  **Independent Replication**: The buyer's advisors must be able to replicate the exact metrics by executing the queries on the logs in the VDR, as required by the [Buyer Reliance Requirements](file:///Users/sac/process-intelligence/ma/define_buyer_reliance_requirements.md).

---

## 3. Related M&A Validation Documents

*   For the mapped list of slides to receipts, see [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md).
*   For the audit steps, see [Auditor Evidence Path](file:///Users/sac/process-intelligence/ma/define_auditor_evidence_path.md).
*   To review the sample M&A mapping, see the [Paper to M&A Claim Mapping Sample](file:///Users/sac/process-intelligence/experiments/paper-to-m&a-claim_mapping_sample.md).