# Control Claim Taxonomy

Process controls are operational constraints designed to prevent fraud, reduce errors, and ensure compliance with external GRC frameworks. During M&A transactions, verifying the effectiveness of these controls is critical for identifying compliance liabilities. This document establishes the Control Claim Taxonomy, defining control classes and their formal validation methods using Linear Temporal Logic (LTL).

## 1. Process Control Classification Matrix

Controls are classified based on their execution mode (automated vs. manual) and their intervention phase (preventive vs. detective).

```
                      ┌──────────────────────────────────────────┐
                      │             Process Controls             │
                      └────────────────────┬─────────────────────┘
         ┌─────────────────────────────┼─────────────────────────────┐
         ▼                             ▼                             ▼
 ┌──────────────┐              ┌──────────────┐              ┌──────────────┐
 │  Automated   │              │  Detective   │              │ Segregation  │
 │  Preventive  │              │ (Post-hoc)   │              │  of Duties   │
 └──────────────┘              └──────────────┘              └──────────────┘
```

| Control Class | Description | Risk Mitigated | Mathematical Validation |
| :--- | :--- | :--- | :--- |
| **Automated Preventive** | Hard system constraints that prevent out-of-order execution. | Process deviations, unauthorized actions. | Reachability analysis on sound Petri Nets (van der Aalst 1998). |
| **Detective (Post-hoc)** | Audit queries executed after the fact to identify leaks. | Undetected errors, financial leakage. | Log-level queries for late approvals or missing receipts. |
| **Segregation of Duties (SoD)** | Rule requiring distinct actors to execute different steps. | Internal fraud, collusion. | LTL constraint checks on resource attributes in event logs. |
| **Manual Authorization** | Human sign-off steps on critical transition paths. | Lack of oversight. | Event attribute analysis confirming valid signing authority. |

## 2. Mathematical Validation via LTL Formulas (Anti-Leakage Constraints)

Process mining engines (e.g., wasm4pm) evaluate control effectiveness by checking if the entire event log $L$ satisfies specific Linear Temporal Logic (LTL) properties.

### A. Segregation of Duties (SoD) Rule
To verify that the creator of a purchase order ($t_1$) is never the approver ($t_2$) for the same case:
* Let $u$ represent any user identifier.
* **Vulnerability Fix**: A unidirectional LTL formula (e.g., checking only future states after $t_1$) allows compliance leakage if $t_2$ occurs chronologically before $t_1$. To ensure absolute protection, the constraint must be bidirectional.
* The formal constraint on any trace $\sigma \in L$ is defined as:
  $$\forall u \in \operatorname{Users}(\sigma), \quad \neg \left( \Diamond (A_1 \land \operatorname{user} = u) \land \Diamond (A_2 \land \operatorname{user} = u) \right)$$
  In standard future-only LTL, this is verified by the conjunction:
  $$\Box \left( (A_1 \land \operatorname{user} = u) \implies \Box (A_2 \implies \operatorname{user} \ne u) \right) \land \Box \left( (A_2 \land \operatorname{user} = u) \implies \Box (A_1 \implies \operatorname{user} \ne u) \right)$$
* **Audit Requirement**: The control is deemed "highly effective" only if the number of violations in $L$ is exactly 0. Any trace where the same user executes both $t_1$ and $t_2$ represents a compliance leakage violation.

### B. Lead Time Bound (SLA Control)
To verify that an invoice approval ($t_2$) always occurs within 10 days of invoice receipt ($t_1$):
* The LTL formula is:
  $$\Box \left( t_1 \implies \Diamond_{\le 10\text{ days}} t_2 \right)$$
* **Audit Requirement**: The control is deemed "effective" if compliance is $\ge 98\%$.

## 3. Related M&A Validation Documents

* For the general taxonomy of diligence claims, see [Diligence Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_diligence_claim_taxonomy.md).
* For process liabilities resulting from control failures, see [Process Liability Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_process_liability_claim_taxonomy.md).
* For classifying operational debt, see [Operational Debt Taxonomy](file:///Users/sac/process-intelligence/ma/define_operational_debt_taxonomy.md).
* For strategic board claims, see [Board Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_board_claim_taxonomy.md).