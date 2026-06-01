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

1.  **Option to Complete**: For any reachable marking $M$ from the initial marking $[i]$, the final marking $[o]$ is reachable:
    $$\forall M \in [i \rangle, \quad [o] \in [M \rangle$$
2.  **Proper Completion**: If marking $M$ contains a token in $o$, then no other places contain tokens:
    $$\forall M \in [i \rangle, \quad (M \ge [o]) \implies (M = [o])$$
3.  **Liveness (No Dead Transitions)**: No transition in $N$ is dead; every step is reachable from $[i]$:
    $$\forall t \in T, \quad \exists M \in [i \rangle, \quad M \xrightarrow{t}$$
4.  **Verification Receipt**: The registration transaction block must contain a soundness proof signed by the compiler witness module.

---

## 3. Academic Foundations and Conformance

*   WF-Nets form the backbone of traditional process conformance and token game replays.
*   For the formal equations of soundness and fitness, see the [Blue River Dam Doctrine](file:///Users/sac/process-intelligence/doctrine/blue-river-dam.md).
*   For details on the paper canon, see [Paper: van_der_aalst_1998_workflow_nets](file:///Users/sac/process-intelligence/sources/papers/paper-canon.md).

---

## 4. M&A Slide-to-Receipt Bridge

To secure operational claims during mergers and acquisitions:
1.  Target process flows must map to a verified WF-Net registered in the Virtual Data Room.
2.  Any slide assertion of process flow soundness must map to a cryptographic receipt.
3.  These receipts are registered in the [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md) and verified against the thresholds in [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).