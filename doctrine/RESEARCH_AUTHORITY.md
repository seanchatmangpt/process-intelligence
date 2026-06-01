# Research Authority Table and GAP_001 Finding

## Authority Table

The research program governs over a stack of execution authorities. Each system has a defined
role, and no system may exceed that role without research program authorization.

| System | Role | Authority Scope | Evidence Status |
|---|---|---|---|
| **~/process-intelligence** | Research authority | Issues verdicts, authorizes downstream change | Active |
| **wasm4pm-compat** | Type foundry | Process-evidence type law, witness lattices, lifecycle states | Confirmed present |
| **wasm4pm** | Execution authority | Mining, conformance, replay, receipts, benchmark gates | Confirmed present |
| **ggen** | Manufacturing machinery | Papers → source shapes, doctests, negative fixtures | Future |
| **Blue River Dam** | Lifecycle authority | Upstream closure layer, governance bounds | Doctrine only |
| **PM4Py** | Comparative oracle | Conformance checking, process discovery, fitness/precision | External |
| **M&A decks** | Board projection surface | Highest-value claim surface, buyer reliance proofs | Future |

---

## GAP_001 — Compat-to-Wasm Bridge Missing

**Finding:** wasm4pm does NOT import wasm4pm-compat. These are two parallel type universes
with zero cross-consumption. The "graduation bridge" described in wasm4pm-compat doctrine
exists as a type surface in wasm4pm-compat (the `graduation` module, `GraduateToWasm4pm` trait)
but wasm4pm has no corresponding import or implementation of that bridge.

**Evidence:**

1. `~/wasm4pm/wasm4pm/Cargo.toml` dependencies — no `wasm4pm-compat` entry:
   ```
   [dependencies]
   wasm4pm-types = { workspace = true }
   wasm4pm-algos = { workspace = true }
   wasm-bindgen = "0.2.92"
   ```
   No `wasm4pm-compat` or `wasm4pm_compat` dependency declared.

2. `~/wasm4pm/wasm4pm/src/` — grep for `wasm4pm_compat` or `wasm4pm-compat` returns no results.
   Zero imports. Zero usage. The compat type system is invisible to wasm4pm.

3. wasm4pm algorithm functions accept `&EventLog` (the wasm4pm-types `EventLog`, not the
   wasm4pm-compat `Evidence<EventLog, Admitted, W>`). Examples:
   ```rust
   pub fn discover_ilp_petri_net_from_log(log: &EventLog, activity_key: &str) -> (PetriNet, f64, f64)
   pub fn discover_performance_dfg_from_log(log: &EventLog, activity_key: &str, ...) -> String
   pub fn discover_simple_process_tree_from_log(log: &EventLog, activity_key: &str) -> String
   ```
   All 6+ algorithm functions take raw, unvalidated `EventLog` — no admission gate.

4. Refusals in wasm4pm-types are `ValidationError(String)` — string-typed catch-all:
   ```rust
   // ~/wasm4pm/crates/wasm4pm-types/src/error.rs
   pub enum Error {
       ValidationError(String),
       ParseError(String),
       ExecutionError(String),
       // ...
   }
   ```
   This is the forbidden pattern that wasm4pm-compat doctrine explicitly bans:
   > "bare `InvalidInput` is forbidden" — wasm4pm-compat/src/admission.rs doctrine

**Severity:** CRITICAL

**Impact:**
- The type law manufactured in wasm4pm-compat has zero runtime enforcement in wasm4pm
- Evidence admitted by wasm4pm-compat's `Admit::admit()` cannot be passed to wasm4pm
  without type erasure — the witness tag, lifecycle state, and refusal history are all lost
- A process that fails wasm4pm-compat admission can still be passed to wasm4pm algorithms
  as raw `EventLog` with no rejection
- The "graduation bridge" (`GraduateToWasm4pm`) in wasm4pm-compat is a one-sided contract:
  wasm4pm-compat declares it but wasm4pm does not implement it

**Status:** OPEN — No mitigation path authorized yet.

---

## Additional Observations

### wasm4pm Error Surface

wasm4pm has two error systems running in parallel:

1. **`Wasm4pmError`** (in `wasm4pm/src/error.rs`) — typed enum for WASM public API:
   `Parse(String)`, `Validation(String)`, `BinaryFormat(String)`, `Algorithm{..}`, `HandleNotFound(String)`
   These are typed enum variants but carry `String` reasons, not named law types.

2. **`Error`** (in `crates/wasm4pm-types/src/error.rs`) — legacy typed enum:
   `ValidationError(String)`, `ParseError(String)`, `ExecutionError(String)` — all string-typed.

Neither error system names specific violated laws (`DanglingEventObjectLink`,
`MissingFinalMarking`, `DeadTransition`, `UnsoundWfNet`). This confirms GAP_001 severity:
the entire refusal surface is string-typed catch-alls.

### Two-Universe Problem

```
wasm4pm-compat type universe:          wasm4pm type universe:
  Evidence<T, Admitted, Ocel20>   ≠     EventLog
  Evidence<T, Raw, Xes1849>       ≠     EventLog
  Refusal<DanglingLink, Ocel20>   ≠     Error::ValidationError(String)
  GraduateToWasm4pm trait         ≠     (no corresponding impl)
```

These are structurally incompatible. Bridging requires a translation layer that currently
does not exist in either codebase.

---

## Research Program Authorization

This finding authorizes the following research program outputs:
- `gaps/GAP_001_COMPAT_WASM_BRIDGE.md` — Full gap documentation
- Future: downstream prompt for wasm4pm refactor (authorized only after gap close plan)
- Future: downstream prompt for compat graduation bridge implementation

**No wasm4pm refactor authorized until gap close plan is produced.**
