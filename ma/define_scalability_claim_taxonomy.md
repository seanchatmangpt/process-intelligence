# Scalability Claim Taxonomy

Growth-stage companies often claim that their operating model is highly scalable (e.g., "We can triple transaction volume with minimal staff increases"). To prevent buyers from discounting these claims, sellers must mathematically prove scalability. This document establishes the Scalability Claim Taxonomy, detailing how process intelligence verifies process scaling limits.

## 1. Scalability Classification Matrix

Scalability is classified into four operational domains: Throughput Scalability, Resource Boundedness, Flow Liveness, and STP Elasticity.

```
                      ┌──────────────────────────────────────────┐
                      │            Process Scalability           │
                      └────────────────────┬─────────────────────┘
         ┌─────────────────────────────┼─────────────────────────────┐
         ▼                             ▼                             ▼
 ┌──────────────┐              ┌──────────────┐              ┌──────────────┐
 │  Throughput  │              │   Resource   │              │     STP      │
 │ Scalability  │              │ Boundedness  │              │  Elasticity  │
 └──────────────┘              └──────────────┘              └──────────────┘
```

| Scalability Class | Description | Business Metric | Verification Method |
| :--- | :--- | :--- | :--- |
| **Throughput Scalability** | The process's ability to maintain low cycle times as volume increases. | Transaction processing capacity. | Stress testing via simulated token injection in sound Petri Nets. |
| **Resource Boundedness** | Ensuring queues (places) do not grow infinitely under load. | Working capital health, backlog prevention. | Verification of $k$-boundedness properties (van der Aalst 1998). |
| **Flow Liveness** | Ensuring higher concurrency does not trigger deadlocks. | Operational reliability, system stability. | Petri Net liveness analysis on the Reachability Graph. |
| **STP Elasticity** | The scalability of automated tasks relative to human tasks. | Marginal cost of scale, efficiency gains. | Event log audit distinguishing API/system execution vs. human resources. |

## 2. Mathematical Proof of Scalability

To substantiate a scalability claim, the process intelligence team must prove two properties:

### A. $k$-Boundedness Verification
A place $p$ in a Petri Net is $k$-bounded if the number of tokens in $p$ never exceeds $k$ for any reachable marking $M$ from the initial marking $M_0$:
$$\forall M \in [M_0\rangle, \quad M(p) \le k$$
* **Verification Protocol**: We simulate process execution under a target transaction arrival rate $\lambda_{\text{target}} = 2 \times \lambda_{\text{baseline}}$. If any queue place $p$ has $k \to \infty$, the process is not bounded, indicating it will collapse under high volumes (operational bottleneck).

### B. Throughput Scaling Limit ($S_{\max}$)
The maximum throughput capacity of a process model $M$ is bounded by the slowest transition (the bottleneck):
$$S_{\max} = \min_{t \in T} \frac{C(t)}{D(t)}$$
* Where $T$ is the set of transitions.
* $C(t)$ is the resource capacity (headcount) assigned to transition $t$.
* $D(t)$ is the mean service time of transition $t$ calculated from the event log.
* **Audit Requirement**: To claim scalability, the bottleneck transition must be identified as an automated activity (where $C(t)$ is elastic) rather than a human-dependent activity.

## 3. Related M&A Validation Documents

* For synergy evaluations, see [Synergy Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_synergy_claim_taxonomy.md).
* For process asset valuations, see [Process Asset Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_process_asset_claim_taxonomy.md).
* For identifying pre-existing bottlenecks, see [Diligence Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_diligence_claim_taxonomy.md).
* For strategic board claims, see [Board Claim Taxonomy](file:///Users/sac/process-intelligence/ma/define_board_claim_taxonomy.md).