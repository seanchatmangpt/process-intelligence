# Lifecycle: Define Autonomic Knowledge Actuation Map

The **Autonomic Knowledge Actuation Map** defines the feedback orchestration protocols that govern how process intelligence autonomously transitions process models across the lifecycle using the MAPE-K (Monitor, Analyze, Plan, Execute, Knowledge) framework.

For the theoretical basis of these transition classes, see the core doctrine in [Autonomic Knowledge Actuation](file:///Users/sac/process-intelligence/doctrine/autonomic-knowledge-actuation.md).

## Autonomic Mapping Matrix

| Stage | MAPE-K Element | Input Event / Log | Analysis Engine / Algorithm | Planning / Optimization | Execution Controller | Transition Class | Knowledge Base Asset |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Design** | **Plan / Knowledge** | N/A | Structural soundness check (Workflow Net checks) | Schema layout, declarative constraints (LTL rules) | Model compiler | $T_{\text{compliance}}$ | Baseline Petri Net, BPMN XML, POWL model |
| **Simulation** | **Analyze** | Synthetic event stream (Monte Carlo logs) | Reachability analysis, queue length calculations | Branching probability calibration | Token Game simulator | $T_{\text{compliance}}$ | Coverability tree, state space graph |
| **Monitoring** | **Monitor** | Real-time streams (XES, OCEL 2.0) | Token replay, A* search alignment conformance | Alert generation protocols | Event stream listener | N/A | Active alignment traces, fitness scores |
| **Repair** | **Execute** | Deviation alerts, deadlock exceptions | S-component decomposition, bypass path analysis | Refactored net layout | WASM engine hot-reloader | $T_{\text{elastic}}$ (local) / $T_{\text{compliance}}$ (global) | Repaired Petri Net, bypass routing rule |
| **Optimization** | **Analyze / Plan** | Accumulated historical event logs | Inductive Miner recursive cut detection | Process tree restructuring, debt reduction | Optimization scheduler | $T_{\text{compliance}}$ | Discovered POWL tree, process debt ledger |
| **Decommission** | **Execute / Knowledge** | Termination signal | Log volume audits, residual capability evaluation | Archival policy selection | Execution authorization revoker | $T_{\text{compliance}}$ | Cryptographic Decommissioning Receipt |

---

## Actuation Trigger Protocols

Autonomic transitions are regulated by three main actuation protocols, partitioned by their safety boundaries:

### 1. Deviation Actuation (Monitor $\to$ Execute)
Depending on the severity of the deviation, the actuation is routed through either the autonomous ($T_{\text{elastic}}$) or executive ($T_{\text{compliance}}$) path.

#### A. Elastic Deviation Actuation ($T_{\text{elastic}}$)
* **Trigger Condition**: Real-time monitoring reports that alignment fitness $0.85 \le f_{\text{align}} < 0.95$ for a moving window of 100 cases.
* **Actuation Sequence**:
  1. **Isolate**: Freeze only the affected S-component $N_s = (P_s, T_s, F_s) \subset W$.
  2. **Redirect**: Route new cases through a pre-compiled local fallback path defined within the S-component boundary.
  3. **Repair**: Invoke local S-component repair and insert a bypass transition (see [Repair Stage](file:///Users/sac/process-intelligence/lifecycle/define_repair-state_process_intelligence.md)).
  4. **Verification**: Confirm that the modified S-component preserves interface invariance and LTL safety:
     $$\operatorname{sound}(W') \equiv \operatorname{true} \quad \land \quad \operatorname{Proj}_{\text{Interface}}(W') = \operatorname{Proj}_{\text{Interface}}(W)$$

#### B. Compliance Deviation Actuation ($T_{\text{compliance}}$)
* **Trigger Condition**: Real-time monitoring reports that alignment fitness $f_{\text{align}} < 0.85$, or a deadlock state is reached.
* **Actuation Sequence**:
  1. **Lockdown**: Halt execution of all active instances associated with the model.
  2. **Escalate**: Raise a high-priority compliance violation alarm to the `ostar-governor` and write a failure record to the immutable ledger.
  3. **Authorize**: A state override or global model rebuild transition cannot be executed without a Governor token `GovToken(Pi_Gov)` containing an HSM-signed cryptographic authorization $\Sigma_{\text{Gov}}$.
  4. **Recover**: Execute rollback to the last compliant state space marking or promote a pre-verified candidate model.

### 2. Debt Actuation (Monitor $\to$ Analyze $\to$ Plan) [Class $T_{\text{compliance}}$]
* **Trigger Condition**: Process Debt $D_p$ exceeds $15\%$ of the total monthly operational cost.
* **Actuation Sequence**:
  1. **Extract**: Aggregate historical OCEL logs and compute bottleneck paths.
  2. **Synthesize**: Run the Inductive Miner to discover a new block-structured candidate Petri net $N_{opt}$ (see [Optimization Stage](file:///Users/sac/process-intelligence/lifecycle/define_optimization-state_process_intelligence.md)).
  3. **Verify**: Prove that the discovered candidate reduces debt and maintains structural soundness:
     $$\operatorname{sound}(N_{opt}) \equiv \operatorname{true} \quad \land \quad D_p(N_{opt}) < D_p(N_{\text{active}})$$
  4. **Deploy**: Perform a structural hot-swap ($W \to N_{opt}$) in the WASM core. Because this replaces the core execution bytecode, it requires explicit Governor validation and signature:
     $$\text{VerifyGovProof}(\Pi_{\text{Gov}}, W, N_{opt}) = \text{True}$$

### 3. Retirement Actuation (Plan $\to$ Execute $\to$ Knowledge) [Class $T_{\text{compliance}}$]
* **Trigger Condition**: Process utility falls below threshold $U_{min}$, or a replacement model is fully activated.
* **Actuation Sequence**:
  1. **Quarantine**: Disable new case initiations ($\lambda_{\text{new}} = 0$) and wait for in-flight cases to terminate.
  2. **Lock**: Revoke WASM execution permissions for the retired Petri Net. Since runtime permission revocation represents an administrative transition, it must be signed by the `ostar-governor` to prevent unauthorized denial of service.
  3. **Seal**: Archive historical event logs in OCEL 2.0 format and write the Cryptographic Decommissioning Receipt $R_d$ to the compliance ledger (see [Decommissioning Stage](file:///Users/sac/process-intelligence/lifecycle/define_decommission-state_process_intelligence.md)):
     $$R_d = \text{Sign}_{K_{\text{priv}}} \left( \text{Hash}(N) \parallel \text{Hash}(L_{\text{final}}) \parallel C_{\text{total}} \parallel F_{\text{final}} \parallel T_{\text{retire}} \right)$$

---

## Related Documents
* Stage details: [Design Stage](file:///Users/sac/process-intelligence/lifecycle/define_design-state_process_intelligence.md) | [Simulation Stage](file:///Users/sac/process-intelligence/lifecycle/define_simulation-state_process_intelligence.md) | [Monitoring Stage](file:///Users/sac/process-intelligence/lifecycle/define_monitoring-state_process_intelligence.md) | [Repair Stage](file:///Users/sac/process-intelligence/lifecycle/define_repair-state_process_intelligence.md) | [Optimization Stage](file:///Users/sac/process-intelligence/lifecycle/define_optimization-state_process_intelligence.md) | [Decommissioning Stage](file:///Users/sac/process-intelligence/lifecycle/define_decommission-state_process_intelligence.md)
* Back to [Lifecycle README](file:///Users/sac/process-intelligence/lifecycle/docs-law__lifecycle_readme.md).