# Naut Generalization and Branchless Hot Path Law

## 1. Naut Proved Branchless Discipline Beats Interpretive Logic in Hostile Conditions

The Naut trading engine demonstrated that latency reduction under hostile market conditions
is not achieved by adding threads, tuning garbage collectors, or parallelizing interpretive
trees. The critical insight is **removing logic-chaos** from the hot path entirely.

In Naut's execution model, conditional branches are a liability: each branch prediction miss
costs nanoseconds of pipeline stall. Under adversarial tick conditions — crossed markets,
liquidity collapse, settlement lock — the branch tree fans out unpredictably, destroying
cache locality and defeating the CPU's branch predictor. Naut proved this definitively through
latency profiling across sustained hostile market replays.

## 2. CONSTRUCT8 Generalizes: [T;8] Fixed Arrays Instead of Vec, u8 Masks Instead of if-chains

CONSTRUCT8 generalizes the Naut discipline to a universal graph delta engine:

- **Fixed arrays**: `[Option<Construct8Triple>; 8]` replaces `Vec<Triple>`. Fixed layout means
  the structure fits within a predictable memory region, enabling compiler loop unrolling and
  preventing heap allocation on the hot path.
- **u8 mask**: A single `u8` bitmask encodes which of the 8 slots are occupied. Masked iteration
  replaces conditional length checks and dynamic dispatch entirely.
- **Typed fields**: Each triple carries `NodeId`, `RelationId` — typed newtypes, not untyped
  integers — so the compiler can reason about representation without runtime checks.
- **Fixed-slot tables**: Market state (`MarketPlanckCell`) uses fixed enum variants (5 state
  dimensions) rather than a dynamic key-value map.

## 3. Branchless = Conditions Lowered into Bounded Masks, Typed Fields, Fixed-Slot Tables

The branchless discipline means:

- Conditional jumps (`if`, `match` with many arms) are **lowered** into arithmetic or mask
  operations wherever possible.
- State transitions use bounded integer arithmetic: `(mask >> slot) & 1` instead of
  `if occupied[slot]`.
- Enum discriminants map directly to integer constants — the compiler emits lookup tables, not
  branch trees.
- All hot-path loops are bounded by the mask population count (`mask.count_ones()`), which is
  a single CPU instruction (`POPCNT`).

This discipline applies uniformly: graph delta application, market Planck cell emission,
vector clock comparison, and event horizon detection all follow the same pattern.

## 4. Need9 = Split, Not Widen

When an operation requires more than 8 elements, CONSTRUCT8 returns
`Err(Construct8Refusal::NeedNine)` (or `C8Error::NeedNine`). This is not an error to be
worked around by widening the array. It is a **architectural signal** that the operation must
be decomposed.

The correct response to `NeedNine` is to split the operation into multiple deltas of at most 8
triples each, applying them sequentially or in parallel as the object lifecycle permits. This
preserves the branchless invariant across the entire call chain: no single delta ever exceeds
the fixed bound.

**Never widen to `[T; 16]` or `Vec<T>` in response to NeedNine.** Widen = surrender of the
branchless contract.

## 5. All Benchmark Claims Are Simulated/Synthetic — No Live Market Data

All benchmarks in `crates/c8-bench/benches/construct8.rs` use synthetic, deterministic tick
observations constructed in-process. No live market feeds, real order books, or recorded market
replays are used. Timing results reflect the execution characteristics of the CONSTRUCT8
implementation under controlled conditions only.

Benchmark results should not be cited as claims about real-market latency. They are existence
proofs that the branchless implementation executes within the expected nanosecond range under
synthetic load.

## 6. ARM64 Intrinsics Are Future Work (PARTIAL_ARCH)

The current implementation relies on the Rust compiler's auto-vectorization and LLVM backend
to emit efficient SIMD or bitwise instructions. Explicit ARM64 NEON intrinsics (e.g.,
`vceqq_u8`, `vandq_u8`) for mask operations are **not yet implemented**.

Status: **PARTIAL_ARCH**

This means:
- On ARM64 (Apple Silicon, AWS Graviton), the compiler may emit efficient code, but this is
  not guaranteed by explicit intrinsic constraints.
- On x86-64, `POPCNT` and bitwise mask instructions are emitted correctly by LLVM.
- Explicit intrinsic bindings are deferred to a future architecture-specific optimization pass.

Until PARTIAL_ARCH is resolved, benchmark results on ARM64 should be treated as indicative
rather than authoritative.
