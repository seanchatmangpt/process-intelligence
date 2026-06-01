# Conformance Checking: PM4Py vs wasm4pm

## What Each System Does

### PM4Py

PM4Py offers two conformance strategies:

**Token Replay** (`pm4py.conformance_diagnostics_token_based_replay`):
- Simulates token multiset propagation through a Petri net.
- Fitness = `1.0 - missing / (consumed + missing)` (Rozinat & van der Aalst).
- Input: a pandas `DataFrame` (or an `EventLog` object, which is a thin dict wrapper).
- Output: a Python `list[dict]` with keys `trace_fitness`, `activated_transitions`,
  `reached_marking`, `missing_tokens`, `remaining_tokens`.
- Precision via ETConformance or Align-ETC: separate call, separate dict.

**Alignment-Based** (`pm4py.conformance_diagnostics_alignments`):
- Optimal alignments via shortest path (A* over the synchronous product net).
- Cost per move: `log_move_cost`, `model_move_cost`, `sync_move_cost`.
- Output: list of dicts with `alignment`, `cost`, `fitness`, `bwc` (best worst case).
- PM4Py computes all five van der Aalst quality dimensions as runtime `float` values.

**What PM4Py cannot enforce**: there is no static guarantee that the `DataFrame` fed
into token replay was validated. Any row can be missing an `activity` column, have
`NaT` timestamps, or carry no case ID — and the conformance call silently proceeds.

### wasm4pm (wasm4pm-algos crate)

wasm4pm implements token replay conformance in
`wasm4pm-algos/src/conformance.rs`:

```rust
pub fn check_conformance_token_replay(
    log: &EventLog,
    model: &DFG,
    activity_key: &str,
) -> Result<ConformanceResult>
```

- Accepts a raw `&EventLog` — **no admission gate, no witness type parameter**.
- Uses a DFG (Directly-Follows Graph) as the model, not a full Petri net.
- Returns a `ConformanceResult` with raw `f64` fitness.
- No alignment-based conformance in wasm4pm-algos as of this writing.
- No precision estimation in the conformance module.

**Critical gap**: `&EventLog` is accepted without structural validation.
A caller can pass an `EventLog` whose traces contain no activities, whose
timestamps are `0`, or whose case IDs are empty strings. The function will
silently compute a fitness score against that malformed input.

### wasm4pm-compat (this crate)

wasm4pm-compat `src/conformance.rs` is a **verdict carrier, not a checker**:
- `Metric<KIND, NUM, DEN>` with `Require<{ NUM <= DEN }>: IsTrue` — out-of-range
  scores are a **compile error**, not a runtime panic.
- `FitnessConst<2, 1>` does not compile. `FitnessConst<3, 4>` compiles (0.75).
- `ConformanceVerdict`, `Deviation`, `SyncMove`, `LogOnlyMove`, `ModelOnlyMove`
  are shape types — they carry verdicts produced by an engine, not by this crate.
- `ConformanceRefusal` names exactly why a verdict cannot be admitted (structured
  law, not a string).

## The Critical Type-Safety Gap

| Layer | Input type | Validated before use? |
|---|---|---|
| PM4Py | `pd.DataFrame` | No — runtime KeyError at best |
| wasm4pm | `&EventLog` | No — raw struct accepted |
| wasm4pm-compat | `Evidence<T, Admitted, W>` (intended) | Yes — compile-time + admission law |

**What wasm4pm SHOULD do**: the conformance function signature should be:

```rust
pub fn check_conformance_token_replay<W: Witness>(
    log: &Admitted<OcelLog, W>,  // or Evidence<OcelLog, Admitted, W>
    model: &DFG,
    activity_key: &str,
) -> Result<ConformanceResult>
```

This means only evidence that has passed through `Admit::admit()` with a named
witness (e.g. `Ocel20`) can be fed into the algorithm. Laundering raw unvalidated
events into fitness scores becomes impossible at the type level.

## What Exists vs What Is Missing

| Capability | PM4Py | wasm4pm | wasm4pm-compat |
|---|:---:|:---:|:---:|
| Token replay | Yes | Yes (DFG-based) | Shape only |
| Alignment conformance | Yes | No | Shape only |
| Precision estimation | Yes | No | Shape only |
| F1 / generalization / simplicity | Yes (runtime) | No | Compile-time bounds |
| Bounds-checked metrics | No | No | Yes (compile-time) |
| Named admission before conformance | No | No | Intended design |
| Named refusal on malformed input | No (exception) | No (Result<_,_>) | Yes (Refusal<R,W>) |

## Board-Level Summary

PM4Py and wasm4pm both accept unvalidated event data before conformance checking.
wasm4pm-compat enforces that any conformance verdict is structurally in-bounds at
the type level — a fitness of 1.5 cannot be represented in a `FitnessConst`. The
remaining gap is that wasm4pm's runtime engine does not yet require admitted
evidence as its input. Closing that gap means changing the function signature to
accept `Evidence<OcelLog, Admitted, Ocel20>` instead of `&EventLog`.
