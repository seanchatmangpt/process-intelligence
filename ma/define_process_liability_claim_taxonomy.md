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

## 2. Mathematical Cost of Process Liabilities (Anti-Leakage Calculations)

To calculate the required valuation haircut for process liabilities, auditors must quantify rework costs, SLA penalties, and compliance leakage risks:

### A. Rework Cost Liability ($L_{\text{rework}}$)
The annual cost of manual corrections and system overrides is:
$$L_{\text{rework}} = \sum_{a \in A_{\text{rework}}} V_a \times C_a$$
* Where $A_{\text{rework}}$ is the set of manual rework activities identified in the event log (e.g., "Change Invoice Price", "Modify Shipping Address").
* $V_a$ is the verified annual frequency of activity $a$ in the log.
* $C_a$ is the fully loaded cost of human labor per execution of activity $a$, including overheads.

### B. SLA Penalty Risk ($L_{\text{SLA}}$)
The total liability from late deliveries or delayed service levels is:
$$L_{\text{SLA}} = \sum_{c \in C_{\text{late}}} P_{\text{penalty}}(c) + \sum_{c \in C_{\text{active}}} \operatorname{Pr}(T(c) > T_{\text{SLA}} \mid \sigma_c) \times P_{\text{penalty}}(c)$$
* Where $C_{\text{late}}$ is the set of completed cases where the throughput time $T(c) > T_{\text{SLA}}$.
* $C_{\text{active}}$ is the set of active (incomplete) cases in the log.
* $\operatorname{Pr}(T(c) > T_{\text{SLA}} \mid \sigma_c)$ is the conditional probability that active case $c$ with prefix trace $\sigma_c$ will exceed the SLA, computed using historical transition latency distributions.
* $P_{\text{penalty}}(c)$ is the contractually mandated penalty for case $c$.

### C. Compliance Leakage Liability ($L_{\text{compliance}}$)
The financial liability resulting from regulatory infractions (e.g., SOX controls, GDPR data retention violations, AML segregation failures) is modeled as:
$$L_{\text{compliance}} = \sum_{r \in \mathcal{R}} \left( N_{\text{violations}}(r, L) \times F_{\text{statutory}}(r) + \operatorname{Pr}(\text{Audit}_{\text{ext}}) \times F_{\text{systemic}}(r) \right)$$
* Where $\mathcal{R}$ is the set of compliance rules modeled as LTL formulas.
* $N_{\text{violations}}(r, L)$ is the count of traces in the log $L$ that violate LTL formula $r$.
* $F_{\text{statutory}}(r)$ is the statutory or regulatory fine per individual infraction.
* $\operatorname{Pr}(\text{Audit}_{\text{ext}})$ is the estimated probability of external regulatory discovery.
* $F_{\text{systemic}}(r)$ is the systemic corporate fine (e.g., percentage of revenue) triggered by structural control failures.

## 3. Related M&A Validation Documents

* For the general taxonomy of operational debt, see [Operational Debt Taxonomy](file:///Users/sac/process-intelligence/ma/define_operational_debt_taxonomy.md).
* For process control taxonomy, see [Control Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_control_claim_taxonomy.md).
* For evaluating post-merger integration risks, see [Integration Risk Taxonomy](file:///Users/sac/process-intelligence/ma/define_integration_risk_taxonomy.md).
* For mapping liabilities to slide residuals, see [Slide-to-Residual Map](file:///Users/sac/process-intelligence/ma/define_slide-to-residual_map.md).