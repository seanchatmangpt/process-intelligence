# Process Asset Claim Taxonomy

A process asset is a formalized sequence of corporate activities that provides a competitive advantage. Highly efficient, compliant, and standardized processes represent intangible intellectual property that increases enterprise value. This document establishes the Process Asset Claim Taxonomy, defining asset classes and their mathematical verification.

## 1. Process Asset Classification

Process assets are categorized into four core classes: Highly Standardized, Proprietary IP, Optimized Resource Configurations, and Straight-Through Processing (STP).

```
                      ┌──────────────────────────────────────────┐
                      │              Process Assets              │
                      └────────────────────┬─────────────────────┘
         ┌─────────────────────────────┼─────────────────────────────┐
         ▼                             ▼                             ▼
 ┌──────────────┐              ┌──────────────┐              ┌──────────────┐
 │    Highly    │              │ Proprietary  │              │   Straight-  │
 │ Standardized │              │  Process IP  │              │   Through    │
 └──────────────┘              └──────────────┘              └──────────────┘
```

| Asset Class | Operational Characteristics | Business Value | Verification Metric |
| :--- | :--- | :--- | :--- |
| **Highly Standardized** | High conformance, low variation across all executions. | Low training costs, rapid scalability, easy automation. | Conformance fitness $f \ge 0.98$ and Trace Entropy $H(L) < 1.0$. |
| **Proprietary Process IP** | Unique, optimized workflow sequences that outperform industry standards. | Operational moat, patented operational sequences. | Process structure complexity and path analysis. |
| **Optimized Resources** | Efficient collaboration networks and low handover delay. | Higher labor productivity, low team friction. | Resource handover latency $T_{\text{handover}} < 12$ hours (Song 2008). |
| **Straight-Through (STP)** | High rate of fully automated, touchless event execution. | Minimal variable costs, high transaction velocity. | Automated event ratio $> 85\%$ in OCEL log. |

## 2. Mathematical Definition of Process Asset Efficiency

To defend the valuation of a process asset, the seller computes the **Asset Efficiency Score ($A_E$)**:

* Let $f(L, M)$ be the fitness of log $L$ on model $M$.
* Let $p(L, M)$ be the precision of log $L$ on model $M$.
* Let $\bar{T}$ be the mean throughput time of all conforming traces in the log.
* The Asset Efficiency Score ($A_E$) is defined as:
  $$A_E = f(L, M) \times p(L, M) \times \frac{1}{\bar{T}}$$
* **Audit Rule**: To claim a process as a core operational asset, $A_E$ must exceed a baseline industry threshold, and $f$ must be verified via optimal alignments (Adriansyah 2014) to be free of significant deviation costs.

## 3. Related M&A Validation Documents

* For the strategic board claims mapping, see [Board Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_board_claim_taxonomy.md).
* For proving process scalability, see [Scalability Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_scalability_claim_taxonomy.md).
* For the seller defensibility requirements, see [Seller Defensibility Requirements](file:///Users/sac/process-intelligence/ma/define_seller_defensibility_requirements.md).
* For the underlying diligence claims, see [Diligence Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_diligence_claim_taxonomy.md).