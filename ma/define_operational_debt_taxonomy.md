# Operational Debt Taxonomy

Operational debt represents the hidden operational costs, system complexities, and structural process inefficiencies that an acquirer inherits. This document establishes the operational debt taxonomy, defining the categories of debt and providing mathematical methods to quantify these liabilities using event log analytics.

## 1. Operational Debt Classification

Operational debt is categorized into four primary quadrants: Process Spaghetti, Compliance Deficit, Legacy Lock-In, and Shadow IT Workarounds.

```
                      ┌──────────────────────────────────────────┐
                      │            Operational Debt              │
                      └────────────────────┬─────────────────────┘
         ┌─────────────────────────────┼─────────────────────────────┐
         ▼                             ▼                             ▼
 ┌──────────────┐              ┌──────────────┐              ┌──────────────┐
 │   Process    │              │  Compliance  │              │  Shadow IT   │
 │  Spaghetti   │              │   Deficits   │              │ Workarounds  │
 └──────────────┘              └──────────────┘              └──────────────┘
```

| Debt Class | Operational Indicator | Business Risk | Metric / Threshold |
| :--- | :--- | :--- | :--- |
| **Process Spaghetti** | High number of unique trace variants, low standardization. | High training costs, slow automation, high error rate. | Trace Entropy $H(L) > 3.0$ or conformance precision $p < 0.70$. |
| **Compliance Deficits** | Systematic violation of internal policies and external GRC laws. | Regulatory penalties, fraud risk, restatement risks. | Violations per 10,000 cases $> 5$ using DECLARE / LTL checks. |
| **Legacy Lock-In** | High latency in activities dependent on old legacy systems. | System replacement costs, low flexibility. | Legacy transition latency $T_{\text{legacy}} > 3 \times$ modern equivalent. |
| **Shadow IT Workarounds** | Disconnected object networks, high spreadsheets/email usage. | Lack of central governance, data leakage, security risk. | High frequency of manual adjustment events (e.g., "Manual Update"). |

## 2. Quantifying Process Spaghetti Debt (Trace Entropy)

To measure the degree of process variation (spaghetti-ness), auditors calculate the Shannon entropy of the event log's trace variants:

* Let $L$ be the event log containing a set of traces.
* Let $V$ be the set of unique trace variants in $L$.
* Let $P(v)$ be the relative frequency of variant $v \in V$ in the log.
* The Trace Entropy ($H(L)$) is defined as:
  $$H(L) = -\sum_{v \in V} P(v) \log_2 P(v)$$
* **Audit Thresholds**:
  * **$H(L) < 1.5$**: Highly standardized process. Minimal operational debt.
  * **$1.5 \le H(L) \le 3.0$**: Moderate variation. Standard due diligence required.
  * **$H(L) > 3.0$**: Process Spaghetti. High operational debt. Valuation haircut recommended due to integration and automation complexity.

## 3. Related M&A Validation Documents

* For details on process liabilities resulting from operational debt, see [Process Liability Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_process_liability_claim_taxonomy.md).
* For evaluating the compliance and control failures, see [Control Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_control_claim_taxonomy.md).
* For evaluating post-merger integration risks, see [Integration Risk Taxonomy](file:///Users/sac/process-intelligence/ma/define_integration_risk_taxonomy.md).
* For mapping these debts to the slide residual risk, see [Slide-to-Residual Map](file:///Users/sac/process-intelligence/ma/define_slide-to-residual_map.md).