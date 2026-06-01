# Lifecycle: Define Repair-State Process Intelligence

The **Repair Stage** is the active intervention phase of the process lifecycle, where structural and behavioral corrections are applied to process models or execution routes to resolve conformance violations and deadlocks.

## Autonomic MAPE-K Mapping
* **Loop Role**: **Execute**
* **Responsibility**: In the Execute phase, the repair engine modifies the Petri Net topology or adjusts decision guard conditions to align the model with new execution realities or to bypass blocked paths.
* **Actuation Trigger**: Initiated automatically when the **Monitoring Stage** reports structural drift or when execution environments hit deadlock exceptions.

---

## Process Repair Algorithms

Process repair must guarantee that the repaired Petri Net $N' = (P', T', F')$ preserves soundness.

### 1. S-Component Decomposition
To isolate the part of the process requiring repair without invalidating the entire model, we decompose the Petri Net into S-components.
* An **S-net** is a Petri Net where every transition has at most one input place and at most one output place:
  $$\forall t \in T, \quad |\bullet t| \le 1 \quad \text{and} \quad |t \bullet| \le 1$$
* We identify an **S-component** $N_s = (P_s, T_s, F_s)$ of a Petri Net $N = (P, T, F)$ such that $P_s \subseteq P$, $T_s \subseteq T$, $F_s = F \cap ((P_s \times T_s) \cup (T_s \times P_s))$, and for every place $p \in P_s$, its pre- and post-sets in $N$ are contained in $T_s$:
  $$\bullet p \subseteq T_s \quad \text{and} \quad p \bullet \subseteq T_s$$

By isolating repairs to a single S-component, we can guarantee that the global soundness properties (liveness and boundedness) are maintained.

### 2. Bypass Transition Insertion
When an undocumented (but valid) business activity $a$ occurs frequently in logs, the repair engine inserts a bypass transition $t_{new}$ labeled with $a$:
1. Identify the deviation's entry place $p_{start}$ and exit place $p_{end}$ using alignment paths.
2. Insert $t_{new} \in T'$ such that $\bullet t_{new} = \{p_{start}\}$ and $t_{new} \bullet = \{p_{end}\}$.
3. Assert Soundness: The repaired WF-net remains sound if and only if there is a path from $i$ to $p_{start}$ and from $p_{end}$ to $o$, and the insertion does not create structural deadlocks (checked via coverability tree expansion).

---

## Standards Alignment

* **POWL Tree Modification**: Since POWL maps directly to block-structured process trees, repairs are often implemented as tree node insertions. For example, replacing a single activity node $A$ with an Exclusive Choice node $\times(A, B)$ when a bypass activity $B$ is introduced. This guarantees that the resulting model is structurally sound by definition.
* **BPMN Refactoring**: Automating the repositioning of task shapes and gateways to represent the repaired paths, updating the BPMN XML model definitions.

---

## M&A Due Diligence Claims
In M&A, the Repair-State validates the **Operational Resilience** of the target's business systems.
* **Buyer Reliance**: The buyer relies on this capability to ensure the business does not halt when external integration factors change.
* **Slide-to-Receipt Map**: PowerPoint assertions claiming "Our system has a self-healing process layer that prevents integration failures" must map to a Repair execution receipt proving that a structural change was successfully compiled and redeployed to downstream engines (e.g. WASM run-times) without service interruption.

---

## Related Documents
* See the [Monitoring Stage](file:///Users/sac/process-intelligence/lifecycle/define_monitoring-state_process_intelligence.md) for exception detection.
* See the [Optimization Stage](file:///Users/sac/process-intelligence/lifecycle/define_optimization-state_process_intelligence.md) for long-term discovery.
* Back to [Lifecycle README](file:///Users/sac/process-intelligence/lifecycle/docs-law__lifecycle_readme.md).