# Truex Receipt Contract — Complete Adapter Specification
## Truex ← CONSTRUCT8 ← Blue River Dam

**Authority:** process-intelligence research program  
**Date Issued:** 2026-06-01  
**Status:** AUTHORITATIVE  
**Agent:** 6 — Truex + Blue River Dam Integration Boundary  

---

## I. EXECUTIVE SUMMARY

This contract specifies the complete adapter interface through which:
1. **CONSTRUCT8** emits bounded witness outputs
2. **Truex** receives witness outputs and applies hook-first admission
3. **Truex** emits receipts sealing the admission/refusal decision
4. **Blue River Dam** routes admitted consequence cells for conformance audit

The contract ensures:
- No raw market data crosses the boundary (only O*)
- No unreceipted claims propagate (only cells with complete receipt lineage)
- No capital flows execute inside the dam (only process evidence)
- No hidden human runtime (explicit queue receipt for deferred judgment)

---

## II. WHAT CONSTRUCT8 EMITS (Witness Outputs)

### 2.1 Allowed Outputs

CONSTRUCT8 emits the following witness objects:

#### A. Planck-Cell Deltas
```rust
pub struct PlanckCellDelta {
    pub id: UUID,
    pub timestamp: u64,               // nanoseconds since epoch
    pub triples: Vec<RdfTriple>,      // max 8 triples per delta
    pub causal_clock: u64,            // monotonic vector clock value
    pub hash: [u8; 32],               // BLAKE3 hash of this delta
}
```

**Constraints:**
- `triples.len() ≤ 8` (enforced by Need9 type system)
- `timestamp` is strictly increasing
- `causal_clock` is monotonically increasing
- `hash` is computed deterministically from (triples, causal_clock)

**Example:**
```json
{
  "id": "delta_2026_06_01_21_51_00_001",
  "timestamp": 1748863860000000000,
  "triples": [
    { "subject": "s:market_state_123", "predicate": "p:price", "object": "o:150.25" },
    { "subject": "s:market_state_123", "predicate": "p:volume", "object": "o:10000" },
    { "subject": "s:market_state_123", "predicate": "p:relation_break", "object": "true" },
    { "subject": "s:graph_state_456", "predicate": "p:observes", "object": "s:market_state_123" }
  ],
  "causal_clock": 42,
  "hash": "5548b5fcac3109bcc176bad6f91e1408cbef34e87b1cba6cdf55a672f64b5694"
}
```

#### B. Adversary-Gap Witness
```rust
pub struct AdversaryGapWitness {
    pub gap_score: u32,               // state-count divergence
    pub logic_basis: Vec<String>,     // states LogicPlayer can name
    pub graph_basis: Vec<String>,     // states GraphPlayer can name
    pub evidence: Vec<u8>,            // serialized proof (gap_score ≥ 1)
}
```

**Constraints:**
- `gap_score` = |graph_basis| - |logic_basis| (must be ≥ 0)
- `logic_basis` is a subset of `graph_basis`
- `evidence` contains the synthetic tick stream and reachability proof

**Example:**
```json
{
  "gap_score": 2,
  "logic_basis": ["price_up", "price_down", "volume_spike"],
  "graph_basis": ["price_up", "price_down", "volume_spike", "relation_break", "liquidity_collapse"],
  "evidence": "base64_encoded_proof"
}
```

#### C. BLAKE3 Receipt Stub
```rust
pub struct C8Receipt {
    pub id: UUID,
    pub delta_id: UUID,
    pub hash: [u8; 32],               // BLAKE3(delta)
    pub timestamp: u64,
    pub signature: Vec<u8>,           // Ed25519 or ECDSA
}
```

**Constraints:**
- `hash` must be BLAKE3 of the corresponding delta
- `signature` is computed over (hash || timestamp)
- All fields must be deterministically reconstructable

**Example:**
```json
{
  "id": "receipt_2026_06_01_21_51_00_001",
  "delta_id": "delta_2026_06_01_21_51_00_001",
  "hash": "5548b5fcac3109bcc176bad6f91e1408cbef34e87b1cba6cdf55a672f64b5694",
  "timestamp": 1748863860000000000,
  "signature": "3045022100...ed25519_signature..."
}
```

### 2.2 Prohibited Outputs

CONSTRUCT8 **must NOT** emit:

| Prohibited | Reason |
|---|---|
| Raw market data (ticks, prices, volumes) | Should be received as input, not produced by witness |
| LLM predictions or candidates | Witness is deterministic, not probabilistic |
| Trading signals or order suggestions | Capital flows forbidden; witness is structural only |
| Unreceipted state mutations | Every delta must carry BLAKE3 receipt |
| Live exchange connection strings | No broker/custodian integration |
| Private keys, credentials, tokens | Witness is public and verifiable |
| Graph deltas > 8 triples | Need9 type system enforces max-8 bound |

---

## III. WHAT TRUEX CONSUMES (Ingress Interface)

### 3.1 Receipt Envelope Format

Truex receives a **receipt envelope** (not raw deltas). The envelope bundles:
1. CONSTRUCT8 witness outputs
2. Metadata (source, timestamp, authority)
3. OCEL 2.0 event log of the witness derivation

```json
{
  "truex_receipt_envelope": {
    "truex_profile": "truex.ocel2.receipt.v1",
    "source": "construct8-market-physics",
    "source_version": "CONSTRUCT8_ALIVE_001",
    "received_at": "2026-06-01T21:51:00Z",
    
    "construct8_witness": {
      "planck_cell_delta": { ... },
      "adversary_gap_witness": { ... },
      "c8_receipt": { ... }
    },
    
    "ocel2": {
      "eventTypes": {
        "WitnessProduced": { "attributes": ["witness_type", "gap_score"] },
        "DeltaEmitted": { "attributes": ["triples_count", "clock_value"] }
      },
      "objectTypes": {
        "GraphState": { "attributes": ["basis_cardinality"] },
        "LogicState": { "attributes": ["basis_cardinality"] }
      },
      "events": [ ... ],
      "objects": [ ... ],
      "event-object": [ ... ],
      "object-object": [ ... ],
      "objectChanges": [ ... ]
    },
    
    "admission_status": "ReceiptAdmitted",
    "ocel2_batch_hash": "c13adf8815ec50ece8b9b9aa7ca3398eeae3c2acd21291deba20a959ad723850",
    "receipt_hash": "2cbb73c977e8b2b490fc9549d5741b5fa2676615d31bff534beb217ce36120b4"
  }
}
```

### 3.2 Ingress Validation (Truex Hook Gates)

When Truex receives a receipt envelope, it validates:

#### Gate 1: Type Check (Σ)
```
Does the envelope match the TruexReceiptEnvelope schema?
  ✓ truex_profile present
  ✓ source and source_version present
  ✓ construct8_witness present
  ✓ ocel2 present
  ✓ receipt_hash present
```

#### Gate 2: Guard Check (H)
```
Are preconditions satisfied?
  ✓ source_version matches acceptable CONSTRUCT8 ALIVE state
  ✓ received_at is within acceptable time window (±5 seconds)
  ✓ admission_status = "ReceiptAdmitted" (not "ReceiptForged", etc.)
```

#### Gate 3: Transition Check (T)
```
Is this a valid state transition?
  ✓ Envelope is not a duplicate (receipt_hash not in ledger)
  ✓ Causal clock is strictly greater than previous delta
  ✓ No temporal anomalies (timestamp is monotonically increasing)
```

#### Gate 4: Policy Check (P)
```
Does this violate any LTL constraints?
  ✓ No capital flow in witness (no CapitalFlow type)
  ✓ No unreceipted claims (all deltas carry c8_receipt)
  ✓ No loops except via explicit replay (causal chain is acyclic)
```

#### Gate 5: Capability Check (C)
```
Does Truex have authority to admit this?
  ✓ Hook authority is registered in trust store
  ✓ Hook has permission to route CONSTRUCT8 witness
  ✓ System resources available (mailbox not full, storage not exceeded)
```

#### Gate 6: Freshness Check (Fresh)
```
Is the timestamp recent?
  ✓ received_at is within ±5 seconds of system clock
  ✓ Not re-admitted after TTL expiration (default TTL = 1 hour)
  ✓ No temporal drift > 1 second between system clock and envelope timestamp
```

#### Gate 7: Receipt Lineage Check (R)
```
Is the receipt properly sealed?
  ✓ receipt_hash verifies:
    receipt_hash = BLAKE3(admission_status || ocel2_batch_hash || expected_path_hash)
  ✓ ocel2_batch_hash verifies:
    ocel2_batch_hash = BLAKE3(canonical_stringify(ocel2))
  ✓ All signatures in c8_receipt verify (Ed25519 or ECDSA)
```

### 3.3 Ingress Refusal Statuses

If **any** gate fails, Truex **refuses** the envelope:

| Gate | Refusal Status | Issued | Reason |
|---|---|---|---|
| Type | `SchemaViolation` | ostar-gate | Envelope field missing or malformed |
| Guard | `InvalidTransition` | ostar-gate | admission_status ≠ "ReceiptAdmitted" |
| Transition | `ReplayDetected` | ostar-operator | receipt_hash already in ledger |
| Policy | `CapitalFlowDetected` | ostar-operator | Capital flow found in witness (terminal) |
| Capability | `AuthorityMismatch` | ostar-gate | Hook authority not registered |
| Freshness | `TemporalOutOfBounds` | ostar-gate | Timestamp drift > 5 seconds |
| Receipt | `ReceiptForged` | ostar-auditor | Hash verification fails |

All refusals generate a **refusal receipt**:
```yaml
refusal_receipt:
  id: "refusal_uuid"
  envelope_id: "envelope_id"
  decision: "REFUSE"
  decision_code: 0x01
  timestamp: "2026-06-01T21:51:00Z"
  reason: "Refusal status code"
  blake3:
    envelope_hash: "blake3_of_original_envelope"
    refusal_proof: "blake3_of_refusal_reason_and_gates"
```

---

## IV. WHAT TRUEX EMITS (Egress Interface)

### 4.1 Admitted Receipt (ADMIT Path)

When all seven gates pass, Truex issues an **admitted receipt**:

```yaml
truex_admission_receipt:
  id: "receipt_uuid"
  authority: "truex"
  source_envelope_id: "envelope_id"
  
  decision:
    status: "ADMIT"
    decision_code: 0x00
    timestamp: "2026-06-01T21:51:00Z"
  
  projection:
    form: "O*"
    delta_form: "construct8_planck_cell"
    closure_signature: "blake3_of_projected_delta"
  
  receipt_lineage:
    truex_gates_passed: [1, 2, 3, 4, 5, 6, 7]
    blake3:
      envelope_hash: "blake3_of_original_envelope"
      decision_hash: "blake3_of_decision_and_timestamp"
      combined_hash: "blake3_of_envelope_hash || decision_hash || timeline_hash"
    
    causal_timeline:
      - epoch: 1
        event: "envelope_received"
        ts: "2026-06-01T21:50:00Z"
      - epoch: 2
        event: "gates_validation_completed"
        ts: "2026-06-01T21:50:30Z"
      - epoch: 3
        event: "projection_to_o_star"
        ts: "2026-06-01T21:50:45Z"
      - epoch: 4
        event: "admission_decision"
        ts: "2026-06-01T21:51:00Z"
    
    lineage_proof: "hsm_ed25519_or_ecdsa_signature_128chars"
  
  mailbox_state:
    durable: true
    persisted_at: "2026-06-01T21:51:15Z"
    motion_tokens: ["token_construct8_witness", "token_o_star_projection"]
  
  promotion_eligible: false  # Will become true after replay stability and accounting proved
```

### 4.2 Refusal Receipt (REFUSE Path)

When **any** gate fails, Truex issues a **refusal receipt**:

```yaml
truex_refusal_receipt:
  id: "refusal_uuid"
  authority: "truex"
  source_envelope_id: "envelope_id"
  
  decision:
    status: "REFUSE"
    decision_code: 0x01
    timestamp: "2026-06-01T21:51:00Z"
  
  refusal_reason:
    gate_failed: 4  # e.g., Gate 4 (Policy check) failed
    reason_code: "CapitalFlowDetected"
    reason_detail: "Envelope contains CapitalFlow type; capital flows not routed through BRD"
  
  receipt_lineage:
    truex_gates_passed: [1, 2, 3]  # Passed through gate 3, failed at gate 4
    blake3:
      envelope_hash: "blake3_of_original_envelope"
      refusal_proof: "blake3_of_gate_failure_and_timestamp"
    
    causal_timeline:
      - epoch: 1
        event: "envelope_received"
        ts: "2026-06-01T21:50:00Z"
      - epoch: 2
        event: "gate_1_2_3_passed"
        ts: "2026-06-01T21:50:30Z"
      - epoch: 3
        event: "gate_4_policy_check_failed"
        ts: "2026-06-01T21:50:45Z"
      - epoch: 4
        event: "refusal_decision"
        ts: "2026-06-01T21:51:00Z"
    
    lineage_proof: "hsm_ed25519_or_ecdsa_signature_128chars"
  
  terminal: true  # Refusal is lawful terminality; envelope does not re-enter unless policy changes
```

### 4.3 Rewrite Receipt (REWRITE Path)

When envelope passes gates but a modification is required (elastic repair or compliance adjustment):

```yaml
truex_rewrite_receipt:
  id: "receipt_uuid"
  authority: "truex"
  source_envelope_id: "envelope_id"
  
  decision:
    status: "REWRITE"
    decision_code: 0x02
    timestamp: "2026-06-01T21:51:00Z"
  
  rewrite_metadata:
    original_delta: { ... }
    rewritten_delta: { ... }
    rewrite_reason: "T_elastic_repair"  # or T_compliance
    rewrite_authority: "ostar-operator"  # or ostar-governor
  
  projection:
    form: "O*"
    delta_form: "construct8_planck_cell_rewritten"
    closure_signature: "blake3_of_rewritten_delta"
  
  receipt_lineage:
    truex_gates_passed: [1, 2, 3, 4, 5, 6, 7]
    blake3:
      original_envelope_hash: "blake3_of_original"
      rewrite_hash: "blake3_of_rewrite_decision_and_deltas"
      combined_hash: "blake3_of_original_hash || rewrite_hash || timeline_hash"
    
    causal_timeline:
      - epoch: 1
        event: "envelope_received"
        ts: "2026-06-01T21:50:00Z"
      - epoch: 2
        event: "gates_validation_completed"
        ts: "2026-06-01T21:50:30Z"
      - epoch: 3
        event: "rewrite_decision_made"
        ts: "2026-06-01T21:50:45Z"
      - epoch: 4
        event: "rewrite_executed"
        ts: "2026-06-01T21:50:55Z"
      - epoch: 5
        event: "rewrite_receipt_issued"
        ts: "2026-06-01T21:51:00Z"
    
    lineage_proof: "hsm_ed25519_or_ecdsa_signature_128chars"
```

### 4.4 Queue Receipt (QUEUE Path)

When external approval or policy review is required:

```yaml
truex_queue_receipt:
  id: "receipt_uuid"
  authority: "truex"
  source_envelope_id: "envelope_id"
  
  decision:
    status: "QUEUE"
    decision_code: 0x03
    timestamp: "2026-06-01T21:51:00Z"
  
  queue_metadata:
    reason: "Awaiting policy review"
    external_condition: "policy_approval_gate"
    queue_depth: 3
    estimated_wait: "PT1H30M"
    queue_timestamp: "2026-06-01T21:51:00Z"
  
  receipt_lineage:
    truex_gates_passed: [1, 2, 3, 4, 5, 6, 7]
    blake3:
      envelope_hash: "blake3_of_original_envelope"
      queue_decision_hash: "blake3_of_queue_reason_and_condition"
    
    causal_timeline:
      - epoch: 1
        event: "envelope_received"
        ts: "2026-06-01T21:50:00Z"
      - epoch: 2
        event: "gates_validation_completed"
        ts: "2026-06-01T21:50:30Z"
      - epoch: 3
        event: "queue_decision_made"
        ts: "2026-06-01T21:51:00Z"
    
    lineage_proof: "hsm_ed25519_or_ecdsa_signature_128chars"
  
  terminal: false  # Queued envelopes may be dequeued later
```

---

## V. ROUTING AFTER ADMISSION

### 5.1 Routing Table

Once Truex issues an **admission receipt** (decision = ADMIT), the consequence cell is routed to Blue River Dam for conformance audit:

```
Admitted Receipt (decision = ADMIT)
    ↓
  Routing Gateway
    ├─ consequence_type = ProcessConsequence?
    │   ├─ YES → Route to wasm4pm evidence adapter
    │   └─ NO → Check next type
    │
    ├─ consequence_type = WorldStateRepresentation?
    │   ├─ YES → Route to coordinate-system alpha validator
    │   └─ NO → Check next type
    │
    ├─ consequence_type = CausalToken?
    │   ├─ YES → Route to trust-object registry
    │   └─ NO → Check next type
    │
    └─ consequence_type = CapitalFlow?
        ├─ YES → REFUSE (should have been caught at Truex gate 4!)
        └─ NO → Route to default conformance auditor
```

### 5.2 Blue River Dam Entry

The admitted consequence cell enters Blue River Dam with:
- Complete receipt lineage from Truex
- All seven gates passed
- Marked as durable and replay-eligible
- `promotion_eligible = false` (will become true after dam gates pass)

---

## VI. EXAMPLE FLOW (CONSTRUCT8 → TRUEX → BRD)

### 6.1 Example: Coordinate-System Alpha Witness

**Step 1: CONSTRUCT8 Produces Witness**
```
GraphPlayer observes synthetic tick stream
  → Constructs RelationBreak state
  → Creates Planck-cell delta (4 triples):
    - s:market_123 p:observes_relation_break true
    - s:market_123 p:price 150.25
    - s:graph_state_456 p:has_basis_size 5
    - s:logic_state_789 p:has_basis_size 3
  → BLAKE3 hash: 5548b5fcac...
  → Emits C8Receipt with signature
  → Emits AdversaryGapWitness { gap_score: 2, ... }
```

**Step 2: Envelope Created**
```
CONSTRUCT8 wrapper creates receipt envelope
  → Bundles Planck-cell delta + C8Receipt + AdversaryGapWitness
  → Creates OCEL 2.0 event log of witness derivation
  → Computes ocel2_batch_hash
  → Computes receipt_hash = BLAKE3(admission_status || ocel2_batch_hash || path_hash)
  → Sets admission_status = "ReceiptAdmitted"
  → Sends to Truex
```

**Step 3: Truex Admission**
```
Truex Hook receives envelope
  → Gate 1 (Type check): Schema valid ✓
  → Gate 2 (Guard check): admitted_status = "ReceiptAdmitted" ✓
  → Gate 3 (Transition check): receipt_hash not in ledger ✓
  → Gate 4 (Policy check): No CapitalFlow type ✓
  → Gate 5 (Capability check): Hook authority registered ✓
  → Gate 6 (Freshness check): Timestamp drift < 5 sec ✓
  → Gate 7 (Receipt check): All hashes verify ✓
  → All gates PASS
  
Truex Operator
  → Creates consequence cell with decision = ADMIT
  → Projections to O* (Planck-cell delta is already O*)
  → Computes admission receipt with combined_hash
  → Signs with HSM key
  → Stores in mailbox (durable)
  → Issues admission receipt
```

**Step 4: Replay Stability**
```
Truex replays the cell 3 times
  → Replay 1: BLAKE3(...) = 5548b5fcac...
  → Replay 2: BLAKE3(...) = 5548b5fcac...
  → Replay 3: BLAKE3(...) = 5548b5fcac...
  → All identical → stability_proven = true
```

**Step 5: Accounting Verification**
```
Truex verifies P-invariant
  → y^T · M_before = [p_source]
  → y^T · M_after = [p_sink]
  → Equation holds → accounting_conserved = true
```

**Step 6: Blue River Dam Entry**
```
Admission receipt sent to dam
  
Dam Gates:
  → Gate 1 (Soundness): WF-net sound? YES ✓
  → Gate 2 (Reachability): Reachability graph bounded? YES ✓
  → Gate 3 (Fitness): fitness(σ, N) ≥ 0.95? YES (0.98) ✓
  
  Auditor observation:
    "Coordinate-System Alpha enables finer state distinctions.
     The graph-based representation achieves higher conformance
     fitness because it can name states the logic-based model cannot.
     Gap-score = 2 validated."
```

**Step 7: Promotion to Trust Object**
```
wasm4pm-compat typestate verification passes
  → Type system proves no illegal transitions
  → Bytecode proof inlined
  
Cell becomes TRUST_OBJECT
  → Promotion eligible = true
  → Can be routed to process mining, evidence aggregation, governance
```

---

## VII. IMPLEMENTATION CHECKLIST

### Truex Hook Implementation
- [ ] Parse receipt envelope (JSON)
- [ ] Validate seven gates (type, guard, transition, policy, capability, freshness, receipt)
- [ ] Issue admission receipt (decision = ADMIT) or refusal receipt (decision = REFUSE)
- [ ] Store in durable mailbox
- [ ] Compute BLAKE3 combined_hash
- [ ] Sign with HSM key (Ed25519 or ECDSA)
- [ ] Prevent capital flows (gate 4 policy check)

### Truex Operator Implementation
- [ ] Implement replay validation (N=3 replays, all hashes identical)
- [ ] Implement P-invariant verification (y^T · M_before = y^T · M_after)
- [ ] Mark cell as STABLE and ACCOUNTED
- [ ] Track promotion eligibility (only after all stages complete)

### Blue River Dam Entry Implementation
- [ ] Accept admitted receipts from Truex
- [ ] Enforce Gate 1 (soundness), Gate 2 (reachability), Gate 3 (fitness ≥ 0.95)
- [ ] Route to conformance auditor
- [ ] Issue conformance receipt

### wasm4pm-compat Integration
- [ ] Accept conformance-checked cells
- [ ] Verify typestate (compile-time proof)
- [ ] Mark as TYPEVERIFIED
- [ ] Promote to trust object

---

## VIII. SEALING STATEMENT

This contract specifies the **complete adapter interface** through which CONSTRUCT8 witness outputs are admitted through Truex and routed to Blue River Dam.

**Key guarantees:**

1. **CONSTRUCT8 is a witness engine, not an execution engine:** Emits bounded deltas, BLAKE3 receipts, and representational witnesses. Does not trade, predict, or execute capital flows.

2. **Truex is a consequence cell admission membrane:** Applies seven independent gates (type, guard, transition, policy, capability, freshness, receipt). Refuses unreceipted claims and capital flows at the boundary.

3. **Receipt is the authority surface:** Every admitted cell carries a complete receipt lineage. No cell crosses the dam boundary without BLAKE3-sealed proof.

4. **Refusal is lawful terminality:** Cells that fail any gate are refused with a specific reason code. Refusal is terminal unless explicit policy override is authorized.

5. **Routing enforces consequence type:** Process evidence is routed to wasm4pm. Capital flows are refused (should not reach dam). Representational witnesses are routed to coordinate-system validators.

**Authority:** Truex Receipt Contract v1 + CONSTRUCT8 Witness Output Spec  
**Status:** AUTHORITATIVE  
**Date:** 2026-06-01  
**Next Phase:** Implement Telco boundary communication to prevent authority smuggling from external systems.
