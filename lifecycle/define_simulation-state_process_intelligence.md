# Lifecycle: Define Simulation-State Process Intelligence

The **Simulation Stage** is the validation phase of the process lifecycle, where process models are executed in a synthetic environment to analyze behavioral properties and performance envelopes.

## Autonomic MAPE-K Mapping
* **Loop Role**: **Analyze** (predictive evaluation)
* **Responsibility**: In the Analyze phase, the simulation engine runs the token game across the state space to detect deadlocks, verify coverability, and project queue lengths.
* **Actuation Trigger**: Underperforming simulation profiles trigger a feedback loop back to the Design state to adjust routing probabilities or capacity allocations.

---

## State Space and Token Game Verification

Before a process is activated, its state space must be explored. The basis of simulation is the **Token Game** execution of Petri Nets.

### Firing Rules
For a Petri Net $N = (P, T, F)$ and marking $M$:
1. A transition $t \in T$ is **enabled** in marking $M$ (denoted $M \stackrel{t}{\to}$) if and only if:
   $$\forall p \in \bullet t, \quad M(p) \ge 1$$
   *(where $\bullet t = \{p \in P \mid (p, t) \in F\}$ is the set of input places of $t$)*
2. Firing an enabled transition $t$ yields a new marking $M'$:
   $$\forall p \in P, \quad M'(p) = M(p) - I(p, t) + O(p, t)$$
   where $I(p, t) = 1$ if $(p, t) \in F$ else 0, and $O(p, t) = 1$ if $(t, p) \in F$ else 0.

### Reachability Graph
The **Reachability Graph** $RG(N, i)$ is a directed graph where:
* Vertices represent all markings reachable from the initial marking $i$, denoted $[N, i\rangle$.
* Directed edges are labeled with the transition that triggered the state transition: $M_1 \stackrel{t}{\to} M_2$.

### Queueing Theory and Throughput
During Monte Carlo simulations, transitions are assigned duration distributions (e.g., exponential, normal). We apply **Little's Law** to estimate queue build-ups:
$$L = \lambda W$$
where:
* $L$ is the average number of active cases in the process queue.
* $\lambda$ is the average arrival rate of new process cases.
* $W$ is the average wait and processing time (throughput time) of a case.

---

## Standards Alignment

* **BPMN 2.0 Simulation (BPSim)**: BPSim defines standardized parameters for simulation scenarios, including resource cost, processing time distributions, and calendar availability.
* **POWL Branching Probabilities**: Process tree operators are augmented with stochastic weights (e.g., choice $\times$ with $p = 0.8$ for path A and $p = 0.2$ for path B) to generate representative event logs.

---

## M&A Due Diligence Claims
In M&A, simulation validates the feasibility of **Post-Merger Synergies**.
* **Buyer Reliance**: The buyer relies on simulation to verify that the target's processes can handle the combined volume of the merged entities.
* **Slide-to-Receipt Map**: PowerPoint assertions claiming "Combining processing centers will yield a 25% cost reduction" must be backed by a simulation receipt showing the reachability graph under double-arrival load ($\lambda' = 2\lambda$) and resource sharing.

---

## Related Documents
* See the [Design Stage](file:///Users/sac/process-intelligence/lifecycle/define_design-state_process_intelligence.md) for structural baselines.
* See the [Monitoring Stage](file:///Users/sac/process-intelligence/lifecycle/define_monitoring-state_process_intelligence.md) for live trace comparison.
* Back to [Lifecycle README](file:///Users/sac/process-intelligence/lifecycle/docs-law__lifecycle_readme.md).