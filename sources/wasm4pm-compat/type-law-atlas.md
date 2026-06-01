# Type-Law Atlas (v30.1.1)

## OCEL-WASM Process-Evidence Topology
In the post-cyberpunk operational theater, the `wasm4pm-compat` foundry maps the strict type-law boundaries of process execution.

### Typestate Axioms
1. **The Principle of Temporal Non-Forging**: A process token $T_i$ cannot exist prior to its requisite structural event $E_{i-1}$.
2. **Causal Determinism in WASM linear memory**: State transitions must be chronologically contiguous within the WebAssembly module's memory bounds.
3. **Evidence Permanence**: OCEL logs emitted from the WASM boundary are immutable. Any attempt to modify a finalized state vector triggers an adversarial trace cascade.

### Type Boundaries
* `ProcessInstance`: Bound to a rigid zero-knowledge proof.
* `EventLog`: Structured purely as an append-only directed acyclic graph (DAG) of state mutations.
