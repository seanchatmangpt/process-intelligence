# Board Claim Taxonomy

Board-level claims are high-impact, strategic assertions presented to the board of directors, investment committees, or executive acquirers. These claims translate raw operational process mining metrics into financial and risk metrics (e.g., EBITDA impact, working capital, compliance liability). This document establishes the taxonomy of board-level claims and their operational mapping.

## 1. Board-Level Claim Classification

Board claims are categorized into four core strategic domains: EBITDA Impact, Working Capital Optimization, GRC Defensibility, and Integration Velocity.

```
                      ┌──────────────────────────────────────────┐
                      │            Board-Level Claims            │
                      └────────────────────┬─────────────────────┘
         ┌─────────────────────────────┼─────────────────────────────┐
         ▼                             ▼                             ▼
 ┌──────────────┐              ┌──────────────┐              ┌──────────────┐
 │    EBITDA    │              │   Working    │              │ Integration  │
 │ Optimization │              │   Capital    │              │   Velocity   │
 └──────────────┘              └──────────────┘              └──────────────┘
```

| Board Claim Class | Strategic Metric | Operational Driver | Diligence Validation Link |
| :--- | :--- | :--- | :--- |
| **EBITDA Optimization** | Margin increase, cost of goods sold (COGS) reduction. | Labor redundancy elimination, rework reduction, automation. | [Diligence Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_diligence_claim_taxonomy.md) |
| **Working Capital** | Days Sales Outstanding (DSO), Days Payable Outstanding (DPO). | Accounts Receivable bottleneck elimination, payment term alignment. | [Diligence Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_diligence_claim_taxonomy.md) |
| **GRC Defensibility** | Mitigation of class-action, regulatory, or tax liabilities. | Automated audit trails, segregation of duties enforcement. | [Control Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_control_claim_taxonomy.md) |
| **Integration Velocity** | Post-Merger Integration (PMI) schedule and synergy capture. | System similarity, process model behavioral equivalence. | [Synergy Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_synergy_claim_taxonomy.md) |

## 2. EBITDA and Working Capital Formulas (Verification Standards)

To maintain board admissibility, financial assertions must be linked to operational process metrics using formal, verifiable mathematical valuation models:

### A. EBITDA Impact from Process Rework
To prevent synergy miscalculations, the rework metric is modeled as an average execution intensity rather than a binary trace flag:
* Let $L$ be the event log of the target process.
* Let $A_{\text{rework}}$ be the set of activities classified as manual rework (e.g., "Change Price", "Re-enter Order").
* Let $N_{\text{rework}}(\sigma, A_{\text{rework}})$ be the count of occurrences of activities from $A_{\text{rework}}$ in trace $\sigma$.
* The baseline and target rework rates ($r$) are defined as the mean rework events per case:
  $$r = \frac{\sum_{\sigma \in L} L(\sigma) \cdot N_{\text{rework}}(\sigma, A_{\text{rework}})}{\sum_{\sigma \in L} L(\sigma)}$$
* The annual EBITDA impact ($E$) is calculated as:
  $$E = V_{\text{annual}} \times (r_{\text{baseline}} - r_{\text{target}}) \times \bar{C}_{\text{rework}}$$
  where $V_{\text{annual}}$ is the projected annual transaction volume, and $\bar{C}_{\text{rework}}$ is the fully-burdened average labor cost per manual rework event.

### B. Days Sales Outstanding (DSO) and Working Capital Release
Working capital release projections must not conflate delivery latency with credit terms. DSO reduction is mapped strictly to the accounts receivable (AR) processing window:
* Let $T_{\text{AR}}$ be the average throughput time (in days) from "Invoice Creation" ($e_{\text{inv}}$) to "Payment Confirmation" ($e_{\text{pay}}$):
  $$T_{\text{AR}} = \frac{\sum_{\sigma \in L_{\text{O2C}}} L(\sigma) \cdot \left[ t(e_{\text{pay}}(\sigma)) - t(e_{\text{inv}}(\sigma)) \right]}{\sum_{\sigma \in L_{\text{O2C}}} L(\sigma)}$$
  where $t(e)$ is the timestamp of event $e$.
* The Working Capital Release ($WC$) resulting from process acceleration is:
  $$WC = \left( \frac{\text{Revenue}_{\text{credit\_annual}}}{365} \right) \times (T_{\text{AR, baseline}} - T_{\text{AR, target}})$$
  where $\text{Revenue}_{\text{credit\_annual}}$ represents the annual credit-based sales revenue of the entity.
* **Verification**: Calculated by the Ostar Auditor running wasm4pm queries verifying the timestamp deltas between invoice and payment receipt objects in the OCEL 2.0 log.

## 3. Related M&A Validation Documents

* For the technical validation rules of underlying diligence claims, see [Diligence Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_diligence_claim_taxonomy.md).
* For synergy calculations, see [Synergy Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_synergy_claim_taxonomy.md).
* For structural requirements of board presentations, see [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).
* For process assets, see [Process Asset Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_process_asset_claim_taxonomy.md).