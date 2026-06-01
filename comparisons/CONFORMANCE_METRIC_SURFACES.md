# Conformance Metric Surfaces: wasm4pm-compat vs wasm4pm vs PM4Py

## The Core Problem with Runtime f64

A conformance algorithm that returns `f64` can return `1.5`, `-0.3`, or `NaN`.
None of those values are legal quality metrics under van der Aalst (2016), which
defines fitness, precision, F1, generalization, and simplicity on `[0, 1]`.
Runtime panics, clamps, or assertions are silent patches on a structural defect.

## wasm4pm-compat: Compile-Time Bounded Metrics

Source: `wasm4pm-compat/src/conformance.rs`

```rust
pub struct Metric<const KIND: QualityMetricKind, const NUM: u64, const DEN: u64>
where
    Require<{ DEN > 0 }>: IsTrue,
    Require<{ NUM <= DEN }>: IsTrue,
```

The two `where` bounds are enforced at **compile time** by the nightly
`generic_const_exprs` feature. If either is violated, `rustc` refuses to
compile the call site.

| Type alias | Example | What it encodes |
|---|---|---|
| `FitnessConst<NUM, DEN>` | `FitnessConst<3, 4>` | 0.75 fitness |
| `PrecisionConst<NUM, DEN>` | `PrecisionConst<1, 2>` | 0.5 precision |
| `F1Const<NUM, DEN>` | `F1Const<0, 1>` | 0.0 F1 |
| `GeneralizationConst<NUM, DEN>` | `GeneralizationConst<7, 8>` | 0.875 generalization |
| `SimplicityConst<NUM, DEN>` | `SimplicityConst<1, 1>` | 1.0 simplicity |

### What cannot compile

```rust
let _: FitnessConst<2, 1> = FitnessConst::new();  // 2/1 > 1 — compile error
let _: PrecisionConst<5, 4> = PrecisionConst::new(); // 5/4 > 1 — compile error
let _: SimplicityConst<3, 2> = SimplicityConst::new(); // 3/2 > 1 — compile error
let _: FitnessConst<1, 0> = FitnessConst::new();  // DEN == 0 — compile error
```

These are not asserts. Not panics. Not tests. They are `rustc` type errors.
No test infrastructure is needed to enforce the bound.

### The five-metric quality profile

`QualityProfile<F_N, F_D, P_N, P_D, F1_N, F1_D, G_N, G_D, S_N, S_D>` groups
all five dimensions. A profile where any single dimension is out of range does
not compile. No partial-profile admits an invalid component silently.

## wasm4pm: Raw f64 Return

Source: `wasm4pm-algos/src/conformance.rs`

```rust
pub fn check_conformance_token_replay(
    log: &EventLog,
    model: &DFG,
    activity_key: &str,
) -> Result<ConformanceResult>
```

`ConformanceResult` carries raw `f64` fields (fitness, trace-level counts).
There is no bound enforcement. A buggy mutation to the fitness formula could
return `1.0 + epsilon` and no type-level check would catch it. The only defense
is a test asserting the value is in range — a test that may not exist or may
not run in CI.

## PM4Py: Runtime Dict, No Bounds

PM4Py returns Python `dict` objects:

```python
result = pm4py.conformance_diagnostics_token_based_replay(log, net, im, fm)
# result[0] == {'trace_fitness': 0.75, 'missing_tokens': 2, ...}
fitness = result[0]['trace_fitness']  # plain float, could be anything
```

The `'trace_fitness'` key holds a Python `float`. PM4Py contains no assertion
that `trace_fitness` is in `[0, 1]`. Runtime `dict` access can raise `KeyError`
if the key is renamed. The value `1.5` or `-0.3` is representable and would
not raise an error.

Precision estimation (ETConformance, Align-ETC) returns its own dict with a
`'log_fitness'` key whose value is also an unbounded `float`.

## Summary Comparison

| Property | wasm4pm-compat | wasm4pm | PM4Py |
|---|:---:|:---:|:---:|
| Enforcement mechanism | Compile-time type bound | None | None |
| Can represent fitness 1.5? | **No — compile error** | Yes — f64 | Yes — float |
| Can represent fitness -0.3? | **No — compile error** | Yes — f64 | Yes — float |
| Can represent DEN=0? | **No — compile error** | N/A | N/A |
| Covers all 5 van der Aalst dimensions | Yes (in type) | Partial (fitness only) | Yes (runtime) |
| Bounds checked in tests | Not needed (type law) | Only if tests exist | Only if tests exist |

## Board-Level Claim

`Between01` turns out-of-range quality metrics from a runtime defect into a
compile-time impossibility. A wasm4pm conformance engine that wraps its results
in `FitnessConst<NUM, DEN>` cannot ship a score of `1.5` regardless of algorithmic
bugs — the call site will not compile.
