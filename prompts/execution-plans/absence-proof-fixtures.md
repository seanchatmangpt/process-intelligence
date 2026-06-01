# Execution Blueprint v30.1.1: Absence-Proof Failure Fixtures for wasm4pm-compat

## 1. Abstract & Objectives
This document defines the "Absence-Proof" testing strategy for the `wasm4pm-compat` component. The core objective is to codify "negative evidence" and "refusal capital" through strict typestate enforcement. Instead of merely testing that valid inputs produce valid outputs, we mandate that invalid configurations, unauthorized state transitions, and conceptually impossible scenarios result in deterministic **compilation failures** or **load-time validation rejections**. 

## 2. Refusal Capital & Negative Evidence
"Refusal Capital" is the measure of a system's capacity to provably reject invalid states at the earliest possible boundary (typically compilation or static analysis). "Negative Evidence" is the cryptographic or structural proof that a particular malicious or invalid state cannot exist because the types or validation logic fundamentally cannot represent it.

For `wasm4pm-compat`, this means enforcing WASM module boundaries, ABI constraints, and memory access patterns using strict TypeScript/Rust typestates.

## 3. Typestate Enforcement Strategy

### 3.1. Phantom Data & Type-Level State Machines
We employ type-level state machines using phantom types to represent the lifecycle of a WASM module within the compatibility layer:
- `Unverified`
- `Validated`
- `Instantiated`
- `Linked`

Transitions between these states must be proven at compile time. An attempt to execute a function from an `Unverified` or `Validated` module must yield a static type error (`tsc` failure).

### 3.2. Compilation-Failure Fixtures (`@ts-expect-error`)
The primary vehicle for absence proofs in the TypeScript compatibility layer is the intentional compilation failure fixture. 

These fixtures reside in `tests/absence-proofs/` and are validated using a custom runner that ensures the TypeScript compiler fails with the *exact expected error code* (e.g., `TS2345`).

#### Fixture Category 1: Cross-Boundary Capability Leakage
Prove that an imported capability cannot be accidentally exported or leaked to an unprivileged internal function.

```typescript
// tests/absence-proofs/enforce-boundary.fail.ts
import { importCapability } from 'wasm4pm-compat/core';

const cap = importCapability('fs_read', { sandboxed: true });

// @ts-expect-error TS2345: Argument of type 'SandboxedCapability' is not assignable to parameter of type 'UnrestrictedCapability'.
exportToUnsafeWasm(cap);
```

#### Fixture Category 2: Typestate Violation
Prove that a module cannot be linked before it is validated.

```typescript
// tests/absence-proofs/state-sequence.fail.ts
import { loadModule, linkModule } from 'wasm4pm-compat/lifecycle';

const module: WasmModule<Unverified> = loadModule('./untrusted.wasm');

// @ts-expect-error TS2345: Argument of type 'WasmModule<Unverified>' is not assignable to parameter of type 'WasmModule<Validated>'.
const linked = linkModule(module, {});
```

#### Fixture Category 3: Memory Safety Constraints
Prove that linear memory offsets cannot be accessed without bounds checks encoded in the type system.

```typescript
// tests/absence-proofs/memory-bounds.fail.ts
import { MemoryView, Pointer } from 'wasm4pm-compat/memory';

const view = new MemoryView(new WebAssembly.Memory({ initial: 1 }));
const ptr: Pointer<number, 'unchecked'> = view.getPointer(0x100000);

// @ts-expect-error TS2345: Argument of type 'Pointer<number, "unchecked">' is not assignable to parameter of type 'Pointer<number, "checked">'.
view.read32(ptr);
```

## 4. Execution Plan & Fixture Runner Integration

1.  **Framework Setup**: Implement a test harness that wraps `tsc --noEmit`. The harness must parse `// @ts-expect-error` directives and fail the test if the compilation *succeeds* or fails with a mismatched error code.
2.  **Rust Component Interop**: For the Rust bindings of `wasm4pm-compat`, utilize `trybuild` crates to enforce compilation-failure fixtures for macro expansion and ABI generation.
3.  **CI Enforcement**: The CI pipeline must run the absence-proof suite (`npm run test:absence`) strictly before behavioral unit tests. A failure in refusal capital proofs halts the pipeline.
4.  **Audit Trail**: Every absence-proof fixture must map to a CVE mitigation or a specific architectural invariance documented in the threat model.

## 5. Success Criteria
- 100% of defined illegal states manifest as compilation errors (either `tsc` or `rustc`).
- Zero reliance on runtime `throw new Error(...)` for structural invalidities mapped in the ontology.
- CI pipeline natively supports and validates expected-failure tests.
