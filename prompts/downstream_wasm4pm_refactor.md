# DOWNSTREAM WASM4PM REFACTOR — Authorized Mandate

**Authority Source:** process-intelligence Gap Register Agent  
**Invoke:** ONLY after PROCESS_INTELLIGENCE_ALIVE_001 is sealed  
**Gap Register:** gaps/GAP_REGISTER.md  
**Priority Matrix:** gaps/GAP_PRIORITY_MATRIX.md  
**Red Team Findings:** adversarial/RED_TEAM_FINDINGS_001.md, adversarial/RED_TEAM_FINDINGS_002.md

---

## Authorization Condition

This mandate is authorized for downstream execution ONLY after the `PROCESS_INTELLIGENCE_ALIVE_001` seal is produced. Do not invoke before that seal exists. Invoking before the seal means acting on unverified research claims.

---

## Gap Closure Priority Sequence

Execute in the order below. P1 gaps are sequential dependencies. P2 and P3 gaps are parallelizable within their tier.

---

### P1-GAP007 — Fix WfNet split-brain in wasm4pm-compat (FIRST)

**Effort:** Low  
**Dependency:** Independent — execute before any other gap  
**Completion criterion:** `WfNet::attest_witnessed()` is removed or gated `pub(crate)` / `#[deprecated]`. `WfNetConst<SOUNDNESS>` is the only non-forgeable WF-net attestation surface. Compile-pass fixtures for `WfNetConst<SOUNDNESS>` pass. The forgeable public method is gone.

**Why first:** Low effort, independent, and closes a type-law integrity defect before the bridge work begins. Every subsequent receipt is cleaner if this is resolved first.

---

### P1-GAP001 — Import wasm4pm-compat; thread Admitted types (CRITICAL PATH)

**Effort:** High  
**Dependency:** GAP_007 closed  
**Completion criterion:**

1. `wasm4pm/Cargo.toml` declares `wasm4pm-compat` as a dependency (path or crate reference).
2. All algorithm function signatures accept `Admitted` types sourced from compat, not raw `EventLog`.
3. wasm4pm implements an intake path that consumes `GraduationCandidate` from compat and routes it into the execution engine.
4. The graduation bridge is two-sided: compat can graduate, wasm4pm can receive.
5. A compile-pass test demonstrates the full chain: `Evidence::raw(log).into_admitted()` → wasm4pm algorithm.

**Why critical path:** This unlocks GAP_002, unblocks the type import needed for GAP_003 and GAP_004 return types, and closes Finding 001 (one-sided graduation bridge).

---

### P1-GAP002 — Replace ValidationError(String) with named law Refusal types

**Effort:** Medium  
**Dependency:** GAP_001 closed (compat must be imported before its Refusal types can be used)  
**Completion criterion:**

1. `ValidationError(String)` is removed from wasm4pm's public API surface.
2. Every validation return type uses `Refusal<R, W>` where `R` is a specific named law (e.g. `DanglingEventObjectLink`, `MissingFinalMarking`, `NonSoundWfNet`) and `W` is a witness marker from compat.
3. Bare `InvalidInput` or string-typed catch-alls are absent from the codebase.
4. A compile-fail fixture demonstrates that `Refusal<InvalidInput, _>` does not compile.

---

### P2-GAP003 — Implement Inductive Miner returning TypedProcessTree

**Effort:** High  
**Dependency:** GAP_001 closed (for TypedProcessTree return type from compat)  
**Parallelizable with:** GAP_004, GAP_008  
**Completion criterion:**

1. `InductiveMiner` struct implemented in wasm4pm.
2. Returns `TypedProcessTree` using the `process_tree.rs` types from compat.
3. Output WF-net is sound by construction (Inductive Miner guarantee).
4. Discovery receipt emitted after successful discovery.
5. Alpha Miner is demoted or marked as producing unsound WF-nets.

---

### P2-GAP004 — Implement alignment-based conformance returning Metric\<FITNESS,N,D\>

**Effort:** High  
**Dependency:** GAP_001 closed (for Metric return type from compat)  
**Parallelizable with:** GAP_003, GAP_008  
**Completion criterion:**

1. `AlignmentConformance` struct implemented in wasm4pm.
2. Returns `Metric<FITNESS, N, D>` using the `conformance.rs` types from compat with `Between01<N, D>` bounds enforced at the type level.
3. Token replay is demoted to a fast-approximation fallback, not the primary conformance surface.
4. A compile-fail fixture demonstrates that a conformance result outside [0,1] does not compile.

---

### P2-GAP008 — Replace E0425 fixtures with structural law fixtures

**Effort:** Medium  
**Dependency:** GAP_001 closed (so correct import paths are available)  
**Parallelizable with:** GAP_003, GAP_004  
**Completion criterion:**

1. All compile_fail fixtures in `tests/ui/compile_fail/` fail on structural error codes: E0308, E0599, E0277, or E0080.
2. No fixture fails on E0425 (type not found) or E0432 (import not found).
3. Every fixture has a `.stderr` file matching the actual compiler diagnostic.
4. A fixture failing for the wrong error code is treated as a defect (same as a compile_fail fixture that accidentally passes).

---

### P3-GAP005 — Implement OCPQ evaluation engine

**Effort:** Medium  
**Dependency:** Benefits from GAP_001 (admitted OCEL logs flow in); not hard-blocked  
**Parallelizable with:** GAP_006  
**Completion criterion:**

1. `OcpqEvaluator` struct implemented in wasm4pm.
2. Accepts admitted OCEL logs (via compat `Admitted` type after GAP_001).
3. Evaluates OCPQ structural shapes from compat `ocpq.rs` against the log.
4. Returns query results with named refusal if structural shapes are violated.

---

### P3-GAP006 — Implement POWL discovery (PowerMiner / mineDG)

**Effort:** Medium  
**Dependency:** Benefits from GAP_001 (admitted logs); not hard-blocked  
**Parallelizable with:** GAP_005  
**Completion criterion:**

1. `PowerMiner` (or `MineDg`) struct implemented in wasm4pm.
2. Returns `TypedPowl` using the `powl.rs` types from compat.
3. The sealed `TreeProjectable` trait from compat is used correctly.
4. A compile-pass fixture demonstrates a lawful `TypedPowl` artifact.

---

## Gap Sequence Summary

```
GAP_007 (independent, low effort — execute first)
    ↓
GAP_001 (critical path — unlocks everything below)
    ↓
GAP_002 (depends on GAP_001)

After GAP_001:
  GAP_003 ──┐
  GAP_004 ──┤── parallel
  GAP_008 ──┘

After GAP_002:
  GAP_005 ──┐── parallel
  GAP_006 ──┘
```

---

## Blocking Claims (per Red Team Finding 010)

Until each gap is closed, the following claims must carry `[BLOCKED: GAP_NNN]`:

| Claim | Blocked By |
|-------|------------|
| Process evidence admitted through compat flows into wasm4pm algorithms | GAP_001 |
| wasm4pm returns named-law refusals | GAP_001, GAP_002 |
| WF-net attestation is non-forgeable | GAP_007 |
| Discovery produces sound process trees | GAP_003 |
| Conformance reports alignment-based fitness | GAP_004 |
| All compile_fail fixtures are true structural law receipts | GAP_008 |
| OCPQ queries execute against admitted OCEL logs | GAP_005 |
| POWL discovery is available | GAP_006 |

---

## Research Authority

- gaps/GAP_REGISTER.md — gap definitions
- gaps/GAP_PRIORITY_MATRIX.md — dependency analysis
- adversarial/RED_TEAM_FINDINGS_001.md — findings 001–007
- adversarial/RED_TEAM_FINDINGS_002.md — findings 008–010 (includes Finding 010 that mandated this sequence)
- /Users/sac/wasm4pm-compat/CLAUDE.md — type-law covenant and ALIVE gate specification
