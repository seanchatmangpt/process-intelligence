# Horse Gate Crosswalk — construct8-market-physics

**Date:** 2026-06-02
**Purpose:** Map firmament-layer gate label to project checkpoint vocabulary.
**Resolves:** GAP_CONSTRUCT8_CAVEAT_002 (GAP_FIRMAMENT_002_CONSTRUCT8)

---

## Crosswalk Table

| Firmament Layer Label | Project Checkpoint Artifact | Status | Notes |
|---|---|---|---|
| Horse Gate | ALIVE_002_INDEPENDENT_REPLAY | PASSED | Receipts: `ALIVE_002_INDEPENDENT_REPLAY_RECEIPT.yaml`, `C8_MARKET_PHYSICS_ALIVE_002.yaml` |
| Horse Gate (ALIVE_003 addendum) | C8_MARKET_PHYSICS_ALIVE_003 | PASSED | Reconciles test count to 48/48; supersedes 35-count in ALIVE_002 receipt |

---

## Terminology Mapping

The firmament orchestration layer (process-intelligence/gaps and audit scripts) uses the label
**"Horse Gate"** to refer to the construct8-market-physics independent replay gate. Within the
project, this gate is documented as:

- Primary receipt: `receipts/ALIVE_002_INDEPENDENT_REPLAY_RECEIPT.yaml`
- Checkpoint YAML: `receipts/C8_MARKET_PHYSICS_ALIVE_002.yaml`
- Addendum receipt: `receipts/C8_MARKET_PHYSICS_ALIVE_003.yaml`

An auditor following a firmament-layer reference to "Horse Gate" for construct8 should resolve
it to the `ALIVE_002_INDEPENDENT_REPLAY` checkpoint family.

---

## Test Count Reconciliation Summary

Three receipts in this project report different test totals. This table documents the authoritative
reconciliation:

| Receipt | Count | Scope | Authority |
|---|---|---|---|
| `ALIVE_002_REPLAY_RECEIPT.yaml` | 35 | Unit tests only (8 crates) | Superseded by ALIVE_003 |
| `construct8_witness_receipt.yaml` | 43 | Unit tests + ablation (counted separately) | Superseded by ALIVE_003 |
| `C8_MARKET_PHYSICS_ALIVE_003.yaml` | 48 | All unit + integration + ablation tests | **AUTHORITATIVE** |
| Live `cargo test --workspace` (2026-06-02) | 48 | As above | Confirms ALIVE_003 |

**Authoritative count: 48/48 tests passing** (as of 2026-06-02 cargo test run)

The 35-count and 43-count receipts were each accurate at their time of issue. They are superseded
by ALIVE_003 which documents the inclusion criteria explicitly and accounts for all test additions
since ALIVE_002.

---

## Resolution Status

| Caveat | Gap ID | Status |
|---|---|---|
| Test count discrepancy (35/41/43) | GAP_CONSTRUCT8_CAVEAT_001 | RESOLVED — ALIVE_003 authoritative at 48 |
| Horse Gate label absent | GAP_CONSTRUCT8_CAVEAT_002 | RESOLVED — this crosswalk file |
| Contradictory receipts (35 vs 43) | GAP_CONSTRUCT8_CAVEAT_003 | RESOLVED — ALIVE_003 supersedes both |

**All three CONSTRUCT8 caveats from GAP_FIRMAMENT_002_CONSTRUCT8 are resolved.**
