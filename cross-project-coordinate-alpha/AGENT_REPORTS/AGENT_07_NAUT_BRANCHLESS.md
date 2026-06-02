# Agent 7: Naut Branchless Hot-Path Generalization

**Status:** PARTIAL
**Date:** 2026-06-01
**Agent:** AGENT_07_NAUT_BRANCHLESS

## Mission

Connect Naut branchless discipline to CONSTRUCT8 honestly, working only from documented
sources since ~/naut is absent on this machine.

## Sources Consulted

- `/Users/sac/process-intelligence/construct8-market-physics/docs/NAUT_GENERALIZATION.md` — EXISTS, full content read
- `/Users/sac/process-intelligence/construct8-market-physics/docs/BRANCHLESS_HOT_PATH_LAW.md` — EXISTS, full content read
- `/Users/sac/process-intelligence/construct8-market-physics/crates/c8-bench/benches/construct8.rs` — EXISTS, full content read
- `rg` search for Naut/branchless/ARM64/SIMD across process-intelligence — returned no results
- `~/naut` — ABSENT

## Findings

### What is grounded

1. **Naut's core insight**: Branchless discipline (remove dynamic allocation, pointer chasing,
   and speculative control flow from the hot path) outperforms interpretive logic under hostile
   market conditions. Documented in NAUT_GENERALIZATION.md with specific mechanisms: branch
   prediction misses, cache locality destruction, predictor defeat under adversarial ticks.

2. **CONSTRUCT8 generalization is structurally complete**: The four elements —
   `[Option<Construct8Triple>; 8]` fixed arrays, `u8` bitmask slot tracking, typed newtypes
   (`NodeId`, `RelationId`), and `NeedNine` refusal — are all directly traceable to Naut's
   discipline as documented.

3. **Benchmark structure exists**: `crates/c8-bench/benches/construct8.rs` contains
   `bench_construct8_apply` (branchless_mask, sizes 1/2/4/8) and `bench_market_planck_cell_emit`.
   The benchmark code is real and reviewable.

4. **Branchless Hot Path Law**: 7-law enumeration exists and is coherent with Naut's principles.

### What is NOT grounded

1. **~/naut absent**: Cannot inspect Naut source code, cannot verify direct code-level
   connections, cannot confirm the Naut latency profiling methodology beyond what NAUT_GENERALIZATION.md
   describes.

2. **No hardware benchmark receipt**: No Criterion run has been captured on this machine.
   Prior version of the adapter contract contained specific ns timings (48.15 ns, 12.73 ns,
   etc.) attributed to a 2026-06-02 run — these had no receipt backing and were removed as
   unverified.

3. **ARM64 NEON intrinsics**: PARTIAL_ARCH status confirmed. No explicit intrinsic bindings
   exist in c8-*. Compiler auto-vectorization only.

4. **M-series throughput**: NOT MEASURED. No ARM64 hardware benchmark receipt.

### Correction applied

The adapter file `naut_hotpath_contract.md` previously contained a fabricated benchmark table
with specific ns measurements and a "VERIFIED & LAWFUL" verdict. These claims had no receipt
backing. This agent removed them and replaced with honest structural documentation only.

## Adapter Written

`/Users/sac/process-intelligence/cross-project-coordinate-alpha/adapters/naut_hotpath_contract.md`

Sections:
- What Naut proved (from docs/public knowledge)
- What CONSTRUCT8 generalizes from Naut (table form)
- What is hardware-specific and NOT yet measured
- What IS honestly measured (structure only, no unverified numbers)
- Branchless Hot Path Law (verbatim from source)
- naut repo status
- Hard Gate

## Hard Gates Enforced

- NO sub-nanosecond or nanosecond claims without hardware benchmark receipt
- NO ARM64 NEON intrinsic claims until PARTIAL_ARCH resolved
- NO live-market latency claims from synthetic benchmarks
- NO specific benchmark timings cited as measured without stored Criterion receipt

## Status Rationale

PARTIAL because:
1. naut repo is absent — source-level inspection impossible
2. No hardware benchmark receipt has been produced
3. ARM64 NEON intrinsics are PARTIAL_ARCH

The generalization doctrine is internally consistent and grounded in the available docs.
The connection is real and honest within the limits of available evidence.
