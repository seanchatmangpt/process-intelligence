# Adapter Boundaries

**Authority:** AGENT_10 Integration Conductor
**Date:** 2026-06-01
**Status:** 5 contracts defined — all AUTHORITATIVE

Contracts define safe inter-project communication without deep coupling. Each contract specifies what a project may consume, what it must emit, and what it must never own.

---

## Contract 1: ggen_construct8_contract.md

**File:** `adapters/ggen_construct8_contract.md`
**Governs:** ggen + open-ontologies ↔ genesis-construct8 (CONSTRUCT8 delta engine)
**Agent:** 04

### Key Rules

1. ggen owns specification and rendering: ontology normalization (μ₁), SPARQL extraction (μ₂), Tera rendering (μ₃), canonicalization (μ₄), receipt emission (μ₅).
2. `GenesisAdapter` trait in `ggen/crates/ggen-membrane/src/lib.rs` is the ONLY legal crossing point between ggen and Genesis. No serde_json, no String, no external types cross into Genesis.
3. genesis-construct8 (`knhk-construct8` crate) owns delta execution and triple materialization — not ggen.
4. ggen must not execute CONSTRUCT8 deltas. That law belongs to genesis-construct8.
5. All receipts must be BLAKE3 over real artifact bytes — no fake receipts.
6. open-ontologies TTL files must not embed private actuation mechanics.
7. ggen templates must not embed process mining oracles (fitness/precision scores belong to wasm4pm).
8. Public TTL surfaces are bound to public namespaces only: schema.org, PROV-O, DCAT, SKOS, ODRL, EARL, SHACL, SPDX, Dublin Core.

---

## Contract 2: wasm4pm_evidence_contract.md

**File:** `adapters/wasm4pm_evidence_contract.md`
**Governs:** wasm4pm-compat (c8-market) ↔ wasm4pm (full execution engine)
**Agent:** 05

### Key Rules

1. `MarketPlanckCell` (c8-market) maps to an OCEL object-centric event: instrument/venue are objects, relation_kind is the activity, causal_time + monotonic_time are the timestamp pair.
2. `Construct8Delta` (c8-graph) maps to a bounded graph mutation event — owned by c8-graph, not wasm4pm.
3. `RepresentationGap` (c8-adversary) maps to conformance deviation in wasm4pm conformance engine.
4. `C8Receipt` (c8-receipts) is the process evidence receipt — BLAKE3 hash required.
5. `VectorClock8` (c8-time) provides causal ordering in OCEL.
6. wasm4pm owns: discovery, conformance, replay, receipts, OCPQ. wasm4pm-compat owns: structure, type laws, witness lattices.
7. wasm4pm must not own market instruments. wasm4pm-compat must not own mining/replay logic.
8. Graduation boundary: MarketPlanckCell → OcelLog conversion is the formal crossing point. A GraduationReason must be declared at crossing.
9. wasm4pm-compat requires nightly Rust — no stable build target until graduation.

---

## Contract 3: truex_receipt_contract.md

**File:** `adapters/truex_receipt_contract.md`
**Governs:** construct8-market-physics → truex → Blue River Dam
**Agent:** 06

### Key Rules

1. Truex consumes CONSTRUCT8 witness outputs, not raw market signals.
2. Permitted inputs: graph-state delta objects, causal time vectors, adversary-gap witnesses, BLAKE3 receipt stubs.
3. Prohibited inputs: raw market data, LLM inference outputs, unreceipted observation claims, Need9 objects (Need9 must be split inside CONSTRUCT8 before reaching truex).
4. Truex emits: consequence receipts sealed with BLAKE3; receipt_hash = BLAKE3(session_id || ocel2_batch_hash || expected_path_hash). Nine refusal statuses enumerated.
5. Blue River Dam admission requires BLAKE3 receipt at every gate — no receipt, no admission.
6. Truex governing axiom: "NO HUMANS IN RUNTIME ACTUATION." Chatman Equation: `R ⊢ A = μ(O*)`.
7. Four Truex laws: no hook no consequence, no receipt no authority, no replay no substrate, no accounting no promotion.
8. BRD is not a trading bot — it is a coordination/control protocol for world-state transitions.
9. **Current risk:** 7,066 uncommitted files in truex. No tests, no receipts. ALIVE verdict blocked.

---

## Contract 4: naut_hotpath_contract.md

**File:** `adapters/naut_hotpath_contract.md`
**Governs:** Naut branchless discipline ↔ CONSTRUCT8 hot-path generalization
**Agent:** 07

### Key Rules

1. Naut's core insight: branchless discipline (remove dynamic allocation, pointer chasing, speculative control flow from hot path) outperforms interpretive logic under hostile conditions.
2. CONSTRUCT8 generalizes Naut's discipline via: `[Option<Construct8Triple>; 8]` fixed arrays, `u8` bitmask slot tracking, typed newtypes (`NodeId`, `RelationId`).
3. Need9 = split, not widen: when an operation requires more than 8 elements, CONSTRUCT8 returns Need9. Never extend the array.
4. ARM64 NEON intrinsics: **PARTIAL_ARCH** — no explicit intrinsic bindings confirmed. No claims about ARM64 NEON until naut repo verified.
5. CONSTRUCT8 benchmarks (c8-bench) provide latency measurements but are not equivalent to Naut's full production validation.
6. naut repo is ABSENT on this machine. All naut-related claims are from documentation only.
7. No naut-specific performance claims may be made in PhD dissertation without naut repo verification.

---

## Contract 5: phd_publication_contract.md

**File:** `adapters/phd_publication_contract.md`
**Governs:** Research corpus → PhD dissertation / public papers IP boundary
**Agent:** 08

### Key Rules

1. **R-01: No broker or exchange names** — prohibited in all dissertation chapters, appendices, figures, captions, acknowledgments, supplemental material, and slides.
2. **R-02: No capital deployment parameters** — position sizes, leverage, risk limits, execution parameters are private IP, not publishable.
3. **R-03: No venue topology** — internal routing topology, dark pool access patterns, co-location strategies are private.
4. **R-04: No model performance on live data** — only synthetic benchmark results may be published.
5. Public-safe theorems: Feature Collapse Theorem, Representational Separability Theorem, Logic Branch Explosion Theorem, Coordinate-System Alpha representation gap proof.
6. Public-safe constructs: CONSTRUCT8 max-8 enforcement (structural proof), Need9 type decomposition, OCEL 2.0 conformance results on synthetic logs, branchless hot-path generalization (architectural discipline only).
7. PhD publication must pass full-text redaction scan before submission. Any match on prohibited terms is a blocker.
8. process-intelligence research program is the authority layer — no downstream claim may exceed what the research program has grounded.
