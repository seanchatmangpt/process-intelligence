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

OCEL graphs must satisfy graph integrity constraints:

1.  **Referential Integrity**: Every link from an event to an object, or an object to another object, must resolve to an existing ID registered in the log:
    $$\forall (e, o) \in \text{Links}_{\text{event-object}}, \quad e \in \text{Events} \land o \in \text{Objects}$$
2.  **Object Lifecycle Monotonicity**: An object's state attributes can only evolve via events. The ledger enforces that attribute value updates correspond to recorded events with matching timestamps.
3.  **Cryptographic Proof Graphs**: To ensure non-forgeability, the graph structure is serialized into a canonical JSON representation and hashed:
    $$\mathcal{H}_{\text{OCEL}} = \operatorname{BLAKE3}\left( \text{Events}_{\text{canon}} \parallel \text{Objects}_{\text{canon}} \parallel \text{Links}_{\text{canon}} \right)$$

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