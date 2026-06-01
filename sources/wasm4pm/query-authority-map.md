# Query Authority Map

This document defines the execution requirements for the query engine capabilities implemented in the Rust-based WebAssembly (`wasm4pm`) execution engine. It specifies the FFI boundaries, query compilation, graph traversal mechanics, and board-level verification mappings.

## 1. FFI Boundary and Memory Architecture for Queries

The query engine is designed to execute high-performance structural and temporal queries over object-centric event logs (OCEL 2.0) in a sandboxed, heap-bounded environment.
- **Relational Integrity Preservation**: Unlike Python/Pandas-based approaches, the WASM engine does not flatten object-centric logs. It parses and retains the full multi-perspective event-to-object and object-to-object relations.
- **Graph Index Layout**: OCEL logs are parsed into an in-memory graph index consisting of:
  - Event list (ID, activity, timestamp, type, attributes).
  - Object list (ID, type, attributes).
  - Event-to-Object (E2O) index: maps event offsets to lists of object offsets.
  - Object-to-Object (O2O) index: maps object offsets to lists of related object offsets.
- **Zero-Copy Traversal**: Query filters are applied directly to the linear memory structures using read-only offset pointers.

---

## 2. Object-Centric Process Querying (OCPQ) Engine (Kuesters 2024)

The WASM engine executes Kuesters 2024-compliant graph-based process queries.

### A. Graph Representation
The engine treats the event log as a heterogeneous directed graph $G = (V_E \cup V_O, E_{E2O} \cup E_{O2O})$, where:
- $V_E$ represents the set of event nodes.
- $V_O$ represents the set of object nodes.
- $E_{E2O} \subseteq V_E \times V_O$ are edges representing event-to-object interactions.
- $E_{O2O} \subseteq V_O \times V_O$ are edges representing object-to-object relationships (e.g., a "line item" object linked to an "order" object).

### B. Temporal Relation Semantics
Query execution involves searching for paths and subgraphs in $G$. The engine supports matching temporal constraint formulas.

For any two events $e_1, e_2 \in V_E$ and an object $o \in V_O$:
1. **Binding Constraint**: $(e_1, o) \in E_{E2O} \wedge (e_2, o) \in E_{E2O}$.
2. **Temporal Order Constraint**: $e_1.\text{timestamp} < e_2.\text{timestamp}$.
3. **Threshold Constraint**: $e_2.\text{timestamp} - e_1.\text{timestamp} \le \Delta t_{max}$.

The query evaluator performs a depth-first traversal on the index to find all satisfying event-object subgraphs.

---

## 3. Slide-to-Receipt M&A Traceability Mapping

For board-level M&A transactions, assertions regarding operational metrics must be cryptographically defensible. The query engine supports a strict **Slide-to-Receipt Mapping Protocol**:

```
[M&A Pitch Slide Claim] 
       │
       ▼ (maps to)
[OCPQ Query Definition]
       │
       ▼ (compiled and executed by)
[Rust WASM Engine (wasm4pm)]
       │
       ▼ (produces)
[Cryptographic Verification Receipt]
```

### Verification Receipt Structure
Every query execution produces a signed receipt output block containing:
1. **Source Log Hash**: SHA-256 of the admitted raw event log.
2. **Query Abstract Syntax Tree (AST) Hash**: SHA-256 of the compiled query.
3. **Execution Metadata**: Timestamp, engine version, and CPU cycle budget used.
4. **Query Results Summary**: Quantitative metrics proving the slide's assertions (e.g., "94.2% of orders were processed within 5 days").
5. **WASM Signature**: An asymmetric cryptographic signature (Ed25519) computed over the receipt fields using the engine's private key, proving the query was executed honestly on the target log.

---

## 4. Query Compilation and Error Boundaries

- **Query Parser**: The engine parses query strings (using a lightweight DSL matching the Kuesters 2024 model) into a binary AST in WASM linear memory.
- **Resource Limits**: To prevent denial-of-service, query execution is bounded by a maximum step count (instruction count). Exceeding this budget results in a `0xFB02` query timeout error.
- **Type Safety**: Invalid references to non-existent event or object attributes are caught during query compilation, yielding a structured validation error payload and preventing runtime memory violations.

---
*Back to [execution-authority-atlas.md](file:///Users/sac/process-intelligence/sources/wasm4pm/execution-authority-atlas.md)*
