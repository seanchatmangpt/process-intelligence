# Mining Authority Map v30.1.2: wasm4pm Execution Engine

**Version:** 30.1.2  
**Authority:** Execution Agent  
**Classification:** Core Execution Specification  
**Date:** 2026-05-31

---

## Executive Summary

Mining Authority within wasm4pm governs **process discovery algorithms, computational resource allocation, and cycle-lineage attestation**. The authority ensures that all discovery operations (Inductive Miner, Heuristics Miner, DFG-based discovery) produce discoverable models that conform to formal guarantees and emit cryptographic receipts that prove correctness.

Unlike the philosophical abstraction in v30.1.1, **v30.1.2 specifies the concrete mining operations** that wasm4pm must implement to claim authority over process discovery.

---

## 1. Process Discovery Algorithms: Mandatory Implementation

### 1.1 Inductive Miner (IM)

**Authority Claim:** wasm4pm must implement the Inductive Miner algorithm per Leemans & Fahland (2013) to guarantee block-structured soundness by construction.

**Mandatory Specification:**
- **Input:** OCEL 2.0 event log $L = (E, O, e2o, attributes)$
- **Output:** Process Tree $\mathcal{T}$ (POWL 2.0 syntax) with formal guarantee: $\operatorname{sound}(\mathcal{T}) \equiv \text{True}$ (soundness by construction)
  
  **Formal Definition of Soundness:**
  Let $N = (P, T, F, M_0, M_f)$ be the Petri Net mapped from Process Tree $\mathcal{T}$, where $P$ is the set of places, $T$ is the set of transitions, $F \subseteq (P \times T) \cup (T \times P)$ is the flow relation, $M_0$ is the source place marking, and $M_f$ is the sink place marking. The Petri Net is sound if and only if:
  1. **Option to complete**: For every marking $M$ reachable from $M_0$, there exists a valid transition firing sequence leading to the final marking $M_f$:
     $$\forall M \in [N, M_0\rangle, \quad M_f \in [N, M\rangle$$
  2. **Proper completion**: For every marking $M$ reachable from $M_0$, if $M$ covers the final marking $M_f$, then it must equal $M_f$ (ensuring no leftover tokens remain in other places):
     $$\forall M \in [N, M_0\rangle, \quad (M \ge M_f) \implies (M = M_f)$$
  3. **No dead transitions**: For every transition $t \in T$, there exists some marking $M$ reachable from $M_0$ that enables $t$:
     $$\forall t \in T, \quad \exists M \in [N, M_0\rangle, \quad M \xrightarrow{t}$$

- **Proof Obligation:** Generate cryptographic receipt proving tree structure satisfies block-structure invariants
- **Parameters:**
  - Noise filtering threshold: $\delta \in [0.0, 1.0]$ (activity frequency cutoff)
  - Activity-to-variant mapping preserved in witness
  - Loop redo-frequency per block recorded in receipt

**Receipt Structure:**
```json
{
  "algorithm": "inductive_miner",
  "input_log_hash": "<SHA-256 of input OCEL>",
  "output_tree_hash": "<SHA-256 of POWL AST>",
  "block_structure_proof": {
    "depth": <tree depth>,
    "leaf_count": <activity count>,
    "loop_blocks": <count of ← → operators>,
    "xor_blocks": <count of × operators>,
    "par_blocks": <count of ∧ operators>
  },
  "soundness_guarantee": "block_structure_sound",
  "noise_filter_delta": <threshold>,
  "timestamp": "<ISO 8601>",
  "signature": "<Ed25519>"
}
```

### 1.2 Heuristics Miner (HM)

**Authority Claim:** wasm4pm must implement Heuristics Miner (Weijters & Ribeiro 2011) to discover models from logs with high noise and variant explosion, producing Directly-Follows Graph (DFG) and Petri Net equivalents.

**Mandatory Specification:**
- **Input:** XES or OCEL event log with activity sequences
- **Output:** 
  - DFG: $G = (A, E_{df}, f)$ where $A$ = activities, $E_{df}$ = directly-follows edges, $f$ = frequency
  - Petri Net: $N = (P, T, F, M_0)$ with places representing activity relations
- **Heuristics:**
  - Dependency measure: $\text{dep}(a, b) = \frac{|\text{a→b}| - |\text{b→a}|}{|\text{a→b}| + |\text{b→a}| + 1}$
  - Long-distance dependencies (l-loops): Traces with $a \to ... \to a$ sequences
  - AND-split/join detection via co-occurrence matrix
- **Loss Report:** DFG flattens concurrent relationships; receipt must quantify lost information
- **Noise Threshold:** Configurable edge/activity cutoff; rejected paths logged with severity

**Receipt Structure:**
```json
{
  "algorithm": "heuristics_miner",
  "input_log_hash": "<SHA-256>",
  "output_net_hash": "<SHA-256>",
  "dependency_threshold": <threshold>,
  "dfg_edges": <count>,
  "long_distance_loops": <count>,
  "lost_concurrency_estimate": <cardinality>,
  "soundness_status": "not_guaranteed",
  "warning": "Petri Net output is approximation; verify against log",
  "signature": "<Ed25519>"
}
```

### 1.3 Directly-Follows Graph (DFG) Mining

**Authority Claim:** wasm4pm must compute DFG directly from event logs with frequency annotations, producing a foundation for conformance checking and variant visualization.

**Mandatory Specification:**
- **Input:** Event sequence with case/object grouping
- **Output:** 
  - Node set: Activities in log order
  - Edge set: $(a, b, f)$ tuples (source, target, frequency)
  - Frequency statistics: min/max/avg transition frequency
- **Variant Discovery:** Enumerate trace variants and rank by frequency
- **Conformance Readiness:** DFG serves as input to PM4Py's conformance checking for baseline fitness before Petri Net alignment
- **Zero-Copy Implementation:** DFG must be constructed in linear memory without data duplication

**Receipt Structure:**
```json
{
  "algorithm": "dfg_mining",
  "input_log_hash": "<SHA-256>",
  "output_dfg_hash": "<SHA-256>",
  "node_count": <activity count>,
  "edge_count": <transition count>,
  "variant_count": <unique traces>,
  "top_5_variants": [
    { "sequence": "<activity sequence>", "frequency": <count>, "percentage": <pct> }
  ],
  "timestamp": "<ISO 8601>",
  "signature": "<Ed25519>"
}
```

### 1.4 Zero-Copy Bitmask Projection for Sub-DFGs

**Authority Claim:** wasm4pm must perform sub-DFG projection using read-only bitmask indexing over the primary event sequence in linear memory. This guarantees that recursive partitioning (e.g., during Inductive Miner decomposition) avoids dynamic log replication and operates with a strictly constant $\mathcal{O}(1)$ memory allocation footprint.

**Formal Specification:**
Let the raw event log be represented contiguously in linear memory as a sequence of events $E = \langle e_0, e_1, \dots, e_{N_e-1} \rangle$, and a set of objects $O = \{ o_0, o_1, \dots, o_{N_o-1} \}$.
A sub-DFG projection is defined by an event bitmask $B \in \{0, 1\}^{N_e}$ where $B[i] = 1$ if event $e_i$ is active in the sub-log projection, and $0$ otherwise.

The projected sub-DFG $G_B = (V_B, E_B, f_B)$ is computed directly from $E$ and $B$:
- **Active Vertex Set:** $V_B = \{ \operatorname{act}(e_i) \mid B[i] = 1 \}$.
- **Directed Edge Set & Frequency Mapping:** For each object $o \in O$, let the active event sequence project as:
  $$\sigma_{B, o} = \langle e_{i_1}, e_{i_2}, \dots, e_{i_k} \rangle \quad \text{where } B[i_j] = 1 \text{ and } o \in \operatorname{e2o}(e_{i_j})$$
  The set of edges $E_B$ consists of all activity pairs $(a, b) = (\operatorname{act}(e_{i_j}), \operatorname{act}(e_{i_{j+1}}))$ for $1 \le j < k$. The frequency $f_B(a, b)$ is the total count of such adjacent transitions summed across all objects $o \in O$.

**Algorithmic Footprint & Allocations ($\mathcal{O}(1)$ Complexity Guarantee):**
To guarantee zero dynamic memory allocations at runtime, wasm4pm pre-allocates the following scratch regions in the transient memory segment:
1. **Bitmask Stack:** A pre-allocated bitmask stack of size $d_{\max} \times \lceil N_e / 64 \rceil$ words of `u64`, where $d_{\max}$ is the recursion depth limit (e.g., $128$).
2. **Frequency Adjacency Matrix:** A flat array of size $|A| \times |A|$ where $|A| \le 1,000$ unique activities, using $\mathcal{O}(|A|^2)$ space.
3. **Traversal State Index:** A temporary tracking array `last_active_event_for_object` of size $N_o$ (`i32` pointers/indices), storing the index of the last active event per object.

**Single-Pass Projection Construction Algorithm:**
1. Initialize the adjacency matrix elements to $0$.
2. Initialize `last_active_event_for_object` array elements to $-1$.
3. Scan through all event indices $i \in [0, N_e - 1]$:
   - Compute bit index: `word_idx = i / 64` and `bit_mask = 1 << (i % 64)`.
   - If `(B[word_idx] & bit_mask) == 0`, skip event $i$.
   - Retrieve activity index `act_idx` of $e_i$.
   - For each object index `obj_idx` associated with $e_i$ via the E2O index:
     - Let `prev_idx = last_active_event_for_object[obj_idx]`.
     - If `prev_idx >= 0`:
       - Retrieve activity index `prev_act_idx` of $e_{\text{prev\_idx}}$.
       - Increment `dfg_matrix[prev_act_idx * |A| + act_idx]` with saturation checking.
     - Update `last_active_event_for_object[obj_idx] = i`.

This algorithm requires exactly zero heap/arena allocations during the mining process, executing in linear time $\mathcal{O}(N_e \cdot \operatorname{deg}_{\max}(e))$ where $\operatorname{deg}_{\max}(e)$ is the maximum number of objects linked to a single event.

---

## 2. Computational Resource Extraction (Cycle Accounting)

### 2.1 Cycle Budgeting per Discovery Phase

wasm4pm operates under strict **cycle budgeting** to prevent discovery algorithms from consuming unbounded compute:

| Phase | Operation | Cycle Budget | Justification |
|-------|-----------|--------------|---------------|
| **Parsing** | XES/OCEL deserialization | 1M cycles | Linear in log size |
| **DFG Construction** | Graph building, frequency counting | 2M cycles | Quadratic in activity count |
| **Inductive Miner** | Tree decomposition, soundness proof | 5M cycles | Worst-case exponential in depth |
| **Heuristics Miner** | Dependency matrix, long-distance analysis | 3M cycles | Matrix operations |
| **Alignment** (per trace) | A* search on Petri Net | 100K cycles | Depends on trace length |
| **Receipt Generation** | Cryptographic signing | 500K cycles | BLAKE3 + Ed25519 |

**Overflow Protocol:**
- If phase exceeds budget: `0xFB01` cycle overflow error
- Partial results: Emit refusal report with last valid milestone
- Retry with reduced complexity: Filter activities by frequency threshold, reduce noise budget

### 2.2 Proof-of-Execution Cycle Lineage

Every cycle consumed must be traceable to an authorized discovery operation:

```rust
pub struct CycleReceipt {
    pub operation: DiscoveryPhase,
    pub cycles_allocated: u64,
    pub cycles_consumed: u64,
    pub utilization_pct: f64,
    pub witness_hash: Blake3Hash,  // Proof of work
    pub timestamp: Timestamp,
    pub authority_signature: Ed25519Signature,
}
```

**Laundry Prevention:** Non-discovery operations (user-provided scripts, AGI reasoning loops) are rejected if they attempt to masquerade as legitimate operations. Detection mechanism:
- Check operation type against authorized discovery kernel set
- Verify that witness hash matches expected output type
- Reject cycles allocated to undefined operations

---

## 3. Raw-Laundering Prevention in Mining

### 3.1 Definition: Raw-Laundering in Discovery Context

**Raw Laundering** in mining = Injecting non-mined artifacts (hand-coded Petri Nets, manually constructed models) into the process ledger while claiming they were algorithmically discovered.

**Prevention:**
1. **Discovery Receipt Enforcement:** Every artifact entering wasm4pm must carry a discovery receipt signed by the mining authority (not just a host process).
2. **Algorithm Attestation:** The receipt must specify which algorithm produced the artifact (IM, HM, DFG, alpha-miner).
3. **Input Log Binding:** The receipt cryptographically binds the output model to the input log hash. A model cannot be reused across logs without a new receipt.

### 3.2 Cycle-Lineage Auditing

If a model is submitted without a discovery receipt:
- **Refusal:** Reject with error code `0xFC02` (missing proof of discovery)
- **Alternative Path:** Compute "post-hoc fitness" against the claimed input log:
  - Run conformance checker on every trace in log
  - If fitness ≥ 0.95: Model is plausibly mined; issue provisional receipt
  - If fitness < 0.95: Model rejected; likely hand-coded or from different log

### 3.3 Unaccounted-For Cycles

Any cycles consumed by discovery that are not accounted for in the cycle receipt trigger:
- **Audit Flag:** Log anomaly in conformance ledger
- **Cycle Refund:** Attributed cycles are revoked; model marked as "unverified"
- **Escalation:** Human auditor review required before operational use

---

## 4. Wasm4pm Mining Authority Boundary

### 4.1 FFI Boundary: Host→Guest

```
[Host Process]
  ├─ Calls: discover_inductive_miner(log_ptr, log_len, noise_threshold)
  │   Returns: (model_ptr, model_len, receipt_ptr, receipt_len)
  │
  └─ Calls: verify_receipt(receipt_ptr, receipt_len, pubkey_ptr)
      Returns: Receipt validation status (OK, INVALID_SIGNATURE, EXPIRED)
```

**Zero-Copy Requirement:** Event log passed as byte buffer; model output returned as serialized POWL/Petri Net; receipt as JSON.

### 4.2 Discovery Kernel: In-Guest (WASM Linear Memory)

Discovery algorithms run entirely within WASM sandbox:
- No access to host filesystem
- No access to external network
- All state is ephemeral; cleared after receipt generation
- Memory bounds enforced by linear memory allocator

### 4.3 Cycle Attestation: Proof-of-Work

Every receipt contains a **cycle witness** (proof that discovery consumed expected cycles):

```rust
pub struct CycleWitness {
    pub challenge: Blake3Hash,  // Deterministic challenge from input
    pub proof: ProofOfWork,     // Partial collision on challenge
    pub cycles_proven: u64,
}
```

This allows external verification (by PM4Py or other auditors) that wasm4pm actually performed the discovery.

### 4.4 Boundary Conditions, Memory Isolation, & Non-Determinism Invariants

To secure the mining process against memory exploits and execution variations, the Mining Authority enforces these invariants:

1. **Graph Density and Memory Allocation Boundaries**:
   - For Heuristics Miner and DFG mining, activity-to-activity dependency matrices scale quadratically ($N_{\text{activities}}^2$).
   - The engine enforces a hard limit of $N_{\text{activities}} \le 1,000$ unique activities. Attempting to parse or mine logs exceeding this threshold aborts with resource limit error `0xFB01`.
   - The total number of events in log $L$ is capped at $E_{\max} = 10,000,000$ events.

2. **Pointer Validation and Guest Isolation**:
   - The pointers `log_ptr`, `model_ptr`, and `receipt_ptr` must lie entirely within the guest WASM linear memory boundaries (`0` to `wasm_memory_size_in_bytes`).
   - The memory reader must not read past `log_len` bytes. Out-of-bounds pointer reads trigger a hardware trap and invalidate execution.
   - Host-provided allocations are cleared and isolated between mining phases.

3. **Deterministic Graph & Tie-Breaking Invariants**:
   - During Heuristics Miner dependency evaluation, if two activity relations $a \to b$ and $a \to c$ exhibit equal dependency measures $\text{dep}(a, b) = \text{dep}(a, c)$, ties must be broken deterministically using lexicographical comparison of the activity label strings.
   - Variant discovery rankings are sorted descending by frequency; traces with equal frequency are sub-sorted lexicographically by their activity sequence hashes to guarantee identical outputs across all architectures.

4. **Instruction Metering and Path Audits**:
   - Instruction counting is enforced through JIT block-level instrumentation.
   - Cycle lineage is checked against expected execution path block hashes to identify cycle-smuggling or laundered AGI compute execution patterns.

---

## 5. Mining Authority vs Conformance Authority: Boundary

**Mining Authority owns:**
- ✅ Discovery algorithm correctness (Inductive Miner soundness by construction)
- ✅ DFG construction and variant discovery
- ✅ Long-distance dependency detection (Heuristics Miner)
- ✅ Cycle budgeting and proof-of-work attestation
- ✅ Discovery receipt generation and cryptographic signing

**Conformance Authority owns:**
- ✅ Alignment between discovered model and actual log
- ✅ Fitness/precision/generalization metrics
- ✅ Admission gate enforcement (θ_fit ≥ 0.95)
- ✅ Rawlog → Evidence<T, State, Witness> type-law wrapping

**Delegation:**
Mining Authority **does not** verify that a discovered model is sound via conformance testing. That is the province of Conformance Authority. Mining Authority's claim is: "We discovered this model using algorithm X; here is the proof."

---

## 6. Duplicated Compat Law to Remove

From `wasm4pm-compat`, the following admission pathways **must not be re-implemented** in wasm4pm mining authority; they belong in Conformance Authority:

| Compat Pathway | Owner | Reason |
|---|---|---|
| Temporal monotonicity validation | Conformance | Validates input log; discovery assumes valid input |
| Type violation (schema mismatch) | Conformance | Type-law boundary enforcement |
| Duplicate event detection | Conformance | Raw-log admission gate |
| Evidence<T, State, Witness> wrapping | Conformance | Type constructor; not discovery operation |
| Loss policy thermodynamics | Conformance | Evaluates what is acceptable loss during format conversion |

---

## 7. Missing Execution Law to Add

### 7.1 Discovery Variant Enumeration API

**Gap:** No specification for enumerating process variants and ranking by frequency. PM4Py provides this; wasm4pm must specify.

**Requirement:**
```rust
pub fn enumerate_variants(log: &Ocel2Log) -> Vec<(Trace, u64)> {
    // Returns: [(trace1, freq1), (trace2, freq2), ...]
    // Sorted descending by frequency
    // Receipt: Top-20 variants with cumulative coverage %
}
```

### 7.2 Long-Distance Dependency Analysis

**Gap:** Heuristics Miner specification mentions long-distance loops but provides no concrete algorithm.

**Requirement:**
```rust
pub fn detect_long_distance_loops(dfg: &DirectlyFollowsGraph, threshold: u32) -> Vec<Loop> {
    // For each activity A, find all paths: A → ... → A with length > threshold
    // Return ordered by frequency
    // Receipt: Top-10 loops, cumulative case coverage
}
```

### 7.3 Noise-Filtered DFG Construction

**Gap:** No specification of how to construct DFG with activity/edge filtering.

**Requirement:**
```rust
pub fn construct_dfg_filtered(
    log: &Ocel2Log, 
    activity_frequency_cutoff: f64,  // % of total events
    edge_frequency_cutoff: u32,      // min occurrence count
) -> (DirectlyFollowsGraph, LossReport) {
    // Return DFG with filtered activities/edges
    // LossReport: events/traces excluded, cardinality
}
```

---

## 8. Related Documents

- [Conformance Authority Map](file:///Users/sac/process-intelligence/sources/wasm4pm/conformance-authority-map.md) — Fitness metrics, alignment, admission gates
- [Execution Authority Atlas](file:///Users/sac/process-intelligence/sources/wasm4pm/execution-authority-atlas.md) — Cross-authority coordination
- [downstream_wasm4pm_refactor.md](file:///Users/sac/process-intelligence/prompts/downstream_wasm4pm_refactor.md) — Implementation requirements
- [sources/papers/paper-canon.md](file:///Users/sac/process-intelligence/sources/papers/paper-canon.md) — Academic foundations (Leemans, Weijters, van der Aalst)

---

## 9. Authority Verdict

**Status:** EXECUTABLE  

Mining Authority specification is complete and actionable. Implementation priority:
1. **Phase 1 (MVP):** DFG mining + Inductive Miner (5 weeks, 1 engineer)
2. **Phase 2:** Heuristics Miner + long-distance analysis (3 weeks)
3. **Phase 3:** Cycle attestation + proof-of-work verification (2 weeks)
4. **Phase 4:** Variant enumeration API + noise filtering tuning (2 weeks)

**Deployment Gate:** No discovery model is admitted to operations until it passes Conformance Authority's fitness threshold (≥0.95).
