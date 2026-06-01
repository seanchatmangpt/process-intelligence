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

OCPQ queries must satisfy strict structural, graph-theoretic, and temporal safety rules, which are validated by the query compiler before execution on the ledger. Let a query logic $Q$ be represented as a tuple $Q = (V_E, V_O, E_{\text{E2O}}, C_T)$, where $V_E$ is the set of event variables, $V_O$ is the set of object variables, $E_{\text{E2O}} \subseteq V_E \times V_O$ represents event-to-object bindings, and $C_T$ is the set of temporal constraints.

1.  **Object Association Safety**:
    Every object variable $o \in V_O$ must be connected to at least one event variable in $V_E$ to prevent infinite Cartesian products:
    $$\forall o \in V_O, \quad \exists e \in V_E \text{ s.t. } (e, o) \in E_{\text{E2O}}$$

2.  **Temporal Constraint Graph Acyclicity (No Cycle Errors)**:
    Temporal constraints must define a logically satisfiable ordering. We construct a directed temporal dependency graph $G_{\text{temporal}} = (V_E, E_{\text{temporal}})$, where:
    $$E_{\text{temporal}} = \{ (v_a, v_b) \mid (v_a, v_b, \text{before}, \Delta) \in C_T \lor (v_b, v_a, \text{after}, \Delta) \in C_T \}$$
    To ensure there are no contradictory temporal constraints (such as $v_a$ occurring before $v_b$ and $v_b$ before $v_a$), $G_{\text{temporal}}$ must be a Directed Acyclic Graph (DAG):
    $$\nexists (v_0, v_1, \dots, v_k) \text{ s.t. } v_0 = v_k \land k \ge 1 \land \forall i \in \{0, \dots, k-1\}, (v_i, v_{i+1}) \in E_{\text{temporal}}$$

3.  **Query Execution Temporal Safety (No Retroactive Anomalies)**:
    Let $T_{\text{exec}}$ be the query execution timestamp. No query match can reference events that occur after the execution time (no future visibility):
    $$\forall v \in V_E, \quad \text{time}(\operatorname{match}(v)) \le T_{\text{exec}}$$
    Furthermore, the matching events must satisfy the temporal constraints:
    $$\forall (v_a, v_b) \in E_{\text{temporal}}, \quad \text{time}(\operatorname{match}(v_a)) < \text{time}(\operatorname{match}(v_b))$$

4.  **Determinism and Hashing**:
    The query compiler ensures that execution plans are strictly deterministic. Query execution results are cached and verified against replay attacks using a BLAKE3 hash over the query, target log, execution timestamp, and results:
    $$\mathcal{H}_{\text{result}} = \operatorname{BLAKE3}\left( \text{QueryID} \parallel \text{QueryLogic}_{\text{canon}} \parallel \text{TargetLogHash} \parallel T_{\text{exec}} \parallel \text{ResultSet}_{\text{canon}} \right)$$

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

---

## 5. Trans-Standard Conversions and Loss Policy

### 5.1 Conversion: OCPQ to SQL/Cypher (Query Compilation)
When executing Object-Centric Process Queries (OCPQ) on relational or graph-based database layers, the OCPQ AST is compiled into equivalent SQL or Cypher query strings.

*   **Structural Loss Policy**:
    1.  **Zero Semantic Loss Mandate**: The query compiler enforces that the translation must preserve the complete execution path semantics. The compiled query must return a mathematically identical result set. Any translation that changes path-matching boundaries is refused.
    2.  **Unsupported Property Filter Pruning**: If the target storage database does not model specific object/event attributes used in the query's `WHERE` clauses, the compiler may prune these filters and issue a warning. If they are critical for correctness, compilation is refused.
    3.  **Path Refusal**: If temporal constraints in OCPQ require look-ahead execution patterns that are not supported deterministically on the target database, the compiler refuses the compilation.
*   **Signed LossReport Output Schema**:
    Every compilation generates a `LossReport` signed by the compiler witness:
    ```json
    {
      "loss_report_id": "lr-ocpq-sql-uuid",
      "timestamp": "2026-06-01T00:00:00Z",
      "source_format": "OCPQ",
      "target_format": "SQL/Cypher",
      "structural_changes": {
        "ast_nodes_translated": 14,
        "pruned_where_expressions": 0,
        "inferred_join_paths": 3,
        "unsupported_attribute_filters_pruned": 1
      },
      "witness_signature": "SIG_ED25519_..."
    }
    ```