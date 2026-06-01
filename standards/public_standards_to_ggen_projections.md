# Public Standards to ggen Projections Mapping

The **ggen** core is the manufacturing engine of the Process Intelligence Research Foundry. It translates static process models (expressed in BPMN, Petri Nets, or POWL trees) into executable, law-bound code modules (Rust compiled to WebAssembly). This document establishes how public standards map to code projections and how generated engines are verified.

For the core lifecycle stages, see [Full-Lifecycle Process Scope](file:///Users/sac/process-intelligence/doctrine/full-lifecycle-process.md).

---

## 1. Code Projection Maps by Standard

The `ggen` engine translates process nodes and relations into typed Rust code patterns:

| Source Standard | Structural Element | Projected Rust Structure | Behavior / Constraint |
| :--- | :--- | :--- | :--- |
| **Petri Net** | `Place` | `struct Place { tokens: usize }` | Tracks state markings at runtime. |
| **Petri Net** | `Transition` | `fn fire_transition(state: &mut State)` | Enforces firing rules and state updates. |
| **BPMN** | `Exclusive Gateway` | `match` statement | Routes execution dynamically based on variables. |
| **BPMN** | `Parallel Gateway` | `tokio::join!` or thread fork | Executes concurrent branches asynchronously. |
| **POWL** | `Loop Node` | `while` / `loop` block | Re-runs the loop body until the exit condition is met. |
| **Declare** | LTL Constraint | FSA state validation | Asserts that every state mutation is valid. |

---

## 2. Projection Verification Gate

When `ggen` compiles a process model to WebAssembly, it registers a projection transaction containing the source model hash, the generated Rust code hash, and the compiled WASM module hash:

```json
{
  "projection_id": "ggen-prj-990e8400-e29b-41d4-a716-446655448888",
  "source_model_hash": "a1b2c3...",
  "generated_code_hash": "d4e5f6...",
  "compiled_wasm_hash": "g7h8i9...",
  "compilation_receipt": {
    "compiler_version": "rustc-1.78.0-wasm32-wasi",
    "soundness_verified": true,
    "wasm_signature": "SIG_ED25519_..."
  }
}
```

---

## 3. Execution Verification

To verify that the compiled WASM module conforms to the original public standard:
1.  The validator loads the compiled WASM module into a WASI runtime.
2.  It replays test event logs (in XES/OCEL format) on the WASM module to verify that state transitions match the original model's markings.
3.  For a sample of `ggen` projections and compiled module benchmarks, see the [ggen Projection Sample](file:///Users/sac/process-intelligence/experiments/ggen_projection_sample.md).