# Lifecycle: Define Process Readiness Taxonomy

The **Process Readiness Taxonomy** defines the operational maturity levels of a process model as it transitions from raw transactional data to a fully autonomic, self-healing workflow.

## Process Readiness Levels (PRL)

The framework defines six process readiness levels:

| Level | Name | Description | Key Gate Requirement |
| :--- | :--- | :--- | :--- |
| **PRL 0** | **Raw Ingestion** | Raw system logs extracted from databases; schemas unstandardized. | Raw data access secured. |
| **PRL 1** | **Standardized Log** | Event logs mapped to IEEE XES or OCEL 2.0 formats; case identifiers and timestamps validated. | Schema validation pass. |
| **PRL 2** | **Structural Design** | Workflow Net designed and certified sound ($\operatorname{sound}(N) \equiv \operatorname{true}$); no dead transitions. | **Gate 1: Soundness Gate** |
| **PRL 3** | **Verified Simulation** | Reachability graph searched; 1-boundedness verified; no deadlocks under simulation. | **Gate 2: Behavioral Bounds** |
| **PRL 4** | **Live Operation** | Compiled to WASM kernel; message queue listeners deployed; active conformance replay running. | **ALIVE Checkpoint** |
| **PRL 5** | **Autonomic Self-Healing** | Full MAPE-K loop active; automated S-component repairs and Inductive Miner optimizations deployed without manual code changes. | **Gate 4 & Gate 5 compliance** |

---

## M&A Operational Valuation

Readiness levels are used during M&A due diligence to calculate the target's **Maturity Rating**:
* **PRL 0-1 (Low Maturity)**: High dependency on manual tasks and unstructured logging. High risk of operational failure and expensive integration.
* **PRL 2-3 (Medium Maturity)**: Well-designed processes with documented models, but lack active enforcement and real-time monitoring.
* **PRL 4-5 (High Maturity - Premium)**: Compiled, WASM-enforced workflows with real-time auditability and autonomic correction. Companies at PRL 4-5 command a premium valuation because their processes are self-documenting and highly resilient.

---

## Related Documents
* Stage details: [Design Stage](file:///Users/sac/process-intelligence/lifecycle/define_design-state_process_intelligence.md) | [Simulation Stage](file:///Users/sac/process-intelligence/lifecycle/define_simulation-state_process_intelligence.md) | [Monitoring Stage](file:///Users/sac/process-intelligence/lifecycle/define_monitoring-state_process_intelligence.md) | [Repair Stage](file:///Users/sac/process-intelligence/lifecycle/define_repair-state_process_intelligence.md) | [Optimization Stage](file:///Users/sac/process-intelligence/lifecycle/define_optimization-state_process_intelligence.md) | [Decommissioning Stage](file:///Users/sac/process-intelligence/lifecycle/define_decommission-state_process_intelligence.md)
* Back to [Lifecycle README](file:///Users/sac/process-intelligence/lifecycle/docs-law__lifecycle_readme.md).