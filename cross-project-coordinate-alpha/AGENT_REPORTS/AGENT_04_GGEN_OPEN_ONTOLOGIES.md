# Agent 04: ggen + Open Ontologies Adapter Boundary

**Date:** 2026-06-01
**Agent:** 04 — ggen + Open Ontologies Adapter Boundary
**Status:** COMPLETE

---

## Mission

Define what ggen and open-ontologies can safely emit and consume with respect to CONSTRUCT8.

---

## Findings

### ggen Identity

ggen v26.5.28 is a deterministic, language-agnostic code generation engine that transforms RDF ontologies into typed source artifacts. Its pipeline has five stages (μ₁-μ₅): ontology normalization, SPARQL extraction, Tera template rendering, canonicalization, and cryptographic receipt generation. It validates every generation through 8 Canonical Proof Gates.

The `genesis-construct8` crate (`knhk-construct8`, located at `/Users/sac/ggen/crates/genesis-construct8/`) is the CONSTRUCT8 delta engine within the ggen workspace. It is a separate crate with its own binary (`knhk8`) and owns delta execution, triple materialization (8 triples at a time), and Receipt structs (BLAKE3 hash + packet/triple counts).

### Key Boundary: GenesisAdapter Trait

The `ggen/crates/ggen-membrane/src/lib.rs` `GenesisAdapter` trait is the ONLY legal crossing point between ggen and Genesis:
- ggen owns contact with the external world (JSON, RDF, APIs)
- Genesis owns consequence (pure A = μ(O))
- No serde_json, no String, no external types cross into Genesis
- Every crossing must produce a Construct8 act bound to a receipt

### Key Boundary: ggen-integration-law.ttl

The file `/Users/sac/open-ontologies/ontology/ggen-integration-law.ttl` formally states the manufacturing contract: ggen ALWAYS loads the ontology, executes SPARQL, renders templates, validates artifacts, and emits receipts. No shortcuts. SHACL failure halts the pipeline.

### Open Ontologies TTL Inventory

352 TTL files found. Key surfaces for ggen:
- **Admissible for rendering:** `public-alignment.ttl`, `public-shapes.ttl`, `shared-receipt-shapes.ttl`, `cell8-core.ttl`, `cell8-shapes.ttl`, `cell8-manufacturing.ttl`, `cell8-profile.ttl`, `cell8-conformance-shapes.ttl`, `ggen-integration-law.ttl`, `cli-open-ontologies.ttl`, `powl-process-mining.ttl`, `wasm4pm-stubs.ttl`, `ghf-core.ttl`, `requirements.ttl`
- **NOT admissible:** `aat-live-rules.ttl` (live actuation), `mcpp-proof-chain.ttl` (mcpp-owned), `ontostar-wasm4pm-integration.ttl` (cross-system state), `revops-manufacturing.ttl` (actuation state)

All admissible files use only public standard namespaces (schema.org, PROV-O, DCAT, SKOS, ODRL, EARL, SHACL, SPDX) or stable project-internal namespaces (`ggen:`, `onto:`, `cli:`).

### Three Surfaces Confirmed

1. **Tera-rendered artifacts from ontology queries** — ggen's core output surface; public-standard bound; governed by `ggen-integration-law.ttl`
2. **SPARQL ASK validation receipts** — `SharedReceiptV1` JSON format validated against `shared-receipt-shapes.ttl`; emitted only after all 8 proof gates pass
3. **GenesisAdapter boundary** — the only legal path from ggen's world into genesis-construct8's delta execution world

---

## What MUST NOT Happen

1. ggen must not become a process miner (no OCEL derivation, no pm4py calls)
2. open-ontologies TTL files must not embed private actuation mechanics
3. .ggen source surfaces must not fake receipts (receipts must be BLAKE3 over real artifact bytes)
4. ggen must not execute Construct8 deltas (that is genesis-construct8's law)
5. ggen templates must not embed process mining oracles (fitness/precision scores belong to wasm4pm)

---

## Contract Artifact

Written to: `/Users/sac/process-intelligence/cross-project-coordinate-alpha/adapters/ggen_construct8_contract.md`

---

## Evidence Sources

- `/Users/sac/ggen/README.md` — ggen v26.5.28 pipeline specification
- `/Users/sac/ggen/ggen.toml` — standard_only=true enforcement
- `/Users/sac/ggen/crates/ggen-membrane/src/lib.rs` — GenesisAdapter boundary trait
- `/Users/sac/ggen/crates/genesis-construct8/src/receipt.rs` — Receipt struct (BLAKE3)
- `/Users/sac/ggen/crates/genesis-construct8/src/bin/genesis8.rs` — delta engine CLI
- `/Users/sac/open-ontologies/ontology/ggen-integration-law.ttl` — manufacturing contract
- `/Users/sac/open-ontologies/ontology/public-alignment.ttl` — public vocab alignment
- `/Users/sac/open-ontologies/ontology/cell8-core.ttl` — Cell8 conformance profile
- `/Users/sac/open-ontologies/ontology/cell8-manufacturing.ttl` — 13 manufacturing gates
- `/Users/sac/open-ontologies/ontology/shared-receipt-shapes.ttl` — SharedReceiptV1 SHACL shapes