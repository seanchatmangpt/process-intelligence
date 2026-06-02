# ggen + Open Ontologies / CONSTRUCT8 Adapter Contract

**Version:** 26.5.28 (ggen) ↔ knhk-construct8 (genesis-construct8 crate)
**Date:** 2026-06-01
**Agent:** 04 — ggen + Open Ontologies Adapter Boundary
**Status:** AUTHORITATIVE (replaces prior stub)

---

## Executive Summary

ggen is a **deterministic code generation engine** (v26.5.28) that transforms RDF ontologies into typed source artifacts via a five-stage μ-pipeline (μ₁ normalization → μ₂ SPARQL extraction → μ₃ Tera rendering → μ₄ canonicalization → μ₅ receipt emission). The CONSTRUCT8 delta engine lives in the `genesis-construct8` crate (`knhk-construct8`, under `ggen/crates/genesis-construct8/`). This contract establishes the boundary between specification and rendering (ggen's domain) and triple materialization / delta execution (genesis-construct8's domain).

The manufacturing contract between ggen and open-ontologies is formally stated in `open-ontologies/ontology/ggen-integration-law.ttl`. The membrane boundary between ggen and Genesis is formally stated in `ggen/crates/ggen-membrane/src/lib.rs` via the `GenesisAdapter` trait.

---

## What ggen CAN emit

### Core ggen Artifacts (all CONSTRUCT8-safe)

1. **Tera-rendered artifacts from ontology queries (public-standard bound)**
   - Source: SPARQL SELECT/CONSTRUCT over public TTL graphs (schema.org, PROV-O, DCAT, SKOS, ODRL, EARL, SHACL, SPDX, Dublin Core)
   - Governed by `ggen-integration-law.ttl` manufacturing contract
   - All public namespaces; no private actuation mechanics in rendered output

2. **SPARQL ASK validation receipts**
   - Boolean proof of ontology admissibility
   - Format: JSON `SharedReceiptV1` — `schema_version: "shared/v1"`, UUID v4 `run_id`, ISO-8601 timestamps, BLAKE3 hash
   - Validated against `shared-receipt-shapes.ttl` before emission
   - Emitted only after all 8 Canonical Proof Gates pass

3. **Manifest files describing construction acts**
   - `.toml` or `.yaml` manifests recording: ontology source URI, SPARQL query file path, template path, output path, receipt reference
   - For CONSTRUCT8: ggen renders a Construct8Delta *description* as a manifest; delta execution belongs to `genesis-construct8` (the `knhk-construct8` crate)

4. **Source-law artifacts from .rq templates**
   - SPARQL `.rq` query files in `.specify/queries/` tree — canonical extraction queries against `open-ontologies` graphs
   - Rendered Rust stubs, CLI domain types, MCP TypeScript bindings — all sourced from `.rq` + `.tera` combinations
   - Evidence: `ggen/.specify/queries/cli/`, `ggen/examples/mcp-a2a-self-hosting/queries/`

### For CONSTRUCT8 Specifically

ggen can emit:
- **TripleRef type definitions** — rendered from RDF shapes as Rust structs (u32 subject, u32 predicate, u32 object)
- **Construct8Delta manifest descriptions** — `.toml`/`.yaml` describing delta structure; actual compilation by `genesis-construct8`
- **SHACL-validated ontology descriptions** of delta surface — proof that the delta shape is admissible
- **Manufacturing receipts** referencing which ontology graph and template produced which artifact

### What ggen CANNOT emit

- Live runtime deltas — requires `genesis-construct8` crate execution
- Hot-path market state or transaction evaluation logic
- Process discovery or conformance mining results
- Behavioral replay or fitness metrics (these belong to wasm4pm)
- State mutation sequences beyond fixed template descriptions

---

## What ggen CAN validate

- **Ontology consistency (TTL shape checks)** — ggen runs SHACL validation against `open-ontologies` shapes before rendering; failure halts the pipeline
- **Template render correctness** — canonicalized output hash comparison for deterministic projection verification
- **Construct8Delta admissibility (SPARQL ASK against ontology)** — pre-admission gate; result recorded as ASK receipt, not as delta commit

---

## What ggen CANNOT own

Per `ggen-membrane/src/lib.rs` membrane boundary law:

| Domain | Authority | ggen Role |
|--------|-----------|-----------|
| **Delta execution** | `genesis-construct8` (`knhk-construct8`) | Describe only; invoke via `GenesisAdapter` trait |
| **Triple materialization** | `genesis-construct8` (8-triples-at-a-time law) | None |
| **Receipt chain** | `genesis-construct8/src/receipt.rs` (BLAKE3 hash, packet/triple counts) | ggen receipts are manufacturing provenance only |
| **Hot-path market state** | Downstream consumer of Construct8Packet stream | None |
| **Process mining / conformance** | wasm4pm (pm4py stack) | None — ggen must not impersonate a process miner |
| **Private actuation mechanics** | Never in any TTL | ggen templates must use only public namespace URIs |

The `GenesisAdapter` trait enforces the boundary: no `serde_json`, no `String`, no external format types cross into Genesis. Every crossing must produce a `Construct8` act bound to a receipt. `from_receipt()` reconstructs from the Receipt (canonical authority), not the source format.

---

## Minimal Adapter

A ggen template renders a Construct8Delta *description* as a `.toml` or `.yaml` manifest. The actual delta is compiled by `genesis-construct8` (`knhk-construct8`), not by ggen.

```
open-ontologies (public TTL graphs)
        |
        | SPARQL SELECT/CONSTRUCT/ASK  (.rq files)
        v
    ggen pipeline (μ₁-μ₅)
        |
        | .tera template rendering
        v
  Artifact + Receipt (manifest, Rust source, TOML/YAML)
        |
        | delta description only  [GenesisAdapter boundary]
        v
  genesis-construct8 (knhk-construct8 crate)
        |  owns: delta execution, triple materialization, Receipt structs
        v
  Construct8Packet stream → Receipt (BLAKE3, packet/triple counts)
```

ggen's adapter role re: CONSTRUCT8: **describe and validate** the delta surface via ontology query and SHACL. Never **execute** a delta or claim receipt chain authority.

---

## Public Ontology Admissibility

TTL files in `open-ontologies/ontology/` confirmed admissible for ggen loading, querying, and rendering:

| File | Public Standards Used | ggen Role |
|---|---|---|
| `public-alignment.ttl` | schema.org, PROV-O, DCAT, SKOS, ODRL, SPDX, SHACL | Primary alignment surface — ggen's canonical ontology source |
| `public-shapes.ttl` | SHACL, public prefixes only | SHACL validation gate before rendering |
| `shared-receipt-shapes.ttl` | SHACL, xsd:, rdfs: only | Validates SharedReceiptV1 format for ggen-emitted receipts |
| `cell8-core.ttl` | PROV-O, DCAT, schema.org, SPDX, EARL, ODRL | Cell8 conformance profile — ggen may query gate concepts |
| `cell8-shapes.ttl` | SHACL, public prefixes | Validates Cell8 artifact RDF graph structure |
| `cell8-manufacturing.ttl` | SKOS, EARL, PROV-O | 13 manufacturing gates as SKOS concepts — ggen may render gate manifests |
| `cell8-profile.ttl` | EARL, SKOS, schema.org, PROV-O | 7-gate profile (Seed→Admit) — ggen may describe gate sequences |
| `cell8-conformance-shapes.ttl` | SHACL, EARL | Conformance shape validation |
| `attestation-shapes.ttl` | SHACL, attest: (stable internal) | Key attestation shapes — ggen may validate trusted key entries |
| `ggen-integration-law.ttl` | ggen:, onto: (canonical project namespaces) | The manufacturing contract itself — ggen loads this as self-law |
| `cli-open-ontologies.ttl` | cli:, onto: (ggen project namespace) | Primary SPARQL extraction source for CLI artifact generation |
| `powl-process-mining.ttl` | powl: (process mining vocab), OWL, RDF | Process law descriptions — ggen may describe, must not mine |
| `wasm4pm-stubs.ttl` | Project stub namespace | Stub interface extraction — ggen renders Rust stubs from this |
| `ghf-core.ttl` | PROV-O, SHACL, SKOS | GitHub Factory profile — ggen may render GHF workflow manifests |
| `requirements.ttl` | Project requirements vocab | Requirements extraction source |

**NOT admissible for ggen rendering (private actuation surfaces):**

| File | Reason |
|---|---|
| `aat-live-rules.ttl` | Live AAT rules with private aat: namespace — runtime actuation, not a rendering source |
| `mcpp-proof-chain.ttl` | mcpp proof chain — owned by mcpp, not a ggen rendering surface |
| `ontostar-wasm4pm-integration.ttl` | Cross-system integration state — ggen must not render from this |
| `revops-manufacturing.ttl` | RevOps test case manufacturing state — ggen reads only; does not render actuation |

---

## What MUST NOT happen

1. **ggen must not become a process miner** — OCEL derivation, event log replay, and pm4py conformance belong exclusively to wasm4pm. ggen may render process law descriptions from `powl-process-mining.ttl` but must not execute mining algorithms.

2. **open-ontologies TTL files must not embed private actuation mechanics** — All TTL files loaded by ggen must use only public standard namespaces or stable project-internal namespaces. Private runtime state (live market data, hot-path actuation, session tokens) must never appear in ontology files.

3. **.ggen source surfaces must not fake receipts** — Receipts emitted by ggen must be BLAKE3-hashed over actual rendered artifact bytes. A receipt that does not correspond to a real manufactured artifact is a defect, not a provenance record.

4. **ggen must not execute Construct8 deltas** — The `genesis-construct8` / `knhk-construct8` crate owns delta execution (lawful triple materialization 8 triples at a time). ggen may render delta descriptions and invoke the crate via `GenesisAdapter`, but must not duplicate or bypass the crate's internals.

5. **ggen templates must not embed process mining oracles** — Templates rendering process mining artifacts must describe law, not compute fitness/precision scores. Those computations belong to wasm4pm's Python pm4py stack.

---

## Reference Implementation Evidence

| Component | Location | Status |
|---|---|---|
| ggen five-stage pipeline | `/Users/sac/ggen/README.md` (μ₁-μ₅) | OPERATIONAL |
| Manufacturing contract | `/Users/sac/open-ontologies/ontology/ggen-integration-law.ttl` | AUTHORITATIVE |
| GenesisAdapter boundary trait | `/Users/sac/ggen/crates/ggen-membrane/src/lib.rs` | ENFORCED |
| genesis-construct8 crate | `/Users/sac/ggen/crates/genesis-construct8/` | OPERATIONAL |
| Receipt struct (BLAKE3) | `/Users/sac/ggen/crates/genesis-construct8/src/receipt.rs` | CANONICAL |
| Public alignment ontology | `/Users/sac/open-ontologies/ontology/public-alignment.ttl` | VERIFIED |
| Cell8 manufacturing gates | `/Users/sac/open-ontologies/ontology/cell8-manufacturing.ttl` | 13 gates confirmed |
| SharedReceiptV1 shapes | `/Users/sac/open-ontologies/ontology/shared-receipt-shapes.ttl` | SHACL-validated |
| SPARQL extraction queries | `/Users/sac/ggen/.specify/queries/` | OPERATIONAL |
| Tera templates | `/Users/sac/ggen/templates/` + `.specify/templates/` | OPERATIONAL |

---

## See Also

- `CONSTRUCT8_PROJECT_CONTRACTS.md` — Project-level agreements
- `/Users/sac/ggen/README.md` — ggen philosophy and command reference
- `/Users/sac/open-ontologies/ontology/public-alignment.ttl` — Public vocab mappings
- `/Users/sac/open-ontologies/ontology/ggen-integration-law.ttl` — Canonical manufacturing contract
- `/Users/sac/ggen/crates/ggen-membrane/src/lib.rs` — GenesisAdapter boundary trait
- `AGENT_03_CONSTRUCT8_WITNESS.md` — genesis-construct8 implementation details
