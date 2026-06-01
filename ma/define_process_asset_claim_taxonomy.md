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

## 2. Mathematical Definition of Process Asset Value and Efficiency

To defend the valuation of a process asset and prevent synergy miscalculations, the seller must compute the volume-scaled and compliance-discounted **Process Asset Value ($V_{\text{asset}}$)** alongside a standardized **Asset Efficiency Score ($A_E$)**:

### A. Standardized Asset Efficiency Score ($A_E$)
To prevent scale distortions from raw throughput time, the efficiency score is normalized against a target service level agreement time $T_{\text{SLA}}$:
$$A_E = f(L, M) \times p(L, M) \times e^{-\frac{\bar{T}}{T_{\text{SLA}}}}$$
where:
* $f(L, M)$ is the alignment-based fitness of log $L$ on model $M$.
* $p(L, M)$ is the state-space precision of log $L$ on model $M$.
* $\bar{T}$ is the mean throughput time of conforming traces in the log.
* $T_{\text{SLA}}$ is the contractually mandated target cycle time. The term $e^{-\frac{\bar{T}}{T_{\text{SLA}}}}$ acts as a latency penalty function bounded in $(0, 1]$.

### B. Process Asset Valuation ($V_{\text{asset}}$)
The financial value contributed by a proprietary standardized process is the volume-weighted operational cost savings, discounted by the process deviation rate:
$$V_{\text{asset}} = V_{\text{annual}} \times f(L, M) \times p(L, M) \times \left( C_{\text{industry}} - C_{\text{actual}} \right)$$
where:
* $V_{\text{annual}}$ is the annual transaction volume processed through model $M$.
* $C_{\text{industry}}$ is the industry-standard benchmark cost per transaction.
* $C_{\text{actual}}$ is the target's actual fully loaded transaction execution cost.
* **Audit Rule**: To claim a process as a core operational asset, $V_{\text{asset}}$ must be positive, $A_E \ge 0.80$, and the log must pass the cryptographic provenance and anti-spoofing checks defined in [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md). Any deviations ($f < 1.0$) act as a direct haircut on the claimed asset value.

## 3. Related M&A Validation Documents

* For the strategic board claims mapping, see [Board Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_board_claim_taxonomy.md).
* For proving process scalability, see [Scalability Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_scalability_claim_taxonomy.md).
* For the seller defensibility requirements, see [Seller Defensibility Requirements](file:///Users/sac/process-intelligence/ma/define_seller_defensibility_requirements.md).
* For the underlying diligence claims, see [Diligence Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_diligence_claim_taxonomy.md).