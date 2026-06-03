# CONSTRUCT8 / Motion Boundary Map

**Agent:** D — CONSTRUCT8 and Motion Boundary Cartographer
**Generated:** 2026-06-01
**Corpus sources:** 12 authoritative documents across knhk, insa, ggen, process-intelligence
**Status:** AUTHORITATIVE — replaces stub

---

## Frame Law (Load-Bearing — Never Violate)

```
attempt → hook → admission/refusal → durable motion → receipt → replay → accounting → promotion
```

```
No hook, no consequence.
No receipt, no authority.
No replay, no substrate.
No accounting, no promotion.
```

SOURCE: `/Users/sac/truex/docs/MANIFESTO.md` (verbatim, appears twice)

---

## 1. CONSTRUCT8 — Authoritative Definition

### 1.1 Primary Structural Definition (Rust Specification)

SOURCE: `/Users/sac/knhk/GENESIS_ARCHITECTURE.md` — Section 4.3

```
Construct8Packet {
  epoch: u64,                 // Logical clock
  law_ref: u64,               // Which law governs this packet
  subjects: [u32; 8],         // 8 subject IRIs (symbol table refs)
  predicates: [u32; 8],       // 8 predicate IRIs
  objects: [u32; 8],          // 8 object IRIs
  kind_mask: u8,              // Type of each triple (literal, IRI, blank)
  valid_mask: u8,             // Which lanes are filled (bits 0-7)
  emit_mask: u8,              // Which lanes to emit (subset of valid_mask)
  block_mask: u8,             // Which lanes block further processing
  order: u32,                 // Sequence number in stream
  receipt_seed: [u8; 32],     // BLAKE3 seed for deterministic hashing
}
```

**Fixed size**: 8 lanes (Need9 law: max 8 triples per packet)
**Bitmask fields**: valid_mask & emit_mask encode which lanes are active
**Deterministic hashing**: receipt_seed ensures bitwise-identical receipts

SOURCE: `/Users/sac/knhk/rust/genesis-construct8/src/models.rs` (lines 87–104)

### 1.2 Kernel Specification Definition

SOURCE: `/Users/sac/knhk/GENESIS_CORE_SPECIFICATION.md`

```rust
/// Construct8: Bounded lane packing (≤8 pairs per construction act)
#[repr(C)]
pub struct Construct8 {
    pub epoch: u64,
    pub relation_id: u32,
    pub lanes: [Pair2; 8],     // Bounded to 8 lanes
    pub valid_mask: u8,        // Bitvector: which lanes are populated
}
```

The kernel exports two deterministic operations:

```
construct8_admission(packet, gate) → Result<(), Refusal>
construct8_receipt(packet, prev_receipt) → Receipt
```

`construct8_admission` evaluates a SPARQL ASK predicate against the packet. If the
ontology rule is violated, the kernel returns `Err(Refusal)` — not a recoverable
exception. The Refusal is a first-class law-enforcement record.

`construct8_receipt` computes `BLAKE3(epoch, lanes, prev_hash, law_ref)`. The receipt
hash is the only promise Genesis makes: `hash(A) = hash(μ(O))`.

### 1.3 Functional Statement (Press Release Source)

SOURCE: `/Users/sac/knhk/ORIGINAL_REQUEST.md`

> "Genesis takes candidate observations admitted through external membranes such as ggen
> and realizes them through deterministic construction, bounded by CONSTRUCT8: no more
> than eight active triples per construction act. Large corpora are not built as giant
> graph objects. They are constructed as packet streams, segments, shards, and corpus
> receipts."

### 1.4 Warm Path Role

SOURCE: `/Users/sac/knhk/doc-examples/warm_guide.md`

CONSTRUCT8 operates in the warm path (≤500µs budget, not the ≤8-tick hot path). It is
the primary emit operation that moves output from the hot path execution context to the
durable RDF graph. The packet itself is bounded to 8 triples; SIMD load/blend/store for
a full 16-lane SIMD operation takes ~50 ticks, which is why CONSTRUCT8 emission is routed
to warm path.

The warm path latency budget for CONSTRUCT8 (1–8 lanes): ≤500µs.

### 1.5 Oxigraph Relationship

SOURCE: `/Users/sac/ggen/CONVO.txt`; `/Users/sac/compiled-cognition-hub/src/main.rs`

- `use unibit_graph::construct8_to_oxigraph` — CONSTRUCT8 bridge to Oxigraph RDF store
- "Long-term state should live on the graph. POWL64 blocks should be graph-resident
  constitutional objects in Oxigraph, constructed and evolved through lawful graph deltas,
  while compact native structures remain the short-lived kinetic form for hot execution."

CONSTRUCT8 is the transit form. Oxigraph (or any durable RDF triplestore) is the
admitted-motion destination. The packet is discarded after admission; the receipt and the
triples persisted to Oxigraph are the durable evidence.

---

## 2. The 8 Mutation Primitives (Lanes)

The corpus does not enumerate 8 distinct named mutation primitives in a single authoritative
list. What is documented is the lane structure:

- Each lane in Construct8Packet is a (subject, predicate, object) triple handle triple.
- A packet holds at most 8 lanes, governed by `valid_mask` (bits 0–7).
- `kind_mask` encodes the RDF type of each object (literal, IRI, blank node).
- `emit_mask` is a subset of `valid_mask` selecting which lanes to project downstream.
- `block_mask` indicates which lanes block further processing in the pipeline.

The insa byte-lane family documented in `/Users/sac/insa/CLAUDE.md` names the related
family: INST8, KAPPA8, Family8, POWL8, CONSTRUCT8 — each a distinct byte-speed kernel
with 8-lane discipline.

The wasm4pm vision document (`/Users/sac/wasm4pm/docs_quarantine/ARCHIVE/docs/vision-2030-hyperthesis.md`)
states: "μ decomposes it — 8 information-theoretic operators, each reducing intent entropy
by ~6.1 nats."

**Finding:** The 8 primitives are the 8 lanes of (subject, predicate, object) handle triples,
not 8 named operation types. The bounded number 8 is the Chatman Constant applied to
construction: the same physics-based reasoning that bounds hot-path execution to ≤8 CPU ticks
also bounds a single construction act to ≤8 triple emissions.

---

## 3. Need9 Rule — Verbatim

### 3.1 RefusalReason Enum Entry

SOURCE: `/Users/sac/knhk/PROOF_SURFACES.md` — Section 4, RefusalReason Enum

```rust
Need9    // Packet overfill (>8 triples)
```

A Construct8Packet that attempts to carry more than 8 active triples is refused with
`RefusalReason::Need9`. This is not an error. It is a durable, auditable law-enforcement
event.

### 3.2 Need9 in Architecture

SOURCE: `/Users/sac/knhk/GENESIS_ARCHITECTURE.md`

```
GENESIS_ARCHITECTURE.md:
  Need9, Need257, MissingSourceAddress, UnauthorizedRelationContext,
  InvalidMultiplicity, DuplicateInflation, ReplayMismatch, ReceiptMismatch,
  ExternalProjectionMismatch, UnreceiptedIndex, BoundaryByteEmissionBypass,
  MockRandomMatter,
```

### 3.3 Need9 Means Decompose — Not Widen

SOURCE: `/Users/sac/insa/AGENTS.md` (verbatim)

> "Do not widen Need9 first: Need9 means decompose, sequence, compose, or add another
> byte lane. It does not mean widen to u16 or Vec."

### 3.4 Need9 Produces Lawful Splits

SOURCE: `/Users/sac/knhk/GENESIS_2030_DFLSS_CHARTER.md` (verbatim)

> "Split law: Need257 and Need9 produce lawful splits, not widened hot payloads"

When a candidate set of triples exceeds 8, the correct response is to split the input into
multiple Construct8Packets (sequenced, each ≤8 lanes), not to widen the data type. Each
packet in the sequence gets its own receipt. The receipt chain links them.

### 3.5 Need9 in Test Design

SOURCE: `/Users/sac/knhk/GENESIS_2030_DFLSS_CHARTER.md`

```
| 4 | refusal model for Need9, Need257, missing source, invalid context |
```

The refusal model requires test fixtures that specifically trigger Need9 and verify the
resulting Refusal artifact has the correct reason, input_digest, and timestamp.

---

## 4. MAPE-K Closure Relationship

### 4.1 Canonical Statement

SOURCE: `/Users/sac/knhk/DOCTRINE_2027.md` (verbatim)

> "MAPE-K embedded as knowledge hooks is the only way to keep the discipline while
> closing the loop at machine speed."

### 4.2 Covenant 3 Binding

SOURCE: `/Users/sac/knhk/DOCTRINE_COVENANT.md` — Covenant 3

> "Feedback Loops Run at Machine Speed (MAPE-K ⊨ Autonomy)"
> "Every workflow has embedded monitoring, analysis, planning, execution, and learning."
> "MAPE-K is not a separate system; it is part of the execution engine."
> "The cycle O → Analyze → Plan → Execute → K → O' runs as fast as telemetry arrives."

MAPE-K violations (per Covenant 3):
- Manual approval steps without automated fallback
- MAPE-K components slower than once per workflow cycle
- Policies not encoded as executable SPARQL
- Knowledge base not updated from execution receipts

### 4.3 CONSTRUCT8 as MAPE-K Execute Output

SOURCE: `/Users/sac/process-intelligence/lifecycle/MAPE_K_MAP.md`

The Execute component in MAPE-K is the actuation step. Each executed action produces a
receipt. The output of Execute is a typed artifact. In the Genesis/KNHK architecture,
Execute → CONSTRUCT8 → Receipt is the completion of the E step.

"The Execute component does not plan. Each action produces a receipt. A receipt-less
execution is not an execution for process intelligence purposes — it is an unwitnessed
intervention."

### 4.4 Receipt Chain as Loop Proof

SOURCE: `/Users/sac/knhk/GENESIS_CORE_SPECIFICATION.md` — Covenant-to-Function Mapping

| Covenant | Kernel Export | Enforcement |
|---|---|---|
| MAPE-K at Machine Speed (3) | `construct8_receipt()` + `segment_receipt()` | Receipt chain proves Monitor→Execute loop at latency bound |

The receipt chain is not telemetry. It is the proof that the MAPE-K loop closed. A cycle
without a receipt is not a closed loop; it is narration.

### 4.5 Knowledge Component

SOURCE: `/Users/sac/knhk/KNHK_PHD_THESIS.md` — Chapter 5.4

```
KNOWLEDGE: Learn and persist
  - Update Σ (ontology) from observations
  - Improve policies from results
  - Store receipts for future analysis
```

The K component in MAPE-K is fed by the Receipt chain emitted by CONSTRUCT8 packets. This
is the closure: observations → CONSTRUCT8 → receipt → Knowledge → improved policies →
new SPARQL ASK gates → next admission cycle.

---

## 5. No Direct Tool-to-State Write Law

### 5.1 Primary Statement

SOURCE: `/Users/sac/insa/AGENTS.md` (verbatim, operating contract)

> "Do not let projection results mutate state: They must re-enter as
> `Observation -> CONSTRUCT8 -> O*`."

### 5.2 CLAUDE.md Enforcement

SOURCE: `/Users/sac/insa/CLAUDE.md` (verbatim, "Never" list)

> "Let projection results (MCP/A2A/HITL) mutate state directly — re-enter via
> `Observation → CONSTRUCT8 → O*`"

This is listed under "Never" — absolute prohibition.

### 5.3 Anti-Pattern 5 from Genesis Core Spec

SOURCE: `/Users/sac/knhk/GENESIS_CORE_SPECIFICATION.md` — Anti-Pattern 5

> "Collapsed form: Kernel includes `construct8_to_ocel(&self) → OcelEvent`
> Why it fails: Violates separation: kernel is consequence law, not data projection.
> Makes kernel depend on downstream format versions → brittleness."

The kernel never writes to external state. It emits a Receipt. The membrane (ggen) owns
all projections. The projection result must re-enter the system as an Observation, which
then goes through CONSTRUCT8 for a new admission cycle if it is to become consequential.

### 5.4 The Seven Separations — Construction ≠ Query

SOURCE: `/Users/sac/knhk/GENESIS_ARCHITECTURE.md` — Section 3.4

> "Construction ≠ Query: Construct8 packet is immutable proof; queries are projections
> over it. Index is read-only projection; cannot alter past receipts."

Query results, tool outputs, and projection results share the same prohibition: they cannot
directly write state. They must re-enter as Observations and be admitted through a
Construct8 gate.

### 5.5 Unproofed Emission is Forbidden

SOURCE: `/Users/sac/insa/AGENTS.md` (verbatim)

> "Do not emit without proof: Unproofed emission is structurally forbidden."

This is the corollary: not only can tools not write state directly, but any emission that
exits the system without producing a receipt is also forbidden.

---

## 6. Corpus Sources Indexed

| # | File | Project | Contribution |
|---|------|---------|-------------|
| 1 | `/Users/sac/knhk/GENESIS_ARCHITECTURE.md` | knhk | Construct8Packet struct, Need9 enum, Seven Separations |
| 2 | `/Users/sac/knhk/GENESIS_CORE_SPECIFICATION.md` | knhk | Kernel Construct8 struct, covenant-to-function map, anti-patterns |
| 3 | `/Users/sac/knhk/PROOF_SURFACES.md` | knhk | RefusalReason enum verbatim with Need9 |
| 4 | `/Users/sac/knhk/GENESIS_2030_DFLSS_CHARTER.md` | knhk | Need9 split law, test fixture requirements |
| 5 | `/Users/sac/knhk/DOCTRINE_2027.md` | knhk | MAPE-K as knowledge hooks canonical statement |
| 6 | `/Users/sac/knhk/DOCTRINE_COVENANT.md` | knhk | Covenant 3 (MAPE-K), Covenant 5 (8-tick) |
| 7 | `/Users/sac/knhk/ORIGINAL_REQUEST.md` | knhk | CONSTRUCT8 functional statement |
| 8 | `/Users/sac/knhk/doc-examples/warm_guide.md` | knhk | Warm path role, ≤500µs budget |
| 9 | `/Users/sac/knhk/GENESIS_CONSTRUCT8_KERNEL_INVENTORY.md` | knhk | Implementation inventory |
| 10 | `/Users/sac/insa/AGENTS.md` | insa | No-mutation law, Need9 decompose rule, unproofed emission ban |
| 11 | `/Users/sac/insa/CLAUDE.md` | insa | Byte-lane family (INST8/KAPPA8/Family8/POWL8/CONSTRUCT8) |
| 12 | `/Users/sac/process-intelligence/lifecycle/MAPE_K_MAP.md` | process-intelligence | MAPE-K Execute → receipt requirement |
| 13 | `/Users/sac/truex/docs/MANIFESTO.md` | truex | Frame law compression (no hook, no consequence...) |
| 14 | `/Users/sac/compiled-cognition-hub/src/main.rs` | compiled-cognition-hub | `construct8_to_oxigraph` bridge reference |
| 15 | `/Users/sac/ggen/CONVO.txt` | ggen | Oxigraph as durable admitted-motion destination |

---

## 7. Bounded Constructive Delta — Summary Definition

CONSTRUCT8 is the bounded constructive delta operator. It is:

1. **Bounded**: At most 8 triple lanes per construction act (Need9 law)
2. **Constructive**: It produces new triples in a durable RDF graph (admitted motion), not a transient calculation
3. **Delta**: It carries only the change, not the full state — a packet stream, not a graph snapshot
4. **Receipted**: Every admitted packet produces a BLAKE3 receipt, chaining to prior receipts
5. **Replayable**: Same packet + same sequence = identical receipt hash (determinism invariant)
6. **Admission-gated**: SPARQL ASK predicate evaluates every packet before admission; refusal is first-class

The construction direction is one-way:

```
Observation → CONSTRUCT8 → Admission Gate → Admitted Triple(s) → Receipt → Oxigraph
                                ↓
                          Refusal (Need9, Need257, ConstraintViolation, etc.)
```

External tools, MCP results, A2A projections, and HITL outputs cannot write directly to
Oxigraph. They must re-enter as Observations, pass through CONSTRUCT8, and be admitted.
No hook, no consequence. No receipt, no authority.
