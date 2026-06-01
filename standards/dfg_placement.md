# DFG Standard Ledger Placement

The **Directly-Follows Graph (DFG)** is a simple process mining representation where nodes are activities and directed edges represent the "directly-follows" relation observed in the event log. While DFGs lack the formal soundness guarantees of Petri nets, they are widely used for performance visualization. This document defines how DFG schemas, frequencies, and transition paths are represented on the ledger.

---

## 1. Topographic Mapping to the Ledger

A DFG is defined as a graph $G = (V, E, w, w_{\text{start}}, w_{\text{end}})$ where:
*   $V$ is the set of activity labels.
*   $E \subseteq V \times V$ is the set of directly-follows transitions.
*   $w: E \rightarrow \mathbb{N}$ maps each edge to its transition frequency.
*   $w_{\text{start}}: V \rightarrow \mathbb{N}$ maps each activity to its frequency as a trace start node.
*   $w_{\text{end}}: V \rightarrow \mathbb{N}$ maps each activity to its frequency as a trace end node.

The ledger represents a DFG using three tables:

| Element | Ledger Class | Key Schema | Description |
| :--- | :--- | :--- | :--- |
| **Activity Node** | `DFGNode` | String (SKOS Concept) | Represents an activity with total occurrence count. |
| **Directly-Follows Edge** | `DFGEdge` | `(source_id, target_id)` | Stores transition frequency $w(e)$ and optional average latency. |
| **Log Source** | `DFGSource` | UUID | Links the DFG to the source XES/OCEL log hash. |

---

## 2. Type-System and Flow Conservation

While DFGs do not support full token game replays, they must satisfy flow conservation properties:

1.  **Flow Conservation**: For any activity $v \in V$, the sum of incoming transition frequencies and start frequencies must equal the sum of outgoing transition frequencies and end frequencies:
    $$\sum_{u \in V} w(u, v) + w_{\text{start}}(v) = \sum_{w \in V} w(v, w) + w_{\text{end}}(v)$$
2.  **Edge Validation**: An edge $(u, v)$ can only exist if there is at least one trace $\sigma = \langle e_1, \dots, e_n \rangle$ in the log where $e_i = u$ and $e_{i+1} = v$:
    $$w(u, v) \ge 1$$
3.  **Cryptographic Representation**: The adjacency matrix and frequency counts are serialized and hashed to create the DFG signature:
    $$\mathcal{H}_{\text{DFG}} = \operatorname{BLAKE3}\left( \text{Matrix}_{\text{adjacency}} \parallel \text{Vectors}_{\text{start\_end}} \right)$$

---

## 3. Academic Foundations and Conformance

*   DFGs are the primary input structure for discovery algorithms like the Heuristic Miner and the Inductive Miner.
*   For comparisons between DFG-based mining and runtime execution, see the [PM4Py vs WASM4PM Matrix](file:///Users/sac/process-intelligence/experiments/pm4py_vs_wasm4pm_capability_matrix.md).
*   For the core rules of standard compliance, see the [Public Standards Gravity](file:///Users/sac/process-intelligence/doctrine/public-standards-gravity.md).

---

## 4. M&A Slide-to-Receipt Bridge

To verify process discovery and bottleneck claims during due diligence:
1.  All bottleneck visualizations in the presentation must map to a DFG adjacency matrix registered on the ledger.
2.  The buyer re-runs the DFG extractor on the raw logs to confirm that transition frequencies and average latencies match the deck metrics within the $10^{-6}$ tolerance specified in [Buyer Reliance Requirements](file:///Users/sac/process-intelligence/ma/define_buyer_reliance_requirements.md).
3.  The DFG validation receipt is registered under [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md).