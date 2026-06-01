# WF-Net Standard Ledger Placement

A **Workflow Net (WF-Net)** (van der Aalst 1998) is a specialized class of Petri Net designed for modeling business processes and workflows. Unlike general Petri nets, a WF-net represents a single case execution with clear start and end states. This document establishes how WF-nets are registered, validated, and verified on the process-intelligence ledger.

---

## 1. Topographic Mapping to the Ledger

A Petri Net $N = (P, T, F)$ is a WF-net if and only if it satisfies three structural requirements:

1.  **Unique Source Place ($i$)**: There is exactly one place $i \in P$ with no incoming arcs:
    $$\bullet i = \emptyset$$
2.  **Unique Sink Place ($o$)**: There is exactly one place $o \in P$ with no outgoing arcs:
    $$o \bullet = \emptyset$$
3.  **Path Connectivity**: Every node $n \in P \cup T$ lies on a directed path from $i$ to $o$.

The ledger verifies these structural invariants at registration time:

| WF-Net Property | Ledger Table | Verification Rule | Description |
| :--- | :--- | :--- | :--- |
| **Source Place** | `SourcePlace` | Count $\equiv 1$ | Enforces a single entry point for case initialization. |
| **Sink Place** | `SinkPlace` | Count $\equiv 1$ | Enforces a single exit point for case termination. |
| **Strong Connectivity**| `ShortCircuitTransition` | Path Reachability | A virtual transition $t^*$ connects $o$ to $i$; the short-circuited net $\overline{N}$ must be strongly connected. |

---

## 2. Structural Soundness Type Laws

The ledger enforces that any WF-net representing an active corporate process must be proven **sound** (van der Aalst 1998):

1.  **Option to Complete**: For any reachable marking $M$ from the initial marking $[i]$ (where $[i]$ denotes the marking with a single token in $i$ and 0 tokens elsewhere), the final marking $[o]$ is reachable:
    $$\forall M \in [N, [i]\rangle, \quad [o] \in [N, M\rangle$$
2.  **Proper Completion**: If marking $M$ is reachable from $[i]$ and contains a token in $o$ (i.e. $M \ge [o]$), then no other places contain tokens:
    $$\forall M \in [N, [i]\rangle, \quad (M \ge [o]) \implies (M = [o])$$
3.  **Liveness (No Dead Transitions)**: No transition in $N$ is dead; every transition is fireable from some marking reachable from $[i]$:
    $$\forall t \in T, \quad \exists M \in [N, [i]\rangle, \quad M \xrightarrow{t}$$
4.  **Verification Receipt**: The registration transaction block must contain a soundness proof signed by the compiler witness module.

---

## 3. Mathematical Verification of WF-Net Soundness and 1-Boundedness

The ledger mathematically verifies soundness and 1-boundedness by analyzing the **short-circuited Petri Net** $\overline{N} = (\overline{P}, \overline{T}, \overline{F})$ associated with $N = (P, T, F)$.

### 3.1. The Short-Circuited Construction
To construct $\overline{N}$:
1. $\overline{P} = P$
2. $\overline{T} = T \cup \{t^*\}$, where $t^*$ is a virtual feedback transition.
3. $\overline{F} = F \cup \{(o, t^*), (t^*, i)\}$

> [!IMPORTANT]
> **Theorem 3 (Soundness Equivalence)**:
> A WF-net $N$ is sound if and only if its short-circuited Petri net $\overline{N}$ is live and bounded under the initial marking $[i]$. Furthermore, if $\overline{N}$ is 1-bounded (safe), then $N$ is 1-sound.

### 3.2. Proof of Properties and Conformance to Soundness
Under the assumption that $\overline{N}$ is live and 1-bounded under $M_0 = [i]$, we prove that all soundness requirements are guaranteed:

1.  **Proof of Proper Completion**:
    Suppose Proper Completion is violated. Then there exists a marking $M \in [N, [i]\rangle$ such that $M(o) \ge 1$ and there exists a place $p \in P \setminus \{o\}$ such that $M(p) \ge 1$. Thus, $M \ge [o] + [p]$.
    In the short-circuited net $\overline{N}$, from marking $M$, the feedback transition $t^*$ is enabled since $M(o) \ge 1$. Firing $t^*$ results in:
    $$M' = M - [o] + [i] \ge [i] + [p]$$
    Since $M' \ge [i]$, we can replay the transition sequence $\sigma$ that led from $[i]$ to $M$ in the subnet $N$. Firing $\sigma$ from $M'$ yields:
    $$M'' = M' - [i] + M \ge M + [p] \ge [o] + 2[p]$$
    By repeating this cycle $n$ times, we can generate a marking $M^{(n)} \ge [o] + n[p]$. Since $n$ can be arbitrarily large, this proves that the place $p$ is unbounded in $\overline{N}$. This contradicts the assumption that $\overline{N}$ is bounded. Hence, boundedness of $\overline{N}$ mathematically guarantees Proper Completion.
2.  **Proof of Option to Complete**:
    Liveness of $\overline{N}$ requires that transition $t^*$ is not dead. Specifically, from any reachable marking $M \in [\overline{N}, [i]\rangle$, there must be a sequence of transitions that enables $t^*$. Transition $t^*$ is enabled if and only if there is a token in place $o$. Once $t^*$ fires, it consumes the token from $o$ and places it in $i$. Thus, the final marking $[o]$ is reachable from any reachable marking $M \in [N, [i]\rangle$. This guarantees the Option to Complete.
3.  **Proof of Liveness**:
    Since $\overline{N}$ is live, no transition in $T \cup \{t^*\}$ is dead. In particular, every transition $t \in T$ is active, ensuring that there are no dead activities in the workflow under the initial marking $[i]$.
4.  **1-Boundedness Under Concurrency**:
    By verifying that the short-circuited net $\overline{N}$ is 1-bounded, we guarantee that during concurrent execution (e.g., parallel paths activated by AND-splits), no place $p \in P$ can ever hold more than one token. This mathematically prevents trace buffer overflows and queue leaks in asynchronous environments.

---

## 4. Academic Foundations and Conformance

*   WF-Nets form the backbone of traditional process conformance and token game replays.
*   For the formal equations of soundness and fitness, see the [Blue River Dam Doctrine](file:///Users/sac/process-intelligence/doctrine/blue-river-dam.md).
*   For details on the paper canon, see [Paper: van_der_aalst_1998_workflow_nets](file:///Users/sac/process-intelligence/sources/papers/paper-canon.md).

---

## 5. M&A Slide-to-Receipt Bridge

To secure operational claims during mergers and acquisitions:
1.  Target process flows must map to a verified WF-Net registered in the Virtual Data Room.
2.  Any slide assertion of process flow soundness must map to a cryptographic receipt.
3.  These receipts are registered in the [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md) and verified against the thresholds in [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).

---

## Section 15: Petri Net Bipartite Arc Law (v30.1.1 Spec)

A Petri net is a tuple $N = (P, T, F, W, M_0)$ where $P$ and $T$ are disjoint finite sets of places and transitions.
The flow relation $F$ must satisfy:
$$F \subseteq (P \times T) \cup (T \times P) \qquad \text{and} \qquad F \cap (P \times P) = \emptyset \land F \cap (T \times T) = \emptyset$$
In `wasm4pm-compat`, this is structurally guaranteed because the only arc constructors are:
$$\text{PlaceToTransitionArc}\langle P, T, Weight \rangle \qquad \text{and} \qquad \text{TransitionToPlaceArc}\langle T, P, Weight \rangle$$
No place-to-place arc can be constructed in the type system.
This structural restriction ensures that bipartite graph invariants cannot be violated, rejecting any invalid net representation at compile time.