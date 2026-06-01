# Downstream Directive: PM4Py Benchmark Comparison

This document defines the requirements for conducting comparative benchmarks between our WebAssembly execution core (`wasm4pm`), the type-law library (`wasm4pm-compat`), and the reference Python process mining library (`PM4Py`).

## 1. Benchmarking Targets and Execution Protocols
Developers must implement automated comparison scripts that evaluate both systems under identical workloads:
- **Throughput Metrics**: Measure the number of traces replayed per second on various process sizes (small, medium, large).
- **Memory Footprint**: Profile memory usage during execution, especially tracking peak memory consumption and garbage collection overhead in PM4Py vs WASM linear memory allocations.
- **WASM Startup Latency**: Benchmark cold-start vs hot-start times for loading the WebAssembly module compared to loading python libraries.

## 2. Type Boundary and Correctness Audits
- **Duck-Typing vs Static Typing**: PM4Py heavily relies on pandas DataFrames, which are dynamic and allow missing or mistyped columns (duck-typing). In contrast, `wasm4pm` relies on strict Rust types. The benchmark must test how each library handles:
  1. Missing timestamp values.
  2. Incorrectly formatted lifecycle status values.
  3. Missing activity labels.
  - Verify that `wasm4pm-compat` raises compilation or parsing errors immediately, whereas PM4Py might fail deep inside the execution logic or yield corrupt results.
- **Result Parity Checks**: Assert that the token game fitness and alignment cost values calculated by `wasm4pm` match those calculated by PM4Py for a given sound Petri Net and event log. Document any divergence due to search heuristic variance or float rounding.
  - **Trace Fitness Equation**:
    $$f(\sigma, N) = 1 - \frac{m(\sigma, N)}{c(\sigma, N)} - \frac{r(\sigma, N)}{p(\sigma, N)}$$
  - **Log Fitness Equation**:
    $$f(L, N) = 1 - \frac{\sum_{\sigma \in L} L(\sigma) \cdot m(\sigma, N)}{\sum_{\sigma \in L} L(\sigma) \cdot c(\sigma, N)} - \frac{\sum_{\sigma \in L} L(\sigma) \cdot r(\sigma, N)}{\sum_{\sigma \in L} L(\sigma) \cdot p(\sigma, N)}$$

## 3. Reference Maps and Atlas Alignment
All implementation details must align with:
- [capability-atlas.md](file:///Users/sac/process-intelligence/sources/pm4py/capability-atlas.md)
- [pm4py_vs_wasm4pm_capability_matrix.md](file:///Users/sac/process-intelligence/experiments/pm4py_vs_wasm4pm_capability_matrix.md)
- [pm4py_vs_compat_type_boundary_matrix.md](file:///Users/sac/process-intelligence/experiments/pm4py_vs_compat_type_boundary_matrix.md)