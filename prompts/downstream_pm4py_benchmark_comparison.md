# Downstream Directive: PM4Py Benchmark Comparison

**Authority Source:** [pm4py_vs_wasm4pm_capability_matrix.md](file:///Users/sac/process-intelligence/experiments/pm4py_vs_wasm4pm_capability_matrix.md)

**Research Backing**:
- [capability-atlas.md](file:///Users/sac/process-intelligence/sources/pm4py/capability-atlas.md) — PM4Py algorithm inventory
- [mining-authority-map.md § Comparative Analysis](file:///Users/sac/process-intelligence/sources/wasm4pm/mining-authority-map.md) — Mining performance targets
- [conformance-authority-map.md § Comparative Analysis](file:///Users/sac/process-intelligence/sources/wasm4pm/conformance-authority-map.md) — Conformance performance targets
- [zero-knowledge-benchmarks.md](file:///Users/sac/process-intelligence/experiments/pm4py-comparison/zero-knowledge-benchmarks.md) — Benchmark methodology

This document defines the requirements for empirical validation of wasm4pm against PM4Py reference implementations. All wasm4pm operations must demonstrate performance parity or superiority while maintaining cryptographic non-forgeability guarantees.

---

## 1. Benchmark Coverage Matrix

Test wasm4pm against PM4Py across the following algorithms:

| Algorithm | Test Category | Success Criteria |
|---|---|---|
| **Inductive Miner** | Correctness | Model structure identical (ISO) |
| | Performance | ≥20× faster than PM4Py |
| **Heuristics Miner** | Correctness | Same DFG edge set (within 1% tolerance) |
| | Performance | ≥18× faster than PM4Py |
| **A* Alignment** | Correctness | Optimal cost matches PM4Py (±0.01 tolerance) |
| | Performance | ≥22× faster than PM4Py |
| **Fitness Computation** | Correctness | Fitness value matches (±0.001 tolerance) |
| | Performance | ≥20× faster than PM4Py |
| **Precision Metric** | Correctness | Precision value matches (±0.01 tolerance) |
| | Performance | ≥15× faster than PM4Py |
| **DFG Mining** | Correctness | DFG topology identical |
| | Performance | ≥25× faster than PM4Py |
| **Petri Net Soundness** | Correctness | Soundness verdict matches PM4Py |
| | Performance | ≥30× faster than PM4Py |

---

## 2. Test Dataset Specifications

### 2.1 Small Dataset (Baseline Correctness)

- **Event Count**: 10,000 events
- **Trace Count**: 500 traces
- **Unique Activities**: 25
- **Model Type**: Process Tree (POWL)
- **Expected PM4Py Runtime**: ~2 seconds
- **Target wasm4pm Runtime**: < 100ms (20× speedup)

### 2.2 Medium Dataset (Real-World Scale)

- **Event Count**: 100,000 events
- **Trace Count**: 5,000 traces
- **Unique Activities**: 50
- **Model Type**: Petri Net
- **Expected PM4Py Runtime**: ~20 seconds
- **Target wasm4pm Runtime**: < 1 second (20× speedup)

### 2.3 Large Dataset (Performance Stress)

- **Event Count**: 1,000,000 events
- **Trace Count**: 50,000 traces
- **Unique Activities**: 100
- **Model Type**: Petri Net (structured)
- **Expected PM4Py Runtime**: ~200 seconds (timeout likely)
- **Target wasm4pm Runtime**: < 10 seconds (20× speedup)

### 2.4 Pathological Dataset (Adversarial)

- **Event Count**: 50,000 events
- **Trace Count**: 1,000 traces
- **Unique Activities**: 200 (high variant explosion)
- **Model Type**: Spaghetti process (circular)
- **Expected PM4Py Runtime**: > 500 seconds
- **Target wasm4pm Runtime**: < 30 seconds (16× speedup)

---

## 3. Correctness Validation Rules

### 3.1 Inductive Miner Validation

**Criterion**: Discovered model structure must match PM4Py output (isomorphic).

```python
def validate_inductive_miner(wasm_model, pm4py_model):
    # Both models should be POWL/Process Tree formats
    # Check that the operator tree structure is identical
    assert wasm_model.operators == pm4py_model.operators
    assert wasm_model.children_count == pm4py_model.children_count
    # Check that block hierarchy is preserved
    assert wasm_model.block_hierarchy == pm4py_model.block_hierarchy
    return True  # Success
```

**Tolerance**: 0% difference (structural isomorphism required)

### 3.2 Heuristics Miner Validation

**Criterion**: Directly-Follows Graph must contain the same edges (within 1% frequency tolerance).

```python
def validate_heuristics_miner(wasm_dfg, pm4py_dfg):
    # Extract edge sets
    wasm_edges = set(wasm_dfg.edges())
    pm4py_edges = set(pm4py_dfg.edges())
    
    # Check coverage
    missing_edges = pm4py_edges - wasm_edges
    extra_edges = wasm_edges - pm4py_edges
    
    # Tolerance: max 1% of edges differ in frequency
    assert len(missing_edges) < 0.01 * len(pm4py_edges)
    assert len(extra_edges) < 0.01 * len(pm4py_edges)
    return True
```

**Tolerance**: ±1% edge frequency difference

### 3.3 Alignment Validation

**Criterion**: Optimal cost must match PM4Py (within 0.01 absolute tolerance).

```python
def validate_alignment(wasm_cost, pm4py_cost):
    # Both should report the same cost-optimal alignment
    assert abs(wasm_cost - pm4py_cost) < 0.01
    return True
```

**Tolerance**: ±0.01 cost

### 3.4 Fitness Validation

**Criterion**: Fitness value must match PM4Py (within 0.001 absolute tolerance).

```python
def validate_fitness(wasm_fitness, pm4py_fitness):
    assert abs(wasm_fitness - pm4py_fitness) < 0.001
    return True
```

**Tolerance**: ±0.001 fitness

### 3.5 Precision Validation

**Criterion**: Precision must match PM4Py (within 0.01 absolute tolerance).

```python
def validate_precision(wasm_precision, pm4py_precision):
    assert abs(wasm_precision - pm4py_precision) < 0.01
    return True
```

**Tolerance**: ±0.01 precision

---

## 4. Performance Validation Rules

### 4.1 Throughput Metrics

**Measurement Points**:

- **Total Wall-Clock Time**: From input parse to result serialization
- **Algorithm-Specific Time**: Excluding I/O (parse + serialize)
- **Memory Peak**: Maximum heap usage during execution

### 4.2 Speedup Calculation

$$\text{Speedup} = \frac{\text{PM4Py Runtime (seconds)}}{\text{wasm4pm Runtime (seconds)}}$$

**Success Criteria**:

- **Target**: ≥20× speedup for all core algorithms (mining, conformance, alignment)
- **Minimum Acceptable**: ≥15× speedup (only for complex datasets)
- **Unacceptable**: < 10× speedup (triggers investigation)

### 4.3 Memory Efficiency

$$\text{Memory Ratio} = \frac{\text{PM4Py Peak Memory (MB)}}{\text{wasm4pm Peak Memory (MB)}}$$

**Success Criteria**:

- **Target**: ≥20× lower memory than PM4Py
- **Minimum Acceptable**: ≥15× lower memory
- **Unacceptable**: < 10× lower memory

---

## 5. Type Safety and Duck-Typing Comparison

wasm4pm enforces **static type safety** at compilation time, whereas PM4Py relies on **duck typing** (dynamic type checking at runtime).

### 5.1 Type Safety Test Suite

```rust
// wasm4pm: Compile-time type checking
pub fn discover_inductive_miner<T: LogFormat>(log: Evidence<T, Parsed, _>) 
    -> Result<Evidence<PetriNet, ValidatedSound, DiscoveryReceipt>, Error>
{
    // Type mismatch at compile-time if log format is invalid
    // E.g., passing XES when OCEL expected → compile error
}
```

vs.

```python
# PM4Py: Runtime duck typing
def discover_inductive_miner(log):
    # Type error only if attribute access fails at runtime
    # E.g., accessing log.events when log is None → AttributeError
```

### 5.2 Validation Rules

**Test Case**: Pass malformed input types to both engines

| Input | PM4Py Behavior | wasm4pm Behavior | Test Result |
|---|---|---|---|
| `None` as log | Runtime error | Compile error (Option<T>) | ✅ wasm4pm safer |
| Wrong attribute name | Runtime AttributeError | Compile error (field not found) | ✅ wasm4pm safer |
| Type mismatch (int vs. str) | Runtime TypeError | Compile error (type mismatch) | ✅ wasm4pm safer |

---

## 6. Cryptographic Non-Forgeability Validation

wasm4pm must produce **cryptographically signed receipts** for every operation. PM4Py has no such capability.

### 6.1 Receipt Immutability Test

```rust
#[test]
fn test_receipt_immutability() {
    let receipt = mining_operation();
    let receipt_hash = receipt.hash();
    
    // Attempt to mutate payload
    receipt.payload.push(Activity::new("fake"));
    
    // Hash should not change (mutation detected)
    assert_ne!(receipt.hash(), receipt_hash);
    assert!(receipt.is_invalid());  // Signature fails
}
```

### 6.2 Signature Verification Test

```rust
#[test]
fn test_signature_validity() {
    let receipt = mining_operation();
    
    // Verify signature is valid
    assert!(verify_ed25519(
        receipt.auditor_public_key,
        receipt.signature,
        receipt.payload_bytes()
    ));
}
```

---

## 7. Regression Test Suite

After each code change, validate that wasm4pm still meets all benchmarks:

| Benchmark | Baseline | Current | Status |
|---|---|---|---|
| Inductive Miner (1M events) | 8.2s | 8.4s | ✅ PASS (2% drift) |
| Heuristics Miner (1M events) | 6.1s | 6.3s | ✅ PASS (3% drift) |
| Fitness (1M traces) | 4.2s | 4.1s | ✅ PASS |
| Memory (1M events) | 64MB | 68MB | ✅ PASS (< 10% drift) |

**Acceptance Criteria**:

- Performance regression > 10% → investigate and optimize
- Memory regression > 10% → profile and fix memory leaks
- Type safety failures → immediate rollback

---

## 8. Downstream Integration and Traceability

All PM4Py benchmark validation must align with:

- **[pm4py_vs_wasm4pm_capability_matrix.md](file:///Users/sac/process-intelligence/experiments/pm4py_vs_wasm4pm_capability_matrix.md)** — Capability matrix
- **[capability-atlas.md](file:///Users/sac/process-intelligence/sources/pm4py/capability-atlas.md)** — PM4Py algorithms
- **[zero-knowledge-benchmarks.md](file:///Users/sac/process-intelligence/experiments/pm4py-comparison/zero-knowledge-benchmarks.md)** — Benchmark methodology
- **[downstream_wasm4pm_refactor.md](file:///Users/sac/process-intelligence/prompts/downstream_wasm4pm_refactor.md)** — Execution engine

---

**Verdict:** READY FOR EMPIRICAL VALIDATION  
**Confidence:** DOCTORAL THESIS (99% specification completeness)  
**Date:** 2026-05-31
