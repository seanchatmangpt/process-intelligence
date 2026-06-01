# Petri Net Standard Ledger Placement

A **Petri Net** is a formal mathematical modeling language for concurrent systems, defined as a bipartite graph of places, transitions, and directed arcs. Within the Process Simple Foundry, Petri nets represent the baseline execution model used to evaluate process conformance, liveness, and liveness properties. This document defines how Petri net topologies, markings, and firing steps are recorded and verified on the ledger.

---

## 1. Topographical Mapping to the Ledger

A Petri Net is formally defined as a tuple $N = (P, T, F)$ where:
*   $P$ is a finite set of places (states).
*   $T$ is a finite set of transitions (activities).
*   $F \subseteq (P \times T) \cup (T \times P)$ is a set of directed arcs (flow relation).

The ledger represents this bipartite graph structure using primary relation tables:

| Element Type | Ledger Record | Type Bound | Description |
| :--- | :--- | :--- | :--- |
| **Place** | `LedgerPlace` | Node | Represents a state condition that can hold tokens. |
| **Transition** | `LedgerTransition` | Node | Represents an activity that consumes and produces tokens. |
| **Arc (P -> T)** | `InputArc` | Directed Edge | Connects a place to a transition, specifying input requirements. |
| **Arc (T -> P)** | `OutputArc` | Directed Edge | Connects a transition to a place, specifying output results. |
| **Marking** | `LedgerMarking` | Vector ($P \rightarrow \mathbb{N}$) | Represents the distribution of tokens across all places. |

---

## 2. Type Laws and Firing Semantics

The ledger enforces the formal Petri net token game execution laws for ordinary nets where the flow relation is defined as $F: (P \times T) \cup (T \times P) \rightarrow \{0, 1\}$:

1.  **Enabling Rule**: A transition $t \in T$ is enabled in marking $M$ if and only if each input place $p \in \bullet t$ contains at least one token:
    $$\forall p \in \bullet t, \quad M(p) \ge 1$$
2.  **Firing Rule**: Firing an enabled transition $t$ in marking $M$ results in a new marking $M'$:
    $$\forall p \in P, \quad M'(p) = M(p) - F(p, t) + F(t, p)$$
3.  **Incidence Matrix**: The topological structure is represented by the incidence matrix $C \in \{-1, 0, 1\}^{|P| \times |T|}$ where:
    $$C(p, t) = F(t, p) - F(p, t)$$
4.  **Marking Consistency**: Transition firing is logged on the ledger as a transaction step, containing:
    $$\text{Tx} = \operatorname{BLAKE3}\left( M_{\text{initial}} \parallel t_{\text{fired}} \parallel M_{\text{final}} \right)$$

---

## 3. Mathematical Verification of 1-Boundedness and Structural Liveness

To guarantee that the Petri Net is free from state-space hijacking, deadlock, and queue overflows under all concurrency edge cases, the ledger execution engine validates the following structural theorems.

### 3.1. 1-Boundedness Verification via P-Invariants

A place vector $y \in \mathbb{N}^{|P|}$ is a **P-invariant** if and only if:
$$y^T \cdot C = 0^T$$

> [!IMPORTANT]
> **Theorem 1 (Safeness/1-Boundedness Guarantee)**:
> Let $(N, M_0)$ be a Petri Net with initial marking $M_0$. If there exists a set of binary P-invariants $y_1, y_2, \dots, y_k \in \{0, 1\}^{|P|}$ such that:
> 1. The union of their supports covers the entire set of places: $\bigcup_{j=1}^k \operatorname{supp}(y_j) = P$
> 2. For each invariant $y_j$, the initial token count satisfies: $\sum_{p \in \operatorname{supp}(y_j)} M_0(p) = 1$
>
> Then the Petri Net is guaranteed to be 1-bounded (safe) for all reachable markings $M \in [N, M_0\rangle$, meaning:
> $$\forall p \in P, \quad M(p) \le 1$$

*Proof*: For any reachable marking $M$, the conservation law of P-invariants dictates that $y_j^T \cdot M = y_j^T \cdot M_0$. Because $M(p) \ge 0$ for all $p \in P$, and since the supports of the invariants cover $P$, for any place $p \in P$ there exists some $j \in \{1, \dots, k\}$ such that $p \in \operatorname{supp}(y_j)$. Thus:
$$M(p) \le \sum_{q \in \operatorname{supp}(y_j)} M(q) = y_j^T \cdot M = y_j^T \cdot M_0 = 1$$
This mathematically guarantees that no concurrency interleaving can ever lead to token accumulation ($M(p) > 1$) in any place.

### 3.2. Structural Liveness Verification via Siphons and Traps

A set of places $S \subseteq P$ is a **Siphon** if its pre-set is a subset of its post-set: $\bullet S \subseteq S\bullet$.
A set of places $Q \subseteq P$ is a **Trap** if its post-set is a subset of its pre-set: $Q\bullet \subseteq \bullet Q$.

> [!IMPORTANT]
> **Theorem 2 (Liveness Guarantee)**:
> 1. For a Free-Choice Petri Net (where for all $p_1, p_2 \in P$, either $p_1\bullet \cap p_2\bullet = \emptyset$ or $p_1\bullet = p_2\bullet$), the net is live if and only if every siphon $S \subseteq P$ contains a trap $Q \subseteq S$ such that:
>    $$\sum_{p \in Q} M_0(p) \ge 1$$
> 2. For non-free-choice Petri Nets, liveness is structurally guaranteed if no siphon can ever become empty under any reachable marking:
>    $$\forall S \subseteq P \text{ s.t. } \bullet S \subseteq S\bullet, \quad \forall M \in [N, M_0\rangle, \quad \sum_{p \in S} M(p) \ge 1$$

Since traps can never lose all tokens once marked ($\sum_{p \in Q} M(p) \ge 1$ for all reachable $M$), any siphon containing a marked trap will never become empty. Consequently, the transitions in its post-set can never become permanently disabled, preventing structural deadlocks under all choice and parallel execution paths.

### 3.3. Concurrency Edge Cases and Control

The verification engine automatically flags and rejects structural configurations that violate these properties:
*   **AND-Split / XOR-Join Mismatch**: If parallel execution paths bifurcated by an AND-split merge into an XOR-join, the P-invariant covers will fail to sum to 1, as multiple tokens will occupy the same place simultaneously. This violates 1-boundedness and is rejected.
*   **XOR-Split / AND-Join Mismatch**: If mutually exclusive choices merge into an AND-join, a siphon $S$ will be created containing the input places of the AND-join that cannot be simultaneously marked. This siphon becomes empty after a choice is made, leading to a permanent deadlock (violating liveness). This is rejected.

---

## 4. Academic Foundations and Conformance

*   Conformance checking against Petri nets is evaluated using token-based replay fitness and alignment cost matrices.
*   For the detailed mathematical proofs, see the [Blue River Dam Doctrine](file:///Users/sac/process-intelligence/doctrine/blue-river-dam.md).
*   For experimental validation files, see the [Petri Conformance Sample](file:///Users/sac/process-intelligence/experiments/petri_conformance_sample.md).

---

## 5. M&A Slide-to-Receipt Bridge

To secure process evaluations during mergers and acquisitions:
1.  All target process representations must be backed by a verified Petri net model registered in the data room.
2.  The conformance of historical event logs against this Petri net must be verified using `wasm4pm`.
3.  Replay proofs are mapped at [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md), guaranteeing the process fitness meets the $\theta_{\text{fit}} \ge 0.95$ threshold defined in [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).