# OCEL Standard Ledger Placement

The **Object-Centric Event Log (OCEL 2.0)** standard (Ghahfarokhi 2021) relaxes the traditional process mining assumption that each event is bound to a single case. OCEL supports events linked to multiple heterogeneous objects (e.g., an event "Pack Order" linked to one *order* object and three *item* objects). This document establishes how OCEL structures are mapped, validated, and verified on the process-intelligence ledger.

---

## 1. Ontological Mapping to the Ledger

OCEL logs represent processes as property graphs. The foundry maps these relational networks to the ledger using three primary relation types:

| OCEL Element | Ledger Representation | Type Schema | Semantic Description |
| :--- | :--- | :--- | :--- |
| **Event** | `LedgerEvent` | Struct | A point-in-time transaction event with a unique ID, activity, timestamp, and attributes. |
| **Object** | `LedgerObject` | Struct | A persistent entity (e.g., invoice, order, container) with a unique ID, type, and dynamic attributes. |
| **Event-to-Object Link**| `EventObjectLink` | Relation | Directed edge mapping an event to a target object with a specific qualifier/role (e.g., `creator`, `modifier`). |
| **Object-to-Object Link**| `ObjectObjectLink`| Relation | Directed edge mapping an object-to-object relationship (e.g., `item_of_order`). |

---

## 2. Type-System and Cryptographic Constraints

OCEL graphs must satisfy strict structural, graph-theoretic, and temporal integrity constraints to prevent graph cycle errors, missing foreign keys, or temporal anomalies on the ledger:

1.  **Referential Integrity**:
    *   **Event-to-Object (E2O) Integrity**: Let $\text{Events}$ be the set of events, $\text{Objects}$ be the set of objects, and $\mathcal{Q}_{\text{E2O}}$ be the set of relationship qualifiers. The set of qualified event-to-object relationships $\mathcal{R}_{\text{E2O}} \subseteq \text{Events} \times \text{Objects} \times \mathcal{Q}_{\text{E2O}}$ must satisfy:
        $$\forall (e, o, q) \in \mathcal{R}_{\text{E2O}}, \quad e \in \text{Events} \land o \in \text{Objects}$$
    *   **Object-to-Object (O2O) Integrity**: Let $\mathcal{Q}_{\text{O2O}}$ be the set of O2O qualifiers. The set of qualified object-to-object relationships $\mathcal{R}_{\text{O2O}} \subseteq \text{Objects} \times \text{Objects} \times \mathcal{Q}_{\text{O2O}}$ must satisfy:
        $$\forall (o_1, o_2, q) \in \mathcal{R}_{\text{O2O}}, \quad o_1 \in \text{Objects} \land o_2 \in \text{Objects}$$

2.  **Graph Acyclicity (No Cycle Errors)**:
    *   The directed graph of object dependencies $G_{\text{O2O}} = (\text{Objects}, E_{\text{O2O}})$, where $E_{\text{O2O}} = \{ (o_1, o_2) \mid \exists q \text{ s.t. } (o_1, o_2, q) \in \mathcal{R}_{\text{O2O}} \}$, must be a Directed Acyclic Graph (DAG):
        $$\nexists (o_0, o_1, \dots, o_k) \text{ s.t. } o_0 = o_k \land k \ge 1 \land \forall i \in \{0, \dots, k-1\}, (o_i, o_{i+1}) \in E_{\text{O2O}}$$

3.  **Temporal Monotonicity and Monotonic Lifecycle**:
    *   Let $\text{time}(e)$ be the timestamp of event $e \in \text{Events}$. Let $A_o(t)$ represent the attribute state of object $o \in \text{Objects}$ at time $t$. An object's state can only evolve via events:
        $$A_o(t_2) \neq A_o(t_1) \implies \exists e \in \text{Events}, q \in \mathcal{Q}_{\text{E2O}} \text{ s.t. } (e, o, q) \in \mathcal{R}_{\text{E2O}} \land t_1 < \text{time}(e) \le t_2$$
    *   The timestamps of all attribute updates $\text{time}(a)$ for object $o$ must match the timestamps of events that are linked to $o$:
        $$\forall a \in \text{attrs}(o), \quad \text{time}(a) \in \{ \text{time}(e) \mid e \in \text{Events} \land \exists q \text{ s.t. } (e, o, q) \in \mathcal{R}_{\text{E2O}} \}$$
    *   For any sequence of events $(e_1, e_2, \dots, e_n)$ acting on a shared object $o$, the event timestamps must be monotonically non-decreasing:
        $$\forall i, j \in \{1, \dots, n\}, \quad e_i \prec_{\text{flow}} e_j \implies \text{time}(e_i) \le \text{time}(e_j)$$

4.  **Cryptographic Proof Graphs**:
    To ensure non-forgeability, the graph structure is serialized into a canonical JSON representation and hashed:
    $$\mathcal{H}_{\text{OCEL}} = \operatorname{BLAKE3}\left( \text{Events}_{\text{canon}} \parallel \text{Objects}_{\text{canon}} \parallel \mathcal{R}_{\text{E2O, canon}} \parallel \mathcal{R}_{\text{O2O, canon}} \right)$$

---

## 3. Academic Foundations and Conformance

OCEL conformance checking uses multi-object alignments (Ghahfarokhi 2021):
*   Traditional case-based conformance is insufficient because of "divergence" and "spaghetti" traces when forced into flat structures.
*   The ledger runs multi-object token game replays to verify place bounds across separate object dimensions.
*   For a sample of OCEL conformance checking, see the [OCEL Lifecycle Sample](file:///Users/sac/process-intelligence/experiments/ocel_lifecycle_sample.md).

---

## 4. M&A Slide-to-Receipt Bridge

Multi-object processes (like procurement spanning multiple invoices and delivery sheets) represent major operational risks:
1.  Board claims about supply chain efficiency must resolve to an OCEL query signature.
2.  The validation queries are mapped at [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md).
3.  The type validation is executed against the `Evidence<T, State, Witness>` lattice. For details, see the [Type-Law Atlas](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/type-law-atlas.md).

---

## 5. Trans-Standard Conversions and Loss Policy

### 5.1 Conversion: OCEL 2.0 to XES (Flattening)
When a multi-object OCEL 2.0 log is flattened to fit a single-perspective XES trace structure, significant structural information is lost.

*   **Structural Loss Policy**:
    1.  **Object-to-Object (O2O) Link Discarding**: All O2O links are completely discarded.
    2.  **Multi-Object Event Splitting / Case Projections**: Events bound to multiple objects must be duplicated across multiple cases or projected onto a single target object type. All other event-object links are pruned.
    3.  **Attribute Pruning**: Non-projected object attributes are pruned.
*   **Signed LossReport Output Schema**:
    ```json
    {
      "loss_report_id": "lr-ocel-xes-uuid",
      "timestamp": "2026-06-01T00:00:00Z",
      "source_format": "OCEL2.0",
      "target_format": "XES",
      "structural_changes": {
        "discarded_o2o_links_count": 450,
        "pruned_e2o_links_count": 1200,
        "pruned_object_attributes_count": 300,
        "duplicate_events_created": 80
      },
      "witness_signature": "SIG_ED25519_..."
    }
    ```

### 5.2 Conversion: XES to OCEL 2.0 (Object-Centric Reconstruction)
When converting a flat trace-based XES log into an Object-Centric Event Log (OCEL 2.0), the converter must reconstruct the underlying multi-object graph.

*   **Structural Loss Policy**:
    1.  **Synthetic Object Materialization**: Case IDs in XES are mapped to object instances of a default type (`CaseObject`). This is a synthetic mapping and does not represent the true physical object topology.
    2.  **Attribute Uplift**: Trace attributes are promoted to object attributes, and event attributes are preserved.
    3.  **Refusal Threshold**: If trace IDs are malformed or missing, the conversion is refused.
*   **Signed LossReport Output Schema**:
    Every conversion generates a `LossReport` signed by the translation witness:
    ```json
    {
      "loss_report_id": "lr-xes-ocel-uuid",
      "timestamp": "2026-06-01T00:00:00Z",
      "source_format": "XES",
      "target_format": "OCEL2.0",
      "structural_changes": {
        "synthetic_objects_created": 150,
        "pruned_attributes_count": 0,
        "unmapped_trace_attributes": 0
      },
      "witness_signature": "SIG_ED25519_..."
    }
    ```