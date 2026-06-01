# GAP PRIORITY MATRIX — Dependency and Sequencing Analysis

**Status:** Active  
**Source:** GAP_REGISTER.md  
**Purpose:** Sequence gap closure by dependency, severity, and enabling value

---

## Priority Matrix

| GAP | Severity | Blocks | Depends On | Enables | Effort | Priority |
|-----|----------|--------|------------|---------|--------|----------|
| GAP_001 | CRITICAL | GAP_002, GAP_003, GAP_004, GAP_005, GAP_006 | — | Provenance chain; type-law admission at wasm4pm boundary | High | P1 |
| GAP_002 | CRITICAL | Board claims requiring named-law refusal evidence | GAP_001 | Named law refusal surface across full stack | Medium | P1 |
| GAP_007 | MINOR | Type-law receipt quality in compat | — | WfNet forgeability eliminated; receipt integrity restored | Low | P1 |
| GAP_003 | MAJOR | Sound WF-net discovery claims | — | `InductiveMiner` returning `TypedProcessTree`; sound WF-net receipts | High | P2 |
| GAP_004 | MAJOR | Alignment conformance claims | — | `AlignmentConformance` returning `Metric<FITNESS,N,D>`; superior fitness | High | P2 |
| GAP_008 | MINOR | Fixture quality; E0425 is not a type-law receipt | — | All compile_fail fixtures are true structural law receipts | Medium | P2 |
| GAP_005 | MAJOR | OCPQ query execution | — | `OcpqEvaluator`; object-centric query surface in wasm4pm | Medium | P3 |
| GAP_006 | MINOR | POWL discovery | — | `PowerMiner` returning `TypedPowl`; POWL discovery surface | Medium | P3 |

---

## Dependency Graph

```
GAP_007 (independent — fix first, low effort)
    |
    v
GAP_001 ──────────────────────────────────────────────────────┐
    |                                                          |
    v                                                          |
GAP_002                                                        |
                                                               |
GAP_003 (independent algorithm, needs compat types) <──────────┤
GAP_004 (independent algorithm, needs compat types) <──────────┤
GAP_008 (independent fixture quality) <────────────────────────┤
GAP_005 (benefits from GAP_001 for admitted OCEL) <────────────┤
GAP_006 (benefits from GAP_001 for admitted logs) <────────────┘
```

---

## P1 Closure Sequence

**Step 1 — GAP_007:** Remove or guard `WfNet::attest_witnessed()`. Low effort, independent. Closes forgeability split-brain before any downstream work begins.

**Step 2 — GAP_001:** Add wasm4pm-compat dependency to wasm4pm `Cargo.toml`. Thread `Admitted` types into all algorithm function signatures. Implement wasm4pm intake path for `GraduationCandidate`. High effort but unlocks the entire P1/P2 queue.

**Step 3 — GAP_002:** Replace `ValidationError(String)` with named `Refusal<R, W>` types. Medium effort, requires GAP_001 to be closed so compat types are importable.

---

## P2 Closure Sequence

GAP_003, GAP_004, GAP_008 are all independent of each other and can proceed in parallel after P1 is closed.

**GAP_003:** Implement `InductiveMiner`. Requires `TypedProcessTree` from compat (needs GAP_001 for the type import, though the algorithm itself is independent).

**GAP_004:** Implement `AlignmentConformance`. Requires `Metric<FITNESS, N, D>` and `Between01` from compat (same dependency profile as GAP_003).

**GAP_008:** Replace E0425 fixtures. Fully independent, can be done before or after GAP_001. Recommended to do after GAP_001 so fixture authors have access to correct import paths.

---

## P3 Closure Sequence

**GAP_005 and GAP_006** are independent of each other and of P2 gaps. They benefit from GAP_001 being closed (so admitted logs flow in) but are not blocked by it for initial implementation.

---

## Critical Path

The critical path to a fully bridged, law-conformant stack is:

```
GAP_007 → GAP_001 → GAP_002
```

Everything else (GAP_003 through GAP_006, GAP_008) is parallelizable after GAP_001.

---

## Blocking Claims

The following board-level and research-level claims are **blocked** until the gaps they depend on are closed:

| Claim | Blocked By |
|-------|------------|
| "Process evidence admitted through compat flows into wasm4pm algorithms" | GAP_001 |
| "wasm4pm returns named-law refusals" | GAP_001, GAP_002 |
| "Discovery produces sound process trees" | GAP_003 |
| "Conformance reports alignment-based fitness" | GAP_004 |
| "OCPQ queries execute against admitted OCEL logs" | GAP_005 |
| "POWL discovery is available" | GAP_006 |
| "WF-net attestation is non-forgeable" | GAP_007 |
| "All compile_fail fixtures are true structural law receipts" | GAP_008 |
