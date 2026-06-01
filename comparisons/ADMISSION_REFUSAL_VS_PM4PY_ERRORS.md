# Admission/Refusal Law vs PM4Py Error Handling

## The Structural Comparison

Both PM4Py and wasm4pm-compat have a mechanism for rejecting invalid input.
The mechanisms are categorically different.

| Property | PM4Py | wasm4pm-compat |
|---|---|---|
| Mechanism | `try/except Exception` | `Result<Admission<T,W>, Refusal<R,W>>` |
| Rejection type | Python `Exception` (a string wrapper) | `Refusal<R, W>` (two type parameters) |
| Reason encoding | A string in `e.args[0]` | `R`: a specific named structural law type |
| Standard encoding | None — no standard is named | `W`: a witness type (e.g. `Ocel20`, `Xes1849`) |
| Exhaustiveness | No — `except Exception` catches everything | Yes — `match` on a closed `R` enum |
| Machine-readability | No — string parsing required | Yes — pattern match on `R` variants |
| Compiler enforcement | No | Yes — `Refusal<R, W>` is a type; wrong `R` is a compile error |

## PM4Py Error Handling

PM4Py validation surfaces errors as Python exceptions:

```python
import pm4py

try:
    log = pm4py.read_xes("malformed.xes")
    result = pm4py.conformance_diagnostics_token_based_replay(log, net, im, fm)
except Exception as e:
    print(f"Error: {e}")
    # e is a string. Which law was violated? Unknown.
    # Which XES attribute was missing? Unknown without string parsing.
    # Was this an OCEL law violation or an XES law violation? Unknown.
```

PM4Py raises `TypeError`, `KeyError`, `ValueError`, `AttributeError`, and
custom `pm4py.exceptions` subtypes — but the catch site typically uses
`except Exception` and treats the message as a human-readable string.

There is no type that says "this rejection is because the OCEL log has a
dangling E2O link to an undeclared object." There is a string that might say
something similar, or might say `"'NoneType' object has no attribute 'trace'"`.

## wasm4pm-compat Admission/Refusal

```rust
use wasm4pm_compat::admission::{Admit, Admission, Refusal};
use wasm4pm_compat::witness::Ocel20;
use wasm4pm_compat::ocel::OcelLog;

match Admit::<OcelLog, Ocel20>::admit(raw_evidence) {
    Ok(admission) => {
        // admission: Admission<OcelLog, Ocel20>
        // The value has crossed the boundary.
        // It is now Evidence<OcelLog, Admitted, Ocel20> after .into_evidence()
    }
    Err(refusal) => {
        // refusal: Refusal<OcelRefusal, Ocel20>
        // R = OcelRefusal — a specific named enum:
        //   OcelRefusal::DanglingEventObjectLink { event_id, object_id }
        //   OcelRefusal::DanglingObjectObjectLink { source_id, target_id }
        //   OcelRefusal::DuplicateObjectId { id }
        //   OcelRefusal::DuplicateEventId { id }
        //   OcelRefusal::EmptyLog
        //   OcelRefusal::FlatteningLoss { dropped_types }
        // W = Ocel20 — the specific standard being satisfied
    }
}
```

The refusal reason `R` carries:
1. **Which structural law was violated** — as a type variant, not a string.
2. **Which object was involved** — as a field in the variant (e.g. `event_id`).
3. **Which standard the admission was attempting to satisfy** — as `W`.

## Named Laws: The Critical Constraint

wasm4pm-compat's admission law states:

> The reason type `R` must name a specific structural law.
> Bare `InvalidInput` or string-typed catch-alls are defects.

This means:

```rust
// Acceptable — specific named law
Refusal::<OcelRefusal::DanglingEventObjectLink, Ocel20>::new(
    OcelRefusal::DanglingEventObjectLink { event_id: "e42", object_id: "obj99" }
)

// Defect — catch-all is forbidden
Refusal::<InvalidInput, Ocel20>::new(InvalidInput("something went wrong"))
// ^ This is not allowed. The design explicitly rejects this.
```

A catch-all refusal reason is detectable in code review because it is a type.
If `R` is `InvalidInput`, the code fails the design law. If `R` is
`DanglingEventObjectLink`, it passes.

## Exhaustiveness: match vs except

PM4Py:
```python
try:
    result = pm4py.conformance_diagnostics_token_based_replay(...)
except ValueError as e:
    handle_value_error()
except KeyError as e:
    handle_key_error()
except Exception as e:
    handle_everything_else()  # This is always reachable — the exception set is open
```

wasm4pm-compat:
```rust
match Admit::admit(raw) {
    Ok(admission) => { /* success path */ }
    Err(refusal) => match refusal.reason() {
        OcelRefusal::DanglingEventObjectLink { event_id, object_id } => { … }
        OcelRefusal::DuplicateObjectId { id } => { … }
        OcelRefusal::EmptyLog => { … }
        // If OcelRefusal gains a new variant, this match becomes non-exhaustive.
        // The compiler forces the caller to handle it.
    }
}
```

The `match` on a closed enum is **exhaustive by construction**. If the
`OcelRefusal` enum gains a new variant (e.g. `MalformedTimestamp`), every
call site that matches on it gets a compile error until the new variant is
handled. PM4Py has no equivalent — a new exception type in a library release
silently falls through to `except Exception`.

## The Witness Parameter W

Both the `Admission<T, W>` and `Refusal<R, W>` carry the witness `W`. This
means:

- An OCEL admission (`Admission<OcelLog, Ocel20>`) cannot be substituted for
  an XES admission (`Admission<XesLog, Xes1849>`). They are different types.
- A refusal from an OCEL admit (`Refusal<OcelRefusal, Ocel20>`) cannot be
  substituted for a refusal from an XES admit. The standard being satisfied
  is encoded in the type, not in a runtime string.

PM4Py has no equivalent. A `KeyError` from `pm4py.read_xes()` and a `KeyError`
from `pm4py.read_ocel_json()` are the same type (`KeyError`). The caller cannot
distinguish which standard was being satisfied.

## Board-Level Claim

Our process validation failures are named laws, not generic exceptions. When an
OCEL log is rejected at the admission boundary, the caller receives a
`Refusal<DanglingEventObjectLink, Ocel20>` — a type that encodes exactly which
structural law was violated and which standard was being satisfied. No string
parsing. No `except Exception`. No silent fallthrough. A compiler-enforced,
machine-readable, exhaustively-handled failure type.

PM4Py's error handling is a string in a box. wasm4pm-compat's is a named law
in a type.
