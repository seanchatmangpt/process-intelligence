# Slide-to-Residual Map

When a seller claims that operations are standardized or that risk is mitigated, auditors must examine the "process residual"—the operational behaviors that do not conform to the documented process model. This document defines the Slide-to-Residual Map and the mathematical metrics used to quantify residual operational risk.

## 1. Mathematical Definition of Process Residuals

The process residual represents the set of event log traces that deviate from the process model's allowed paths.

* Let $L$ be the event log and $M$ be the process model.
* The residual log $R$ is defined as the subset of traces $\sigma \in L$ that do not have perfect fitness ($f = 1.0$) on model $M$:
  $$R = \{ \sigma \in L \mid f(\sigma, M) < 1.0 \}$$

To evaluate the risk of this residual, we calculate two metrics: **Residual Weight** and **Residual Entropy**.

### A. Residual Weight ($W_R$)
Residual Weight measures the fraction of transactions that deviate from the standard model:
$$W_R = \frac{|R|}{|L|}$$
* A slide assertion claiming "98% process standardization" is equivalent to claiming a Residual Weight $W_R \le 0.02$.

### B. Residual Entropy ($H_R$)
Residual Entropy measures the diversity and chaos of the deviating behavior. If all deviations follow a single alternative path, the entropy is low, representing a known workaround. If deviations are highly random, the entropy is high, representing operational chaos.
* Let $V_R$ be the unique trace variants in the residual log $R$.
* Let $P(v)$ be the relative frequency of variant $v \in V_R$ within $R$.
* The Residual Entropy ($H_R$) is:
  $$H_R = -\sum_{v \in V_R} P(v) \log_2 P(v)$$
* **Risk Boundaries**:
  * **$H_R < 1.0$**: Low risk. Deviations are predictable workarounds that can be easily standardized.
  * **$H_R \ge 2.5$**: High risk. Deviations are chaotic, indicating high potential for fraud, manual errors, and hidden operational liabilities.

## 2. Residual Verification Schema

Slide claims of "low risk" must link to a residual verification block:

```json
{
  "slide_id": "9b8c-7d6e",
  "assertion": "Operations are fully standardized and controlled",
  "residual_metrics": {
    "residual_weight": 0.032,
    "residual_entropy": 0.87,
    "critical_violations_in_residual": 0
  },
  "verdict": "Admissible (Low residual chaos and zero compliance leakage)"
}
```

## 3. Related M&A Validation Documents

* For the general mapping of slide assertions, see [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md).
* For the taxonomy of operational debts, see [Operational Debt Taxonomy](file:///Users/sac/process-intelligence/ma/define_operational_debt_taxonomy.md).
* For integrating target processes and mitigating post-merger risk, see [Integration Risk Taxonomy](file:///Users/sac/process-intelligence/ma/define_integration_risk_taxonomy.md).
* For classifying process liabilities, see [Process Liability Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_process_liability_claim_taxonomy.md).