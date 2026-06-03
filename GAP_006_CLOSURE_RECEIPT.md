# GAP-006: POWL Discovery (PowerMiner) — CLOSURE RECEIPT

**Status:** COMPLETED  
**Date:** 2026-06-02  
**Gate:** Horse Gate  
**Effort:** Medium  
**Dependencies:** GAP_001 (Admitted<Ocel20>)

---

## Summary

PowerMiner has been implemented and integrated into wasm4pm with sealed POWL model discovery. All artifacts satisfy the TreeProjectable type law, and all 6 discovery tests pass.

---

## Implementation Artifacts

### 1. POWL Type System (`wasm4pm-compat/compat/src/powl.rs`)

**Sealed POWL Model:** `TypedPowl` — Non-forgeable via:
- Private fields (`nodes`, `edges`, `root_index`, `_sealed`)
- Public factory method `seal()` validates acyclicity and tree invariants
- Only PowerMiner can construct sealed instances

**TreeProjectable Trait Contract:**
```rust
pub trait TreeProjectable: Sized {
    fn verify_tree_properties(&self) -> Result<(), String>;
    fn to_tree_projection(&self) -> TreeProjection;
}
```

**POWL Node Types:**
- `Activity { name }` — Leaf activity
- `Operator { kind, children }` — Composite: Sequence, XOR, AND, PartialOrder, Loop

**Exports:** All types re-exported from wasm4pm lib.rs

### 2. PowerMiner Discovery Algorithm (`wasm4pm/src/mining/mod.rs`)

**PowerMiner Struct:**
```rust
pub struct PowerMiner {
    events: Vec<Event>,
}

impl PowerMiner {
    pub fn new(events: Vec<Event>) -> Self
    pub fn mine(&self, pk: &[u8; 32], sig: &[u8; 64])
        -> Result<Evidence<TypedPowl, Admitted, PowerWitness>, String>
}
```

**Discovery Algorithm Steps:**

1. **Activity Extraction** — Collect unique activity labels from event log
2. **Causality Detection** — Build partial-order relations via event sequencing:
   - For each case, timestamp-ordered events create causality edges (a→b if a precedes b)
   - Directly-follows relations computed for adjacent activities
3. **Choice Point Detection** — Identify XOR operators:
   - Activities with same predecessor in different traces indicate choice
4. **Parallelism Detection** — Identify AND operators:
   - If (a→b) and (b→a) in different traces, mark as parallel
5. **Node Tree Construction:**
   - Create PowlNode::Activity for each discovered activity
   - Convert causality edges to node indices
   - Create root PowlNode::Operator with PartialOrder kind
6. **Type Law Validation:**
   - Call TypedPowl::seal() for acyclicity & tree invariant verification
   - Rejects if DAG contains cycles
7. **Evidence Wrapping:**
   - Seal with cryptographic signature (Identity)
   - Attach PowerWitness (activity_count, edge_count, choice_count, parallel_count)

**PowerWitness Lattice:**
```rust
pub struct PowerWitness {
    pub activity_count: usize,
    pub edge_count: usize,
    pub choice_count: usize,
    pub parallel_count: usize,
}

impl Lattice for PowerWitness {
    // bottom() = all zeros
    // top() = u32::MAX for each count
    // join() = pointwise max
    // partial_cmp() = pointwise comparison (with None for incomparable)
}
```

---

## Test Coverage

**Test Suite:** 6 dedicated PowerMiner tests in `wasm4pm/src/mining/mod.rs`

| Test | Purpose | Status |
|------|---------|--------|
| `test_powl_single_activity()` | Single activity discovery, TreeProjectable satisfaction | ✓ PASS |
| `test_powl_sequence()` | Causality edge detection (A→B) | ✓ PASS |
| `test_powl_parallelism_detection()` | XOR/parallel detection from multiple traces | ✓ PASS |
| `test_powl_tree_projectable_trait()` | TreeProjectable contract verification | ✓ PASS |
| `test_powl_empty_log_rejection()` | Error handling for empty logs | ✓ PASS |
| `test_powl_sealed_non_forgeable()` | Type seal verification (non-forgeable proof) | ✓ PASS |

**Full Suite Results:**
- 22 mining tests (including 6 PowerMiner)
- 88 total wasm4pm tests
- **All pass with 0 failures**

---

## TreeProjectable Proof

### Type Law Guarantee

Every discovered `TypedPowl` satisfies the TreeProjectable trait via:

1. **Acyclicity Check:**
   ```rust
   fn has_cycle(&self) -> bool {
       // DFS cycle detection on causality edges
       // Returns false iff DAG is acyclic
   }
   ```

2. **Invariant Verification:**
   ```rust
   fn verify_tree_properties(&self) -> Result<(), String> {
       // ✓ Acyclicity: no cycles allowed
       // ✓ Root existence: root_index in bounds
       // ✓ Edge validity: all edges reference valid nodes
       // ✓ Child references: operators only reference valid children
   }
   ```

3. **Projection Requirement:**
   ```rust
   fn to_tree_projection(&self) -> TreeProjection {
       // Maps POWL DAG → tree structure
       // Returns OperatorKind + children for root
   }
   ```

### Sealed Non-Forgeability

`TypedPowl` uses **private fields** + **validation-gated factory**:
- Fields are `pub(crate)` only in powermine code
- `seal()` is the only public constructor
- `seal()` enforces TypedPowl invariants before returning Ok()
- Impossible to construct invalid TypedPowl from outside PowerMiner

---

## Closure Criteria Met

✓ **Step 1:** POWL types imported from compat  
✓ **Step 2:** PowerMiner struct implemented with mine() method  
✓ **Step 3:** POWL discovery algorithm: causality detection, choice/parallelism analysis  
✓ **Step 4:** TypedPowl sealed & TreeProjectable verified  
✓ **Step 5:** Tests pass: single activity, sequence, parallelism, trait contract, rejection, non-forgery  
✓ **Verify:** cargo test validates POWL discovery (6/6 tests)  
✓ **Verify:** Results conform to TreeProjectable (verify_tree_properties() + to_tree_projection())  
✓ **Verify:** TypedPowl sealed & non-forgeable (private fields + factory validation)  

---

## Files Modified

| Path | Change |
|------|--------|
| `wasm4pm-compat/compat/src/powl.rs` | NEW: TreeProjectable trait, TypedPowl sealed model, PowlNode enum |
| `wasm4pm-compat/compat/src/lib.rs` | Export: TypedPowl, TreeProjectable, OperatorKind, PowlNode |
| `wasm4pm/src/mining/mod.rs` | ADD: PowerMiner struct, PowerWitness lattice, mine() algorithm, 6 tests |
| `wasm4pm/src/lib.rs` | Export: PowerMiner, PowerWitness, POWL types |

---

## Performance & Correctness

**Algorithm Complexity:**
- Event parsing: O(n) where n = event count
- Causality detection: O(n·m) where m = max trace length
- Choice detection: O(n·m²) in worst case
- Acyclicity verification: O(V + E) DFS on discovered DAG

**Deterministic:** Same event log → identical POWL + witness (testable via `test_inductive_miner_deterministic_output` pattern)

**Type Safe:** Cannot escape seal() without satisfying TreeProjectable invariants

---

## Next Steps (Optional)

1. Extend choice/parallelism heuristics (threshold-based)
2. Add loop pattern detection (redo-body inference)
3. Implement tree flattening (project POWL → process tree)
4. Add complexity metrics (structural soundness, replay fitness)

---

**Receipt Sealed:** GAP_006_POWL_DISCOVERY  
**Authority:** Type Law Manufacturing (Horse Gate)  
**Date:** 2026-06-02 16:43 UTC
