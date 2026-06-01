# Architectural Comparison: PM4Py vs. wasm4pm

Detailed structural and capability comparison between the Python-based PM4Py library and the low-level WASM-based wasm4pm engine.

## 1. Architectural Overview & Context
This document evaluates the architectural paradigms of the CPython-based process mining reference library PM4Py and the low-level WebAssembly-based execution core wasm4pm. PM4Py is designed as a data science prototyping tool, running inside single-threaded GIL-bound processes. wasm4pm is built as an edge-native, zero-trust state execution engine for secure, real-time process auditing and inline compliance verification.

| Attribute | PM4Py (Python) | wasm4pm (WebAssembly / Rust) |
| :--- | :--- | :--- |
| **Target Environment** | Data science notebooks, offline analysis | Edge sidecars, realtime inline auditing |
| **Type Safety** | Dynamic (Python duck-typing, NaN columns) | Static (Rust type-law lattice, Option<T>) |
| **Concurrency Model** | GIL-bound (thread pool / multiprocessing) | Parallel WASM worker instances |
| **Memory Footprint** | Large (Pandas DataFrames, boxed objects) | Minimal (< 16MB static linear memory) |
| **Cryptographic Integrity** | None (requires post-hoc hashing) | Active (every state transition signed) |
| **Verification Latency** | High (interpreter overhead, dynamic typing) | Zero-latency structural typestate checks |
| **Mathematical Soundness** | Post-hoc manual verification solvers | Embedded real-time Petri Net soundness |

For capability verification, see the [Capability Matrix](file:///Users/sac/process-intelligence/experiments/pm4py_vs_wasm4pm_capability_matrix.md) and [Type Boundary Matrix](file:///Users/sac/process-intelligence/experiments/pm4py_vs_compat_type_boundary_matrix.md).

---

## 2. Deep Technical Breakdown

### 2.1 Execution Environments & GIL Bottlenecks
PM4Py runs on the CPython virtual machine. Concurrency is limited by the Global Interpreter Lock (GIL), forcing the system to utilize process forks (multiprocessing) for multi-core scaling, which copies memory structures and degrades performance. 
In contrast, wasm4pm executes compiled WebAssembly bytecode within a sandboxed virtual machine (e.g., Wasmtime or Wasmer). Multi-threading is achieved via isolated, parallel WASM worker instances, maintaining clean boundaries and bypassing runtime locks.

### 2.2 Type Safety and Compilation
PM4Py utilizes Pandas DataFrames as its primary database. Pandas lacks static schema enforcement, causing missing column values to silently coerce to `NaN` (Float64), poisoning calculations. For detailed structural issues, see the [Oracle Vulnerability Audit](file:///Users/sac/process-intelligence/sources/pm4py/oracle-vulnerability.md).
wasm4pm leverages Rust's strict compile-time typestate pattern and algebraic data types (`Option<T>`). Program states represent valid markings; invalid state transitions are unrepresentable in bytecode and compile-time checked.

### 2.3 Memory Management and Representation
PM4Py stores event logs as boxed Python objects within memory-fragmented heaps, resulting in 10x-20x inflation.
wasm4pm represents event logs as flat, contiguous byte arrays in WebAssembly's linear memory. Replay routines achieve zero-copy deserialization using arena-allocated data buffers, preventing garbage collection (GC) pauses.

### 2.4 Cryptographic Integrity
PM4Py lacks built-in security controls. Traces can be altered in memory post-facto.
wasm4pm implements real-time active transition signing. Every event ingestion constructs an immutable cryptographic hash chain signed via BLAKE3 + Ed25519, verified in-memory.

---

## 3. Mathematical Foundations of Process Verification

### A. Workflow Net (WF-net) Soundness, Liveness, and Boundedness
A Workflow Net (WF-net) is a Petri Net $N = (P, T, F)$ with a dedicated source place $i \in P$ ($\bullet i = \emptyset$) and sink place $o \in P$ ($o \bullet = \emptyset$), where every node $n \in P \cup T$ lies on a path from $i$ to $o$. Let $M$ be a marking representing a multiset of tokens on $P$. The initial marking is $M_0 = [i]$ and the final marking is $M_f = [o]$. Soundness (van der Aalst 1998) requires:

1. **Option to Complete**: From any reachable marking $M$ from $M_0$, the final marking $M_f$ is reachable:
   $$\forall M \in [M_0\rangle, \quad M_f \in [M\rangle$$

2. **Proper Completion**: If a reachable marking $M$ contains a token in the sink place $o$, it must contain no other tokens:
   $$\forall M \in [M_0\rangle, \quad M(o) \ge 1 \implies M = [o]$$

3. **Liveness (No Dead Transitions)**: For any transition $t \in T$, there exists a reachable marking $M$ that enables $t$:
   $$\forall t \in T, \quad \exists M, M' \in [M_0\rangle \quad \text{s.t. } M \xrightarrow{t} M'$$

4. **Boundedness**: The net is bounded if the token count in all places remains below a positive integer $k$:
   $$\exists k \in \mathbb{N}^+ : \forall M \in [M_0\rangle, \forall p \in P, \quad M(p) \le k$$

wasm4pm embeds static checkers validating these soundness properties in-memory, whereas PM4Py executes post-hoc analysis via python solvers.

### B. Optimal Alignment Calculations
When logs contain deviations, simple token replay fails. We calculate the optimal alignment $\gamma_{\text{opt}}$ between a trace $\sigma \in L$ and the WF-net $N$. An alignment is a sequence of moves $(m_x, m_y) \in (T \cup \{\gg\}) \times (\Sigma \cup \{\gg\})$ where $\Sigma$ is the alphabet of activities and $\gg$ represents a skip.

Let $c$ be the cost function:
- Synchronous move: $c(t, a) = 0$ if activity name of $a$ matches transition $t$.
- Move on log: $c(\gg, a) > 0$.
- Move on model: $c(t, \gg) > 0$.

The optimal alignment minimizes the total cost of deviations:
$$\gamma_{\text{opt}}(\sigma, N) = \operatorname{argmin}_{\gamma \in \text{Align}(\sigma, N)} \sum_{(t, a) \in \gamma} c(t, a)$$

The overall log fitness $f(L, N)$ is computed as:
$$f(L, N) = 1 - \frac{\sum_{\sigma \in L} L(\sigma) \cdot c(\gamma_{\text{opt}}(\sigma, N))}{\sum_{\sigma \in L} L(\sigma) \cdot c(\gamma_{\text{worst}}(\sigma, N))}$$

where $\gamma_{\text{worst}}(\sigma, N)$ aligns the trace completely via moves-on-log and model steps to reach $M_f$ from $M_0$. For execution details, see the [Slide-to-Replay Map](file:///Users/sac/process-intelligence/ma/define_slide-to-replay_map.md) and the [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).

### C. Linear Temporal Logic (LTL) Semantics in Declarative Conformance
For declarative compliance (e.g., checking DECLARE constraints), compliance rules are formulated as LTL formulas over trace execution. Let a trace be $\sigma = \langle e_1, e_2, \dots, e_n \rangle$ of length $n$. For index $i \in \{1, \dots, n\}$:
- $\sigma, i \models a \iff \text{activity}(e_i) = a$
- $\sigma, i \models \neg \varphi \iff \sigma, i \not\models \varphi$
- $\sigma, i \models \varphi \land \psi \iff \sigma, i \models \varphi \text{ and } \sigma, i \models \psi$
- $\sigma, i \models \mathcal{X} \varphi \iff i < n \text{ and } \sigma, i+1 \models \varphi$ (Next)
- $\sigma, i \models \mathcal{F} \varphi \iff \exists j \in \{i, \dots, n\} \text{ s.t. } \sigma, j \models \varphi$ (Eventually)
- $\sigma, i \models \mathcal{G} \varphi \iff \forall j \in \{i, \dots, n\}, \sigma, j \models \varphi$ (Always)
- $\sigma, i \models \varphi \mathbin{\mathcal{U}} \psi \iff \exists j \in \{i, \dots, n\} \text{ s.t. } \sigma, j \models \psi \text{ and } \forall k \in [i, j-1], \sigma, k \models \varphi$ (Until)

wasm4pm compiles these temporal rules directly into Deterministic Finite Automata (DFA) state transition matrices, represented in WebAssembly linear memory as lookup arrays, executing $O(1)$ state transitions per event with zero memory allocation. PM4Py relies on runtime AST traversal, incurring high CPU cycles.

---

## 4. Verification Assets & Linkages
- To audit the performance of PM4Py and wasm4pm under zero-knowledge environments, see the [Zero-Knowledge Benchmarks](file:///Users/sac/process-intelligence/experiments/pm4py-comparison/zero-knowledge-benchmarks.md).
- To inspect full capabilities of the Python baseline, see the [PM4Py Capability Atlas](file:///Users/sac/process-intelligence/sources/pm4py/capability-atlas.md).
- For edge loss policies under high workloads, see the [Loss Policy Map](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/loss-policy-map.md).

