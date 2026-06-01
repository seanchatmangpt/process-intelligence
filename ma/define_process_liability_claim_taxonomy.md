# Process Liability Claim Taxonomy

Process liabilities represent hidden operational risks, compliance violations, and structural failures that expose an acquirer to financial losses or legal penalties. During due diligence, buyers use process mining to detect these liabilities and negotiate valuation haircuts. This document establishes the Process Liability Claim Taxonomy and the formulas used to calculate liability costs.

## 1. Process Liability Classification

Process liabilities are divided into four critical quadrants: Compliance Violations, SLA Leakage, Fraud Vulnerabilities, and Rework Cost Loops.

```
                      ┌──────────────────────────────────────────┐
                      │            Process Liabilities           │
                      └────────────────────┬─────────────────────┘
         ┌─────────────────────────────┼─────────────────────────────┐
         ▼                             ▼                             ▼
 ┌──────────────┐              ┌──────────────┐              ┌──────────────┐
 │  Compliance  │              │ SLA Leakage  │              │ Rework Cost  │
 │  Violations  │              │  (Penalties) │              │    Loops     │
 └──────────────┘              └──────────────┘              └──────────────┘
```

| Liability Class | Description | Financial Impact | Verification Metric |
| :--- | :--- | :--- | :--- |
| **Compliance Violations** | Infractions against legal or industry standards (SOX, GDPR, AML). | Regulatory fines, lawsuits, reputational damage. | Non-zero violation count via DECLARE/LTL checks on logs. |
| **SLA Leakage** | Failures to meet customer or vendor service level agreements. | Contractual penalties, customer churn. | Traces exceeding SLA thresholds ($T(c) > T_{\text{SLA}}$). |
| **Fraud Vulnerability** | Deficiencies in internal controls, allowing unauthorized actions. | Asset theft, financial misstatement. | Absence of four-eyes verification steps in approval traces. |
| **Rework Cost Loops** | High frequencies of redundant manual operations and data correction. | Excess SG&A costs, low resource efficiency. | Rework loop count and cost allocation. |

## 2. Mathematical Cost of Process Liabilities

To calculate the valuation adjustment required for a process liability:

### A. Rework Cost Liability ($L_{\text{rework}}$)
The annual financial drain caused by redundant manual correction steps is:
$$L_{\text{rework}} = \sum_{a \in A_{\text{rework}}} V_a \times C_a$$
* Where $A_{\text{rework}}$ is the set of manual rework activities (e.g., "Change Invoice Price", "Correct Shipping Address").
* $V_a$ is the annual frequency of activity $a$ in the log.
* $C_a$ is the fully loaded cost of human labor per execution of activity $a$.

### B. SLA Penalty Risk ($L_{\text{SLA}}$)
The total liability from late deliveries or delayed services is:
$$L_{\text{SLA}} = \sum_{c \in C_{\text{late}}} P_{\text{penalty}}(c)$$
* Where $C_{\text{late}}$ is the set of cases where the throughput time $T(c) > T_{\text{SLA}}$.
* $P_{\text{penalty}}(c)$ is the contractually mandated penalty for case $c$.

## 3. Related M&A Validation Documents

* For the general taxonomy of operational debt, see [Operational Debt Taxonomy](file:///Users/sac/process-intelligence/ma/define_operational_debt_taxonomy.md).
* For process control taxonomy, see [Control Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_control_claim_taxonomy.md).
* For evaluating post-merger integration risks, see [Integration Risk Taxonomy](file:///Users/sac/process-intelligence/ma/define_integration_risk_taxonomy.md).
* For mapping liabilities to slide residuals, see [Slide-to-Residual Map](file:///Users/sac/process-intelligence/ma/define_slide-to-residual_map.md).