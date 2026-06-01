# Synergy Claim Taxonomy

M&A premiums are primarily justified by projected synergies. However, traditional synergy estimates are often subjective. This document establishes the synergy claim taxonomy, defining how process intelligence models, calculates, and proves synergy capture using objective event log evidence.

## 1. Synergy Claim Categories

Synergy claims are classified into four categories: Process Harmonization, Operational Cost Reduction, System Rationalization, and Best-Practice Adoption.

```
                  ┌──────────────────────────────────────────┐
                  │             Synergy Claims               │
                  └────────────────────┬─────────────────────┘
         ┌─────────────────────────────┼─────────────────────────────┐
         ▼                             ▼                             ▼
 ┌──────────────┐              ┌──────────────┐              ┌──────────────┐
 │   Process    │              │ Operational  │              │    System    │
 │Harmonization │              │Cost Reduction│              │Rationaliztn. │
 └──────────────┘              └──────────────┘              └──────────────┘
```

| Synergy Class | Description | Valuation Impact | Verification Metric |
| :--- | :--- | :--- | :--- |
| **Process Harmonization** | Merging and standardizing target and buyer processes. | Accelerated post-merger integration, reduced training. | Behavioral profile similarity ($Sim \ge 0.85$, Weidlich 2011). |
| **Operational Cost Reduction (OCR)** | Eliminating redundant manual steps and duplicated approvals. | Direct SG&A and COGS savings. | Activity reduction via Inductive Miner path pruning (Leemans 2013). |
| **System Rationalization** | Retiring overlapping IT systems and consolidation. | Reduced software license fees and maintenance overhead. | System-to-activity object mapping using OCEL 2.0 (Ghahfarokhi 2021). |
| **Best-Practice Adoption** | Porting the most efficient process flow from one entity to the other. | Immediate margin uplift on the lower-performing entity. | Cross-entity performance replay against a unified reference model. |

## 2. Mathematical Synergy Formulation

### A. Process Harmonization (Behavioral Similarity)
To prove that target and buyer processes can be harmonized with minimal friction, we compute their behavioral profile similarity:
* Let $P_1$ be the behavioral profile of target process model $M_1$.
* Let $P_2$ be the behavioral profile of buyer process model $M_2$.
* The similarity index ($\operatorname{Sim}(M_1, M_2)$) is calculated as:
  $$\operatorname{Sim}(M_1, M_2) = \frac{\left| P_1 \cap P_2 \right|}{\left| P_1 \cup P_2 \right|}$$
* **Admissibility Boundary**: A harmonization synergy claim is only valid if $\operatorname{Sim}(M_1, M_2) \ge 0.70$. Lower similarity indicates significant integration friction and high customization risk.

### B. System Rationalization Potential
* When consolidations occur (e.g., migrating target's CRM transactions to buyer's CRM), the cost reduction is proven by locating system-specific object linkages in the OCEL 2.0 event log.
* The system license saving $S_L$ is:
  $$S_L = \sum (C_{\text{target\_sys}} - C_{\text{migration\_overhead}})$$
  where the active usage of target systems is verified by checking the frequency of system-triggered events in the log.

## 3. Related M&A Validation Documents

* For identifying integration risks associated with synergy targets, see [Integration Risk Taxonomy](file:///Users/sac/process-intelligence/ma/define_integration_risk_taxonomy.md).
* For proving scalability during integration, see [Scalability Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_scalability_claim_taxonomy.md).
* For mapping synergy claims directly to slide validation receipts, see [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md).
* For strategic board claims, see [Board Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_board_claim_taxonomy.md).