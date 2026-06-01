# GAP_001 — Compat-to-Wasm Bridge Missing

**Gap ID:** GAP_001
**Severity:** CRITICAL
**Status:** OPEN
**Discovered:** 2026-05-31
**Source:** doctrine/RESEARCH_AUTHORITY.md

---

## Summary

wasm4pm does NOT import wasm4pm-compat. The two crates are parallel type universes
with zero cross-consumption. The "graduation bridge" described in wasm4pm-compat
doctrine exists as a trait declaration only — wasm4pm has no corresponding implementation.
All wasm4pm algorithm functions accept raw, unvalidated `EventLog` with no admission gate.

---

## Expected Capability

Based on wasm4pm-compat doctrine (BLUE_RIVER_DAM.md, graduation.rs):

1. wasm4pm should accept `Evidence<T, Admitted, W>` from wasm4pm-compat, not raw `EventLog`
2. The `GraduateToWasm4pm` trait in wasm4pm-compat should have an implementation in wasm4pm
3. Refusals in wasm4pm should name violated laws, not carry `String` messages
4. An admitted wasm4pm-compat type should be the only legal input to wasm4pm algorithms

The law: "The doorway must not become the throne room." (BLUE_RIVER_DAM.md)
The corollary: "The throne room must accept only door-admitted evidence."

---

## Observed Capability

Evidence gathered from source code inspection:

### 1. No wasm4pm-compat Dependency in wasm4pm

File: `~/wasm4pm/wasm4pm/Cargo.toml`

```toml
[dependencies]
wasm4pm-types = { workspace = true }
wasm4pm-algos = { workspace = true }
wasm-bindgen = "0.2.92"
js-sys = "0.3"
wasm-bindgen-futures = "0.4.42"
```

`wasm4pm-compat` does not appear. Zero dependency. Zero import.

### 2. Algorithm Functions Accept Raw EventLog

Files: `~/wasm4pm/wasm4pm/src/ilp_discovery.rs`, `process_tree.rs`, `performance_dfg.rs`

```rust
pub fn discover_ilp_petri_net_from_log(log: &EventLog, activity_key: &str) -> (PetriNet, f64, f64)
pub fn discover_performance_dfg_from_log(log: &EventLog, ...) -> String
pub fn discover_simple_process_tree_from_log(log: &EventLog, activity_key: &str) -> String
```

`EventLog` here is `wasm4pm-types::EventLog` — raw, unvalidated, not admitted under any witness.
No admission gate. No lifecycle state. No witness tag. The type carries no proof of admission.

### 3. String-Typed Refusals in wasm4pm-types

File: `~/wasm4pm/crates/wasm4pm-types/src/error.rs`

```rust
pub enum Error {
    ValidationError(String),   // forbidden pattern per wasm4pm-compat doctrine
    ParseError(String),
    ExecutionError(String),
    HashError(String),
    ProvenanceError(String),
    BudgetExceeded(String),
    StateError(String),
    NotFound(String),
    SerializationError(String),
    Unknown(String),
}
```

Every error variant carries a `String` reason. None carry named law types.

wasm4pm-compat doctrine explicitly bans this pattern:
> "bare `InvalidInput` is forbidden" (wasm4pm-compat/src/admission.rs)
> "The reason type `R` must name a specific structural law"

### 4. GraduateToWasm4pm Trait is One-Sided

wasm4pm-compat declares:
```rust
// wasm4pm-compat/src/graduation.rs
pub trait GraduateToWasm4pm { ... }
```

wasm4pm implements: nothing. The trait is orphaned — declared on the doorway side, never
picked up by the throne room.

---

## Impact Assessment

| Impact | Description |
|---|---|
| Type law enforcement | Zero — compat type law has no runtime effect in wasm4pm |
| Admission gate | Missing — non-admitted evidence can enter all 6 algorithm functions |
| Refusal law | Missing — `ValidationError(String)` is a string catch-all, not named law |
| Witness discipline | Missing — no witness tag survives the wasm4pm boundary |
| Lifecycle state | Missing — `Evidence<T, Raw, W>` and `Evidence<T, Admitted, W>` look identical to wasm4pm |
| Receipt chain | Broken — receipts from wasm4pm-compat admission cannot be linked to wasm4pm execution |
| Graduation bridge | Orphaned — `GraduateToWasm4pm` trait exists in compat, has no implementation in wasm4pm |

---

## Two-Universe Diagram

```
wasm4pm-compat type universe         wasm4pm type universe
─────────────────────────────        ─────────────────────────
Evidence<T, Raw, Ocel20>             EventLog (raw, untyped)
Evidence<T, Admitted, Ocel20>   ≠    EventLog (raw, untyped)
Refusal<DanglingLink, Ocel20>        Error::ValidationError(String)
GraduateToWasm4pm trait              (no implementation)
─────────────────────────────        ─────────────────────────

         NO BRIDGE EXISTS BETWEEN THESE UNIVERSES
```

A process that wasm4pm-compat refuses can still be passed to wasm4pm algorithms.
A process that wasm4pm-compat admits carries no proof of admission inside wasm4pm.

---

## Required Remediation Path

To close GAP_001, all of the following must be true:

1. **Dependency**: wasm4pm must add `wasm4pm-compat` as a workspace dependency
2. **Algorithm signatures**: At minimum one algorithm function must accept
   `Evidence<EventLog, Admitted, W>` rather than raw `EventLog`
3. **GraduateToWasm4pm**: wasm4pm must implement the `GraduateToWasm4pm` trait
   for its execution types
4. **Named refusals**: At minimum one wasm4pm error variant must carry a named law
   type, not a `String`
5. **Receipt link**: wasm4pm execution receipts must reference the wasm4pm-compat
   admission receipt that authorized the input

No partial remediation constitutes gap close. All five conditions must be met.

---

## Authorization

**No downstream wasm4pm refactor is authorized until the remediation path above is approved
by the research program and documented in a follow-up gap close checkpoint.**

Next authorized action: research program to draft downstream prompt `prompts/downstream_wasm4pm_refactor.md`
with GAP_001 remediation requirements.
