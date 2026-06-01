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

## 2. EBITDA and Working Capital Formulas

To maintain board admissibility, financial assertions must be linked to operational process metrics using formal valuation formulas:

### A. EBITDA Impact from Process Rework
* **Formula**: The EBITDA impact ($E$) is a function of the rework rate ($r$), transaction volume ($V$), and average cost per manual rework event ($C_r$):
  $$E = V \times (r_{\text{baseline}} - r_{\text{target}}) \times C_r$$
* **Verification**: The rework rate $r$ must be computed by counting self-loops and redundant transitions in the event log (e.g., "Re-enter Order Data") using the wasm4pm execution core.

### B. Days Sales Outstanding (DSO) Reduction
* **Formula**: DSO is reduced by accelerating the Order-to-Cash (O2C) billing cycle. The working capital release ($WC$) is:
  $$WC = \text{Revenue}_{\text{daily}} \times (T_{\text{baseline}} - T_{\text{target}})$$
  where $T$ represents the average throughput time from "Deliver Goods" to "Receive Payment".
* **Verification**: Calculated directly from timestamp deltas in the XES/OCEL event logs.

## 3. Related M&A Validation Documents

* For the technical validation rules of underlying diligence claims, see [Diligence Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_diligence_claim_taxonomy.md).
* For synergy calculations, see [Synergy Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_synergy_claim_taxonomy.md).
* For structural requirements of board presentations, see [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).
* For process assets, see [Process Asset Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_process_asset_claim_taxonomy.md).