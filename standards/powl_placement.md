# POWL Standard Ledger Placement

The **Partial Order Workflow Language (POWL)** (Leemans 2013) is a process modeling language that extends process trees to support partial orders. A POWL model represents process execution as a hierarchical tree structure where leaves are activities and internal nodes are operators, including sequence ($\rightarrow$), loop ($\circlearrowleft$), concurrency ($\land$), exclusive choice ($\lor$), and partial order ($\text{PO}$). This document defines how POWL trees are registered, validated, and verified on the process-intelligence ledger.

---

## 1. Hierarchical Tree Mapping to the Ledger

POWL trees are serialized into the ledger as nested functional expressions. For example, a process where $A$ is followed by a choice between $B$ and $C$, concurrent with $D$, is represented as:
$$\mathcal{T} = \rightarrow(A, \land(\lor(B, C), D))$$

The ledger decomposes this hierarchy into a node-arc table:

| Node ID (UUID) | Operator Type | Parent Node ID | Left Child | Right Child / Children List |
| :--- | :--- | :--- | :--- | :--- |
| `node_root` | `Sequence` | `null` | `node_A` | `node_concurrent` |
| `node_concurrent`| `Parallel` | `node_root` | `node_choice`| `node_D` |
| `node_choice` | `ExclusiveChoice`| `node_concurrent`| `node_B` | `node_C` |

---

## 2. Type-System and Structural Soundness

A key advantage of POWL is that it guarantees **soundness by construction** for its standard operators. However, the partial order operator ($\text{PO}$) allows custom directed acyclic graphs (DAGs) of sub-activities. The ledger enforces:

1.  **Acyclicity**: The partial order DAG must be acyclic:
    $$\forall v \in V_{\text{PO}}, \quad (v, v) \notin E_{\text{PO}}^*$$
2.  **Soundness Verification**: For any partial order node, the underlying DAG must have a single start node (source) and a single end node (sink), ensuring execution behaves as a block.
3.  **Witness Signature**: When a POWL tree is compiled to a Petri net for conformance testing, the compiler outputs a cryptographic compilation proof:
    $$\text{Proof}_{\text{comp}} = \operatorname{BLAKE3}\left( \text{POWL}_{\text{tree}} \parallel \text{PetriNet}_{\text{net}} \right)$$

---

## 3. Academic Foundations and Conformance

POWL models are natively mined using the Inductive Miner (Leemans 2013):
*   Inductive Miner detects cuts in the Directly-Follows Graph (DFG) to construct the process tree recursively.
*   For details on autonomic modifications to POWL structures, see [Autonomic Knowledge Actuation](file:///Users/sac/process-intelligence/doctrine/autonomic-knowledge-actuation.md).
*   For experimental tree projections, see the [POWL Projection Sample](file:///Users/sac/process-intelligence/experiments/powl_projection_sample.md).

---

## 4. M&A Slide-to-Receipt Bridge

Process trees represent structurally sound targets. Diligencing an optimized process involves:
1.  Verifying the POWL tree's structural completeness and its alignment fitness.
2.  Mapping the POWL model hash to slide claims under [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md).
3.  Ensuring that all structural modifications are logged as transactions signed by the engine witness.

---

## 5. Trans-Standard Conversions and Loss Policy

### 5.1 Conversion: POWL to Petri Net (Sound Compilation)
When compiling a hierarchical Partial Order Workflow Language (POWL) tree to an executable Petri Net, the compiler maps operators and leaf nodes to place-transition structures.

*   **Structural Loss Policy**:
    1.  **Block-Structure Flattening**: The hierarchical block-structured nesting of operators (e.g., sequences, loops, concurrency, exclusive choices) is completely flattened into a flat place-transition network. The parent-child operator scopes and structural boundary nodes are lost.
    2.  **Partial Order Graph Mapping**: The Partial Order (PO) operator's directed acyclic graph (DAG) of activities is compiled into fork-join parallel subnets. If the PO DAG contains cycles or is not reducible to a single source-sink block, compilation is refused.
*   **Signed LossReport Output Schema**:
    Every compilation generates a `LossReport` signed by the compiler witness:
    ```json
    {
      "loss_report_id": "lr-powl-petri-uuid",
      "timestamp": "2026-06-01T00:00:00Z",
      "source_format": "POWL",
      "target_format": "PetriNet",
      "structural_changes": {
        "flattened_operators_count": 8,
        "partial_order_synchronizations_created": 4,
        "tree_nesting_levels_flattened": 3
      },
      "witness_signature": "SIG_ED25519_..."
    }
    ```