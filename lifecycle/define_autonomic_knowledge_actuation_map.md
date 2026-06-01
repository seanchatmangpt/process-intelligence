# Lifecycle: Define Autonomic Knowledge Actuation Map

The **Autonomic Knowledge Actuation Map** defines the feedback orchestration protocols that govern how process intelligence autonomously transitions process models across the lifecycle using the MAPE-K (Monitor, Analyze, Plan, Execute, Knowledge) framework.

## Autonomic Mapping Matrix

| Stage | MAPE-K Element | Input Event / Log | Analysis Engine / Algorithm | Planning / Optimization | Execution Controller | Knowledge Base Asset |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Design** | **Plan / Knowledge** | N/A | Structural soundess check (Workflow Net checks) | Schema layout, declarative constraints (LTL rules) | Model compiler | Baseline Petri Net, BPMN XML, POWL model |
| **Simulation** | **Analyze** | Synthetic event stream (Monte Carlo logs) | Reachability analysis, queue length calculations | Branching probability calibration | Token Game simulator | Coverability tree, state space graph |
| **Monitoring** | **Monitor** | Real-time streams (XES, OCEL 2.0) | Token replay, A* search alignment conformance | Alert generation protocols | Event stream listener | Active alignment traces, fitness scores |
| **Repair** | **Execute** | Deviation alerts, deadlock exceptions | S-component decomposition, bypass path analysis | Refactored net layout | WASM engine hot-reloader | Repaired Petri Net, bypass routing rule |
| **Optimization**| **Analyze / Plan**| Accumulated historical event logs | Inductive Miner recursive cut detection | Process tree restructuring, debt reduction | Optimization scheduler | Discovered POWL tree, process debt ledger |
| **Decommission**| **Execute / Knowledge**| Termination signal | Log volume audits, residual capability evaluation | Archival policy selection | Execution authorization revoker | Cryptographic Decommissioning Receipt |

---

## Actuation Trigger Protocols

Autonomic transitions are regulated by three main actuation protocols:

### 1. Deviation Actuation (Monitor $\to$ Execute)
* **Trigger Condition**: Real-time monitoring reports that alignment fitness $f_{align} < 0.90$ for a moving window of 100 cases, or a deadlock state is reached.
* **Actuation**:
  1. Freeze the affected S-component.
  2. Route new cases through a safe fallback path.
  3. Invoke S-component repair and insert bypass transitions (see [Repair Stage](file:///Users/sac/process-intelligence/lifecycle/define_repair-state_process_intelligence.md)).

### 2. Debt Actuation (Monitor $\to$ Analyze $\to$ Plan)
* **Trigger Condition**: Process Debt $D_p$ exceeds $15\%$ of the total monthly operational cost.
* **Actuation**:
  1. Aggregate historical OCEL logs.
  2. Execute the Inductive Miner to discover a new block-structured model (see [Optimization Stage](file:///Users/sac/process-intelligence/lifecycle/define_optimization-state_process_intelligence.md)).
  3. Deploy the optimized model, replacing the legacy model in the execution core.

### 3. Retirement Actuation (Plan $\to$ Execute $\to$ Knowledge)
* **Trigger Condition**: Process utility falls below threshold, or replacement model is fully activated.
* **Actuation**:
  1. Revoke runtime WASM permissions.
  2. Archive logs to cold OCEL storage.
  3. Write the Cryptographic Decommissioning Receipt to the audit ledger (see [Decommissioning Stage](file:///Users/sac/process-intelligence/lifecycle/define_decommission-state_process_intelligence.md)).

---

## Related Documents
* Stage details: [Design Stage](file:///Users/sac/process-intelligence/lifecycle/define_design-state_process_intelligence.md) | [Simulation Stage](file:///Users/sac/process-intelligence/lifecycle/define_simulation-state_process_intelligence.md) | [Monitoring Stage](file:///Users/sac/process-intelligence/lifecycle/define_monitoring-state_process_intelligence.md) | [Repair Stage](file:///Users/sac/process-intelligence/lifecycle/define_repair-state_process_intelligence.md) | [Optimization Stage](file:///Users/sac/process-intelligence/lifecycle/define_optimization-state_process_intelligence.md) | [Decommissioning Stage](file:///Users/sac/process-intelligence/lifecycle/define_decommission-state_process_intelligence.md)
* Back to [Lifecycle README](file:///Users/sac/process-intelligence/lifecycle/docs-law__lifecycle_readme.md).