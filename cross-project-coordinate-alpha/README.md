# Cross-Project Coordinate Alpha

## Mission

Coordinate-system alpha coordination surface for Sean Chatman research corpus.
This workspace is the integration conductor for the full research ecosystem:
construct8-market-physics, ggen, open-ontologies, wasm4pm, wasm4pm-compat,
truex, pcp, naut, phd-thesis, and process-intelligence.

---

## Status: ALIVE (with PARTIAL sub-components)

| Component | Status | Notes |
|---|---|---|
| construct8-market-physics | ALIVE | 38/38 tests, 4/4 demos, Need9 enforced, BLAKE3 receipts |
| ggen | ALIVE | v26.5.28, GenesisAdapter boundary enforced |
| open-ontologies | ALIVE | 607 tests, SharedReceiptV1, public namespaces only |
| wasm4pm | ALIVE | 60 algorithms, OCEL 2.0 native |
| phd-thesis | ALIVE | IP boundary defined, redaction rules enforced |
| process-intelligence | ALIVE | Research foundry, doctrine immutable |
| wasm4pm-compat | PARTIAL | Nightly-only, no stable build target |
| truex | PARTIAL | 7,066 uncommitted files, no tests, no receipts |
| pcp | PARTIAL | No tests |
| naut | PARTIAL | Repo absent on this machine, PARTIAL_ARCH for ARM64 NEON |
| ggen-spec-kit | PARTIAL | 43 modified files |
| knhk | PARTIAL | 16,876 uncommitted files, compilation errors |
| compiled-cognition-hub | PARTIAL | No tests, no receipts |

---

## Construct8 Witness

**ALIVE** at `~/process-intelligence/construct8-market-physics`

- 38/38 tests pass (upgraded from 35 to 38 in Agent 03 audit)
- 4/4 demos execute to completion
- Need9 type system enforced (max-8 triple bound at construction)
- BLAKE3 receipts verified (hash, state transition, tamper detection)
- RepresentationGap proven (gap score = 2: LogicPlayer vs GraphPlayer)
- No live trading dependencies (reqwest/broker/exchange absent from crates)
- No LLM runtime dependencies (openai/anthropic/llm absent from runtime crates)

---

## Adapter Contracts (5)

All 5 adapter contracts are in `adapters/`:

1. `ggen_construct8_contract.md` — ggen + open-ontologies ↔ genesis-construct8
2. `wasm4pm_evidence_contract.md` — wasm4pm-compat ↔ wasm4pm full engine
3. `truex_receipt_contract.md` — construct8 → truex → Blue River Dam
4. `naut_hotpath_contract.md` — Naut branchless ↔ CONSTRUCT8 hot-path
5. `phd_publication_contract.md` — Research corpus → PhD / public papers IP boundary

---

## Receipts

All receipts in `receipts/`:

- `census_receipt.yaml` — Cross-project census (16 projects queried, 14 found)
- `construct8_witness_receipt.yaml` — Construct8 ALIVE_001 witness audit
- `validation_receipt.yaml` — No-live-trading PASS, no-runtime-LLM PASS
- `ip_boundary_receipt.yaml` — Public IP boundary sealed
- `integration_receipt.yaml` — Adapter boundaries defined
- `doctrine_receipt.yaml` — Doctrine alignment verified
- `ggen_contract_receipt.yaml` — ggen adapter contract sealed
- `truex_contract_receipt.yaml` — Truex receipt contract sealed
- `alive_receipt.yaml` — Coordinate-system alpha ALIVE verdict

---

## Validation Scripts

All scripts in `scripts/`:

```
./scripts/validate_cross_project.sh    # Orchestrate all checks
./scripts/check_no_live_trading.sh     # Scan for live financial trading
./scripts/check_no_runtime_llm.sh      # Scan for runtime LLM API calls
./scripts/check_public_ip_boundary.sh  # Scan public docs for private IP
./scripts/emit_receipts.sh             # Regenerate all receipts
./scripts/census.sh                    # Rerun cross-project census
```

---

## To Rerun

```bash
cd ~/process-intelligence/cross-project-coordinate-alpha
./scripts/validate_cross_project.sh
./scripts/check_no_live_trading.sh
./scripts/check_no_runtime_llm.sh
./scripts/emit_receipts.sh
```

---

## Critical Risks

1. **truex** — 7,066 uncommitted files, no tests, no receipts. Highest operational risk.
2. **knhk** — 16,876 uncommitted files, compilation errors. Second-highest operational risk.
3. **naut** — Repo absent on this machine. PARTIAL_ARCH for ARM64 NEON intrinsics.
4. **wasm4pm-compat** — Nightly Rust only. No stable build target.

---

## Immutability Note

`doctrine/` files in process-intelligence are immutable — never rebase, only addend.
Checkpoint files (ALIVE/PARTIAL/FAILED verdicts) are permanent as issued.
