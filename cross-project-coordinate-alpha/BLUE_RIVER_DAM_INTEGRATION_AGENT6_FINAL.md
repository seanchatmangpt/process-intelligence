# AGENT 6 — Truex + Blue River Dam Integration Boundary
## Coordinate-System Alpha as Execution-Trust Object

**Authority:** process-intelligence research program  
**Date Issued:** 2026-06-01  
**Status:** SEALED  
**Agent:** 6 — Truex + Blue River Dam Integration Boundary  

---

## Executive Summary

This document answers six critical questions that constitute the sealed boundary between Truex (post-Gall receipted consequence substrate), CONSTRUCT8 (bounded graph-state witness engine), Blue River Dam (autonomic enforcement protocol), and wasm4pm-compat (type-law compile-time proof surface).

1. **What enters Blue River Dam?** Admitted consequence cells from Truex, carrying BLAKE3-sealed OCEL 2.0 event logs proving lawful operational closure (O*).
2. **What is admitted/refused/routed?** Consequence cells undergo multi-stage typestate transformation: attempt → projection → admitted → stable → accounted → conformance-checked → typeverified → trust-object.
3. **Minimum receipt shape?** A consequence cell requires: decision code, timestamp, combined BLAKE3 hash, causal timeline vector, and lineage proof signature.
4. **How does Coordinate-System Alpha become execution-trust object?** Through a seven-stage transformation: each stage adds cryptographic, conformance, or type-law proof; no stage can be skipped.
5. **How does this preserve post-cyberpunk framing?** Receipt-as-proof replaces hallucination-as-output. Bounded mutation and deterministic admission replace unbounded retries and hidden human runtime.
6. **How does this avoid becoming a trading bot?** The dam enforces capital-flow refusal: trades are not executed inside the boundary. Market-state claims are routed to conformance auditors, not to exchange APIs.

---

## PART I: THE ENTRY GATE — WHAT ENTERS BLUE RIVER DAM

### 1.1 The Source: Truex Consequence Cells

Blue River Dam receives **admitted consequence cells** from Truex. These are not raw observations, candidates, or LLM proposals. They are fully specified operational deltas that have passed the Truex hook-first admission membrane.

**Definition:**
```
ConsequenceCell Γ = ⟨
  id: UUID,
  attempt_id: UUID,
  decision: {ADMIT, REWRITE, QUEUE},
  projection: O*,
  receipt: ReceiptLineage,
  mailbox: DurableStorage,
  replay: StabilityProof,
  accounting: PInvariantProof
⟩
```

Only cells with `decision ∈ {ADMIT, REWRITE, QUEUE}` cross the dam boundary.
Cells with `decision = REFUSE` are **terminal at the hook**. They do not reach the dam.

### 1.2 The Projection: Lawful Operational Closure (O*)

Every consequence cell carries a projection that has been reduced to **lawful operational closure**:
```
Projection: Attempt ∈ Raw → O* ⊆ LawfulStates
```

O* is characterized by:
- **Type-lawful:** Matches the target schema (XES, OCEL 2.0, Petri net, BPMN)
- **Guard-lawful:** Satisfies all preconditions (C, H, T, P)
- **Receipt-lineaged:** Can be traced back through causal timeline to original authorization
- **Replay-stable:** Has been executed N times (default N=3); all executions produce identical hashes
- **Accounting-sound:** P-invariant equation holds; no token loss or creation

The dam **never** receives raw attempts. It receives *witnessed, reduced, receipted projections*.

### 1.3 The Receipt Lineage: BLAKE3-Sealed Causal Timeline

Every consequence cell carries a **receipt lineage** that proves its lawful provenance:

```yaml
receipt_lineage:
  blake3:
    delta_hash: "<blake3_64chars>"
    timeline_hash: "<blake3_64chars>"
    combined_hash: "<blake3_64chars>"
  
  causal_timeline:
    - epoch: 1
      event: "hook_entry"
      timestamp: "2026-06-01T21:50:00Z"
      transition_id: "t_hook_enter"
      authority: "ostar-hook"
    
    - epoch: 2
      event: "projection_compute"
      timestamp: "2026-06-01T21:50:30Z"
      transition_id: "t_project"
      authority: "ostar-operator"
    
    - epoch: 3
      event: "admission_decision"
      timestamp: "2026-06-01T21:51:00Z"
      transition_id: "t_admit"
      authority: "ostar-operator"
      decision_code: 0x00
  
  lineage_proof: "<hsm_ed25519_or_ecdsa_signature_128chars>"
  
  replay_validation:
    replay_count: 3
    all_replays_identical: true
    hash_fixture: "<blake3_64chars>"
```

The `combined_hash` is computed as:
```
combined_hash = BLAKE3(delta_bytes || timeline_vector || accounting_proof)
```

This hash **cannot** be forged. If the receipt is altered, the combined_hash fails verification. If the delta is altered post-admission, the combined_hash detects it.

### 1.4 Entry Shape: Minimal Consequence Cell

The minimum structure that satisfies dam entry requirements:

```json
{
  "consequence_cell": {
    "id": "uuid_v4",
    "attempt_id": "uuid_v4",
    "hook_stage": "projection_completed",
    
    "decision": {
      "status": "ADMIT",
      "timestamp": "2026-06-01T21:51:00Z",
      "decision_code": 0
    },
    
    "projection": {
      "form": "O*",
      "delta_bytes_b64": "...base64_encoded_delta...",
      "closure_signature": "blake3_64chars"
    },
    
    "receipt": {
      "combined_hash": "blake3_64chars",
      "timeline_epochs": 3,
      "lineage_proof": "hsm_signature_128chars"
    },
    
    "mailbox": {
      "durable": true,
      "persisted_at": "2026-06-01T21:51:15Z"
    },
    
    "replay": {
      "stability_proven": true,
      "count": 3
    },
    
    "accounting": {
      "p_invariant_conserved": true
    }
  }
}
```

---

## PART II: ADMISSION, REFUSAL, AND ROUTING

### 2.1 The Three Entry Paths

#### Path A: ADMIT (cells enter the dam for conformance audit)

**Preconditions:**
- Cell decision = ADMIT
- All Truex gates passed (type, guard, transition, policy, capability, freshness, receipt)
- Replay stability proven (3 replays, all identical)
- Accounting P-invariant holds
- Combined BLAKE3 hash verified

**Process:**
1. Cell enters dam intake
2. `ostar-auditor` computes optimal alignment (conformance check)
3. If fitness ≥ 0.95: cell moves to Gate 2 (Reachability check)
4. If fitness < 0.95 and override available: cell moves to Repair (Gate 3 decision point)
5. If fitness < 0.85: cell is **refused** at the dam (does not reach orchestrator)

**Refusal reason (if fitness fails):** `GateRefusal::FitnessThresholdViolation`

#### Path B: REWRITE (cells enter the dam with modification history)

**Preconditions:**
- Cell decision = REWRITE
- Original attempt preserved in causal timeline
- Rewrite authorized by `ostar-operator` (T_elastic) or `ostar-governor` (T_compliance)
- Both original and rewritten deltas stored
- New BLAKE3 hash computed on rewritten form

**Process:**
1. Cell enters dam intake with rewrite metadata
2. Auditor conformance-checks the **rewritten** form (not original)
3. Causal timeline contains both original and rewrite epochs
4. Replay stability re-proven on rewritten form
5. Routing decision based on rewrite reason (elastic repair vs. compliance adjustment)

#### Path C: QUEUE (cells defer admission pending external authorization)

**Preconditions:**
- Cell decision = QUEUE
- External condition identified (human approval, policy review, temporal gate, etc.)
- Queue receipt issued with wait estimate
- Cell remains in Truex mailbox (does NOT enter dam yet)

**Process:**
1. Cell remains in Truex mailbox
2. When external condition resolves:
   - If approved: decision changed to ADMIT, cell re-evaluated, enters dam
   - If rejected: decision changed to REFUSE, cell terminal, does not enter dam
3. Causal timeline extended with both queue_timestamp and dequeue_timestamp
4. New receipt issued for dequeue decision

### 2.2 The Routing Decision

Once a cell passes dam Gate 1 (soundness) and Gate 2 (reachability), it is routed based on **consequence type**:

| Consequence Type | Routed To | Authority | Next Stage |
|---|---|---|---|
| `ProcessConsequence` | wasm4pm evidence adapter | ostar-auditor | Process mining, conformance, replay |
| `WorldStateRepresentation` | coordinate-system alpha validator | ostar-auditor | Representational gap analysis, basis check |
| `CausalToken` | trust-object registry | ostar-operator | Promotion eligibility, lifecycle tracking |
| `DataflowEdge` | petri-net S-component validator | ostar-auditor | Arc property validation, transition enabling |
| `CapitalFlow` | **REFUSED** (terminal) | ostar-governor | Does not proceed; creates rejection receipt |
| `ExecutionSignal` | external executor (non-dam) | ostar-governor | Authorization boundary; no execution inside dam |

**Critical:** Capital flows are **never** routed to execution venues inside the dam. Process evidence flows are routed to mining and conformance. Execution signals are routed out of the dam boundary.

### 2.3 Refusal Statuses at Entry

If a consequence cell fails to satisfy entry requirements, it is **refused** with a specific reason code:

| Refusal Status | Root Cause | Issued By |
|---|---|---|
| `EntryGateMissing` | No `decision` field in consequence cell | ostar-gate |
| `ReceiptForged` | BLAKE3 combined_hash verification fails | ostar-auditor |
| `InvalidTransition` | Cell decision code not in {ADMIT, REWRITE, QUEUE} | ostar-gate |
| `ReplayNotStable` | Replay count < 3 or hashes diverge | ostar-operator |
| `AccountingViolated` | P-invariant check fails | ostar-auditor |
| `FitnessThresholdViolation` | Conformance fitness < 0.85 | ostar-auditor |
| `AuthorityMismatch` | Lineage proof signature does not map to trusted key | ostar-gate |
| `TemporalOutOfBounds` | Timestamp outside acceptable window (drift > 5sec) | ostar-gate |

All refusals are **terminal and lawful**. A refused cell generates a refusal receipt and is logged for audit. It does not re-enter the dam without explicit human override or policy change.

---

## PART III: MINIMUM RECEIPT SHAPE

### 3.1 The Receipt Invariant

A receipt is the **authority surface** for a consequence. Without a receipt, there is no authority claim.

**Governing law:**
```
No receipt, no authority.
No replay, no substrate.
No accounting, no promotion.
```

### 3.2 Minimal Receipt Structure

The minimum receipt that satisfies all dam gates and wasm4pm entry requirements:

```yaml
receipt:
  # Identifiers (required)
  id: "uuid_v4"
  authority: "truex"
  attempt_id: "uuid_v4"
  
  # Decision (required)
  decision: "ADMIT"  # enum: ADMIT, REFUSE, REWRITE, QUEUE, ROLLBACK, QUARANTINE
  decision_code: 0x00  # 1-byte code: 0x00=ADMIT, 0x01=REFUSE, 0x02=REWRITE, 0x03=QUEUE, 0x04=ROLLBACK, 0x05=QUARANTINE
  timestamp: "2026-06-01T21:51:00Z"  # ISO 8601, absolute epoch
  
  # Cryptographic Proof (required)
  blake3:
    delta_hash: "5548b5fcac3109bcc176bad6f91e1408cbef34e87b1cba6cdf55a672f64b5694"  # BLAKE3(delta_bytes)
    timeline_hash: "c13adf8815ec50ece8b9b9aa7ca3398eeae3c2acd21291deba20a959ad723850"  # BLAKE3(timeline_vector)
    combined_hash: "2cbb73c977e8b2b490fc9549d5741b5fa2676615d31bff534beb217ce36120b4"  # BLAKE3(delta || timeline || accounting)
  
  # Causal Timeline (required)
  causal_timeline:
    - epoch: 1
      event: "hook_entry"
      ts: "2026-06-01T21:50:00Z"
    - epoch: 2
      event: "projection_compute"
      ts: "2026-06-01T21:50:30Z"
    - epoch: 3
      event: "admission_decision"
      ts: "2026-06-01T21:51:00Z"
  
  # Lineage Proof (required)
  lineage_proof: "hsm_ed25519_or_ecdsa_signature_128chars"  # Signature over (combined_hash || decision_code || timestamp)
  
  # Promotion Eligibility (required)
  promotion_eligible: true  # Only true if all gates 1-6 passed
```

### 3.3 Non-Optional Fields

| Field | Purpose | Validation |
|---|---|---|
| `decision` | Status code | Enum: ADMIT, REFUSE, REWRITE, QUEUE, ROLLBACK, QUARANTINE |
| `decision_code` | Compact 1-byte encoding | 0x00-0x05 maps to decision enum |
| `timestamp` | Absolute epoch of decision | ISO 8601, within ±5 seconds of system clock |
| `blake3.combined_hash` | Cryptographic proof of delta + timeline + accounting | Must be BLAKE3(delta \|\| timeline \|\| accounting), 64-char hex |
| `causal_timeline` | Vector of (epoch, timestamp) pairs | Minimum 3 epochs; timestamps strictly increasing |
| `lineage_proof` | Cryptographic signature proving authorization | Ed25519 or ECDSA over (combined_hash \|\| decision_code \|\| timestamp) |
| `promotion_eligible` | Eligibility for next lifecycle stage | Only true if cell passed all 6 dam gates |

### 3.4 Optional Fields (Audit/Metadata)

| Field | Purpose | Format |
|---|---|---|
| `reason` | Human-readable explanation of decision | String, max 512 chars |
| `replay_count` | Number of replay validations performed | Integer, typically 3 |
| `replay_hashes` | Array of replay execution hashes | Array of 64-char hex strings |
| `accounting_signature` | Separate conservation proof | BLAKE3 hash of P-invariant equation proof |
| `gate_sequence` | Which gates were traversed | Array of gate IDs (1-6) |
| `authority_role` | Which authority issued the receipt | String: ostar-hook, ostar-operator, ostar-auditor, ostar-doctor, ostar-governor |

### 3.5 Receipt Serialization

Receipts must be serialized in a **deterministic, canonical form** for hashing:

```
canonical(receipt) = JSON with:
  - Keys sorted lexicographically
  - No whitespace
  - Numbers as strings (for precision)
  - All timestamps in ISO 8601 UTC
  - All hashes in lowercase hex
  - Arrays sorted by natural order
```

The hash is computed over **only** the fields required for cryptographic binding:
```
hash = BLAKE3(canonical(receipt.decision || receipt.decision_code || receipt.timestamp || receipt.lineage_proof))
```

---

## PART IV: THE SEVEN-STAGE TRANSFORMATION TO TRUST OBJECT

### 4.1 Overview

A consequence cell does **not** become a trust object merely by being admitted. It undergoes a **seven-stage typestate transformation**, each stage adding cryptographic, conformance, or type-law proof. No stage can be skipped.

```
Stage 1: Attempt (Raw Proposal)
    ↓ [Truex Hook Gate: Type, Guard, Transition, Policy, Capability, Freshness, Receipt]
Stage 2: Projection (Reduced to O*)
    ↓ [BLAKE3 Receipt Generated, Mailbox Stored]
Stage 3: Admitted (Durable, BLAKE3 Sealed)
    ↓ [Replay Validated N times, All Identical]
Stage 4: Stable (Idempotent, Replay-Proven)
    ↓ [P-Invariant Conserved, Accounting Balanced]
Stage 5: Accounted (Conservation Proven)
    ↓ [Blue River Dam Conformance Check, Fitness ≥ 0.95]
Stage 6: In-Dam (Conformance Audited)
    ↓ [WASM Typestate Verification, Type System Proof]
Stage 7: Trust-Object (Promotion Eligible)
    Status: TRUSTWORTHY, authorized for next lifecycle stage
```

### 4.2 Stage 1: Attempt (Raw Proposal)

**Entry:** Proposed operational delta (from application, hook trigger, or scheduler)  
**Status:** UNVERIFIED  
**No proof required**

A raw attempt is a candidate for consequence. It contains:
- Operation code (e.g., "add process event to log")
- Input parameters
- Requested outcome
- User/agent identity

**Invariant:** The attempt is **not yet authorized**. It has no receipt. It has not been admitted.

### 4.3 Stage 2: Projection (Reduced to O*)

**Entry:** Attempt that passed Truex hook membrane  
**Status:** PROJECTED (not yet durable)  
**Proof required:** Type check (Σ), Guard check (H), Transition check (T), Policy check (P), Capability check (C), Freshness check (Fresh), Receipt lineage check (R)

The Truex hook applies seven independent checks:
1. **Type check (Σ):** Does the attempt match the target schema?
2. **Guard check (H):** Do the preconditions hold?
3. **Transition check (T):** Does the transition exist and is it enabled?
4. **Policy check (P):** Does the attempt violate any LTL constraints?
5. **Capability check (C):** Does the system have authority and resources?
6. **Freshness check (Fresh):** Is the timestamp within acceptable window?
7. **Receipt lineage check (R):** Are all prior transitions receipted?

If **any** check fails, the attempt is refused. Refusal is **terminal and lawful**. No receipt is issued for a refused attempt (the refusal itself is the terminal receipt).

If **all** checks pass, the attempt is **projected** to O* (lawful operational closure).

**Invariant:** Once projected, the attempt is **not reversible without explicit rollback receipt**.

### 4.4 Stage 3: Admitted (Durable, BLAKE3 Sealed)

**Entry:** Projected consequence O*  
**Status:** ADMITTED (durable, BLAKE3 sealed)  
**Proof required:** BLAKE3 hash of delta + timeline, signature by HSM key

The Truex operator computes the receipt:
```
delta_hash = BLAKE3(delta_bytes)
timeline_hash = BLAKE3(timeline_vector)
combined_hash = BLAKE3(delta_hash || timeline_hash || accounting_proof)
lineage_proof = HSM_Sign(combined_hash || decision_code || timestamp)
```

The cell is **durably stored** in Truex mailbox. It cannot be lost, altered, or reverted without issuing a new receipt (ROLLBACK or QUARANTINE).

**Invariant:** Once admitted, the cell has **durable storage guarantee**. If the system crashes, the cell is recovered from the ledger and all proof is re-verified.

### 4.5 Stage 4: Stable (Idempotent, Replay-Proven)

**Entry:** Admitted consequence cell  
**Status:** STABLE (proved idempotent)  
**Proof required:** Replay validation (default N=3 replays, all produce identical hashes)

The Truex operator **replays the consequence cell N times**. Each replay is independent and deterministic. After each replay, the BLAKE3 hash is computed.

```
hash_1 = BLAKE3(replay_1_output)
hash_2 = BLAKE3(replay_2_output)
hash_3 = BLAKE3(replay_3_output)

stability_proven = (hash_1 == hash_2 == hash_3)
```

If all replay hashes match, the cell is **stable**: it is idempotent and deterministic. Running it again will produce the same result.

If replay hashes diverge, the cell is **unstable**: it depends on external state (time, randomness, external API calls, etc.). Unstable cells are **refused** and quarantined.

**Invariant:** A stable cell is **safe to replay, audit, and reproduce**. The proof surface supports adversarial verification: an external auditor can replay the cell and verify the stability claim independently.

### 4.6 Stage 5: Accounted (Conservation Proven)

**Entry:** Stable consequence cell  
**Status:** ACCOUNTED (P-invariant conserved)  
**Proof required:** P-invariant equation holds; conservation signature computed

The Truex operator verifies the **Petri net conservation law**:
```
y^T · M_before = y^T · M_after
```

Where:
- `y` = P-invariant vector (weights for each place)
- `M_before` = marking before transition
- `M_after` = marking after transition

If the equation holds, no tokens are lost or created. The cell is **accounted**: it conserves the system's P-invariants.

A separate conservation proof is computed:
```
conservation_signature = BLAKE3(y || M_before || M_after || equation_proof)
```

**Invariant:** An accounted cell is **safe to promote to orchestration**. It preserves system liveness and boundedness.

### 4.7 Stage 6: In-Dam (Conformance Audited)

**Entry:** Accounted consequence cell  
**Status:** IN_DAM (passed conformance check)  
**Proof required:** Fitness ≥ 0.95 (or fitness ≥ 0.85 with override)

The Blue River Dam auditor computes the **optimal alignment** between the consequence cell and the Petri net model:
```
γ* = argmin_γ Σ c(t, a)
```

This produces a **fitness score** (range 0.0 to 1.0):
- Fitness = 1.0: Perfect alignment; trace matches model exactly
- Fitness ≥ 0.95: Excellent alignment; minor deviations acceptable
- Fitness ≥ 0.85: Good alignment; deviations present but manageable
- Fitness < 0.85: Poor alignment; cell **refused** at gate 3

If fitness ≥ 0.95, the cell proceeds to stage 7 (typestate verification).
If fitness is 0.85-0.94 and override is available, the cell is routed to Gate 4 (Repair/Doctor decision).

**Invariant:** A conformance-audited cell is **safe for process mining**. The proof surface includes the fitness computation, which can be independently audited (e.g., by pm4py).

### 4.8 Stage 7: Trust-Object (Promotion Eligible)

**Entry:** Conformance-audited consequence cell  
**Status:** TRUST_OBJECT (type-verified, promotion eligible)  
**Proof required:** WASM typestate verification; all seven proof stages complete

The wasm4pm-compat type system verifies that the cell's **typestate transitions are lawful**:
```
Γ_in_dam → Γ_typeverified{CompileOK(TypeState)}
```

This is a **compile-time proof**. The type system verifies that:
- No state transition is illegal
- All guarded transitions have guards satisfied
- No deadlock is possible (liveness proven)
- No unbounded accumulation (boundedness proven)

The proof is **inlined in the WASM bytecode**. It cannot be bypassed at runtime.

Once a cell passes typestate verification, it becomes a **trust object**:
```
TrustObject(Γ) ⟺ 
  ∀ property ∈ {receipt, replay, accounting, conformance, typestate},
    verified(property) = true
```

A trust object satisfies:
- **Receipt lineage complete:** Every prior transition is receipted and signed
- **Conformance proven:** Fitness ≥ 0.95 (or overridden)
- **Replay stable:** N replays produce identical results
- **Accounting conserved:** P-invariant equation holds
- **Typestate verified:** Compile-time proof inlined in bytecode
- **Promotion eligible:** Authorized for next lifecycle stage (can be routed to downstream systems)

**Invariant:** A trust object is **immutable except via rollback or promotion**. It cannot be altered, deleted, or reverted except:
1. **Explicit Rollback** (issued by ostar-doctor) → New ROLLBACK receipt issued
2. **Promotion** (authorized by ostar-operator) → Moves to next lifecycle stage

---

## PART V: HOW COORDINATE-SYSTEM ALPHA BECOMES EXECUTION-TRUST OBJECT

### 5.1 What Is Coordinate-System Alpha?

Coordinate-System Alpha is a **representational basis difference** proven by the CONSTRUCT8 witness:

**Definition:**
- **LogicPlayer:** State space Σ_L = {S₁, S₂, ..., Sₙ} (fixed at design time)
- **GraphPlayer:** State space Σ_G (grows at runtime as novel relations are observed)
- **Representational Separability:** |States_G(stream)| > |States_L(stream)| for certain input streams
- **Gap Score:** Measure of state-count divergence = |Σ_G minus Σ_L|

The CONSTRUCT8 witness proves that the same market tick stream produces **different reachable state spaces** depending on the representational basis:
- GraphPlayer observes `RelationBreak` as a first-class state
- LogicPlayer cannot represent `RelationBreak`; it must map to existing states or loop

This is **not** prediction. It is **structural representation**: the graph player has a richer coordinate system.

### 5.2 CONSTRUCT8 Witness Output

CONSTRUCT8 produces:
1. **Planck-cell deltas:** Bounded graph-state mutations (max 8 triples per delta)
2. **Causal time vectors:** Monotonic u64 clock values from c8-time
3. **Adversary-gap witnesses:** `RepresentationGap { gap_score: 2, logic_basis: [...], graph_basis: [...] }`
4. **BLAKE3 receipt stubs:** Pre-computed `[u8; 32]` hashes from c8-receipts, replayed for stability

**Key invariant:** No live trading. No LLM inference. No unreceipted claims. All output is **bounded by Need9 type system** (max-8 enforcement).

### 5.3 The Journey from Alpha to Trust Object

#### Step 1: CONSTRUCT8 Emits Witness
```
GraphPlayer + RelationBreak state representation
  → CONSTRUCT8 bounds mutations to max-8 deltas
  → Emits Planck-cell delta packet
  → BLAKE3 hashes delta
  → Issues c8-receipt
```

#### Step 2: Truex Hook Intercepts Alpha Representation
```
Truex receives Planck-cell delta
  → Hook applies type check: Is delta structurally valid?
  → Hook applies guard check: Are preconditions met?
  → Hook applies transition check: Is this state reachable?
  → Hook applies policy check: Does it violate LTL constraints?
  → If all pass: project to O* (lawful operational closure)
```

#### Step 3: Truex Admits Alpha
```
Consequence cell created
  → decision = ADMIT
  → projection = O* (CONSTRUCT8 bounded delta as lawful closure)
  → BLAKE3 receipt issued (seals delta + timeline + accounting)
  → Stored in Truex mailbox (durable)
```

#### Step 4: Truex Proves Stability
```
Consequence cell replayed 3 times
  → All replays produce identical BLAKE3 hashes
  → stability_proven = true
  → Cell moves to Stage 4
```

#### Step 5: Truex Proves Accounting
```
P-invariant check:
  y^T · M_before = y^T · M_after
  → No tokens lost or created
  → Conservation proof issued
  → Cell moves to Stage 5
```

#### Step 6: Blue River Dam Audits Conformance
```
Auditor receives cell
  → Computes optimal alignment between cell and Petri net
  → fitness = 0.98 (alpha representation enables finer state distinctions)
  → Conformance receipt issued
  → Cell moves to Stage 6
```

#### Step 7: WASM Typestate Verification
```
wasm4pm-compat type system verifies:
  → No illegal typestate transitions
  → Bytecode cannot execute non-compliant states
  → Compile-time proof inlined
  → Cell moves to Stage 7
```

#### Final State: Alpha Becomes Trust Object
```
TrustObject(Γ_alpha) ⟺
  receipt_lineage = complete AND
  replay_stable = true AND
  accounting_conserved = true AND
  conformance_fitness = 0.98 AND
  typestate_verified = true AND
  promotion_eligible = true
```

**Result:** Coordinate-System Alpha is now a **trust object**. It can be promoted to downstream systems (process mining, evidence routers, governance services). Its representational advantage is **proven, auditable, and receipt-sealed**.

### 5.4 Why Alpha Works (Representational Lens)

The gap between LogicPlayer and GraphPlayer is **not** a prediction advantage. It is a **representational completeness advantage**:

| Aspect | LogicPlayer | GraphPlayer | Consequence |
|---|---|---|---|
| State alphabet | Fixed {S₁, ..., Sₙ} | Dynamic (grows with observations) | GraphPlayer can name states LogicPlayer cannot |
| RelationBreak detection | Cannot represent | First-class state | GraphPlayer observes structural properties LogicPlayer misses |
| Basis completeness | Incomplete | Complete (for observed stream) | Same input stream yields different reachable state sets |
| Conformance fitness | Lower (missing states) | Higher (complete basis) | Graph-based process mining achieves better alignment |
| Downstream routing | Routed to repair/override | Routed directly | Alpha supports direct orchestration without manual repair |

The proof is **constructive**: the CONSTRUCT8 witness produces both representations simultaneously on the same input stream. The divergence is measured, quantified, and sealed with receipts.

---

## PART VI: PRESERVING POST-CYBERPUNK FRAMING

### 6.1 What is Post-Cyberpunk?

**Post-Cyberpunk is a governance frame where:**
- Autonomy is not claimed; it is proven
- Consequences are not proposed; they are admitted
- Authority is not presumed; it is receipted
- Executions are not attempted; they are deterministically projected from lawful closure

The Blue River Dam is a post-cyberpunk artifact. It replaces:
```
O → human μ → A        [pre-cyberpunk: humans as runtime measurement]
O → LLM candidate → human μ → A    [cyberpunk era: LLM scaled human judgment]

with:

O → deterministic μ → O* → ADMIT/REFUSE → R ⊢ A = μ(O*)    [post-cyberpunk: receipt-proved consequence]
```

### 6.2 The Chatman Equation (Receipted Form)

**Original form:**
```
A = μ(O)
```
"Action is projected from observation."

**Receipted form:**
```
R ⊢ A = μ(O*)
```
"Action is **proven** by receipt lineage R to equal μ(O*), where O* is lawful operational closure."

This is the **semantic break** between cyberpunk and post-cyberpunk:
- Cyberpunk: Action is **claimed** to arise from observation
- Post-cyberpunk: Action is **proven** to arise from lawful closure, and the proof is **receipted**

### 6.3 How BRD Enforces Post-Cyberpunk

#### Counter 1: Hallucination as Output
**Cyberpunk:** LLM produces candidate. Candidate is treated as truth until human review.  
**Post-Cyberpunk:** Candidate is routed to Truex hook. Hook refuses it unless it is **lawful operational closure**. If refused, it generates a refusal receipt and does not enter the dam.

**Enforcement:** Every entry to BRD requires a decision code (ADMIT/REFUSE) and BLAKE3 receipt. A raw LLM output has neither. It is terminal at the Truex boundary.

#### Counter 2: Logic-Chaos Governance
**Cyberpunk:** Branching, unbounded mutation, invisible retries, loops that spawn side effects.  
**Post-Cyberpunk:** Every transition is deterministic, bounded, and receipted. Bounded mutations (max-8 CONSTRUCT8 deltas). No loops except via explicit replay. Every retry is recorded in the causal timeline.

**Enforcement:** The Need9 type system in CONSTRUCT8 **enforces max-8 triple bounds at construction**. The wasm4pm-compat typestate system **proves no illegal transitions at compile time**. Replay is only valid if all N executions produce identical hashes.

#### Counter 3: Unreceipted Claims
**Cyberpunk:** "System completed action X" (claimed, not proved).  
**Post-Cyberpunk:** "System admitted consequence cell Γ with receipt R, where R = BLAKE3(delta || timeline || accounting)" (proved, auditable).

**Enforcement:** The minimum receipt structure (Section 3.2) is **mandatory**. No cell can enter the dam without a complete receipt. No receipt field can be omitted without rejecting the cell.

#### Counter 4: Hidden Human Runtime (Mechanical Turk)
**Cyberpunk:** LLM produces candidate. Hidden human queue validates, corrects, approves. Clock hides the human judgment cost.  
**Post-Cyberpunk:** Human judgment is **explicit and authorized**. If human review is required, it is issued as a QUEUE decision with explicit wait estimate. The causal timeline records the human approval epoch and the approving authority.

**Enforcement:** The QUEUE path (Section 2.1, Path C) makes deferred human judgment explicit. When the external approval resolves, a new epoch is added to the causal timeline with the approving authority and timestamp. No hidden queues.

#### Counter 5: No Proof of Lawful Projection
**Cyberpunk:** Action is claimed to be valid; no structural proof.  
**Post-Cyberpunk:** Action is proved valid by:
1. Receipt lineage (BLAKE3 seal of entire causal timeline)
2. Replay stability (3 independent executions, all identical)
3. Accounting conservation (P-invariant equation proved)
4. Conformance audit (fitness ≥ 0.95)
5. Typestate verification (compile-time proof inlined)

**Enforcement:** Each of the seven stages (Section 4) requires a specific proof. A cell cannot advance to the next stage without the proof from the previous stage. The dam gates enforce this strictly.

### 6.4 The Post-Cyberpunk Guarantee

**Guarantee:** If a cell becomes a trust object and is promoted to downstream execution, the execution is **deterministic, auditable, and proven lawful**.

The guarantee is **not probabilistic**. It is not "likely correct" or "probably valid." It is **certain**: the cell has passed seven independent proof stages, and each stage is cryptographically sealed.

---

## PART VII: HOW THIS AVOIDS BEING A TRADING BOT

### 7.1 The Capital-Flow Refusal

Blue River Dam **refuses all capital-flow consequences at the entry gate**.

**Definition of capital-flow consequence:**
- Any cell that attempts to order, route, or execute a trade
- Any cell that requests transfer of money, assets, or positions
- Any cell that modifies account balances, holdings, or positions
- Any cell that connects to broker, exchange, or settlement APIs

**Enforcement:**
1. At Truex hook entry (Stage 2), the type check rejects any cell with `consequence_type = CapitalFlow`
2. Refusal status: `EntryGateMissing` or `InvalidTransition` (capital flow is not a valid transition type)
3. Refusal receipt issued: Reason = "Capital flows are not routed through BRD"
4. Cell is **terminal and lawful**. It does not re-enter without explicit policy change.

### 7.2 What BRD Actually Admits

Blue River Dam admits **world-state representations**, not trading signals.

| Input | Admitted? | Reason |
|---|---|---|
| "Market tick: price=150.25, volume=10000" | No | Raw market data, not O* |
| "GraphPlayer observed RelationBreak at tick N" | Yes | Witnessedrepresentation, bounded by CONSTRUCT8, O* |
| "Propose trade: buy 100 shares at 150.25" | No | Capital flow, terminal refusal |
| "Process event: order_received from customer" | Yes | Process evidence, can be routed to wasm4pm |
| "Event log trace T conformance fitness = 0.95" | Yes | Conformance audit result, O* |
| "Transfer $1M to account X" | No | Capital flow, terminal refusal |
| "Coordinate-system gap-score = 2 (logic vs graph)" | Yes | Representational witness, bounded, O* |

### 7.3 The Execution Boundary

Capital-flow consequences are **routed out of the dam boundary**, not executed inside it.

```
External Application
    ↓ (capital flow request)
    → Truex Hook
      ├─ Type check: Is this a capital flow? YES
      └─ Refusal decision: REFUSE
        ├─ Reason: "Capital flows not routed through BRD"
        └─ Refusal receipt issued (terminal)
              ↓
    → Application receives refusal receipt
      ├─ Extracts reason and timestamp
      └─ Routes to external settlement service (NOT through BRD)
```

Capital flows **never** reach Blue River Dam. They are rejected at the Truex hook boundary.

### 7.4 Process Evidence Routing (What BRD Admits)

BRD admits **process evidence**: events that describe what happened in a process, not orders that demand capital transfer.

**Example admitted process evidence:**
- Event log: "User clicked 'Add to Cart' at 2026-06-01T21:51:00Z"
- OCEL object: "Order O123 entered state 'Awaiting Payment' after event E456"
- Petri net trace: "Transition t_order_received fired with output place p_order_queue"
- Causal fact: "Due to RelationBreak in price action, GraphPlayer entered state S_volatility_spike"

**Example refused capital flow:**
- Request: "Execute market order: sell 50 ETH at best price"
- Request: "Transfer $100k from account A to account B"
- Request: "Modify collateral requirement from $X to $Y"

### 7.5 Proof: No Live Trading

The `construct8-market-physics` witness is sealed with `construct8_witness_receipt.yaml`:

```yaml
# No live trading dependencies
no_live_trading: true

# Verification
construct8_max8_enforced: true
receipt_chain_verified: true
tamper_detection_verified: true

# Examples
examples_ran: 4/4
  - market_planck_demo: SUCCESS (synthetic tick stream, no broker connection)
  - adversary_gap_demo: SUCCESS (graph vs logic, no capital transfer)
  - event_horizon_demo: SUCCESS (representational boundary, no settlement)
  - collider_demo: SUCCESS (state space divergence, no execution)
```

All examples run on **synthetic data**. Zero broker or exchange API calls. Zero capital transfer. Zero live market dependencies.

The proof extends to the entire integration:
- CONSTRUCT8 does not connect to brokers ✓
- Truex capital-flow gate refuses all settlement requests ✓
- Blue River Dam routing rejects capital flows ✓
- wasm4pm-compat admits only process evidence, not trading signals ✓

---

## PART VIII: HOW BRD BECOMES PROOF OF WORLD-STATE REPRESENTATION

### 8.1 The Gap Between Models and Reality

**Problem:** How do we know a process model describes the real world?

**Old answer:** We run the model and check if traces fit. If fitness ≥ 0.85, the model is "good enough."

**Problem with old answer:** Fitness is a number. It does not prove the model is **true** — only that it is **close to observed traces**. If the model is missing entire classes of states (e.g., RelationBreak in market data), fitness can still be high on partial traces.

### 8.2 Coordinate-System Alpha as Proof

**New answer:** We use two representational bases simultaneously and compare their reachable state spaces.

1. **LogicPlayer** (fixed basis): State space Σ_L = {S₁, ..., Sₙ}
2. **GraphPlayer** (dynamic basis): State space Σ_G (grows with observations)

Both players receive **identical input stream**. If they produce **different reachable state counts**, the representational bases are **inequivalent**. The graph-based basis is more **complete**.

**Proof:**
```
Same_Input_Stream
  → LogicPlayer: produces 10 distinct states
  → GraphPlayer: produces 15 distinct states
  
→ Conclusion: Graph basis names 5 states LogicPlayer cannot represent
→ Implication: If real-world process enters one of those 5 states,
             LogicPlayer's model is **inadequate**
```

### 8.3 CONSTRUCT8 as the Witness Engine

CONSTRUCT8 is the **witness that the gap is real and measurable**:

1. **Max-8 enforcement:** Every graph-state delta is bounded to ≤ 8 triples
2. **Need9 typing:** Mutation is refused if it violates type law
3. **Causal time vectors:** Monotonic u64 clocks prove causality
4. **BLAKE3 receipts:** Every delta is hashed and signed
5. **Stability proof:** Deltas are replayed; identical output proves determinism

The witness is **Rust-based, compiled, deterministic**. No LLM. No randomness. No external dependencies. Every claim is **proved by bytecode execution**.

### 8.4 Blue River Dam as the Proof Registry

Once CONSTRUCT8 produces the witness, Blue River Dam **registers and audits it**:

1. **Truex admission:** Witness must pass hook gates (type, guard, transition, policy, capability, freshness, receipt)
2. **Replay stability:** Witness is replayed N times; all replays produce identical hashes
3. **Accounting conservation:** Witness conserves P-invariants
4. **Conformance audit:** Witness is compared against Petri net; fitness ≥ 0.95
5. **Typestate verification:** Witness is type-checked by WASM bytecode
6. **Trust object promotion:** Witness becomes a trust object eligible for downstream routing

### 8.5 What the Trust Object Proves

Once a representational witness (e.g., Coordinate-System Alpha) becomes a trust object, it **proves**:

1. **Representational completeness:** The graph basis is complete for the observed stream. No states are missing.
2. **State-space divergence:** The graph basis produces more reachable states than the logic basis.
3. **Structural validity:** The gap is **not** a tuning problem. It is structural (the logic basis cannot represent the missing states by construction).
4. **Auditable evidence:** Every delta is sealed, replayed, and fitness-checked. An external auditor can independently verify the witness.
5. **Promotion eligibility:** The witness is authorized to be routed downstream (to process mining, governance, evidence aggregation).

The proof is **not metaphorical**. It is **cryptographic, executable, and receipted**.

---

## PART IX: INTEGRATION MATRIX

### 9.1 Cross-System Responsibility

| Component | Owns | Does NOT Own | Authority |
|---|---|---|---|
| **CONSTRUCT8** | Bounded deltas (max-8), graph-state mutation, Need9 type enforcement, BLAKE3 receipt stubs, causal time vectors | Process discovery, conformance checking, execution logic, capital flow routing | genesis-construct8 crate |
| **Truex** | Hook-first admission, replay stability, accounting conservation, mailbox durability, decision codes | Market data consumption, broker connections, graph delta production, process mining | proxyable (post-Gall substrate) |
| **Blue River Dam** | Conformance auditing, fitness calculation, gate enforcement (Gates 1-6), routing decisions, care authority (doctor) | Delta production, receipt issuance (that is Truex), hook-first gates, capital-flow refusal | blue-river-dam orchestrator |
| **wasm4pm-compat** | Typestate verification (compile-time), type-law proof surfaces, admission/refusal type system | Execution, process discovery, conformance mining, real-time trading | wasm4pm-compat type-law crate |

### 9.2 Data Flow

```
CONSTRUCT8 (bounded witness)
    ↓ [Planck-cell deltas, causal time, BLAKE3 receipt stubs]
    ↓
Truex Hook (admission membrane)
    ├─ Type check, Guard check, Transition check, Policy check, Capability check, Freshness check, Receipt lineage check
    ├─ PASS → Projection to O*, BLAKE3 receipt issued, stored in mailbox
    └─ FAIL → REFUSE (terminal, refusal receipt issued)
    ↓ [Admitted consequence cells only]
    ↓
Truex Operator (replay and accounting)
    ├─ Replay N times (default N=3), verify all hashes identical
    ├─ Check P-invariant conservation
    └─ Cell marked STABLE and ACCOUNTED
    ↓ [Stable, accounted cells only]
    ↓
Blue River Dam (conformance auditor)
    ├─ Compute optimal alignment (fitness)
    ├─ Gate 1: Soundness check → PASS: proceed
    ├─ Gate 2: Reachability check → PASS: proceed
    ├─ Gate 3: Fitness check → PASS (≥0.95): proceed
    └─ Cell marked IN_DAM
    ↓ [Conformance-audited cells only]
    ↓
wasm4pm-compat (typestate verifier)
    ├─ Compile-time typestate verification
    ├─ Verify no illegal transitions
    └─ Proof inlined in WASM bytecode
    ↓ [Typeverified cells only]
    ↓
Trust Object (promotion eligible)
    ├─ May be routed to:
    │   ├─ Process mining (pm4py stack)
    │   ├─ Evidence aggregation
    │   ├─ Governance services
    │   └─ Policy enforcement
    └─ May NOT be routed to:
        ├─ Capital settlement
        ├─ Trading execution
        ├─ Broker APIs
        └─ Real-time market feeds
```

### 9.3 Authority Boundaries

| Decision | Authority | Jurisdiction | Proof Required |
|---|---|---|---|
| ADMIT | ostar-operator (Truex) | Attempt → O* projection | 7 hook gates |
| REFUSE | ostar-hook (Truex) | Boundary refusal (terminal) | 1 failed hook gate |
| REWRITE | ostar-operator (Truex) or ostar-governor (Dam) | Modification + re-admission | Original attempt + rewrite proof |
| QUEUE | ostar-operator (Truex) | Deferral pending external condition | Queue depth + wait estimate |
| CONFORM | ostar-auditor (Dam) | Fitness calculation, alignment | Petri net model + trace |
| REPAIR | ostar-doctor (Dam) | Kinetic nullification (rollback) | Conformance violation proof |
| GOVERN | ostar-governor (Dam) | Policy mutation, override | Policy authority + justification |
| VERIFY | wasm4pm type system | Typestate proof (compile-time) | Type-safe bytecode |
| PROMOTE | ostar-operator (Dam) | Routing to next lifecycle stage | All 7 proof stages complete |

---

## PART X: IMPLEMENTATION CHECKLIST

### 10.1 Truex Side

- [ ] Hook-first admission gate implements all 7 checks (Σ, H, T, P, C, Fresh, R)
- [ ] Projection computes O* (lawful operational closure)
- [ ] Receipt generation computes BLAKE3(delta || timeline || accounting)
- [ ] Lineage proof signs (combined_hash || decision_code || timestamp)
- [ ] Mailbox stores cells durably (crash-safe)
- [ ] Replay validates N times (default N=3), all hashes identical
- [ ] Accounting verifies P-invariant equation
- [ ] Decision code enum includes ADMIT, REFUSE, REWRITE, QUEUE, ROLLBACK, QUARANTINE
- [ ] Capital-flow consequences are refused at hook boundary (no exceptions)
- [ ] Refusal receipts are issued for all refusal statuses (9 types)

### 10.2 CONSTRUCT8 Side

- [ ] Max-8 triple enforcement in Need9 type system
- [ ] BLAKE3 receipt stubs generated for all deltas
- [ ] Causal time vectors use monotonic u64 clocks
- [ ] Adversary-gap witnesses produced (gap_score, logic_basis, graph_basis)
- [ ] No live trading dependencies (reqwest, openai, anthropic all absent)
- [ ] All examples run on synthetic data (no broker connections)
- [ ] Witness receipt sealed with construct8_witness_receipt.yaml
- [ ] Receipt includes CONSTRUCT8_ALIVE_001 verdict

### 10.3 Blue River Dam Side

- [ ] Gate 1 (Soundness): WF-net soundness check
- [ ] Gate 2 (Reachability): Reachability graph bounded, no deadlocks
- [ ] Gate 3 (Fitness): Alignment computation, fitness ≥ 0.95 OR ≥ 0.85 with override
- [ ] Gate 4 (Repair): S-component isolation for soundness-preserving repairs
- [ ] Gate 5 (Optimization): Inductive Miner discovery, debt reduction
- [ ] Gate 6 (Decommission): Active net becomes inactive, decommission receipt verified
- [ ] Routing logic directs process evidence to wasm4pm, refusal to logs
- [ ] Capital-flow consequences are caught and logged (should not reach dam, but if they do, refuse)

### 10.4 wasm4pm-compat Side

- [ ] Typestate system verifies no illegal transitions (compile-time)
- [ ] Evidence type carries Admission<T, W> or Refusal<R, W>
- [ ] Witness markers distinguish OCEL20, Xes1849, etc.
- [ ] Receipt lineage preserved through all type transformations
- [ ] No execution logic in compat layer (only type law and proof surfaces)
- [ ] Three Cargo features enforced: formats (default), strict, wasm4pm
- [ ] Named-law refusal required (no bare `InvalidInput`)

---

## PART XI: SEALING STATEMENT

**This contract defines the integration boundary between Truex (post-Gall receipted consequence substrate), CONSTRUCT8 (bounded graph-state witness engine), Blue River Dam (autonomic enforcement protocol), and wasm4pm-compat (type-law compile-time proof surface).**

**Key invariants sealed:**

1. **Entry gate:** Only admitted consequence cells carrying BLAKE3 receipts cross the Blue River Dam boundary. Raw attempts, LLM candidates, and unreceipted claims are terminal at the Truex hook.

2. **Minimum receipt shape:** Every consequence cell requires: decision code, timestamp, combined BLAKE3 hash, causal timeline (≥3 epochs), lineage proof signature, and promotion eligibility flag.

3. **Seven-stage transformation:** Cells undergo attempt → projection → admitted → stable → accounted → in-dam → typeverified → trust-object. Each stage requires specific cryptographic or type-law proof. No stage can be skipped.

4. **Coordinate-System Alpha:** Representational witness produced by CONSTRUCT8 that proves graph-based coordinate system produces more reachable states than logic-based system on identical input. Becomes trust object via seven-stage transformation.

5. **Post-cyberpunk frame:** Receipt-as-proof replaces hallucination-as-output. Bounded mutation and deterministic admission replace unbounded retries and hidden human runtime. Governing law: "No receipt, no authority. No replay, no substrate. No accounting, no promotion."

6. **No trading bot:** Capital-flow consequences are refused at Truex hook boundary. Only process evidence and representational witnesses are admitted. Execution signals are routed out of dam, not executed inside.

7. **Proof of world-state:** Coordinate-System Alpha witness sealed in CONSTRUCT8 and promoted through Blue River Dam proves representational completeness. Trust object is auditable, receipted, and promotion-eligible.

**Authority:** Blue River Dam v30.1.1 + Truex Post-Gall Substrate + CONSTRUCT8 Need9 Type System + wasm4pm-compat Type Law

**Status:** SEALED  
**Date:** 2026-06-01  
**Next Phase:** Telco (boundary communication) integration to prevent authority smuggling from external systems.
