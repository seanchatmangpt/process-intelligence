# AGENT 07 — Branchless Hot Path Benchmarks

## Mission

Manufacture the criterion benchmark harness for CONSTRUCT8 branchless hot path operations,
generalize the Naut branchless discipline into canonical doctrine, and verify cargo check PASS.

## Files Written

| File | Purpose |
|---|---|
| `crates/c8-bench/Cargo.toml` | Criterion dev-dependency, `[[bench]]` harness = false target |
| `crates/c8-bench/src/lib.rs` | Crate-level doc comment only (benchmarks live in benches/) |
| `crates/c8-bench/benches/construct8.rs` | Criterion benchmark groups: branchless_mask, market_planck_cell_emit |
| `docs/NAUT_GENERALIZATION.md` | Six-section doctrine: Naut proof, generalization, branchless law, Need9 split, synthetic data disclaimer, PARTIAL_ARCH |
| `docs/agents/AGENT_07_BENCH.md` | This file |

## Benchmark Groups

### bench_construct8_apply

BenchmarkGroup `branchless_mask` with `BenchmarkId::new("branchless_mask", n)` for n in [1, 2, 4, 8].
Each iteration constructs a fresh `GraphField` and applies a pre-built `Construct8Delta` of n triples.
Measures the branchless masked iteration path through `GraphField::apply_construct8`.

### bench_market_planck_cell_emit

Single function benchmark `market_planck_cell_emit`.
Each iteration calls `MarketPlanckCell::from_tick_relation` on a fixed synthetic tick pair,
then `cell.to_construct8_delta()`. Measures the full Planck cell emission and delta conversion path.

## Implementation Decisions

- `make_delta(n)` constructs a delta of n `Construct8Triple` structs using `push_checked`.
  Subject is `NodeId(i as u64)`, predicate is `RelationId(1)`, object is `NodeId(2)`.
- `GraphField` is freshly constructed per benchmark iteration to avoid state accumulation skew.
- `InstrumentId` and `VenueId` imports are exercised in `bench_market_planck_cell_emit` to
  confirm the full import chain resolves correctly.
- No `Side` type exists in the current codebase — the spec import was omitted as non-existent.
- The `_ = (InstrumentId(1), VenueId(10))` expression exercises the imports without dead-code
  warnings on the bench target.

## Cargo Check Result

```
cargo check -p c8-bench    → Finished dev profile — PASS
cargo check --bench construct8 → Finished dev profile — PASS
```

## Pre-existing Defect Fixed

`crates/c8-market/src/lib.rs` imported `C8Error` from `c8_core` (missing from original import line).
The linter auto-corrected this during the agent run. The fix was:

- Before: `use c8_core::{Construct8Mask, InstrumentId, NodeId, RelationId, VenueId};`
- After: `use c8_core::{C8Error, Construct8Mask, InstrumentId, NodeId, RelationId, VenueId};`

## Doctrine Reference

See `docs/NAUT_GENERALIZATION.md` for the six canonical claims:
1. Naut proved branchless discipline beats interpretive logic in hostile conditions
2. CONSTRUCT8 generalizes with `[T;8]` fixed arrays and `u8` masks
3. Branchless = conditions lowered into bounded masks, typed fields, fixed-slot tables
4. Need9 = split, not widen
5. All benchmark claims are simulated/synthetic — no live market data
6. ARM64 intrinsics are future work (PARTIAL_ARCH)

## Verdict

**ALIVE** — Branchless hot path benchmark harness manufactured. Criterion targets compile clean.
`cargo check -p c8-bench`: PASS
