---
gap: FIRMAMENT_002_CONSTRUCT8
project: construct8
date: 2026-06-02
status: CLOSED
severity: MAJOR
gate: Horse Gate
closed: 2026-06-02
closure_commit: dcc627ec99770a327bdf65b443c5019e53ba95ce
---

# Gap: construct8

## Summary

The construct8-market-physics project holds an ALIVE_001 verdict and has accumulated two receipt artifacts (ALIVE_002_REPLAY_RECEIPT.yaml and construct8_witness_receipt.yaml) that report contradictory test totals (35/35 vs 43/43), while a live cargo test run produces 41 passing tests — a third distinct count. Additionally, the checkpoint vocabulary used by the orchestrating firmament layer ("Horse Gate") has no internal receipt entry in the project itself, creating a naming mismatch that undermines cross-layer traceability. These discrepancies do not invalidate the existing ALIVE_001 verdict, but they are receipt inconsistencies and must be resolved before any ALIVE_002 or higher checkpoint can be issued with confidence.

## Gap Register

### GAP_CONSTRUCT8_CAVEAT_001 — Test count discrepancy between receipts and live suite

- **Severity:** MAJOR
- **Category:** MISSING_ARTIFACTS
- **Specific Blocker:** Receipts claim 35/35 passing tests; live `cargo test` produces 41 passing tests. The delta of 6 is attributable to two sources: c8-core has 8 live tests vs 6 in the receipt (the two additional are panic tests `mask_set_out_of_bounds_panics` and `mask_has_out_of_bounds_panics` which run live but were excluded from the receipt count), and c8-adversary ablation tests (3) were counted separately from the adversary crate tests (5+3=8 total vs 5 in the receipt). The receipt was authoritative at time of issue but the live suite has diverged.
- **Remediation:** Run the full `cargo test --workspace` suite, enumerate all passing tests by crate, reconcile the 41-test live count against both existing receipts, and issue a corrected receipt that either (a) documents the inclusion criteria used in the 35-count receipt or (b) supersedes it with a new authoritative 41-count receipt. Document which tests are excluded from official counts and why.
- **Effort:** 1–4 hours

### GAP_CONSTRUCT8_CAVEAT_002 — "Horse Gate" label absent from project receipt vocabulary

- **Severity:** MAJOR
- **Category:** MISSING_ARTIFACTS
- **Specific Blocker:** The firmament orchestration layer references this project's readiness gate as "Horse Gate." The project's own receipt vocabulary uses "ALIVE_002_INDEPENDENT_REPLAY" as the checkpoint name. No receipt file, doctrine entry, or crosswalk within the project maps "Horse Gate" to "ALIVE_002_INDEPENDENT_REPLAY." Cross-layer traceability is broken: an auditor following the firmament gate reference cannot resolve it to a project artifact.
- **Remediation:** Add a one-line crosswalk entry to the project's ALIVE_002 receipt or a dedicated `receipts/HORSE_GATE_CROSSWALK.md` file that maps `Horse Gate (firmament) → ALIVE_002_INDEPENDENT_REPLAY (project)`. This is a documentation gap, not a code gap — no test changes required.
- **Effort:** 1–4 hours

### GAP_CONSTRUCT8_CAVEAT_003 — Two receipts within the same project report contradictory test totals

- **Severity:** MAJOR
- **Category:** MISSING_ARTIFACTS
- **Specific Blocker:** `construct8_witness_receipt.yaml` claims 43/43 tests while `ALIVE_002_REPLAY_RECEIPT.yaml` claims 35/35. Both are present in the same project. The 43-count is believed to include ablation crate tests counted separately (43 = 35 + 8), but no receipt documents this arithmetic or the inclusion criteria. A receipt chain with internally contradictory totals cannot be relied upon as proof of conformance.
- **Remediation:** Annotate both receipts with explicit inclusion criteria (which crates and test types are counted). If the 43-count is the more complete figure, mark `ALIVE_002_REPLAY_RECEIPT.yaml` as superseded and issue a reconciliation note. If the 35-count reflects an intentional scope exclusion (e.g., ablation tests are out of scope for ALIVE), document that policy explicitly so the witness receipt can be understood as counting a superset.
- **Effort:** 1–4 hours

## ALIVE Conditions Blocked

The following ALIVE_002 gate conditions cannot be certified until these gaps are closed:

1. **Receipt chain integrity** — ALIVE_002 requires a single authoritative test count that is consistent across all receipts in the project. GAP_CONSTRUCT8_CAVEAT_001 and GAP_CONSTRUCT8_CAVEAT_003 together mean no single authoritative count exists.
2. **Cross-layer traceability** — ALIVE_002 issued under the "Horse Gate" label cannot be traced from the firmament layer to a project artifact. GAP_CONSTRUCT8_CAVEAT_002 blocks this linkage.
3. **Independent replay certification** — An independent auditor replaying ALIVE_002 cannot confirm which test count to expect (35, 41, or 43). This breaks the independent replay requirement.

## Resolution Path

1. Run `cargo test --workspace --no-fail-fast 2>&1 | grep -E "^test .* (ok|FAILED)"` and produce a complete, crate-annotated list of all passing tests.
2. Reconcile the live 41-count against `ALIVE_002_REPLAY_RECEIPT.yaml` (35) and `construct8_witness_receipt.yaml` (43). Identify every test in the delta and document its crate, name, and reason for prior exclusion or separate counting.
3. Issue a corrected receipt (`receipts/ALIVE_002_RECONCILED_RECEIPT.yaml`) that declares the authoritative count, the inclusion criteria, and explicitly supersedes or annotates the prior two receipts.
4. Add `receipts/HORSE_GATE_CROSSWALK.md` mapping `Horse Gate (firmament layer)` → `ALIVE_002_INDEPENDENT_REPLAY (project checkpoint)`.
5. Re-run the ALIVE_002 independent replay against the reconciled receipt to confirm all assertions pass with the corrected count.
6. Issue a `checkpoint` commit declaring `GAP_FIRMAMENT_002_CONSTRUCT8 RESOLVED` with a reference to the reconciled receipt.

## Doctrine Note

A receipt chain that cannot be independently replayed to a single consistent test count is not a receipt — it is a claim, and claims without evidence are inadmissible under the Van der Aalst Constitution.

---

## Resolution Addendum — 2026-06-02

**Status:** CLOSED
**Closure commit:** dcc627ec99770a327bdf65b443c5019e53ba95ce
**[GAP_CLOSURE: GAP_FIRMAMENT_002_CONSTRUCT8]**

All three caveats resolved:

**GAP_CONSTRUCT8_CAVEAT_001 (test count discrepancy):** Live `cargo test --workspace` on 2026-06-02 confirms 48/48 passing tests. Receipt C8_MARKET_PHYSICS_ALIVE_003.yaml (commit b615c24) documents the authoritative per-crate breakdown (c8-adversary 9, c8-core 8, c8-graph 4, c8-instruments 5, c8-market 5, c8-receipts 11, c8-time 6). ALIVE_003 explicitly supersedes the 35-count (ALIVE_002_REPLAY_RECEIPT.yaml) and 43-count (construct8_witness_receipt.yaml) receipts.

**GAP_CONSTRUCT8_CAVEAT_002 (Horse Gate label absent):** Created `construct8-market-physics/receipts/HORSE_GATE_CROSSWALK.md` mapping "Horse Gate (firmament)" → "ALIVE_002_INDEPENDENT_REPLAY (project checkpoint)". Cross-layer traceability is now established.

**GAP_CONSTRUCT8_CAVEAT_003 (contradictory receipt totals):** HORSE_GATE_CROSSWALK.md annotates all prior receipt counts with inclusion criteria and declares ALIVE_003 the single authoritative source. ALIVE_002_REPLAY_RECEIPT.yaml (35 tests) used original crate scope; construct8_witness_receipt.yaml (43 tests) included ablation crate; ALIVE_003 (48 tests) is the complete workspace count.

Verification: `cargo test --workspace` exits 0, 48 tests pass, 0 fail, 0 dirty files post-commit.
