# Lifecycle: Define Design-State Process Intelligence

The **Design Stage** is the initiation phase of the process intelligence lifecycle, where the process is modeled, verified for structural correctness, and declared.

## Autonomic MAPE-K Mapping
* **Loop Role**: **Plan** & **Knowledge**
* **Responsibility**: In the Plan phase, the target process topology is defined. In the Knowledge phase, structural assertions (such as place/transition counts, arc flows, and routing constraints) are saved as the baseline truth.
* **Actuation Trigger**: Transitioning from Design to Simulation is permitted only if the model satisfies the **Blue River Dam Gate 1: Structural Soundness**.

---

## Mathematical Soundness Constraints (van der Aalst 1998)

A process model at the Design state is formally represented as a Petri Net $N = (P, T, F)$ where:
* $P$ is a finite set of places.
* $T$ is a finite set of transitions ($P \cap T = \emptyset$).
* $F \subseteq (P \times T) \cup (T \times P)$ is a set of directed arcs (flow relation).

A Petri Net $N$ is a **Workflow Net (WF-net)** if and only if:
1. **Source Place**: There is a unique input place $i \in P$ such that $\bullet i = \emptyset$ (no incoming arcs).
2. **Sink Place**: There is a unique output place $o \in P$ such that $o \bullet = \emptyset$ (no outgoing arcs).
3. **Connectivity**: Every node $n \in P \cup T$ lies on a directed path from $i$ to $o$.

### Classical Soundness Formulations
A WF-net $N$ is **sound** if and only if:
1. **Option to Complete**: From any marking $M$ reachable from the initial marking $i$ (denoted $i \to^* M$), there exists a firing sequence $\sigma$ that reaches the final marking $o$.
   $$\forall M \in [N, i\rangle, \quad o \in [N, M\rangle$$
2. **Proper Completion**: The final marking $o$ is the only marking containing a token in the sink place $o$ reachable from $i$.
   $$\forall M \in [N, i\rangle, \quad (M \ge o) \implies (M = o)$$
3. **No Dead Transitions**: There are no dead transitions under the initial marking $i$.
   $$\forall t \in T, \exists M \in [N, i\rangle, \quad M \stackrel{t}{\to}$$

---

## Standards Alignment

To ensure execution interoperability, the Design state supports two core model representations:
1. **BPMN 2.0**: The Business Process Model and Notation standard is used for business-level visualizations. BPMN elements (gateways, tasks, events) are compiled into underlying WF-nets via formal translation mappings.
2. **POWL (Process Trees with Orc / Workflow Log structures)**: A block-structured process model notation. POWL guarantees soundness by design, as it constructs processes recursively using structured operators:
   * **Sequence** ($\rightarrow$)
   * **Exclusive Choice** ($\times$)
   * **Parallel Concurrency** ($\wedge$)
   * **Loop** ($\circlearrowleft$)

---

## M&A Due Diligence Claims
In M&A transactions, the Design-State model serves as the **Target Process Model**.
* **Buyer Reliance**: The buyer relies on this model to verify the target company's stated operating procedures.
* **Defensibility**: Claims about process design must prove that the designed model is mathematically sound (free from deadlocks or livelocks) prior to system integration, eliminating the risk of operational freezes.

---

## Related Documents
* See the [Simulation Stage](file:///Users/sac/process-intelligence/lifecycle/define_simulation-state_process_intelligence.md) for pre-execution checks.
* Review [Autonomic Knowledge Actuation Map](file:///Users/sac/process-intelligence/lifecycle/define_autonomic_knowledge_actuation_map.md) to understand MAPE-K mapping.
* Back to [Lifecycle README](file:///Users/sac/process-intelligence/lifecycle/docs-law__lifecycle_readme.md).