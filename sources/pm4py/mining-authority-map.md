# PM4Py Mining Authority Map

**Replay Agent Trace:** Comprehensive algorithm-level mapping of discovery miners.

**Scope:** Inductive Miner, Alpha Miner, Heuristics Miner, DFG, Process Tree

**Last Updated:** 2026-05-31

---

## 1. Algorithm Steps & Execution Traces

### 1.1 Inductive Miner (IM)

**Functions:**
- `discover_process_tree_inductive(log, parameters)`
- `discover_petri_net_inductive(log, parameters)`

**Variants:**
- `im` — Basic inductive miner
- `imd` — Directly-follows variant (eager cut)
- `imf` — Frequency-aware variant
- `imc` — Composite (multiple cuts concurrently)

**Algorithm Steps:**

1. **Base Case Detection**
   - Empty trace → empty tree
   - Single activity trace → leaf node (activity)
   - All same activity → loop node (A*)

2. **Activity Filtering**
   - Filter traces by `noise_threshold` (default: 0.0)
   - Remove traces with occurrence < threshold × total_cases
   - Traces below threshold are discarded from DFG computation

3. **Directly-Follows Graph (DFG) Construction**
   - Build activity pairs (A → B) with frequency counts
   - Start/End markers added to DFG
   - Frequency stored: `dfg[(A, B)] = count`

4. **Cut Detection (Primary)**
   - **Sequence Cut:** No activity follows all predecessors consistently
     - Look for partition: `activities_before` || `activities_after` with no cross-edges
     - If found: return `SEQUENCE(left_tree, right_tree)`
   
   - **Parallel Cut:** All reachable pairs have directly-follows edges
     - Partition activities into non-interactive groups
     - If found: return `PARALLEL(subtrees)`
   
   - **Exclusive Choice Cut:** Only one activity active per trace prefix
     - Partition by which activity starts each trace
     - If found: return `XOR(subtrees)`
   
   - **Loop Cut:** Activity A → ... → B → A (cycle detected)
     - Partition: redo-body and loop-back edges
     - If found: return `LOOP(subtree_redo, subtree_exit)`

5. **Fallthrough Strategy** (if no cut found)
   - `disable_fallthroughs=False` (default): Use frequency-based greedy recovery
     - Extract most-frequent activity pair
     - Recursively mine left/right partitions
     - Result: LOOP or XOR fallback
   - `disable_fallthroughs=True`: Fail gracefully (return SKIP node)

6. **Recursion**
   - Apply steps 2-5 to each partition recursively
   - Base cases: single activity, empty, all-same

7. **Process Tree to Petri Net Conversion** (if petri_net_inductive called)
   - Tree nodes → transitions and places
   - Sequence: sequential arcs
   - XOR: choice places
   - Parallel: synchronization
   - Loop: place with self-arc

**Soundness Guarantee:** Yes
- Inductive construction prevents deadlock/livelock
- Every tree structure maps to sound Petri net

**Dataframe Spine (Input):**
```
case:concept:name | concept:name | time:timestamp | [attributes...]
─────────────────────────────────────────────────────
CASE_001          | ActivityA   | 2026-05-31...   |
CASE_001          | ActivityB   | 2026-05-31...   |
CASE_002          | ActivityA   | 2026-05-31...   |
```

**Key Parameters:**
| Parameter | Type | Default | Impact |
|-----------|------|---------|--------|
| `noise_threshold` | float | 0.0 | Traces < threshold × total are removed |
| `multi_processing` | bool | False | Parallel recursion (not recommended for small logs) |
| `disable_fallthroughs` | bool | False | Strict (no greedy recovery) if True |

**Failure Signatures:**
- **Signature 1:** Noise threshold filters entire log → empty tree + warning
- **Signature 2:** No cut found + fallthrough disabled → SKIP node (incomplete model)
- **Signature 3:** Recursive depth explosion (unbalanced tree) → Memory exhaustion
- **Signature 4:** Degenerate log (all cases identical) → Single-activity tree
- **Signature 5:** Cyclic trace with missing start/end → Loop cut over-fitted to noise

---

### 1.2 Alpha Miner

**Functions:**
- `discover_petri_net_alpha(log, parameters)`
- `discover_petri_net_alpha_plus(log, parameters)`

**Algorithm Steps:**

1. **Data Preparation**
   - Extract start activities: first activity in each trace
   - Extract end activities: last activity in each trace
   - Extract directly-follows pairs: `(A, B)` where A immediately precedes B
   - Extract causality pairs: `A →* B` (A eventually leads to B)

2. **Footprints Derivation**
   - **Concurrency Matrix:** Activities A and B are concurrent if `A → B` AND `B → A`
   - **Ordering Matrix:** Activities A and B are ordered if `A → B` XOR `B → A` (not both)
   - **Conflict Matrix:** Activities A and B conflict if `NOT (A → B) AND NOT (B → A) AND NOT concurrent`

3. **Place Discovery**
   - Create place for each unique input/output set
   - Input set of activity A: {activities that directly precede A in any trace}
   - Output set of activity A: {activities that directly follow A in any trace}
   - Place p represents (input_set, output_set) if compatible

4. **Transition Creation**
   - One transition per unique activity in log
   - Initial place (→ start activities)
   - Final place (← end activities)

5. **Arc Assignment**
   - (Input place → transition) if input set matches place precondition
   - (Transition → output place) if output set matches place postcondition

6. **Implicit Place Removal**
   - Remove redundant places (arcs that are already covered by other transitions)
   - Simplify net topology

**Alpha-Plus Variant:**
- Handles short loops (L1L2 → repeated activity pairs)
- Adds implicit places for loop handling
- Slightly improved robustness over classic alpha

**Soundness Guarantee:** Conditional
- **Only for:** Structured logs (sequential, no arbitrary concurrency)
- **Requires:** Complete log (all start/end events present)
- **Fails for:** Logs with missing start/end, unstructured concurrency

**Dataframe Spine (Input):**
```
case:concept:name | concept:name | time:timestamp
─────────────────────────────────────────────────
CASE_001          | Start       | 2026-05-31...
CASE_001          | ActivityA   | 2026-05-31...
CASE_001          | ActivityB   | 2026-05-31...
CASE_001          | End         | 2026-05-31...
```

**Key Constraint:**
- Requires explicit START and END events in log
- Missing start/end → places cannot be constructed correctly

**Failure Signatures:**
- **Signature 1:** No explicit start activity → start place empty, model invalid
- **Signature 2:** Multiple concurrent unordered activities → over-constrained net (false precedence arcs)
- **Signature 3:** Implicit loops (activity repeats without visible loop edge) → Missing loop places
- **Signature 4:** Incomplete traces (missing end) → Dangling transitions, incorrect final place
- **Signature 5:** Short repetitions (L1L2) → Classic alpha misses, alpha-plus partially recovers

---

### 1.3 Heuristics Miner

**Functions:**
- `discover_heuristics_net(log, parameters)`
- (Output: HeuristicsNet object; convertible to Petri net via utility)

**Algorithm Steps:**

1. **Frequency Counting**
   - Count directly-follows edges: `freq[A → B]`
   - Count loop-back edges: `freq[A → A]` (self-loops)
   - Count start/end occurrences

2. **Dependency Calculation**
   - For each pair (A, B):
     ```
     dependency(A, B) = 
         (freq[A → B] - freq[B → A]) / 
         (freq[A → B] + freq[B → A] + 1)
     ```
   - Ranges [-1, 1]; 1 = strong A→B dependency, -1 = reverse, 0 = equal or absent
   - Compare against `dependency_threshold` (default: 0.5)

3. **Concurrency Detection**
   - Compute AND-measure (Causal/Frequency co-occurrence):
     ```
     and_measure(A, B) = 
         (freq[A → B] × freq[B → A]) /
         (freq[A → B] + freq[B → A] + 1)
     ```
   - If `and_measure > and_threshold` (default: 0.65): A and B are concurrent

4. **Loop Detection**
   - Self-loop: `freq[A → A] > 0` → Loop2 gate (A → A option)
   - Loop-back cycles: Dependency chain A → B → C → A with high frequency
   - `loop_two_threshold` (default: 0.5): Minimum dependency strength for loop inclusion

5. **HeuristicsNet Construction**
   - Nodes: unique activities
   - Edges: activities with dependency ≥ `dependency_threshold`
   - Edge weight: dependency value (for visualization)
   - Concurrency pairs: marked (separate from sequence edges)
   - Loop gates: marked for self-loops and cycles

6. **Quality Annotation**
   - Calculate positive/negative occurrence counters per activity
   - Positive: traces that progress; negative: traces that violate expectations
   - Result: HeuristicsNet with per-activity quality metrics

**Conversion to Petri Net:**
- HeuristicsNet → PetriNet via `convert_heuristics_net_to_petri_net(hnet)`
- Process: Add places between dependent activities, synchronization for concurrency
- Soundness: Not guaranteed (heuristic miner prioritizes noise tolerance over structural guarantees)

**Dataframe Spine (Input):**
```
case:concept:name | concept:name | time:timestamp
─────────────────────────────────────────────────
CASE_001          | ActivityA   | 2026-05-31...
CASE_001          | ActivityB   | 2026-05-31...
CASE_002          | ActivityA   | 2026-05-31...
```

**Key Parameters:**
| Parameter | Type | Default | Impact |
|-----------|------|---------|--------|
| `dependency_threshold` | float | 0.5 | Min strength to include edge |
| `and_threshold` | float | 0.65 | Concurrency detection threshold |
| `loop_two_threshold` | float | 0.5 | Loop edge inclusion threshold |
| `start_activities` | Set | Auto-detected | Override auto-detected starts |
| `end_activities` | Set | Auto-detected | Override auto-detected ends |

**Failure Signatures:**
- **Signature 1:** Very noisy log → dependency scores flatten, sparse net
- **Signature 2:** Threshold set too high → Over-selective edges, disconnected fragments
- **Signature 3:** Threshold set too low → Over-permissive edges, spurious concurrency
- **Signature 4:** Missing start/end activity markers → Start/end nodes orphaned
- **Signature 5:** Implicit multi-loop cycles → Only strong loop-back edges detected, others ignored

---

### 1.4 Directly-Follows Graph (DFG)

**Functions:**
- `discover_dfg(log, parameters)` → Tuple[Dict, Dict, Dict]
- `discover_dfg_typed(log, parameters)` → DirectlyFollowsGraph
- `discover_performance_dfg(log, parameters)` → Dict with timing

**Algorithm Steps:**

1. **Activity Pair Enumeration**
   - For each trace: iterate consecutive activity pairs (A, B)
   - Record: `dfg[(A, B)] += 1`

2. **Start Activity Tracking**
   - First activity of each trace: `start_activities[activity] += 1`

3. **End Activity Tracking**
   - Last activity of each trace: `end_activities[activity] += 1`

4. **Output Format**
   - **Tuple form:** 
     ```
     (
         dfg_dict,         # {(A, B): frequency, ...}
         start_dict,       # {activity: count, ...}
         end_dict          # {activity: count, ...}
     )
     ```
   - **Typed form:**
     ```
     DirectlyFollowsGraph(
         graph=dfg_dict,
         start_activities=start_dict,
         end_activities=end_dict
     )
     ```

5. **Performance DFG (Optional)**
   - Add timing metrics: (min_wait, max_wait, median_wait) per edge
   - Uses timestamp differences: `B.timestamp - A.timestamp`

**Dataframe Spine (Input):**
```
case:concept:name | concept:name | time:timestamp
─────────────────────────────────────────────────
CASE_001          | ActivityA   | 2026-05-31T10:00:00Z
CASE_001          | ActivityB   | 2026-05-31T10:05:00Z
CASE_002          | ActivityA   | 2026-05-31T10:10:00Z
```

**Key Parameters:**
| Parameter | Type | Default | Impact |
|-----------|------|---------|--------|
| `case_id_key` | str | 'case:concept:name' | Column name for case grouping |
| `activity_key` | str | 'concept:name' | Column name for activity |
| `timestamp_key` | str | 'time:timestamp' | Column name for ordering |

**Failure Signatures:**
- **Signature 1:** Single-activity traces → DFG contains no edges (only start/end)
- **Signature 2:** Timestamp missing/malformed → Order undefined, DFG invalid
- **Signature 3:** Duplicate timestamps in trace → Arbitrary edge direction
- **Signature 4:** Missing case ID → All events merged, global DFG created (incorrect grouping)
- **Signature 5:** Performance DFG with negative wait times → Timestamp ordering violated

---

### 1.5 Process Tree

**Generated By:**
- Inductive Miner (primary source)
- Genetic Algorithm Miner (secondary)
- Other heuristic discovery methods

**Node Types:**

| Node Type | Syntax | Semantics | Petri Net Equivalent |
|-----------|--------|-----------|----------------------|
| **Activity** | `leaf` | Execute activity | Single transition |
| **Sequence** | `→(A, B)` | A then B (ordered) | Sequential arcs (A place B place) |
| **XOR (Choice)** | `⊕(A, B, C)` | One of A, B, C (exclusive) | Choice place with multiple transitions |
| **Parallel** | `∥(A, B, C)` | All of A, B, C (concurrent) | Synchronization bar, parallel firing |
| **Loop** | `⟲(A, B)` | A repeat until B (do-until) | Place with self-arc to A's start |
| **SKIP** | `ε` | Silent (epsilon) transition | Transparent pass-through |

**Tree Serialization:**

```python
ProcessTree(
    operator='→',  # Sequence
    children=[
        ProcessTree(operator=None, label='ActivityA'),  # Leaf
        ProcessTree(operator='⊕', children=[
            ProcessTree(operator=None, label='ActivityB'),
            ProcessTree(operator=None, label='ActivityC')
        ])
    ]
)
```

**Fitness Computation (via conformance check):**
- Replay each trace through tree
- Count correct/incorrect activity executions
- Fitness = correct_activities / total_activities

**Failure Signatures:**
- **Signature 1:** SKIP nodes in critical path → Model too permissive (low precision)
- **Signature 2:** Deep nesting (>10 levels) → Tree explosion, hard to interpret
- **Signature 3:** Loop node with no exit path → Potential infinite loop (unsound)
- **Signature 4:** Unbalanced children (one child much larger than others) → Degenerate structure

---

## 2. Assumption Violations

### 2.1 Inductive Miner Assumptions

| Assumption | Violation | Consequence |
|-----------|-----------|-------------|
| **Complete Start/End Markers** | Some cases missing START or END | Cut detection fails; incorrect tree structure |
| **Acyclic DFG in Partitions** | Strongly connected subgraph in partition | Recursion infinite loop (implementation guards against via depth limit) |
| **Noise Threshold < Log Size** | Threshold removes all traces | Empty tree returned |
| **Deterministic Ordering** | Timestamps non-unique within trace | Edge direction arbitrary (depends on sort stability) |
| **No Implicit Activities** | Activity name collision (e.g., "A" and " A") | Treats as different; incorrect merges |

### 2.2 Alpha Miner Assumptions

| Assumption | Violation | Consequence |
|-----------|-----------|-------------|
| **Explicit Start Activity** | Log has no clear START event | Start place empty; model cannot initialize |
| **Explicit End Activity** | Log has no clear END event | Final place orphaned; dangling transitions |
| **Complete Traces** | Traces interrupted (mid-process) | Footprints incomplete; missing ordering info |
| **No Implicit Concurrency** | Unordered activities (both A→B and B→A) | Places over-constrained; model becomes too strict |
| **Short Loops (L1L2 only)** | 3+ activity cycles without explicit loop edges | Classic alpha fails; alpha-plus partially recovers |
| **Infrequent Noise** | Outlier traces with rare activity order | Footprints corrupted; incorrect causality |

### 2.3 Heuristics Miner Assumptions

| Assumption | Violation | Consequence |
|-----------|-----------|-------------|
| **Threshold Calibration** | Dependency threshold misset | Sparse (high threshold) or dense (low threshold) net |
| **Representative Frequencies** | Few traces; outliers dominate | Dependency scores unstable; noisy edges included |
| **No External Noise** | Random activities injected (stress test) | Concurrency measure inflated; false parallelism |
| **Deterministic Activity Sets** | Activities appear/disappear by timestamp | Start/end auto-detection incorrect |
| **Binary Activity Relationship** | Activity has weak dependency to both neighbors | Falls between thresholds; edge missing |

### 2.4 DFG Assumptions

| Assumption | Violation | Consequence |
|-----------|-----------|-------------|
| **Sequential Case Definition** | Case ID spans multiple processes | DFG merges unrelated traces; incorrect graph |
| **Ordered Timestamps** | Clock skew (timestamps non-monotonic) | Edge direction reversed or ambiguous |
| **Acyclic Traces (for simple graphs)** | Trace repeats activity A → B → A | DFG shows cycle (correct), but single-path traces only show partial structure |
| **No Duplicate Timestamps** | Activities occur simultaneously | Order undefined; DFG arbitrarily selects edge |

---

## 3. OCEL-Specific Limitations

### 3.1 Object-Centric DFG (OCDFG)

**Limitation 1: Object Flattening**
- OCDFG creates per-object-type DFGs by flattening multi-object events
- Event with participants [Obj1, Obj2] → Edges added to both object DFGs independently
- **Problem:** Inter-object causality lost; synchronization dependencies invisible

**Limitation 2: Event-to-Object Mapping Asymmetry**
- Events mapped to objects; objects not mapped back to causality
- **Problem:** Cannot reconstruct which object caused activity transition

**Limitation 3: Business Hours Filtering**
- `business_hours=True` removes off-hours events
- **Problem:** If process naturally has off-hours steps, model becomes incomplete (silent gaps)

**Limitation 4: Temporal Ordering of Multi-Object Events**
- Events with same timestamp but multiple objects → Order within event undefined
- **Problem:** OCDFG treats as simultaneous, but actual execution order unknown

### 3.2 Object-Centric Petri Net (OCPN)

**Limitation 1: Per-Object-Type Nets**
- OCPN generates independent Petri net per object type
- **Problem:** Synchronization between object types requires external composition (not automatic)

**Limitation 2: Missing Cross-Object Tokens**
- Token-based replay on OCPN does not enforce cross-object state consistency
- **Problem:** Object A can reach final marking while Object B is orphaned (no soundness for multi-object)

**Limitation 3: Inductive Miner Variant Selection**
- Parameter `inductive_miner_variant` (im/imd/imf) applies uniformly to all object types
- **Problem:** Some object types may fit better with different variant; no per-type tuning

### 3.3 Event-to-Object (ETOT) Graph

**Limitation 1: Relation Cardinality**
- ETOT records event-object pairs; does not capture n-way object groupings
- **Problem:** 3+ objects involved in single event → Binary pairs lose semantic grouping

**Limitation 2: No Lifecycle Semantics**
- ETOT does not distinguish "created", "modified", "deleted" semantics
- **Problem:** All object-event relationships treated uniformly (no state transitions)

### 3.4 Object-to-Object (OTG) Graph

**Limitation 1: Event Labeling**
- OTG labels edges with event names; multiple events on same object pair → Single aggregated edge
- **Problem:** Cannot distinguish "invoice to payment" vs. "payment to refund" if both involve same object pair

**Limitation 2: No Temporal Ordering**
- OTG counts object interactions; timestamps ignored
- **Problem:** Cyclic object interactions (A→B→A) cannot be distinguished from sequential

---

## 4. Dataframe Spine Specifics

### 4.1 Standard EventLog Spine (XES/CSV)

**Required Columns:**
```
case:concept:name | concept:name | time:timestamp
─────────────────────────────────────────────────
```

**Optional Columns (Custom Attributes):**
```
org:resource | org:role | org:group | cost:value | [domain-specific...]
```

**Parsing Rules:**

1. **Case ID Parsing:** 
   - Column name: `case:concept:name` (or configurable)
   - Type: String or integer
   - **Failure:** Non-unique case IDs per row → Duplicate handling (duplicates merged or error)

2. **Activity Parsing:**
   - Column name: `concept:name`
   - Type: String
   - **Failure:** Whitespace inconsistency (" ActivityA" vs "ActivityA") → Treated as different activities

3. **Timestamp Parsing:**
   - Column name: `time:timestamp`
   - Format: ISO 8601 or UNIX epoch
   - **Failure:** Ambiguous format (MM/DD vs DD/MM) → Incorrect ordering

4. **Trace Assembly:**
   - Group rows by case:concept:name
   - Sort by time:timestamp within each group
   - Create ordered list: [Activity1, Activity2, ...]

**Dataframe Engine (pm4py.log_converter):**
- Accepts: pd.DataFrame, pm4py.EventLog (dict-based), file paths
- Converts to: Canonical format (list of dicts per trace)
- Cache: No persistent cache; re-parses on each discovery call

### 4.2 OCEL Spine (JSON/XML/SQLite)

**Required Columns (OCEL 2.0):**
```
ocel:event_id | ocel:event_type | ocel:timestamp | ocel:omap
─────────────────────────────────────────────────────────────
EVENT_001     | activity_name   | 2026-05-31...  | [object_id, ...]
```

**Object Map (ocel:omap):**
- Array of object IDs involved in event
- **Parsing:** Flattened to binary (event, object) pairs for OCDFG/OTG computation

**OCEL Specifics:**
- **ocel:event_id:** Unique event identifier
- **ocel:event_type:** Activity name (equivalent to concept:name)
- **ocel:timestamp:** Event time
- **ocel:omap:** List of object references
- **ocel:ovmap:** Object version map (state changes per object per event) — optional

**Conversion to Flat EventLog:**
- OCEL → Flat: Duplicate event for each object
  ```
  CASE_001 (ObjA) | ActivityX | T1
  CASE_001 (ObjB) | ActivityX | T1
  ```
- **Loss:** Original n-way object grouping lost; converted to n binary pairs

---

## 5. Failure Signatures Summary Table

| Algorithm | Signature | Root Cause | Observable Symptom |
|-----------|-----------|-----------|----------------------|
| **Inductive** | Tree SKIP nodes in critical path | Undetected cut + fallthrough enabled | Low precision conformance |
| **Inductive** | Empty tree returned | Noise threshold removes all traces | Zero-activity model |
| **Alpha** | Empty start place | No explicit START event in log | Model cannot be initialized |
| **Alpha** | Dangling transitions | No explicit END event in log | Orphaned final transitions |
| **Alpha** | Over-constrained net | Concurrent unordered activities | False dependency edges |
| **Alpha** | Missing loop places | 3+ cycle patterns in log | Loop transitions unreachable |
| **Heuristics** | Sparse net (disconnected fragments) | `dependency_threshold` too high | Activities not linked |
| **Heuristics** | Dense net (spurious edges) | `dependency_threshold` too low | Over-permissive model |
| **DFG** | No edges, only start/end | Single-activity traces | Linear graph (trivial) |
| **DFG** | Reversed edge direction | Timestamp clock skew | Causality backward |
| **OCDFG** | Lost inter-object causality | Object flattening | Cannot sync object types |
| **OCPN** | Orphaned objects | No cross-object token sync | Some objects stuck in final state |

---

## 6. Execution Flow Diagram

```
EventLog / DataFrame Input
    ↓
[Trace Assembly & Validation]
    ├→ Parse case:concept:name → Trace grouping
    ├→ Parse concept:name → Activity list
    ├→ Parse time:timestamp → Temporal ordering
    └→ Optional: Noise filter (inductive_miner only)
    ↓
[Algorithm Dispatch]
    ├→ Inductive Miner
    │   ├→ DFG construction (traces → activity pairs)
    │   ├→ Recursive cut detection (sequence/xor/parallel/loop)
    │   └→ Process Tree output (+ Petri net conversion optional)
    │
    ├→ Alpha Miner
    │   ├→ Footprint derivation (causality matrix)
    │   ├→ Place discovery (input/output sets)
    │   ├→ Transition + arc assignment
    │   └→ Petri net output
    │
    ├→ Heuristics Miner
    │   ├→ Frequency counting (DFG edges + self-loops)
    │   ├→ Dependency calculation (threshold filtering)
    │   ├→ Concurrency detection (AND-measure)
    │   └→ HeuristicsNet output (+ optional Petri net conversion)
    │
    └→ DFG
        ├→ Activity pair enumeration
        ├→ Start/end activity tracking
        └→ DFG dict/typed object output
    ↓
[Conformance Check (Optional)] → Fitness/Precision metrics
    ├→ Token-Based Replay (Petri nets only)
    ├→ Alignments (Petri nets only)
    ├→ Footprints (all models)
    └→ Log Skeleton (lightweight)
    ↓
Output: Process Model (Tree/Net/HeuristicsNet/DFG)
```

---

## 7. Critical Assumptions & Proof Gates

### 7.1 Proof Gate: "Log is Complete"

**Definition:** Every case has explicit START and END events; all intermediate activities present.

**Checked By:** 
- Alpha Miner (fails silently if false)
- Inductive Miner (tolerates partial; adds SKIP if needed)

**Failure Consequence:**
- Alpha: Invalid causality matrix; places not discovered
- Inductive: SKIP nodes inserted; model loses precision

**How to Verify:**
```python
# Check for explicit start/end
start_activities = [t[0] for trace in log for t in [trace]]
end_activities = [t[-1] for trace in log for t in [trace]]
if len(set(start_activities)) > 1 or len(set(end_activities)) > 1:
    # Multiple entry/exit points; may indicate incomplete traces
    pass
```

### 7.2 Proof Gate: "Activities are Deterministic"

**Definition:** Same activity name always has same semantics; no silent/implicit activities.

**Checked By:** 
- Manual inspection (PM4Py assumes this)
- Footprint-based conformance (detects violations post-hoc)

**Failure Consequence:**
- Over/under-fitted model (activity merged or split incorrectly)

### 7.3 Proof Gate: "Concurrency is Explicit"

**Definition:** If A and B can occur concurrently, both A→B and B→A appear in log frequently.

**Checked By:** Heuristics Miner (via AND-measure)

**Failure Consequence:**
- Alpha Miner: Treats as ordered (missing parallelism)
- Inductive Miner: Detects parallel cut if frequencies balanced; else XOR

---

## 8. Performance Characteristics

| Algorithm | Time Complexity | Space Complexity | Scalability | Notes |
|-----------|-----------------|------------------|-------------|-------|
| **Inductive** | O(n × log_depth) | O(n + tree_size) | Medium (recursive) | Depth can explode; recommend `disable_fallthroughs=True` |
| **Alpha** | O(n²) for footprint | O(activities²) | Good (quadratic) | Classic, wasm-portable |
| **Heuristics** | O(n + edges) | O(activities²) | Excellent | Linear sweep; threshold-sensitive |
| **DFG** | O(n) | O(activities²) | Excellent | Minimal; direct enumeration |
| **OCDFG** | O(n × objects) | O(objects² + events) | Medium (object flattening) | Per-object-type DFGs |

---

## 9. Authority Mapping Summary

**PM4Py Mining Authority** (as of v2.7.22.1):

1. **Inductive Miner:** Sound, recursive; best for structured logs; tolerates noise (with thresholds)
2. **Alpha Miner:** Fast, footprint-based; requires complete, structured logs; wasm-portable
3. **Heuristics Miner:** Noise-tolerant, threshold-driven; no soundness guarantee; best for discovery
4. **DFG:** Minimal model; no control flow; fastest, lightest; used as foundation for other miners
5. **Process Tree:** Primary output of inductive miner; serializable; convertible to Petri nets

**Key Variance Points:**
- **Noise Handling:** Inductive (parameter-driven) vs. Heuristics (frequency-driven)
- **Concurrency:** Alpha (assumes explicit) vs. Heuristics (detects via AND-measure) vs. Inductive (cut-based)
- **Loop Handling:** Alpha (footprints + L1L2 fix) vs. Inductive (recursive LOOP cut) vs. Heuristics (loop-two-threshold)
- **OCEL Support:** OCDFG/OCPN available; flattens object relationships; per-object-type mining only

**Doctrine Alignment:**
> The product is CodeManufactory; RevOps is merely proof that CodeManufactory works.

PM4Py mining algorithms are the **oracle baseline**; deviation is measured against discovery fidelity (fitness vs. model complexity trade-off). Object-centric mining in PM4Py demonstrates OCEL flattening limitations; CodeManufactory must enforce cross-object causality and lifecycle consistency where PM4Py silently drops it.

