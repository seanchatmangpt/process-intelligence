# WebAssembly Component Model Capability Map
## WIT Interface Specification and Guidelines for Wasm4pm compatibility
## Non-Browser Component Boundary Lane

This document defines the architectural division of labor, type projections, and interface contracts for the WebAssembly Component Model (`cargo-component`, `wit-bindgen`, `wasm-tools`) in the non-browser runtime environment of the Process Intelligence workspace (`wasm4pm-compat`).

---

## 1. Prime Architecture Split

The process intelligence architecture relies on a strict separation of concerns:
> **Structure belongs in `wasm4pm-compat`. Execution belongs in `wasm4pm`.**

When compiling for the WebAssembly Component Model (WASM Component Model v0.2.0 and later), this law translates to a structural separation of WebAssembly interfaces:

```
                  ┌───────────────────────────────────────────┐
                  │          RUST COMPAT SOURCE LAW           │
                  │   Evidence<T, State, W> / Refusal<R, W>   │
                  └─────────────────────┬─────────────────────┘
                                        │
                         Projections & Serialization
                                        │
                                        ▼
                  ┌───────────────────────────────────────────┐
                  │             WIT BOUNDARY GATE             │
                  │         (wasm4pm-compat.wit schema)       │
                  └─────────────────────┬─────────────────────┘
                                        │
                  ┌─────────────────────┴─────────────────────┐
                  ▼                                           ▼
      ┌───────────────────────┐                   ┌───────────────────────┐
      │     COMPAT WORLD      │                   │     ENGINE WORLD      │
      │    (wit-bindgen)      │                   │   (cargo-component)   │
      ├───────────────────────┤                   ├───────────────────────┤
      │ Structural records,   │                   │ Replay solvers, A*    │
      │ enums, variants, and  │                   │ space search, miner   │
      │ admission checks.     │                   │ algorithms, queries.  │
      └───────────────────────┘                   └───────────────────────┘
```

The boundary separates compile-time type safety checking from runtime WebAssembly Component interface definitions.

| Problem / Boundary Area | Rust-Only (Internal) | WIT Component Boundary (External) | Rationale |
| :--- | :--- | :--- | :--- |
| **Type-Level Verification** | Typestates (`Raw`, `Admitted`), `PhantomData`, trait bounds | Mapped to explicit string-tags, enum-variants, and record structures | WIT has no native concept of Rust's zero-sized type parameters, phantom variables, or const-generic arithmetic. |
| **Error Enforcement** | Compile-fail UI tests, compile-time witness bounds | Concrete WIT `result<ok, error>` using a WIT `variant` for Refusals | Prevents generic error laundering. Downstream hosts must inspect structured error variants to identify the specific named law violated. |
| **Logic & Solvers** | Transition firing, A* search, discovery miners, index traversals | Mapped to imported/exported functions defined in the `engine-world` | Compat is strictly structure-only. Any data-traversal execution must graduate to the engine and are excluded from the `compat-world`. |
| **Memory Management** | Rust lifetimes (`'a`), owned allocations, borrowing | Managed via Component Model Canonical ABI (`cabialloc`, lift/lower) | The Canonical ABI manages cross-boundary heap allocations using a shared linear memory, freeing callers from manual pointer arithmetic. |

---

## 2. WebAssembly Component Model Toolchain

For non-browser host targets (such as Wasmtime host runners, CLI plugins, and cloud microservices), the interface is described in WebAssembly Interface Type (WIT) files and compiled using standard tools:

1. **`wasm-tools`**: Syntactically validates the `.wit` schemas, parses components, and verifies compliance of target binary WASM files against target world structures.
2. **`cargo-component`**: Subcomponent builder that acts as a wrapper around Rust's compiler, automatically calling `wit-bindgen` to generate local Rust bindings from `.wit` files and weaving them into the final component Wasm binary.
3. **`wit-bindgen`**: Generates source-level language bindings (Rust, Go, C++, Python) from WIT interface files, wrapping the Canonical ABI serialization macros.

---

## 3. Typestate-to-WIT Projection Law

Internal Rust types (which contain generics, trait bounds, and phantom types) must be projected into concrete WIT structures. The mapping is governed by the following rules:

### 3.1. Evidence State Projection
Rust's `EvidenceState` is represented as type-level markers (`Raw`, `Parsed`, `Admitted`, `Refused`, `Projected`, `Exportable`, `Receipted`). Because these are zero-sized phantom markers, they are mapped at the boundary to a concrete WIT enum:

```wit
enum evidence-state {
    raw,
    parsed,
    admitted,
    refused,
    projected,
    exportable,
    receipted
}
```

### 3.2. Universal Evidence Carrier
The Rust struct `Evidence<Payload, State, Witness>` maps to a flat, owned WIT record:

```wit
record evidence-dto {
    payload: string,           // Serialized JSON of structure T
    state-tag: evidence-state, // The projected lifecycle phase
    witness-tag: witness-key,  // Grounding law authority
    timestamp-ns: u64          // Production epoch nanoseconds
}
```

### 3.3. Named Law Refusals
To satisfy the **Named Law Refusal Doctrine**, all error paths must map to a WIT `result<ok, error>` where `error` is a variant containing every known structural refusal reason. Catch-all strings are forbidden:

```wit
variant refusal-reason {
    dangling-event-object-link(string),
    missing-final-marking(string),
    empty-trace(string),
    missing-case-id(string),
    invalid-timestamp-order(string),
    unsound-wf-net(string),
    projection-name-required(string)
}

record refusal-dto {
    reason: refusal-reason,
    witness: witness-key,
    message: string
}
```

---

## 4. WIT Boundary Interface Definition

Below is the conceptual layout of the Wasm interface definitions for the `wasm4pm-compat` boundary.

### 4.1. Types Interface (`wasm4pm:compat/types`)
Defines all shared record, enum, and variant types, representing process model definitions, event log formats, and receipts.

### 4.2. World Division (`wasm4pm:compat/worlds`)
- **`compat-world`**: Exports function signatures for parsing, validating, and admitting event logs. It does not contain any execution endpoints.
- **`engine-world`**: Imports `compat-world` types and extends the interface with high-overhead computational exports (such as `discover-model` and `compute-conformance`).

---

## 5. Audit-Driven Boundary Enforcement

To ensure developers do not introduce execution logic into `wasm4pm-compat` component targets, the build and CI pipeline runs automated checks against the generated component binary:
1. Validate that the compiled `.wasm` component matches the `compat-world` signature exactly.
2. Confirm the absence of exported solver symbols (`replay*`, `solver*`, `mine*`, `discover*`, `conformance*`) in the compatibility Wasm file.
3. Validate that no unsafe pointer logic or raw memory writes are exported.
