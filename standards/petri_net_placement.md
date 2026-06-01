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

The ledger enforces the formal Petri net token game execution laws:

1.  **Enabling Rule**: A transition $t \in T$ is enabled in marking $M$ if and only if each input place $p \in \bullet t$ contains at least one token:
    $$\forall p \in \bullet t, \quad M(p) \ge 1$$
2.  **Firing Rule**: Firing an enabled transition $t$ in marking $M$ results in a new marking $M'$:
    $$\forall p \in P, \quad M'(p) = M(p) - F(p, t) + F(t, p)$$
3.  **Boundedness Verification**: The ledger automatically rejects any model or execution run that violates 1-boundedness of control places, preventing trace buffer overflows.
4.  **Marking Consistency**: Transition firing is logged on the ledger as a transaction step, containing:
    $$\text{Tx} = \operatorname{BLAKE3}\left( M_{\text{initial}} \parallel t_{\text{fired}} \parallel M_{\text{final}} \right)$$

---

## 3. Academic Foundations and Conformance

*   Conformance checking against Petri nets is evaluated using token-based replay fitness and alignment cost matrices.
*   For the detailed mathematical proofs, see the [Blue River Dam Doctrine](file:///Users/sac/process-intelligence/doctrine/blue-river-dam.md).
*   For experimental validation files, see the [Petri Conformance Sample](file:///Users/sac/process-intelligence/experiments/petri_conformance_sample.md).

---

## 4. M&A Slide-to-Receipt Bridge

To secure process evaluations during mergers and acquisitions:
1.  All target process representations must be backed by a verified Petri net model registered in the data room.
2.  The conformance of historical event logs against this Petri net must be verified using `wasm4pm`.
3.  Replay proofs are mapped at [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md), guaranteeing the process fitness meets the $\theta_{\text{fit}} \ge 0.95$ threshold defined in [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).