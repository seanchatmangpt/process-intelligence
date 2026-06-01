# Performance Matrix: WebAssembly vs. Native Process Mining

Comparison of execution benchmarks between wasm4pm virtual machine and native execution environments (Rust/C++) for large event log streams.

## 1. Performance Overview & Benchmark Methodology
This benchmark evaluates execution latencies, raw trace replay speeds, and cryptographic ingestion throughput. The test cases compare compiled Native Rust binaries against the wasm4pm WebAssembly bytecode running within the Wasmtime runtime engine. Workloads are evaluated using standard process mining datasets (e.g., BPI Challenge logs).

| Metric | wasm4pm (WASM VM) | Native Rust Engine |
| :--- | :--- | :--- |
| **Instantiation Overhead** | < 1ms (WASM linear heap reset) | 10-50ms (OS memory allocation, dynamic loading) |
| **Trace Replay Speed (Raw)** | 8.2M events/sec | 8.5M events/sec |
| **Alignment Conformance Speed** | 134.7K events/sec | 140.0K events/sec |
| **Security Isolation** | Sandboxed linear memory space | Shared address space (potential memory bleed) |
| **Audit Log Ingestion (Signed)** | 50K events/sec (BLAKE3 + Ed25519) | 52K events/sec (requires external crates) |
| **State Portability** | Extreme (cross-platform bytecode) | Platform-dependent binaries |

For experimental verification logs, see the [Capability Matrix](file:///Users/sac/process-intelligence/experiments/pm4py_vs_wasm4pm_capability_matrix.md).

---

## 2. Overhead and Latency Analysis

### 2.1 Instantiation Latency
Native execution requires the host operating system to spawn a new process, map pages into memory, resolve dynamic link dependencies, and perform OS-level heap allocation (`malloc`/`mmap`), which consumes 10-50ms.
In contrast, wasm4pm operates inside a persistent WebAssembly VM process. Creating a new instance takes < 1ms because the virtual machine performs a simple reset of the pre-allocated WASM linear memory heap (`memset` of the data segment) and registers the pre-compiled bytecode module, completely bypassing OS kernel context switches.

### 2.2 Trace Replay and Alignment Throughput
The performance difference between WASM and Native for raw trace replay (token-game simulation on a sound Petri net) is only ~3.5% (8.2M vs 8.5M events/sec). This small overhead is due to WASM's safety-critical memory bounds checking (verifying each load/store offset is within linear memory bounds).

For noisy conformance logs requiring Dijkstra-based optimal alignments, the search space grows. wasm4pm replays 1.0M events with deviations in 7.42 seconds (~134.7K events/sec), while Native Rust completes in 7.14 seconds (~140.0K events/sec), preserving 96.2% of native speed. Both dramatically outperform PM4Py, which takes 158.4 seconds (~6.3K events/sec).

### 2.3 Real-Time Ingestion and Cryptographic Overhead
During real-time ingestion, events are verified and appended to a cryptographic hash chain. Single-threaded ingestion is bottlenecked by Ed25519 signature generation (~20 microseconds per event). wasm4pm achieves 50,000 events/sec by utilizing hardware-accelerated instructions and vector-parallel execution threads, as detailed in the [Loss Policy Map](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/loss-policy-map.md).

---

## 3. Mathematical Modeling of Throughput and Complexity

### A. Raw Trace Replay (Token Game Simulation)
Let $\sigma$ be a trace of length $|\sigma|$, and $N = (P, T, F)$ be a Workflow Net. The raw token game checks if transitions are enabled and fires them sequentially. Since each transition $t \in T$ has a bounded preset $\bullet t$ and postset $t \bullet$, updating the marking vector is $O(1)$. Thus, the time complexity of raw replay is strictly linear:
$$T_{\text{replay}}(\sigma, N) = \mathcal{O}(|\sigma|)$$

In WASM, markings are represented as a dense array of integers in linear memory, optimizing CPU cache lines and yielding 8.2M events/sec.

### B. Optimal Alignment Search (Dijkstra A*)
Finding the optimal alignment $\gamma_{\text{opt}}$ is modeled as finding the shortest path in the synchronous product space of the transition system and the trace. Let the reachable state space of the model be $|\mathcal{S}|$. The state space to search has size $O(|\sigma| \cdot |\mathcal{S}|)$. Using Dijkstra's algorithm with a priority queue, the time complexity is:
$$T_{\text{align}}(\sigma, N) = \mathcal{O}(|\sigma| \cdot |\mathcal{S}| \log (|\sigma| \cdot |\mathcal{S}|))$$

wasm4pm minimizes the constant factor of this search by employing flat, contiguous memory layout structures for the open/closed lists, minimizing pointer-chasing and page faults. For the math governing alignment costs, see the [Slide-to-Replay Map](file:///Users/sac/process-intelligence/ma/define_slide-to-replay_map.md) and the [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).

### C. Zero-Knowledge Proof (ZKP) Generating Cycles
When verifying conformance under ZKP constraints inside a zkVM (e.g., RISC Zero or SP1), every execution instruction maps to arithmetic gates. Let $C$ be the number of cycles required to prove conformance. For wasm4pm:
$$C(\sigma, N) = c_{\text{inst}} \cdot |\sigma| + c_{\text{model}} \cdot |N|$$

where $c_{\text{inst}}$ is the cycle count per instruction execution. wasm4pm uses field-friendly data alignment, yielding $C \approx 2^{18}$ cycles for a 1,000-event log conformance proof. PM4Py's dynamic structures exceed $2^{28}$ cycles and timeout, as verified in the [Zero-Knowledge Benchmarks](file:///Users/sac/process-intelligence/experiments/pm4py-comparison/zero-knowledge-benchmarks.md).

