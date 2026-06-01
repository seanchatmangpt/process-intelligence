# Diligence Claim Taxonomy

During the due diligence phase of an M&A transaction, sellers present numerous assertions regarding the target's operating model, efficiency, and compliance. This document establishes the formal classification (taxonomy) of diligence claims, mapping each claim type to its mathematically required process validation method.

## 1. Diligence Claim Classification Matrix

Diligence claims are divided into four primary domains: Performance, Compliance, Structural Integrity, and Resource/Cost Efficiency.

```
                  ┌──────────────────────────────────────────┐
                  │          Diligence Claims                │
                  └────────────────────┬─────────────────────┘
         ┌─────────────────────────────┼─────────────────────────────┐
         ▼                             ▼                             ▼
 ┌──────────────┐              ┌──────────────┐              ┌──────────────┐
 │ Performance  │              │  Compliance  │              │  Structural  │
 │  & Latency   │              │    & GRC     │              │  Integrity   │
 └──────────────┘              └──────────────┘              └──────────────┘
```

| Claim Class | Description | Example Claim | Formal Validation Method |
| :--- | :--- | :--- | :--- |
| **Performance & Latency** | Assertions about process velocity, throughput times, and bottlenecks. | "Average order-to-cash cycle time is under 4.2 days." | Event-to-event timestamp delta analysis on XES/OCEL logs. |
| **Compliance & GRC** | Assertions about adherence to internal controls, laws, and regulations. | "Zero segregation of duties (SoD) violations in procurement." | Linear Temporal Logic (LTL) checks and DECLARE rule validation on event logs. |
| **Structural Integrity** | Assertions about the robustness, simplicity, and sanity of the process flow. | "The billing process is fully standardized with 98% conformance." | Alignment-based conformance fitness calculation ($f \ge 0.95$, Adriansyah 2014). |
| **Resource & Cost** | Assertions about cost savings, resource allocation, and automation levels. | "Straight-through processing (STP) rate for invoicing is 88%." | Transition classification (automated vs. manual) in OCEL 2.0 object-centric logs. |

## 2. Technical Validation Rules by Domain

### A. Performance & Latency Claims
* **Calculation Rule**: Throughput time for a case $c$ in event log $L$ is defined as $T(c) = t(\text{last}(c)) - t(\text{first}(c))$, where $t(e)$ is the timestamp of event $e$.
* **Audit Requirement**: The seller must present the cumulative distribution function (CDF) of throughput times, demonstrating that outliers do not represent a significant liability.

### B. Compliance & GRC Claims
* **Calculation Rule**: Regulatory controls are modeled as LTL formulas. For example, a four-eyes principle on invoice approval:
  $$\Box (\text{Activity} = \text{"Approve Invoice"} \implies \text{User}_{\text{Approve}} \ne \text{User}_{\text{Create}})$$
* **Audit Requirement**: The execution core must query the entire event log history and output a list of zero violations or quantify the exact leakage rate.

### C. Structural Integrity Claims
* **Calculation Rule**: Models must be mined using algorithms that guarantee soundness, such as the Inductive Miner (Leemans 2013).
* **Audit Requirement**: The seller must prove that the mined model does not allow deadlocks and that there are no "hidden" undocumented process loops.

### D. Resource & Cost Claims
* **Calculation Rule**: Resource costs are calculated using object-centric relations in OCEL 2.0 (Ghahfarokhi 2023), mapping event resources to transactional costs.
* **Audit Requirement**: Proving that automation claims (STP) are backed by event attributes indicating system-level execution agents (e.g., API callers, RPA bots) rather than human usernames.

## 3. Related M&A Validation Documents

* For the high-level board claim classification, see [Board Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_board_claim_taxonomy.md).
* For identifying operational debt within this taxonomy, see [Operational Debt Taxonomy](file:///Users/sac/process-intelligence/ma/define_operational_debt_taxonomy.md).
* For detailed rules of board admissibility, see [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).
* For verifying compliance controls, see [Control Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_control_claim_taxonomy.md).