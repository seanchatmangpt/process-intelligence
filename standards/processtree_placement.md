# Process Tree Standard Ledger Placement

A **Process Tree** is a hierarchical, block-structured representation of a business process. Unlike arbitrary graphs, process trees define process models recursively using operators and subtrees, guaranteeing soundness by construction (no deadlocks, no livelocks, option to complete). This document establishes how process trees are serialized, registered, and validated on the process-intelligence ledger.

---

## 1. Hierarchical Mapping to the Ledger

A Process Tree is defined as a directed tree where leaf nodes represent activities (or silent steps $\tau$) and internal nodes represent control-flow operators:

| Operator Symbol | Name | Semantics |
| :--- | :--- | :--- |
| $\rightarrow$ | Sequence | Executes children from left to right. |
| $\times$ | Exclusive Choice | Executes exactly one of the children. |
| $\land$ | Parallel (AND) | Executes all children concurrently. |
| $\lor$ | Inclusive Choice (OR)| Executes one or more children. |
| $\circlearrowleft$ | Loop | Executes left child (do), then right child (redo) iteratively. |

The ledger registers process trees as structural trees (UUID-based nested parent-child pointers) matching the following database layout:

```json
{
  "node_id": "tree-root-uuid",
  "node_type": "OperatorNode",
  "operator": "Sequence",
  "children": [
    { "node_id": "leaf-A-uuid", "node_type": "ActivityNode", "activity": "Receive Order" },
    {
      "node_id": "operator-AND-uuid",
      "node_type": "OperatorNode",
      "operator": "Parallel",
      "children": [
        { "node_id": "leaf-B-uuid", "node_type": "ActivityNode", "activity": "Pack Goods" },
        { "node_id": "leaf-C-uuid", "node_type": "ActivityNode", "activity": "Generate Invoice" }
      ]
    }
  ]
}
```

---

## 2. Type Laws and Structural Invariants

The ledger enforces strict rules for process tree correctness:

1.  **Acyclicity and Arity**: The tree must contain no cycles in parent pointers, and operator nodes must satisfy arity requirements (unary for loops, binary or n-ary for sequence/choice/concurrency).
2.  **Soundness by Design**: The parser verifies that the tree contains only supported operators. Since these operators have known translations to sound Petri net subnets, the resulting model is guaranteed sound (van der Aalst 2016).
3.  **Cryptographic Blueprint**: A process tree is serialized into a canonical representation and hashed:
    $$\mathcal{H}_{\text{tree}} = \operatorname{BLAKE3}\left( \text{Tree}_{\text{canonical}} \right)$$

---

## 3. Mathematical Verification of Soundness and 1-Boundedness by Induction

We formally verify that any Process Tree $\mathcal{T}$ constructed recursively using leaf nodes (activities $\Sigma$ or silent tasks $\tau$) and operators $\{\rightarrow, \times, \land, \circlearrowleft\}$ compiles to a sound and 1-bounded Workflow Net (WF-net) $N(\mathcal{T})$.

Let $S(\mathcal{T})$ be the induction hypothesis: "The compiled Petri Net $N(\mathcal{T})$ is a sound, 1-bounded WF-net with a unique source place $i$ and unique sink place $o$."

### 3.1. Base Cases
*   **Case 1: Activity Node** ($\mathcal{T} = a$ where $a \in \Sigma$):
    $N(a)$ has places $\{i, o\}$, transition $\{a\}$, and arcs $\{(i, a), (a, o)\}$.
    *   *Connectivity*: $i \rightarrow a \rightarrow o$ is a path covering all nodes.
    *   *1-Boundedness*: Firing $a$ from $[i]$ yields $[o]$. For all reachable markings $M \in [N, [i]\rangle$ and all $p \in \{i, o\}$, $M(p) \le 1$.
    *   *Soundness*: Final marking $[o]$ is reachable, and when reached, no other tokens exist. The transition $a$ is not dead.
*   **Case 2: Silent Step** ($\mathcal{T} = \tau$):
    Identical to Case 1, substituting $a$ with a silent transition $\tau$.
    Thus, $S(\mathcal{T})$ holds for all base cases.

### 3.2. Inductive Steps
Assume that $S(\mathcal{T}_1), \dots, S(\mathcal{T}_k)$ hold for subtrees $\mathcal{T}_1, \dots, \mathcal{T}_k$, with corresponding sound, 1-bounded WF-nets $N_1, \dots, N_k$, sources $i_1, \dots, i_k$, and sinks $o_1, \dots, o_k$.

#### A. Sequence Operator: $\mathcal{T} = \rightarrow(\mathcal{T}_1, \dots, \mathcal{T}_k)$
The net $N(\mathcal{T})$ is constructed by merging the sink $o_j$ of $N_j$ with the source $i_{j+1}$ of $N_{j+1}$ for all $j \in \{1, \dots, k-1\}$. The global source is $i = i_1$ and the global sink is $o = o_k$.
*   *1-Boundedness & Liveness*: Since execution is strictly sequential, at any marking $M$, at most one subnet $N_j$ contains tokens. By induction, each $N_j$ is a 1-bounded and sound WF-net, ensuring it consumes its entry token and eventually places exactly one token in its sink $o_j = i_{j+1}$ without leaving residual tokens. The entire sequence completes at $o_k$ with no leakage.

#### B. Exclusive Choice Operator: $\mathcal{T} = \times(\mathcal{T}_1, \dots, \mathcal{T}_k)$
The net $N(\mathcal{T})$ merges all sources $i_j$ into a single global source $i$, and all sinks $o_j$ into a single global sink $o$.
*   *1-Boundedness*: Under the initial marking $[i]$, a choice is made to fire the initial transition of some subnet $N_j$. The other subnets remain completely unmarked. Since $N_j$ is 1-bounded by induction, the overall net is 1-bounded.
*   *Soundness*: The selected subnet $N_j$ is sound by induction, guaranteeing it will reach $[o]$ with no other tokens in the system. Since every transition in every branch remains fireable under its respective choice, liveness is preserved.

#### C. Parallel Concurrency Operator: $\mathcal{T} = \land(\mathcal{T}_1, \dots, \mathcal{T}_k)$
We introduce a global source $i$, global sink $o$, split transition $t_{\text{split}}$, and join transition $t_{\text{join}}$. Arcs are added:
$$F_{\text{new}} = \{(i, t_{\text{split}})\} \cup \{(t_{\text{split}}, i_j) \mid 1 \le j \le k\} \cup \{(o_j, t_{\text{join}}) \mid 1 \le j \le k\} \cup \{(t_{\text{join}}, o)\}$$
*   *1-Boundedness*: Firing $t_{\text{split}}$ puts exactly 1 token in each sub-source $i_j$. Since each $N_j$ is disjoint and 1-bounded under $[i_j]$ by induction, no place can ever contain more than 1 token.
*   *Soundness*: Each concurrent branch $N_j$ is sound and terminates by placing exactly 1 token in $o_j$ and emptying its internal places. When all branches complete, the system marking is $\sum_{j=1}^k [o_j]$. The join transition $t_{\text{join}}$ then fires, consuming all tokens from $o_j$ and producing exactly 1 token in $o$. This guarantees proper completion and the option to complete. Liveness holds as no sub-transition is dead.

#### D. Loop Operator: $\mathcal{T} = \circlearrowleft(\mathcal{T}_{\text{do}}, \mathcal{T}_{\text{redo}})$
We merge the entry of the loop with the source of $N_{\text{do}}$, and introduce choice structures to route the output of $N_{\text{do}}$ either to the global sink $o$ or to the source of $N_{\text{redo}}$. The output of $N_{\text{redo}}$ is routed back to the source of $N_{\text{do}}$.
*   *1-Boundedness*: The do and redo subnets are executed sequentially in an alternating loop. At most one subnet is active at any given time, preventing token multiplication. Thus, 1-boundedness is preserved.
*   *Soundness*: From the output of $N_{\text{do}}$, the choice to exit to the sink $o$ is always reachable, ensuring the option to complete. Liveness is maintained because both subnets can be executed repeatedly.

By induction, all Process Trees composed of these operators are structurally live and 1-bounded WF-nets.

---

## 4. Academic Foundations and Conformance

*   Process trees are discovered recursively from logs using the Inductive Miner (Leemans 2013).
*   For details on autonomic tree manipulations, see [Autonomic Knowledge Actuation](file:///Users/sac/process-intelligence/doctrine/autonomic-knowledge-actuation.md).
*   For the paper canon definitions, see the [Paper Canon](file:///Users/sac/process-intelligence/sources/papers/paper-canon.md).

---

## 5. M&A Slide-to-Receipt Bridge

To verify process model assertions during M&A transactions:
1.  Target operating model process trees must be registered in the virtual data room.
2.  Any slide assertion based on process trees must match a cryptographic receipt generated by `wasm4pm`.
3.  These receipts are registered under [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md) and checked against [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).