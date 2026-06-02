# Naut Branchless Hot-Path Contract

## What Naut proved (from docs/public knowledge)

NautilusTrader ARM64 engine: branchless discipline outperforms interpretive logic
in hostile live-market conditions. Fixed-width SIMD over packed arrays eliminates
branch-prediction pressure at CPU level.

Specifically documented in NAUT_GENERALIZATION.md (construct8-market-physics/docs):

- Conditional branches in hot paths cause nanosecond-scale pipeline stalls under adversarial
  tick conditions (crossed markets, liquidity collapse, settlement lock)
- Branch tree fans out unpredictably under hostile conditions, destroying cache locality and
  defeating the CPU branch predictor
- Naut proved this through latency profiling across sustained hostile market replays
- The fix: remove logic-chaos from the hot path entirely, not by adding threads or tuning GC

## What CONSTRUCT8 generalizes from Naut

| Naut principle | CONSTRUCT8 realization |
|---|---|
| Fixed-width SIMD over packed arrays | `[Option<Construct8Triple>; 8]` — fixed layout, no heap |
| Mask-based selection replaces conditionals | `u8` bitmask encodes slot occupancy; `(mask >> slot) & 1` replaces `if occupied[slot]` |
| Bounded loop width prevents predictor explosion | All hot-path loops bounded by `mask.count_ones()` (single `POPCNT` instruction) |
| State fits predictable memory region | Fixed enum variants in `MarketPlanckCell` — 5 state dimensions, not a dynamic key-value map |
| Typed newtypes for zero-cost reasoning | `NodeId`, `RelationId` — compiler reasons about representation without runtime checks |

### Need9 = split, not widen

When an operation requires more than 8 elements, CONSTRUCT8 returns
`Err(Construct8Refusal::NeedNine)`. This is an architectural signal: decompose the operation
into multiple deltas of at most 8 triples, not a reason to widen to `[T; 16]` or `Vec<T>`.

Widen = surrender of the branchless contract.

This mirrors the Naut discipline: when the hot path would need to widen, the correct response
is structural decomposition upstream — not widening the lane.

## What is hardware-specific and NOT yet measured

- **ARM64 NEON intrinsics**: PARTIAL_ARCH — explicit intrinsics (`vceqq_u8`, `vandq_u8`) are
  not yet implemented in c8-*. Compiler auto-vectorization is relied on instead.
- **M-series specific throughput**: NOT MEASURED — no ARM64 hardware benchmark receipt exists.
  On ARM64 (Apple Silicon, AWS Graviton), compiler may emit efficient code but this is not
  guaranteed by explicit intrinsic constraints.
- **Sub-nanosecond claims**: DISQUALIFIED until a local hardware benchmark receipt is produced.
  Current benchmarks are synthetic and do not constitute live-market latency evidence.

NOTE: The previous version of this file contained specific measured benchmark numbers
(e.g., "48.15 ns", "12.73 ns") attributed to a 2026-06-02 Criterion run. No hardware
benchmark receipt exists to verify these numbers. Those figures are UNVERIFIED and have been
removed. They must not be cited as evidence until a real Criterion receipt is produced by
running `cargo make bench` on this machine and capturing the output.

## What IS honestly measured (structure only)

Criterion benchmarks exist in `crates/c8-bench/benches/construct8.rs` (construct8-market-physics):

- `bench_construct8_apply` — `branchless_mask` group, delta sizes 1, 2, 4, 8 triples
  - Uses `GraphField::apply_construct8(delta)` with `Construct8Delta` at each size
- `bench_market_planck_cell_emit` — `MarketPlanckCell::from_tick_relation` + `to_construct8_delta`
  - Uses synthetic `TickObservation` constructed in-process

All benchmarks use synthetic, deterministic tick observations. No live market feeds, real
order books, or recorded market replays are used. Per NAUT_GENERALIZATION.md section 5:

> "Benchmark results should not be cited as claims about real-market latency. They are
> existence proofs that the branchless implementation executes within the expected nanosecond
> range under synthetic load."

No receipt from a local run of these benchmarks has been captured and stored.
Status: STRUCTURE KNOWN, RESULTS UNVERIFIED.

## Branchless Hot Path Law (from BRANCHLESS_HOT_PATH_LAW.md)

1. Logic is chaos normalized by software culture.
2. Any unbounded state-space mechanism is disqualified from the hot path.
3. Branchless does not mean no conditions anywhere — conditions modeled as state masks.
4. Branchless means conditions are lowered into bounded masks, typed fields, tables, or cold-path decomposition.
5. Need9 means split: any operation needing >8 elements violates the cognitive boundary.
6. LLMs are cold-path manufacturing/explanation surfaces only.
7. Hot-path behavior must be benchmarkable.

## naut repo status

ABSENT at ~/naut on this machine. This contract is based on:
- Documented public discipline in `construct8-market-physics/docs/NAUT_GENERALIZATION.md`
- Documented law in `construct8-market-physics/docs/BRANCHLESS_HOT_PATH_LAW.md`
- Benchmark structure in `crates/c8-bench/benches/construct8.rs`

Integration requiring inspection of the Naut source code cannot proceed until the naut repo
is present locally. Marking this adapter PARTIAL.

Trigger condition: `test -d ~/naut` returns EXISTS.

## Hard Gate

NO sub-nanosecond, nanosecond, or M3-specific claims without a hardware benchmark receipt.
NO ARM64 NEON intrinsic claims until PARTIAL_ARCH is resolved and receipts produced.
NO live-market latency claims from synthetic benchmarks.
NO specific benchmark numbers (ns timings) cited as measured without a stored Criterion receipt.

---

**Status:** PARTIAL
**Reason:** naut repo absent; prior version contained unverified benchmark numbers (removed);
honest claims documented from available sources only
**Last Updated:** 2026-06-01
**Agent:** AGENT_07_NAUT_BRANCHLESS
