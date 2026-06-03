# GAP REGISTER — Master Gap Index

**Status:** Completed  
**Scope:** wasm4pm-compat ↔ wasm4pm integration gaps, type-law quality gaps  
**Maintained by:** process-intelligence Gap Register Agent

---

## GAP_001 — CRITICAL — compat-wasm bridge missing

**Severity:** CRITICAL  
**Status:** Closed

wasm4pm does NOT import wasm4pm-compat. Two parallel type universes exist with no connection.

- wasm4pm accepts raw `EventLog` in all algorithm functions
- Graduation bridge in compat is one-sided (no intake in wasm4pm)
- `GraduationCandidate` is defined in compat but wasm4pm has no function that accepts it
- Process evidence admitted through compat type law is thrown away at the wasm4pm boundary

**Resolution:** Closed. Added `wasm4pm-compat` dependency to `wasm4pm`'s `Cargo.toml`. Changed algorithm signatures to accept `Admitted` types sourced from compat. Implemented `GraduateToWasm4pm` trait and added an intake path in wasm4pm to consume `GraduationCandidate`.

---

## GAP_002 — CRITICAL — named law refusals missing in wasm4pm

**Severity:** CRITICAL  
**Status:** Closed

`ValidationError(String)` is not a named law refusal. It carries no structural law identity, no witness type, and no specific reason type. This makes wasm4pm non-conformant with the compat type-law covenant.

**Current state:** wasm4pm returns `ValidationError(String)` across its validation surfaces.  
**Required state:** Named `Refusal<R, W>` where `R` is a specific structural law (e.g. `DanglingEventObjectLink`, `MissingFinalMarking`) and `W` is a witness marker.

**Resolution:** Closed. Replaced generic `ValidationError(String)` with named `Refusal` types. The bridge in GAP_001 enables import of these types, ensuring compile-time and runtime conformity.

---

## GAP_003 — MAJOR — Inductive Miner missing from wasm4pm

**Severity:** MAJOR  
**Status:** Closed

Alpha Miner is present in wasm4pm but produces unsound WF-nets. Inductive Miner provides the soundness guarantee that Alpha Miner lacks. Without Inductive Miner, wasm4pm cannot produce process trees with guaranteed soundness.

**Gap:** No `InductiveMiner` implementation exists. No `TypedProcessTree` output from discovery.

**Resolution:** Closed. Implemented `InductiveMiner` in wasm4pm, returning a `TypedProcessTree` from compat to guarantee block-structured soundness.

---

## GAP_004 — MAJOR — alignment-based conformance missing from wasm4pm

**Severity:** MAJOR  
**Status:** Closed

Token replay is present in wasm4pm but alignment-based conformance is the superior method. Token replay produces a fitness approximation; alignment produces an exact optimal-alignment fitness with precision.

**Gap:** No `AlignmentConformance` implementation. No `Metric<FITNESS, N, D>` output from conformance checking.

**Resolution:** Closed. Implemented `AlignmentConformance` in wasm4pm returning a `Metric<FITNESS, N, D>` under `Between01` bounds from the compat crate.

---

## GAP_005 — MAJOR — OCPQ evaluation missing from wasm4pm

**Severity:** MAJOR  
**Status:** Closed

OCPQ structural shapes are defined in wasm4pm-compat `ocpq.rs`. There is no execution engine in wasm4pm that evaluates OCPQ queries against an object-centric event log.

**Gap:** `OcpqEvaluator` does not exist in wasm4pm. The structural types exist in compat but have no runtime counterpart.

**Resolution:** Closed. Implemented the `OcpqEvaluator` query engine in wasm4pm to execute query shapes against admitted `Ocel20` logs.

---

## GAP_006 — MINOR — POWL discovery missing from wasm4pm

**Severity:** MINOR  
**Status:** Closed

POWL types are defined in wasm4pm-compat `powl.rs`. No `mineDG` miner or equivalent POWL discovery algorithm exists in wasm4pm.

**Gap:** No `PowerMiner` or equivalent. No path from an event log to a `TypedPowl` artifact.

**Resolution:** Closed. Implemented `PowerMiner` returning a sealed `TypedPowl` that conforms to the `TreeProjectable` trait.

---

## GAP_007 — MINOR — WfNet split-brain in wasm4pm-compat

**Severity:** MINOR  
**Status:** Closed

`WfNet::attest_witnessed()` is forgeable (the method is public). `WfNetConst<SOUNDNESS>` is sealed (correct). The two exist in the same crate with contradictory forgeability guarantees.

**Gap:** A caller can call `WfNet::attest_witnessed()` and obtain an attested WF-net without going through the lawful soundness verification path. This undermines the type-law receipt claim for WF-net soundness.

**Resolution:** Closed. Made `WfNet::attest_witnessed()` `pub(crate)` and marked it as `#[deprecated]`, enforcing use of `WfNetConst<SOUNDNESS>` as the correct non-forgeable surface.

---

## GAP_008 — MINOR — E0425 absence-proof fixtures in wasm4pm-compat

**Severity:** MINOR  
**Status:** Closed

Some `compile_fail` fixtures in `tests/ui/compile_fail/` fail because a type is not found (E0425 — cannot find type in scope), not because a law structurally prevents the operation. An E0425 failure is an import error, not a type-law receipt.

**Gap:** Fixtures failing on E0425 are not valid type-law receipts. They would pass if someone added the correct `use` statement, regardless of the law's intent.

**Resolution:** Closed. Replaced all E0425-failing compile-fail fixtures with structural fixtures failing on type-law diagnostics (E0308, E0599, E0277, E0080).

---

## Summary Table

| GAP | Severity | Status | Priority |
|-----|----------|--------|----------|
| GAP_001 | CRITICAL | Closed | P1 |
| GAP_002 | CRITICAL | Closed | P1 |
| GAP_003 | MAJOR | Closed | P2 |
| GAP_004 | MAJOR | Closed | P2 |
| GAP_005 | MAJOR | Closed | P3 |
| GAP_006 | MINOR | Closed | P3 |
| GAP_007 | MINOR | Closed | P1 |
| GAP_008 | MINOR | Closed | P2 |

---

## FIRMAMENT_002 Gap Closure Sweep — 2026-06-03

**Sweep Date:** 2026-06-03  
**Agent:** Gap Closure Sweep Agent  
**Branch:** phd-thesis-corpus-manufacture-001

This sweep closed or classified all outstanding FIRMAMENT_002 project-level gaps across
the process-intelligence firmament. Each gap has an individual document in `gaps/`.

### FIRMAMENT_002 Gap Summary Table

| Gap ID | Project | Gate | Severity | Final Status | Closure Commit |
|--------|---------|------|----------|-------------|----------------|
| FIRMAMENT_002_GGEN | ggen | Dung Gate | BLOCKING | Closed | 01cc8b0 (process-intelligence) |
| FIRMAMENT_002_LIVING_LSP_GALL_CODEMANUFACTORY | living-lsp-gall | Inspection Gate | MAJOR | Closed | b7151f6 (process-intelligence) |
| FIRMAMENT_002_KNOWLEDGE_HOOKS_TRUEX | truex | Sheep Gate | BLOCKING | External Action Required | 65a614c (process-intelligence) |
| FIRMAMENT_002_NEHEMIAH_52 | nehemiah-52 | Fish Gate | BLOCKING | Closed | bf9146d (process-intelligence) |
| FIRMAMENT_002_PROCESS_INTELLIGENCE_CORE | process-intelligence | Fountain Gate | MAJOR | Closed | 6bb6d7fb (process-intelligence) |
| FIRMAMENT_002_WASM4PM_COMPAT | wasm4pm-compat | Horse Gate | BLOCKING | Closed | 4bb57f4 (process-intelligence) |
| FIRMAMENT_002_PROMPT_MANUFACTORY | prompt-manufactory | Water Gate | BLOCKING | Closed | b0e1ae8 (process-intelligence) |
| FIRMAMENT_002_LINKEDIN_PUBLIC_CANON | linkedin-public-canon | Fish Gate | BLOCKING | External Action Required | 27b8c02 (process-intelligence) |
| FIRMAMENT_002_BLUE_RIVER_DAM | blue-river-dam | Old Gate | MAJOR | Closed | b8c213e (process-intelligence) |
| FIRMAMENT_002_CONSTRUCT8 | construct8 | Horse Gate | MAJOR | Closed | dcc627ec (construct8) |
| FIRMAMENT_002_WASM4PM | wasm4pm | Inspection Gate | MAJOR | Closed | ba1d9118 (wasm4pm) |

### External Action Required Summary

Two gaps require author action that cannot be automated:

**FIRMAMENT_002_KNOWLEDGE_HOOKS_TRUEX** — GAP_003: All 130 CLI receipts in
`/Users/sac/truex/.truex/receipts/` remain refused. The truex CLI project-cell
initialization fails with "Missing project path" / "Project cell not initialized".
Requires operator to fix the CLI project-cell registry and run `truex init` then
`truex prove` to produce at least one `status:admitted` receipt.

**FIRMAMENT_002_LINKEDIN_PUBLIC_CANON** — GAP_003, GAP_004, GAP_005, GAP_007: All
local artifacts are manufactured (post drafts, landing page HTML, newsletter draft,
series definition). Publication requires authenticated browser actions: publish POST_001
to LinkedIn, deploy landing page to GitHub Pages, create and publish newsletter on
Substack/Beehiiv/Ghost, and record all public URLs in `PUBLICATION_REGISTRY.yaml`.
