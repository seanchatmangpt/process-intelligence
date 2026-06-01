# WASM Boundary Law: Specta + tsify
## Unified Interface Specification for WASM-powered Process Intelligence Workbenches

This document establishes the official architectural division of labor between compile-time TypeScript type generation and runtime WebAssembly (WASM) ABI crossings within the `~/process-intelligence` ecosystem and its downstream execution engines (`wasm4pm`, `wasm4pm-compat`).

---

## 1. Prime Architecture Split

The central principle of the WASM boundary is the separation of **TypeScript Type Specification (the Law)** from **Binary Memory Marshal (the ABI)**.

```
┌────────────────────────────────────────────────────────┐
│               RUST LAW SOURCE SYSTEM                   │
│   (wasm4pm-compat / Evidence<T, State, W> / Petri Net)  │
└───────────────────────────┬────────────────────────────┘
                            │
            ┌───────────────┴───────────────┐
            ▼                               ▼
┌───────────────────────┐       ┌───────────────────────┐
│        SPECTA         │       │    TSIFY / SERDE      │
│ (TypeScript Law Gate) │       │ (WASM ABI Crossing)   │
├───────────────────────┤       ├───────────────────────┤
│ Describes signatures, │       │ Controls actual memory│
│ commands, DTO types,  │       │ layout, serialization,│
│ and query interfaces. │       │ and native JS bridging.│
└───────────┬───────────┘       └───────────┬───────────┘
            │                               │
            └───────────────┬───────────────┘
                            ▼
┌────────────────────────────────────────────────────────┐
│                   TYPESCRIPT FAÇADE                    │
│   (Process Intelligence Workbench / Livestream HUD)    │
└────────────────────────────────────────────────────────┘
```

| Problem / Boundary Area | Selected Tool | Role / Alignment |
| :--- | :--- | :--- |
| **Human-facing TypeScript Law** | **Specta** | Describes DTO structures, command signatures, query envelopes, and return interfaces. |
| **WASM ABI boundary crossing** | **tsify + wasm-bindgen** | Manages the raw Rust↔JS value passing, heap pointers, and low-level bindings. |
| **Native JS Object serialization** | **serde-wasm-bindgen** | Transmutes rich Rust types into native JavaScript values (fast, zero-overhead). |
| **Custom typescript overrides** | **wasm-bindgen Custom Sections** | Appends hand-controlled `.d.ts` declaration patches (used strictly as an escape hatch). |

---

## 2. Division of Labor Specification

### 2.1. Rust Law to TypeScript Projections
Internal Rust law types (such as `Evidence<T, State, W>` or `Admission<T, W>`) carry type-level parameters, zero-sized lifecycle tokens, and phantom markers. These types **must not** cross the WASM ABI directly. Instead, they project to explicit boundary objects:

```rust
// 1. Rust Law Type (Self-Attesting & Unforgeable)
pub struct Evidence<T, State, W> {
    pub value: T,
    _state: PhantomData<State>,
    _witness: PhantomData<W>,
}

// 2. Projected WASM Boundary DTO (Specta + Tsify annotated)
#[derive(Clone, Debug, Serialize, Deserialize, specta::Type, tsify::Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct EvidenceDto {
    pub value: String,
    pub state_tag: String,
    pub witness_key: &'static str,
}
```

### 2.2. Command Surface and Facades
For workbench interactions, Specta is the type exporter for functions and API layouts:

```rust
// Specifying the TypeScript command signature directly in Rust via Specta
#[specta::rpc]
pub fn verify_conformance(trace: Vec<String>) -> Result<ReceiptDto, RefusalDto> {
    // Execution graduates to wasm4pm
    // Boundary values cross via tsify
    unimplemented!()
}
```

---

## 3. Reference Model: `PROCESS_INTELLIGENCE_WASM_TS_BOUNDARY_001`

### 3.1. Internal Rust Law (Sources)
`wasm4pm-compat` governs type safety:
* `Evidence<T, State, W>` — Universal process-evidence carrier.
* `Admission<T, W>` — Non-forgeable admitted state representation.
* `Refusal<R, W>` — Sealed structural law violation type.
* `LossPolicy` & `LossReport` — Mathematical projections bounds.
* `Receipt` — Attested proof objects.

### 3.2. Boundary Projections
Explicit DTO definitions created strictly for JS interop:
* `EvidenceDto` — Serialized evidence bundle.
* `AdmissionDto` — Attested admission record.
* `RefusalDto` — Explanatory law failure diagnostics.
* `ReceiptDto` — Cryptographic replaying witness receipt.
* `OcpqQueryDto` — Multi-perspective query inputs.

---

## 4. Maturity Matrix

The tool selection is governed by the following interface characteristics:

| Option | WASM ABI Fit | TS Type Quality | Command Surface Fit | Refusal/Receipt Gate Fit | Standard Usage |
| :--- | :---: | :---: | :---: | :---: | :--- |
| **Specta** | 3 | 5 | 5 | 5 | **Primary** — Describes TypeScript-facing API interfaces. |
| **tsify** | 5 | 4 | 2 | 4 | **Primary** — Transports structs/enums through wasm-bindgen. |
| **wasm-bindgen** | 5 | 3 | 4 | 4 | **Core** — The underlying ABI compiler and memory marshal. |
| **serde-wasm-bindgen** | 5 | 3 | 2 | 4 | **Helper** — Optimized native JavaScript type conversion. |
| **typescript_custom_section** | 4 | 5 | 2 | 3 | **Escape Hatch** — Inline custom TS declarations. |
| **ts-rs** | 2 | 5 | 2 | 5 | **Avoid for WASM** — Better suited for native Node backend bindings. |

---

## 5. Livestream-Streaming Compliance Linkage

Under the **Dr. Wil van der Aalst AGI Livestreaming Standards (Epoch v30.1.1)**, the stream overlays and telemetry feeds must consume outputs conforming strictly to these boundary rules. 

1. **Schema Integrity**: Telemetry Auditor Agents inspect incoming JSON streams generated by WASM models. The TypeScript type wrappers outputted by Specta must match the audited telemetry attributes (`streamFps`, `streamLatency`, `streamFrameDrops`) exactly.
2. **Optimal Replay Solver**: Conformance check structures passing to the A* solver must map directly to `AlignmentConformance` signatures exported via the Specta-tsify pipeline.
3. **Immutable Ledger Block Chains**: The ledger block sequence audits require strict schema alignment:
   $$H_i = \operatorname{SHA-256}(i \mathbin{\Vert} t \mathbin{\Vert} \text{case\_id} \mathbin{\Vert} \text{payload} \mathbin{\Vert} H_{i-1})$$
   The `ReceiptDto` schemas exported by Specta declare the tamper-evident field layouts for stream validation.
