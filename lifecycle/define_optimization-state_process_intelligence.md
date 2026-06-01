# Lifecycle: Define Optimization-State Process Intelligence

The **Optimization Stage** is the process improvement phase, where historical event data is analyzed to restructure process flows, eliminate process debt, and maximize efficiency.

## Autonomic MAPE-K Mapping
* **Loop Role**: **Analyze** & **Plan**
* **Responsibility**: In the Analyze phase, structural bottlenecks and process debt are identified. In the Plan phase, the process is re-synthesized using discovery algorithms to create a more efficient, sound model.
* **Actuation Trigger**: When process debt exceeds established limits, the optimization engine triggers a model redeployment to update the live system.

---

## Inductive Mining & Process Discovery (Leemans 2013)

To guarantee that the optimized model is sound, the optimization engine utilizes the **Inductive Miner** algorithm. The Inductive Miner discovers block-structured models represented as Process Trees (or POWL structures).

### Directly-Follows Graph (DFG)
Let $L$ be an event log. The Directly-Follows Graph is a graph $G = (A, E)$ where:
* $A$ is the set of activities in $L$.
* $E \subseteq A \times A$ is the set of edges. An edge $(a, b) \in E$ exists if $a$ is immediately followed by $b$ in some trace.

### Recursive DFG Partitioning (Cuts)
The Inductive Miner recursively splits the set of activities $A$ into partitions using four primary **cuts**:
1. **Exclusive Choice Cut ($\times$)**: $A$ is partitioned into $A_1, A_2, \dots, A_n$ such that there are no directly-follows edges between different partitions:
   $$\forall a \in A_i, b \in A_j \; (i \neq j), \quad (a, b) \notin E \quad \text{and} \quad (b, a) \notin E$$
2. **Sequence Cut ($\rightarrow$)**: $A$ is partitioned into $A_1, A_2, \dots, A_n$ such that edges only flow forward:
   $$\forall a \in A_i, b \in A_j \; (i < j), \quad (b, a) \notin E$$
3. **Parallel Cut ($\wedge$)**: $A$ is partitioned into $A_1, A_2, \dots, A_n$ such that the partitions are fully connected in both directions in the DFG (simulating concurrency).
4. **Loop Cut ($\circlearrowleft$)**: $A$ is partitioned into $A_{start}$ and $A_{redo}$ such that all traces start and end in $A_{start}$, and any movement to $A_{redo}$ must return to $A_{start}$.

If a cut is detected, the log is split, and the miner is called recursively on each sub-log. If no cut is found, the algorithm falls back to a filtering threshold or a flower model. Because it produces a process tree, the resulting Petri Net is **guaranteed to be sound**.

---

## Process Debt Quantification

We quantify **Process Debt** ($D_p$) to prioritize optimization efforts:
$$D_p = \sum_{c \in C} \left( Cost_{non\_conformance}(c) + Cost_{delay}(c) + Cost_{overhead}(c) \right)$$
where:
* $Cost_{non\_conformance}(c)$ is the cost of manual overrides and audit penalties for trace $c$.
* $Cost_{delay}(c)$ is the queueing delay cost modeled via Little's Law.
* $Cost_{overhead}(c)$ is the cost of redundant or duplicated steps in the process flow.

---

## Standards Alignment

* **POWL & Process Trees**: The natural output of Inductive Mining is a Process Tree, which is compiled directly into a POWL structure.
* **OCEL 2.0 Resource Allocation**: Optimization uses object-centric data to discover resource-to-task bottlenecks, suggesting optimized role definitions and task queues.

---

## M&A Due Diligence Claims
In M&A, optimization proves the **Synergy Capture** post-acquisition.
* **Buyer Reliance**: The buyer relies on this data to project exactly how much cost can be extracted from the target's operations.
* **Slide-to-Receipt Map**: Slides stating "Post-acquisition optimization will eliminate 15% of waste by consolidating procurement" must map to an Optimization run receipt showing the Inductive Miner's process tree before and after removing redundant loop paths.

---

## Related Documents
* See the [Repair Stage](file:///Users/sac/process-intelligence/lifecycle/define_repair-state_process_intelligence.md) for quick corrections.
* See the [Decommissioning Stage](file:///Users/sac/process-intelligence/lifecycle/define_decommission-state_process_intelligence.md) for retired models.
* Back to [Lifecycle README](file:///Users/sac/process-intelligence/lifecycle/docs-law__lifecycle_readme.md).