# Mining Authority Map

This document defines the execution requirements for the process discovery and mining capabilities implemented in the Rust-based WebAssembly (`wasm4pm`) execution engine. It specifies the FFI boundaries, algorithms, memory management, and mathematical foundations for mining process models from event logs.

## 1. FFI Boundary and Memory Architecture for Mining

The Rust WASM engine executes all mining operations within a sandboxed linear memory space. No external heap allocations or garbage collection from the host (Python/JS) are permitted.

### Ingestion Interface
- **Memory Pointer**: The host passes a 64-bit unsigned integer representing the memory address of the serialized event log.
- **Zero-Copy Parser**: The WASM engine parses the log directly from linear memory using flat binary schemas (FlatBuffers matching OCEL 2.0 or IEEE XES standards).
- **Arena Allocation**: Memory is managed using a region-based arena allocator to prevent fragmentation during trace parsing and directly-follows graph (DFG) construction.

### Output Serialization
- The discovered process model is serialized in linear memory as a flat binary payload containing:
  - Node list (Places, Transitions with labels).
  - Arc list (Source, Target, Weights).
  - Control Flow operators (Sequence, Choice, Parallel, Loop).
- The FFI returns a 64-bit value encoding:
  - High 32 bits: Offset pointer of the serialized model in WASM linear memory.
  - Low 32 bits: Byte length of the serialized model.

---

## 2. Mining Algorithms in the Rust WASM Engine

The engine executes three primary mining algorithms directly in WebAssembly.

### A. Alpha Miner (van der Aalst 1998)
The engine executes the classical Alpha Miner to discover a workflow net ($WF$-net) from a set of event traces.

1. **Footprint Matrix Construction**:
   Scan traces to build relation pairs over active event classes $T_L$:
   - Direct succession ($a >_L b$): $a$ is immediately followed by $b$ in at least one trace.
   - Causality ($a \rightarrow_L b$): $a >_L b$ and $b \not>_L a$.
   - Parallelism ($a \,||\,_L b$): $a >_L b$ and $b >_L a$.
   - Conflict ($a \#_L b$): $a \not>_L b$ and $b \not>_L a$.

2. **Place Identification (Maximal Pairs)**:
   Find pairs of activity sets $(A, B)$ such that:
   - For all $a \in A$ and $b \in B$, $a \rightarrow_L b$.
   - For all $a_1, a_2 \in A$, $a_1 \#_L a_2$.
   - For all $b_1, b_2 \in B$, $b_1 \#_L b_2$.
   Compute the set of maximal pairs $Y_L$ by filtering non-maximal $(A, B)$ pairs.

3. **Petri Net Reconstruction**:
   - Place set $P_L$: Create a place $p_{(A, B)}$ for each maximal pair $(A, B) \in Y_L$, plus input place $i_L$ and output place $o_L$.
   - Transition set $T_L$: Create a transition for each unique event class.
   - Arc set $F_L$: 
     - Connect $i_L$ to transitions that start traces.
     - Connect transitions that end traces to $o_L$.
     - Connect $a \in A$ to $p_{(A, B)}$ and $p_{(A, B)}$ to $b \in B$.

### B. Heuristics Miner
To handle noise and infrequent paths, the engine implements a dependency-graph heuristic miner.

1. **Dependency Measure Calculation**:
   For each pair of activities $A$ and $B$, compute:
   $$Dependency(A, B) = \frac{|A \Rightarrow B| - |B \Rightarrow A|}{|A \Rightarrow B| + |B \Rightarrow A| + 1}$$
   where $|A \Rightarrow B|$ is the count of direct successions of $A$ followed by $B$.
   For self-loops:
   $$Dependency(A, A) = \frac{|A \Rightarrow A|}{|A \Rightarrow A| + 1}$$

2. **Threshold Filtering**:
   - Reject relations where $Dependency(A, B) < \theta_{dependency}$ (default: $0.9$).
   - Reject relations where $|A \Rightarrow B| < \theta_{frequency}$ (default: $1$).
   - Apply relative-to-best thresholds to prune redundant parallel pathways.

3. **Petri Net Synthesis**:
   Convert the filtered dependency graph into a sound Petri Net by inserting structural places representing joint splits and joins.

### C. Inductive Miner (Leemans 2013)
The engine executes the Inductive Miner to guarantee sound, block-structured process models.

1. **Directly-Follows Graph (DFG) Generation**:
   Construct the DFG $(G, s, e)$ from the sublog, where nodes are activities and edges are direct successions.

2. **Cut Detection**:
   Scan the DFG to identify one of the four formal cuts:
   - **Sequence Cut**: Partition the activities into $E_1, E_2, \dots, E_k$ such that all edges between partitions flow strictly from left to right.
   - **Exclusive Choice Cut (XOR)**: Partition the activities into $E_1, E_2, \dots, E_k$ such that there are no edges in the DFG between any two partitions.
   - **Parallel Cut (AND)**: Partition the activities into $E_1, E_2, \dots, E_k$ such that for every pair of partitions, all activities are fully connected in both directions.
   - **Loop Cut (XOR-Loop)**: Partition the activities into a start partition $E_1$ and redo partitions $E_2, \dots, E_k$ satisfying loop connectivity constraints.

3. **Recursion & Fallbacks**:
   - Recursively apply cut detection on projected sublogs.
   - If no cut is detected, apply filtering to remove infrequent edges (Inductive Miner - Infrequent) and retry.
   - Fall back to the flower model (allowing any sequence of active activities) if no cut can be resolved.

4. **Tree to Net Translation**:
   Translate the resulting Process Tree (operators: Sequence $\rightarrow$, Choice $\times$, Parallel $\wedge$, Loop $\circlearrowleft$) into a Workflow Net using standard soundness-preserving templates.

---

## 3. Sandboxed Safety and Panic Handling

- **Crash Prevention**: All FFI entries are wrapped in `std::panic::catch_unwind`. Any internal Rust panic (e.g. index out of bounds during DFG traversal) is caught, and a structured error code `0xFA01` is returned.
- **Traceability Link**: The output binary payload contains the cryptographic hash of the input log, creating an immutable link from the source data to the mined model receipt.

---
*Back to [execution-authority-atlas.md](file:///Users/sac/process-intelligence/sources/wasm4pm/execution-authority-atlas.md)*
