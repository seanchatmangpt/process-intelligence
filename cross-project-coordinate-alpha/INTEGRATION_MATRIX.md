# Cross-Project Integration Matrix

**Authority:** AGENT_10 Integration Conductor
**Date:** 2026-06-01
**Status:** ALIVE

---

| Project | Role | Consumes | Emits | Must Not Own | Adapter Surface | Validation | ALIVE |
|---|---|---|---|---|---|---|---|
| construct8-market-physics | Witness/Proof surface | synthetic ticks | Planck cells, receipts, deltas | live trading, LLM runtime | c8-* crates | cargo test (38/38) | ALIVE |
| ggen | Ontology manufacturing | .ttl/.rq/.tera | rendered artifacts, SPARQL receipts | delta engine, market state | ggen_construct8_contract.md | cargo check | ALIVE |
| open-ontologies | Public ontology layer | standards (OCEL, RDF, SHACL) | public TTL surfaces, SharedReceiptV1 | private actuation | phd_publication_contract.md | TTL validate (607 tests) | ALIVE |
| wasm4pm | Process evidence engine | OCEL logs | conformance reports, Evidence<T,Executed,Receipt> | market instruments | wasm4pm_evidence_contract.md | cargo test | ALIVE |
| wasm4pm-compat | Type-law compatibility | type laws from c8-* | witness lattices, MarketPlanckCell | mining/replay engine | compat boundary only | compat tests | PARTIAL |
| truex | Consequence/coordination | receipted states (CONSTRUCT8 deltas) | admitted motions, BLAKE3-sealed OCEL 2.0 proofs | brokers/wallets | truex_receipt_contract.md | MANIFESTO tests | PARTIAL |
| pcp | Post-Cyberpunk framing | proof receipts | frame artifacts | trading logic | BRD integration | docs review | PARTIAL |
| naut | Branchless hot-path | ARM64 discipline (documented) | performance benchmarks | live market feeds | naut_hotpath_contract.md | ABSENT (repo missing) | PARTIAL |
| phd-thesis | Dissertation | all public docs from corpus | published theorem set | private IP | phd_publication_contract.md | redaction check | ALIVE |
| process-intelligence | Research foundry | all research sources | doctrine, checkpoints, receipts | production deployment | research boundary | audit | ALIVE |
| ggen-mcp | MCP integration | ggen pipeline | MCP-served artifacts | direct market state | ggen membrane | cargo test | ALIVE |
| ggen-spec-kit | Spec generation | JTBD ontologies | spec artifacts | execution engine | TTL shapes | poe test | PARTIAL |
| knhk | SyncEngine substrate | CONSTRUCT8 deltas | compiled output | receipt forgery | genesis-construct8 crate | cargo test (25 tests) | PARTIAL |
| compiled-cognition-hub | Cognition coordination | reasoning inputs | coordination signals | live trading | hub boundary | none present | PARTIAL |

---

## Verdict Summary

| Status | Count | Projects |
|---|---|---|
| ALIVE | 7 | construct8-market-physics, ggen, open-ontologies, wasm4pm, ggen-mcp, phd-thesis, process-intelligence |
| PARTIAL | 7 | wasm4pm-compat, truex, pcp, naut, ggen-spec-kit, knhk, compiled-cognition-hub |
| BLOCKED | 0 | — |

---

## Critical Integration Laws

1. **Max-8 lane law** — CONSTRUCT8 processes a maximum of 8 triples per delta. All consuming projects must respect this bound.
2. **No unreceipted admission** — Blue River Dam requires a BLAKE3 receipt at every admission gate.
3. **GenesisAdapter is the only crossing point** — ggen external I/O must cross into Genesis exclusively via `GenesisAdapter` trait. No serde_json, no String, no external types cross into Genesis.
4. **Need9 split before truex** — Refusals must be resolved inside CONSTRUCT8 before reaching the truex consequence boundary.
5. **Naut PARTIAL_ARCH** — ARM64 NEON intrinsics are documented as PARTIAL. No claims about intrinsic bindings until naut repo is verified on this machine.
6. **Truex 7,066 uncommitted** — Truex is the highest operational risk. ALIVE verdict blocked until receipts and tests created.
7. **knhk 16,876 uncommitted** — Second-highest operational risk. SyncEngine compilation errors need merge before ALIVE.
8. **wasm4pm-compat nightly-only** — No stable Rust build target. Graduation to stable required before full ALIVE.
