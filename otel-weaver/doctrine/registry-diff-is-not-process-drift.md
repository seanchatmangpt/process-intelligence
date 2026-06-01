# registry-diff-is-not-process-drift.md

**Authority:** `/Users/sac/process-intelligence/otel-weaver/doctrine`  
**Status:** ALIVE — anchored at REGISTRY_DIFF_NOT_DRIFT_001  

---

## The Schema-Reality Separation

A process intelligence architecture must separate the schema registry from the execution space. We enforce this boundary with a fundamental rule:

> **Weaver diffs are not process drift.**

A **Weaver Diff** is a change in the semantic conventions schema (e.g., changing an attribute name from `http.status_code` to `http.response.status_code`). **Process Drift** is a shift in the actual execution behavior of a business process (e.g., users bypassing approval gates, or transaction completion latency increasing due to a new resource bottleneck). 

Confusing these two concepts collapses the nominal category of *schema definition* into *execution reality*.

---

## Weaver Diffs: Schema Transitions

A Weaver diff operates in the metadata plane. It is calculated by comparing two states of the semantic conventions registry:

$$\Delta R = R_{\text{new}} \ominus R_{\text{old}}$$

- **Domain**: YAML configuration files, telemetry schema registries, Protobuf definitions, and code-generated instrumentation wrappers.
- **Goal**: Standardize the vocabulary and structural expectations of incoming telemetry feedstock.
- **Execution impact**: None. Applying a Weaver diff does not change a single line of business logic, nor does it alter how users execute tasks. It only changes how those tasks are labeled and validated in telemetry logs.

---

## Process Drift: Behavioral Divergence

Process drift operates in the execution plane. It is detected by analyzing event logs over time relative to a reference process model $M$ or historical execution baselines:

$$\Delta L = \text{Drift}(L_{\text{current}}, L_{\text{baseline}}, M)$$

- **Domain**: Object Centric Event Logs (OCEL), Petri net conformance reports, and transition latency metrics.
- **Goal**: Identify deviations in process pathing, activity sequence, cycle time, and resource allocation.
- **Execution impact**: High. Process drift indicates a change in how the business operates in reality. It represents real-world drift in behavior, compliance, or system performance.

---

## Comparison Matrix

| Attribute | Weaver Diff ($\Delta R$) | Process Drift ($\Delta L$) |
| :--- | :--- | :--- |
| **Object of Change** | The metadata schema (names, types, structure) | The real-world execution (activities, sequences, performance) |
| **Source of Truth** | Semantic convention repository (Git commit logs) | Runtime event logs (database transactions, actor activity) |
| **Detection Method** | Schema diff tools (e.g., YAML diff, Weaver compatibility checks) | Process mining algorithms (e.g., concept drift detection, conformance replay) |
| **Remediation** | Update parser mapping or code generation targets | Retrain staff, modify business logic, or update normative models |
| **Regulatory Impact** | Adjusts documentation and audit trail definitions | Indicates potential compliance violations or operational risks |

---

## The Danger of Conflating the Two

1. **The False Positive**: An organization updates its telemetry schema, changing the activity label from `verify_order` to `order.gate.verify`. The process mining dashboard shows a sudden drop in compliance and alerts on massive process drift because `verify_order` has "disappeared". In reality, the process is unchanged; only the telemetry label drifted.
2. **The False Negative**: A system undergoes significant operational drift. Customer support representatives begin bypass-approving orders over \$10k without manager sign-off. However, because the telemetry payload structure remains 100% compliant with the current OTel Weaver schema, the telemetry dashboard reports "Zero Schema Errors". If schema validation is conflated with process monitoring, this critical compliance breach goes unnoticed.

To prevent these hazards, the process intelligence engine must isolate schema translation ($\Delta R$) inside the ingestion layer (via translation mappings and `LossReports`) so that the downstream conformance checker only evaluates real execution behavioral drift ($\Delta L$).

---

## References

- [doctrine/otel-weaver-is-feedstock.md](file:///Users/sac/process-intelligence/otel-weaver/doctrine/otel-weaver-is-feedstock.md)
- [doctrine/PROCESS_INTELLIGENCE_IS_NOT.md](file:///Users/sac/process-intelligence/doctrine/PROCESS_INTELLIGENCE_IS_NOT.md)
- [standards/OTEL_WEAVER.md](file:///Users/sac/process-intelligence/standards/OTEL_WEAVER.md)
