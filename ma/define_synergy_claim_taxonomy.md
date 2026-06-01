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

## 2. Mathematical Synergy Formulation (Anti-Miscalculation Protocols)

To prevent synergy miscalculations and post-merger value leakage, financial projections must be grounded in formal process compatibility and risk-adjusted cost models.

### A. Process Harmonization (Behavioral Profile Similarity under Semantic Mapping)
To prove that target process model $M_1 = (P_1, T_1, F_1)$ and buyer process model $M_2 = (P_2, T_2, F_2)$ can be harmonized, we calculate their behavioral similarity. A naive Jaccard index fails due to differing activity labels. We define:
* Let $\mathcal{C} \subseteq T_1 \times T_2$ be a verified semantic correspondence mapping (based on shared ontology and transaction types).
* Let $r_M(a, b) \in \{\rightarrow, \leftarrow, \parallel, +\}$ be the behavioral relation (strict order, reverse order, interleaving/concurrency, or exclusivity) between transitions $a$ and $b$ in model $M$ as defined in Weidlich 2011.
* We define the similarity index $\operatorname{Sim}(M_1, M_2, \mathcal{C})$ over the mapped pairs as:
  $$\operatorname{Sim}(M_1, M_2, \mathcal{C}) = \frac{\sum_{((a_1, a_2), (b_1, b_2)) \in \mathcal{C} \times \mathcal{C}} \delta(r_{M_1}(a_1, b_1), r_{M_2}(a_2, b_2))}{|\mathcal{C} \times \mathcal{C}|}$$
  where $\delta(x, y) = 1$ if $x = y$, and $0$ otherwise.
* **Admissibility Boundary**: A harmonization synergy claim is only valid if $\operatorname{Sim}(M_1, M_2, \mathcal{C}) \ge 0.75$. Values below this threshold indicate extreme structural divergence, which historically triggers costly custom software workarounds.

### B. Risk-Adjusted System Rationalization NPV
Migrating the target's IT transactions to the buyer's systems (e.g., ERP consolidation) is verified by querying system attributes in the OCEL 2.0 log. To prevent overestimating synergies, the system license saving must be calculated as a Net Present Value (NPV) adjusted for migration delays:
* Let $C_{\text{target\_sys}}(t)$ be the target's annual maintenance and license costs for the legacy system.
* Let $C_{\text{buyer\_incremental}}(t)$ be the buyer's incremental license and hosting cost to support the target's transaction volume.
* Let $C_{\text{migration}}(t)$ be the capital expenditure and labor cost for database migration in year $t$.
* Let $\beta_t \in [0, 1]$ be the compliance leakage and timeline overrun probability (derived from historical process drift and migration bottlenecks).
* The Net Present Value of the synergy savings ($NPV(S_L)$) over a horizon of $T$ years at discount rate $r$ is:
  $$NPV(S_L) = \sum_{t=1}^{T} \frac{(1 - \beta_t) \cdot C_{\text{target\_sys}}(t) - C_{\text{buyer\_incremental}}(t) - C_{\text{migration}}(t)}{(1 + r)^t}$$
  where the active transaction volume and legacy system-triggered events are verified directly from event log metrics.

## 3. Related M&A Validation Documents

* For identifying integration risks associated with synergy targets, see [Integration Risk Taxonomy](file:///Users/sac/process-intelligence/ma/define_integration_risk_taxonomy.md).
* For proving scalability during integration, see [Scalability Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_scalability_claim_taxonomy.md).
* For mapping synergy claims directly to slide validation receipts, see [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md).
* For strategic board claims, see [Board Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_board_claim_taxonomy.md).