# AGENT 5 — Vector Clock and Monotonic Time Engine

## Mission

Manufacture the `c8-time` crate: 8-lane vector clocks with standard happens-before semantics,
a strictly monotonic nanosecond stamp backed by `Instant` + `AtomicU64`, and a causal
observation alignment API for distributed market traces.

## Files Manufactured

| File | Action |
|---|---|
| `crates/c8-time/Cargo.toml` | Updated — edition 2021, c8-core path dep |
| `crates/c8-time/src/lib.rs` | Full rewrite to specification API |
| `docs/agents/AGENT_05_VECTOR_CLOCK.md` | This file |

## API Surface

### `VectorClockCompare`

Enum: `Before | After | Concurrent | Equal`

Standard happens-before relation outcomes. Derived via component-wise dominance:

- `Equal` — self <= other AND other <= self (all lanes identical)
- `Before` — self <= other in every lane, not equal
- `After` — other <= self in every lane, not equal
- `Concurrent` — neither dominates

### `VectorClock8`

`{ lanes: [u64; 8] }` — `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`

| Method | Signature | Semantics |
|---|---|---|
| `zero` | `() -> Self` | All-zero clock |
| `tick_lane` | `(&mut self, lane: usize)` | Saturating increment of one lane |
| `merge` | `(&self, other: &Self) -> Self` | Component-wise max; returns new clock |
| `compare` | `(&self, other: &Self) -> VectorClockCompare` | Standard vector clock compare |
| `lanes` | `(&self) -> &[u64; 8]` | Borrow the lane array |

`merge` returns a new `Self` (pure function, matches spec `&self` receiver).

### `MonotonicStamp`

`{ epoch: Instant, last_ns: AtomicU64 }`

| Method | Signature | Semantics |
|---|---|---|
| `new` | `() -> Self` | Anchor to current `Instant::now()` |
| `now_ns` | `(&self) -> u64` | Elapsed nanos; never decreases (CAS loop) |
| `assert_not_before` | `(&self, prior_ns: u64) -> bool` | True iff `now_ns() >= prior_ns` |

`impl Default` delegates to `new()`.

### `CausalObservation`

`{ vector_clock: VectorClock8, monotonic_ns: u64, actor_lane: usize }`

| Method | Signature | Semantics |
|---|---|---|
| `align` | `(prior: &Self, incoming: &Self) -> VectorClockCompare` | Returns `prior.vector_clock.compare(&incoming.vector_clock)` |

## Tests

| Test name | What it proves |
|---|---|
| `zero_clocks_are_equal` | Zero-initialized clocks compare Equal |
| `tick_lane_creates_causal_after` | Single lane tick makes clock After its prior; inverse is Before |
| `independent_lane_ticks_are_concurrent` | Ticks on different lanes with no shared history are Concurrent |
| `merge_dominates_both_prior_clocks` | Merged clock is After both inputs |
| `monotonic_time_never_regresses` | Three successive `now_ns()` calls are non-decreasing |
| `causal_align_distinguishes_concurrent_from_ordered` | Ordered pair yields Before; independent ticks yield Concurrent |

## Cargo Check Result

`cargo check -p c8-time` — PASS (no errors, no warnings)

## Verdict

ALIVE — Vector clock compare, monotonic time, and causal alignment manufactured and verified.
