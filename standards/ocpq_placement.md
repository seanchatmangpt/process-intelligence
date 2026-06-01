# OCPQ Standard Ledger Placement

The **Object-Centric Process Query (OCPQ)** language (Kuesters 2024) is the formal query language for analyzing multi-object execution paths. In the Process Intelligence Research Foundry, OCPQ queries act as the executable code for audit checks and compliance verifications. This document establishes how OCPQ queries, execution plans, and results are registered and verified on the ledger.

---

## 1. Query-to-Ledger Mapping

An OCPQ query specifies relationships between events, object classes, and execution paths. The ledger registers each query and its result set as a transaction block:

```json
{
  "query_metadata": {
    "query_id": "ocpq-550e8400-e29b-41d4-a716-446655440000",
    "raw_query_string": "MATCH (e1:Event {activity: 'Create Order'})-[o:Order]->(e2:Event {activity: 'Approve Invoice'}) WHERE e2.timestamp - e1.timestamp > duration('P3D')",
    "target_log_hash": "a4b2c1..."
  },
  "execution_receipt": {
    "engine_version": "wasm4pm-core-1.4.2",
    "result_row_count": 1420,
    "result_set_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    "witness_signature": "SIG_ED25519_..."
  }
}
```

The OCPQ abstract syntax tree (AST) maps to ledger relations:

| OCPQ Concept | Ledger Equivalent | Type Bound | Description |
| :--- | :--- | :--- | :--- |
| `MATCH` Clause | Graph Relation Query | Node-Arc Pattern | Defines the paths matching object-centric event flows. |
| `Event` Node | `LedgerEvent` Node | Class Pattern | Filters events based on activity label or attributes. |
| `Object` Edge | `EventObjectLink` | Attribute Pattern | Matches the object types connecting multiple events. |
| `WHERE` Constraint | Audit Constraint | Logical Expression | Boolean logic evaluating event attributes or latencies. |

---

## 2. Type-System and Query Safety

OCPQ queries must satisfy structural safety rules:

1.  **Object Association Safety**: Any variable bound to an object class must be connected to at least one Event node. Infinite cartesian products of objects are rejected at compile time.
2.  **Determinism**: The query compiler ensures that execution plans are strictly deterministic. Caching query results is protected by hashing:
    $$\mathcal{H}_{\text{result}} = \operatorname{BLAKE3}\left( \text{QueryID} \parallel \text{TargetLogHash} \parallel \text{ResultSet} \right)$$
    This prevents replay attacks where historic query results are substituted.

---

## 3. Academic Foundations and Conformance

OCPQ queries are compiled into OCPQ Trees and evaluated against OCEL databases (Kuesters 2024):
*   For details on compiling process queries, see the [Paper Canon](file:///Users/sac/process-intelligence/sources/papers/paper-canon.md).
*   For experimental query runs, see the [OCPQ Board Query Sample](file:///Users/sac/process-intelligence/experiments/ocpq_board-query_sample.md).

---

## 4. M&A Slide-to-Receipt Bridge

To verify operational claims during due diligence:
1.  All slide claims (e.g., "bottlenecks resolved, invoice cycle times cut by 4 days") must list the compiling OCPQ query hash.
2.  The buyer re-runs the OCPQ query against the event log in the virtual data room.
3.  The results must match the seller's registered ledger output, satisfying the [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).