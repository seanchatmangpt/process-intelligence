# Type Safety Comparison: PM4Py vs wasm4pm vs wasm4pm-compat

## The Central Question

Can an unvalidated, malformed event log be laundered into algorithm results
without detection?

- PM4Py: yes, always.
- wasm4pm: yes — raw `EventLog` accepted everywhere.
- wasm4pm-compat: no — `Evidence<T, Admitted, W>` is the only lawful input to
  admitted operations. The compiler enforces it.

## PM4Py: Runtime Hints Only

PM4Py is a Python library. Its type annotations are PEP 484 hints — they are
checked by `mypy` if you run it, ignored by CPython at runtime.

```python
def conformance_diagnostics_token_based_replay(
    log: EventLog,
    petri_net: PetriNet,
    initial_marking: Marking,
    final_marking: Marking,
    ...
) -> List[Dict[str, Any]]:
```

The return type is `List[Dict[str, Any]]` — a list of arbitrary dicts. The
`EventLog` argument can be any Python object; there is no enforcement. Passing
a `DataFrame` with no `case:concept:name` column raises a runtime `KeyError`
inside PM4Py, not a type error at the call site.

**What PM4Py cannot prevent**:
- Empty event logs fed to discovery algorithms.
- Traces with no activities fed to token replay.
- An `EventLog` where `pm4py.get_event_attribute_values(log, 'concept:name')`
  returns an empty dict (no activities at all).

## wasm4pm: Rust Types, Raw EventLog Accepted

wasm4pm uses Rust's type system, which is strong and static. However, the
conformance and discovery function signatures accept `&EventLog` directly:

```rust
pub fn check_conformance_token_replay(
    log: &EventLog,
    model: &DFG,
    activity_key: &str,
) -> Result<ConformanceResult>

pub fn discover_alpha(log: &EventLog, activity_key: &str) -> Result<PetriNet>
pub fn discover_heuristic(log: &EventLog, activity_key: &str) -> Result<DFG>
```

`EventLog` is defined in `wasm4pm-types`. It is a plain struct: no admission
gate, no witness type parameter, no structural validation requirement.

**What wasm4pm cannot prevent**:
- Calling `check_conformance_token_replay` with an `EventLog` whose traces all
  have zero events. The function computes a fitness of 1.0 by the early-continue
  branch: `if activities.is_empty() { fitting_traces += 1; continue; }`.
- Calling `discover_alpha` with an `EventLog` built by constructing the struct
  directly with empty `traces`. No Alpha+ relations are computed but the call
  succeeds.
- Calling any algorithm before the event log has been structurally validated
  (timestamps in range, activity key present, case IDs non-empty). The Rust
  type system does not express this constraint on `&EventLog`.

## wasm4pm-compat: Evidence<T, State, W> Compile-Time Enforcement

wasm4pm-compat introduces three layers:

**Layer 1: State tokens** (`src/state.rs`)
```rust
pub enum Raw {}
pub enum Parsed {}
pub enum Admitted {}
pub enum Refused {}
pub enum Projected {}
pub enum Exportable {}
pub enum Receipted {}
```

These are empty enums used as `PhantomData` type parameters. `Evidence<T, Raw, W>`
and `Evidence<T, Admitted, W>` are **different types**. The compiler enforces this.

**Layer 2: Witness markers** (`src/witness.rs`)
```rust
pub struct Ocel20;
pub struct Xes1849;
pub struct WfNetSoundnessPaper;
// ... one per paper, standard, or named law
```

Each implements the `Witness` trait with const metadata (`KEY`, `TITLE`, `YEAR`,
`FAMILY`). `Admission<T, Ocel20>` and `Admission<T, Xes1849>` are different
types — an OCEL admission cannot be silently used as an XES admission.

**Layer 3: Evidence carrier** (`src/evidence.rs`)
```rust
pub struct Evidence<T, State, W> {
    value: T,
    _state: PhantomData<State>,
    _witness: PhantomData<W>,
}
```

The only path to `Evidence<T, Admitted, W>` is through `Admit::admit()`:

```rust
pub trait Admit<T, W>: Sized {
    type Reason;
    fn admit(raw: Evidence<T, Raw, W>) -> Result<Admission<T, W>, Refusal<Self::Reason, W>>;
}
```

**What wasm4pm-compat prevents**:
- Constructing `Evidence<OcelLog, Admitted, Ocel20>` without passing through
  an `Admit` impl. The `Admitted` constructor is `pub(crate)`.
- Using an OCEL-admitted value where an XES-admitted value is required. The
  witness type parameter `W` distinguishes them.
- Passing raw, unvalidated events to any function whose signature requires
  `Evidence<_, Admitted, _>`. The compiler rejects the call.
- Laundering: there is no coercion from `Evidence<T, Raw, W>` to
  `Evidence<T, Admitted, W>`. None. No cast, no transmute, no unsafe.

## The Laundering Test

"Laundering" means passing raw unvalidated events into an algorithm and getting
results back without any validation having occurred.

| System | Can launder? | How it fails (if at all) |
|---|:---:|---|
| PM4Py | **Yes** | Runtime KeyError or silent wrong result |
| wasm4pm | **Yes** | Silent wrong result (fitting traces = all empty) |
| wasm4pm-compat | **No** | `rustc` type error at the call site |

## Admission vs Exception: Named Laws vs Strings

PM4Py error handling:
```python
try:
    result = pm4py.conformance_diagnostics_token_based_replay(log, net, im, fm)
except Exception as e:
    print(f"Error: {e}")  # a string
```

wasm4pm-compat refusal:
```rust
match Admit::admit(raw_evidence) {
    Ok(admission) => { /* evidence is Admitted<OcelLog, Ocel20> */ }
    Err(refusal) => {
        // refusal: Refusal<DanglingEventObjectLink, Ocel20>
        // R = DanglingEventObjectLink: a specific structural law
        // W = Ocel20: which standard was being satisfied
        // No strings. No generics. A type.
    }
}
```

The refusal reason `R` must be a **specific named law type** — e.g.
`DanglingEventObjectLink`, `MissingFinalMarking`, `EmptyTrace`. A bare
`InvalidInput` or `GenericError` reason type is a design defect in wasm4pm-compat.

## Summary

Only wasm4pm-compat prevents laundering raw unvalidated events into algorithm
results. PM4Py and wasm4pm both accept unvalidated inputs, with PM4Py failing at
runtime and wasm4pm silently computing incorrect results. The wasm4pm-compat
`Evidence<T, State, W>` type with `pub(crate)` `Admitted` constructor is the
only mechanism in this stack that makes unvalidated input a **compile-time
impossibility** rather than a runtime defect.
