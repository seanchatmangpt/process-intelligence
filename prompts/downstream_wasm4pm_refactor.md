# Downstream Refactoring Directive: wasm4pm Execution Core

This document outlines the mandatory design and implementation requirements for refactoring the WebAssembly-based execution engine (`wasm4pm`). The refactored core must serve as a production-grade, mathematically sound execution engine that bridges formal process mining theory with sandboxed runtime environments.

## 1. WebAssembly FFI Boundary and Memory Architecture
- **Linear Memory Management**: The engine must use a single, pre-allocated linear memory buffer. To avoid memory fragmentation, implement a custom allocator (e.g., `wee_alloc`) or a double-buffered arena allocator for trace parsing.
- **Zero-Copy Parsing**: Event data must be passed across the FFI boundary using flat binary representations. Event logs should be serialized using FlatBuffers or a custom binary schema matching the OCEL 2.0 standard.
- **FFI Signature Safety**: All exposed FFI functions must return standard error codes or 64-bit pointers encoding both the offset and length of the response payload.
- **Panic Boundaries**: Catch all Rust panics at the FFI boundary using `std::panic::catch_unwind` and translate them into structured error codes to prevent WebAssembly runtime crashes.

## 2. Formal Petri Net Engine and Token Game Replay
The core engine must contain a complete, mathematically rigorous implementation of a Workflow Net ($WF$-net) token game.
- **Soundness Analyzer**: Before replaying, verify that the Petri Net is a valid $WF$-net:
  1. There is exactly one source place $i$ such that $\bullet i = \emptyset$.
  2. There is exactly one sink place $o$ such that $o \bullet = \emptyset$.
  3. Every node $n \in P \cup T$ is on a path from $i$ to $o$.
  4. Liveness and Boundedness must be checked using a coverability graph construction to guarantee no deadlocks and that the net is 1-bounded.
- **Token Replay Mathematics**:
  For each trace $\sigma$ replayed on the Petri Net, track:
  - $p$: Produced tokens.
  - $c$: Consumed tokens.
  - $m$: Missing tokens (tokens that had to be artificially added to fire an enabled transition).
  - $r$: Remaining tokens (tokens left in the net after the trace finishes, excluding the sink place $o$).
- **Fitness Equation**:
  The trace fitness $f(\sigma, N)$ must be calculated as:
  $$f(\sigma, N) = \frac{1}{2}\left(1 - \frac{m}{c}\right) + \frac{1}{2}\left(1 - \frac{r}{p}\right)$$
  The log fitness $f(L, N)$ is the average of trace fitness weighted by trace frequency:
  $$f(L, N) = \frac{1}{2}\left(1 - \frac{\sum_{\sigma \in L} L(\sigma) \cdot m(\sigma, N)}{\sum_{\sigma \in L} L(\sigma) \cdot c(\sigma, N)}\right) + \frac{1}{2}\left(1 - \frac{\sum_{\sigma \in L} L(\sigma) \cdot r(\sigma, N)}{\sum_{\sigma \in L} L(\sigma) \cdot p(\sigma, N)}\right)$$
- **Axiom 2 Runtime Verification Monitor**:
  The engine must dynamically enforce lattice monotonicity at every transition firing step to prevent backtracking, duplicate execution, or trace laundering:
  1. **Monotonicity Verification**: For each transition firing $S_{k-1} \xrightarrow{t} S_k$, the engine must compute the step witness $W_{\text{step}}$ and update the running witness $W_k = W_{k-1} \sqcup W_{\text{step}}$. It must verify that $W_{k-1} \sqsubseteq W_k$ by asserting that $W_{k-1} \sqcup W_k = W_k$.
  2. **Rejection Semantics**: If the join operation results in a conflict ($W_{k-1} \sqcup W_{\text{step}} = \top$), the engine must immediately halt execution, roll back the active transaction, and return a structured FFI error mapped to a `RefusalReport::NonMonotonicWitnessTransition`.
  3. **No Overrides**: Axiom 2 violations represent fundamental algebraic contradictions and can never be bypassed, even with a board override signature.
  4. **Performance Bounds**: The eager witness join check must be optimized (e.g., using bitwise representation of markings and structural transition matrices) to ensure that the verification overhead remains strictly below 5% for logs containing up to 1,000,000 events.

## 3. Conformance and Alignment
- **Optimal Alignments**: Implement the $A^*$ search algorithm for computing optimal alignments between trace $\sigma$ and Petri Net $N$ based on Adriansyah 2014. Define a cost function $K$ where:
  - Move on log: $K(a, \gg) = 1$
  - Move on model: $K(\gg, t) = 1$ (except for silent transitions $\tau$, where $K(\gg, \tau) = 0$)
  - Move on both: $K(a, t) = 0$ if label(t) = a, else $\infty$.
- **Process Trees and POWL**: Support direct translation of POWL (Partial Order Workflow Language) structures to Petri Nets, ensuring sequence, loop, choice, and parallel (AND-split/join) operators maintain structural soundness.

## 4. Public Standards Enforcement
- **XES**: Enforce strict lifecycle transitions (e.g., schedule, start, complete).
- **OCEL 2.0**: Parse object types, event-to-object mappings, and object-to-object relationships without flattening data tables.

## 5. Downstream Integration and Traceability
All implementation details must align with:
- [execution-authority-atlas.md](file:///Users/sac/process-intelligence/sources/wasm4pm/execution-authority-atlas.md)
- [pm4py_vs_wasm4pm_capability_matrix.md](file:///Users/sac/process-intelligence/experiments/pm4py_vs_wasm4pm_capability_matrix.md)