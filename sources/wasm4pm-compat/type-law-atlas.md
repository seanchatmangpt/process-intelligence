# Type-Law Atlas: wasm4pm-compat Surface Inventory

**Version:** 30.1.1  
**Authority:** Discovery Agent  
**Classification:** Source Standard  
**Date:** 2026-05-31

---

## Executive Summary

The `wasm4pm-compat` type-law surface comprises five codependent legal domains that establish the structural, cryptographic, and conformance boundaries for process execution evidence within the WebAssembly-based process mining engine. This atlas inventories each domain, maps their interconnections, and identifies structural gaps against the doctrine that *if the code says it worked but the event log cannot prove a lawful process happened, then it did not work* (van der Aalst Constitution).

---

## 1. Evidence<T, State, Witness> Type Law

### 1.1 Core Structure

The `Evidence<T, State, Witness>` generic struct is the primary vessel for admissibility of process execution artifacts. All evidence admitted into the OCEL ledger must be wrapped in this type.

**Definition:**
```rust
pub struct Evidence<T, State, Witness> {
    pub payload: T,              // XesTrace, OcelEventLog, BpmnInstanceData
    pub state: State,            // PetriNetMarking, BpmnTokenConfiguration
    pub witness: Witness,        // WitnessState, ReplayReceipt, AlignmentProof
    pub epoch: u64,             // Monotonic counter, prevents replay attacks
    pub signature: IdentitySignature,  // Authority signature (Ed25519)
    pub hash: Blake3Hash,       // BLAKE3([payload || state || witness || epoch || sig])
}
```

**Type Parameters:**
- **`T`**: Implements `Serialize`. Must be a valid event log payload (XES, OCEL 2.0, BPMN instance data).
- **`State`**: Implements `Serialize`. Must represent a deterministic marking or configuration in the process model.
- **`Witness`**: Implements `Serialize` + `Lattice` trait. Represents alignment progress or token game replay proof.

### 1.2 Admissibility Axioms

Every instance of `Evidence<T, State, Witness>` must satisfy three strict, verifiable axioms to be admitted into the process ledger:

#### Axiom 1: Cryptographic Binding (Non-Forgeability)
$$\mathcal{H} = \text{BLAKE3}(\text{Serialize}(T) \parallel \text{Serialize}(State) \parallel \text{Serialize}(Witness) \parallel \text{Serialize}(\text{epoch}) \parallel \text{Serialize}(\text{sig}))$$

**Implication:** Any mutation of payload, state, witness, epoch, or signature invalidates the hash. The system must reject the evidence block.

**Enforcement:** 
- The `hash` field is computed at construction time.
- The host WASM runtime cannot modify `hash` post-construction.
- Admission requires `hash == compute_hash(self)`.

#### Axiom 2: Replay Soundness (Lattice Monotonicity)
For sequential evidence blocks $E_1 = \langle T, S_1, W_1 \rangle$ and $E_2 = \langle T, S_2, W_2 \rangle$ with transition activity $t$:

$$S_1 \xrightarrow{t} S_2 \quad \text{(valid state transition)}$$
$$W_1 \sqsubseteq W_2 \quad \text{(witness monotonic increase)}$$

**Implication:** Witness state can only move monotonically upward in the lattice. A witness cannot "go backward" or "refresh" to lower evidence. If $W_1$ contains proof that a transition $t$ fired, then $W_2$ must also contain that proof (possibly enriched with additional evidence).

**Enforcement:**
- Before admitting $E_2$, verify that `witness_2.join(witness_1) == witness_2` (lattice absorption property).
- If the join produces `Top` (contradiction), reject $E_2$ and halt processing.

#### Axiom 3: Signature Admissibility (Authority Binding)
$$\text{VerifySignature}(\text{PublicKey}_{\text{Authority}}, \text{sig}, \mathcal{H}) \equiv \text{True}$$

**Implication:** The signer must be an authorized auditor, runner, or proof-validator role. The signature binds the authority to the entire evidence block.

**Enforcement:**
- Maintain a registry of authorized public keys keyed by role.
- Reject evidence bearing a signature from an unknown or revoked key.
- The epoch field prevents signature replay across distinct execution contexts.

### 1.3 Receipt-Shaped Objects

An `Evidence<T, State, Witness>` becomes **receipt-shaped** when:
- `State` reaches a terminal marking (final place in a Petri Net, or final gateway state in BPMN).
- `Witness` reaches a state indicating complete trace replay with fitness ≥ threshold.
- The `hash` is signed by an auditor role (not just a runner).

**Receipt Invariant:** Once an evidence block is receipt-shaped, it must be immutable. Any attempt to re-process or modify it must be rejected at the type level.

---

## 2. Admission-Refusal Law

### 2.1 Admission Pathways

Evidence is admitted to the OCEL ledger only if **all** of the following hold:

| Criterion | Check | Refusal Signature |
|-----------|-------|-------------------|
| **Cryptographic Integrity** | BLAKE3 hash computes correctly | `HashMismatch` |
| **Temporal Monotonicity** | Event timestamps respect local WASM clock, no backwards drift | `TemporalAnomaly` |
| **Causal Connectivity** | All event references (`parent_id`, `object_id`) exist | `CausalDisconnect` |
| **Type Soundness** | Payload serialization matches schema, no buffer overflow attempts | `TypeViolation` |
| **Authority Signature** | Ed25519 signature valid against registered public key | `InvalidSignature` |
| **Witness Lattice Join** | New witness lattice entry does not conflict with previous (no Top) | `LatticeViolation` |

**Default Posture:** Deny admission until all checks pass.

### 2.2 Refusal Signatures (Terminal Rejections)

If any admission criterion fails, the evidence block is rejected with a terminal refusal signature:

- **`HashMismatch`**: The computed BLAKE3 does not match the stored hash. The evidence is corrupted or forged.
- **`TemporalAnomaly`**: Event timestamp precedes the previous event's timestamp in the WASM runtime's monotonic clock.
- **`CausalDisconnect`**: An event references a non-existent parent or object ID. The trace is incomplete or fragmented.
- **`TypeViolation`**: Buffer overflow attempt or type confusion in deserialization. Security boundary violation.
- **`InvalidSignature`**: Ed25519 signature fails verification. Authority is unknown or key is revoked.
- **`LatticeViolation`**: The witness join produces `Top` (contradiction). Conflicting evidence detected.

**Consequence of Refusal:** The evidence block is not admitted to the ledger. An audit trail entry is logged. If the refusal is due to a boundary-critical criterion (temporal anomaly, causal disconnect, lattice violation), the WASM runtime must initiate a graceful halt.

### 2.3 Boundary Control Matrix

```
┌─────────────────────────────────────────────────────────────────┐
│                    ADMISSION BOUNDARY                            │
├──────────────────────┬──────────────────────┬──────────────────┤
│  Criterion Group     │   Pass (Admit)       │   Fail (Refuse)  │
├──────────────────────┼──────────────────────┼──────────────────┤
│ Cryptographic        │ Hash computes        │ HashMismatch     │
│                      │ Signature valid      │ InvalidSignature │
├──────────────────────┼──────────────────────┼──────────────────┤
│ Temporal             │ Monotonic clock OK   │ TemporalAnomaly  │
│                      │ No skew > threshold  │                  │
├──────────────────────┼──────────────────────┼──────────────────┤
│ Causal Soundness     │ All refs exist       │ CausalDisconnect │
│                      │ No orphaned events   │                  │
├──────────────────────┼──────────────────────┼──────────────────┤
│ Type Safety          │ Schema match         │ TypeViolation    │
│                      │ No buffer overflow   │                  │
├──────────────────────┼──────────────────────┼──────────────────┤
│ Evidence Coherence   │ No lattice conflict  │ LatticeViolation │
│                      │ Witness ⊆ new        │                  │
└──────────────────────┴──────────────────────┴──────────────────┘
```

---

## 3. Loss Policy & Loss Report Law

### 3.1 Conformance Degradation Model

In adversarial or high-throughput environments, perfect trace fidelity is physically impossible. The Loss Policy defines the thermodynamic boundaries of acceptable evidence attrition.

**Alignment-Based Fitness:**

Given a trace $\sigma$ and model $N$, an optimal alignment $\gamma_{\text{opt}}$ minimizes the cost:
$$\gamma_{\text{opt}}(\sigma) = \arg\min_{\gamma} \sum_{(x, y) \in \gamma} w(x, y)$$

The fitness metric is:
$$\text{fitness}(L, N) = 1 - \frac{\sum_{\sigma \in L} L(\sigma) \cdot w(\gamma_{\text{opt}}(\sigma))}{\sum_{\sigma \in L} L(\sigma) \cdot w(\theta_{\text{worst}}(\sigma))}$$

where:
- **Sync-moves** $(a, t)$: cost 0 (perfect alignment).
- **Log-only moves** $(a, \gg)$: cost $w_L(a)$ (event not in model).
- **Model-only moves** $(\gg, t)$: cost $w_M(t)$ (transition not in log).

### 3.2 Loss Policy Structure

A `LossPolicy` defines the tolerance thresholds:

```rust
pub struct LossPolicy {
    pub max_log_moves_pct: f64,        // % of total events allowed as log-only moves
    pub max_model_moves_pct: f64,      // % of total transitions allowed as model-only moves
    pub min_fitness_threshold: f64,    // [0.0, 1.0] minimum acceptable fitness
    pub log_move_weight: f64,          // Cost weight for log-only moves
    pub model_move_weight: f64,        // Cost weight for model-only moves
}
```

**Permissible Loss (Degradation Allowed):**
- Metadata attrition when WASM memory utilization approaches 90%.
- Probabilistic sampling of rapidly repetitive state transitions (flood-state events).
- Minor timestamp drift within ±100ms bounds (local clock jitter).

**Absolute Unacceptable Loss (Terminal Rejection):**
- Loss of causal linkage between START and END states.
- Corruption of the cryptographic signature binding the evidence.
- Any violation of lattice monotonicity.

### 3.3 Loss Report Certification

A `LossReport` is emitted after replaying a trace:

```rust
pub struct LossReport {
    pub trace_id: String,
    pub log_moves: Vec<String>,        // List of log-only move activities
    pub model_moves: Vec<String>,      // List of model-only move transitions
    pub total_cost: f64,
    pub fitness: f64,                  // [0.0, 1.0]
    pub is_compliant: bool,            // fitness >= policy.min_fitness_threshold
}
```

**Compliance Evaluation:**

A trace is compliant if:
$$\text{fitness} \ge \text{policy.min\_fitness\_threshold} \,\wedge\, \frac{|\text{log\_moves}|}{|\text{events}|} \le \text{policy.max\_log\_moves\_pct} \,\wedge\, \frac{|\text{model\_moves}|}{|\text{transitions}|} \le \text{policy.max\_model\_moves\_pct}$$

**Receipt Binding:** The `LossReport` becomes part of the evidence trail. When signed by an auditor, it serves as a **conformance receipt** certifying that the trace fitness meets admissibility standards for M&A board claims.

---

## 4. Witness Lattice Law

### 4.1 Lattice Algebraic Structure

The witness lattice $(W, \sqsubseteq, \sqcup, \bot, \top)$ is a bounded join-semilattice that validates cumulative process execution evidence.

**Lattice Elements:**

A witness $w \in W$ is a tuple:
$$w = (L_{\text{sub}}, M_{\text{sub}}, \gamma_{\text{sub}})$$

where:
- $L_{\text{sub}}$: Observed event subsequence.
- $M_{\text{sub}}$: Path or marking in the process model.
- $\gamma_{\text{sub}}$: Alignment mapping between $L_{\text{sub}}$ and $M_{\text{sub}}$.

**Partial Order** ($\sqsubseteq$):

For $w_1 = (L_1, M_1, \gamma_1)$ and $w_2 = (L_2, M_2, \gamma_2)$:

$$w_1 \sqsubseteq w_2 \iff (L_1 \subseteq L_2) \,\wedge\, (M_1 \subseteq M_2) \,\wedge\, (\gamma_1 = \gamma_2|_{L_1})$$

**Interpretation:** $w_2$ contains all evidence in $w_1$, plus potentially more, and their alignments are consistent.

### 4.2 Join Operator Properties

The join $w_1 \sqcup w_2$ combines two evidence blocks:

**Valid Join (Normal Case):**
$$w_1 \sqcup w_2 = w_3 = (L_1 \cup L_2, M_1 \cup M_2, \gamma_1 \cup \gamma_2)$$

provided the alignments do not conflict.

**Join Axioms:**
1. **Idempotence**: $w \sqcup w = w$
2. **Commutativity**: $w_1 \sqcup w_2 = w_2 \sqcup w_1$
3. **Associativity**: $(w_1 \sqcup w_2) \sqcup w_3 = w_1 \sqcup (w_2 \sqcup w_3)$
4. **Absorption**: $w_1 \sqsubseteq w_2 \implies w_1 \sqcup w_2 = w_2$

**Conflict Resolution (Lattice Top):**

If the join operation detects conflicting evidence (e.g., the same event aligned to two different transitions, or a Petri Net marking exceeds boundedness), the join collapses to the **top element**:
$$w_1 \sqcup w_2 = \top$$

**Implication of Top:** The system has detected a contradiction. Evidence admission is terminated. An audit alert is issued.

### 4.3 WitnessState and Declare Lattices

Concrete implementation of witness state types in `wasm4pm-compat`:

```rust
// 1. Replay Witness State (for Petri Nets, BPMN, POWL, Process Trees)
pub enum WitnessState {
    Bottom,                              // No evidence (empty trace, initial marking)
    PartialReplay {
        trace_indices: Vec<usize>,       // Indices of replayed events
        marking: Vec<String>,            // Active place labels in Petri Net
        cost: u32,                       // Total alignment cost so far
    },
    Top,                                 // Contradiction detected
}

// 2. Declare Constraint Valuation Value (for LTLf satisfaction checking)
pub enum ConstraintValue {
    Bottom,                              // Not yet evaluated
    PossiblySatisfied,                   // Satisfied under current prefix but could be violated, or requires future events
    Satisfied,                           // Permanently satisfied (immutable)
    Violated,                            // Permanently violated (immutable)
    Top,                                 // Contradiction detected
}

// 3. Declare Constraints Witness State (maps constraint IDs to their valuation value)
pub enum DeclareWitnessState {
    Bottom,                              // Empty constraints or all Bottom
    Evaluated(std::collections::HashMap<String, ConstraintValue>),
    Top,                                 // Contradiction in constraint evaluations
}

// 4. Unified Witness State (product lattice of Replay and Declare witness states)
pub enum UnifiedWitnessState {
    Bottom,                              // Both components are Bottom
    Active {
        replay: WitnessState,
        declare: DeclareWitnessState,
    },
    Top,                                 // Either component is Top (contradiction)
}
```

**Lattice Operations:**

- **Bottom**: Empty evidence, initial state of all process executions.
- **Active / PartialReplay**: Cumulative evidence of events replayed, model state reached, and constraint evaluations.
- **Top**: Irreconcilable conflict. Halts further evidence integration.

### 4.4 Model-Specific Conformance Laws

| Model Type | State Representation | Witness Lattice Join Rule | Top Condition |
|---|---|---|---|
| **Petri Net** | Multi-set marking $M: P \to \mathbb{N}$ | Union of place sets if disjoint; merge if concurrent | Marking violates boundedness, or two events fire same transition with conflicting pre-conditions |
| **BPMN 2.0** | Token set on sequence flows + gateway state | Union of active flow paths | Gateway split/join mismatch, or conflicting loop back-edges |
| **Process Tree** | Active operator state ($\to, \times, \wedge, \circlearrowleft$) | Merge sub-tree executions | Sibling blocks execute concurrently violating sequence operator |
| **POWL** | Active partial-order node | Merge respecting dependency order | Partial order dependency violated (event before required predecessor) |

---

## 5. Structural Gaps & Forgeability Boundaries

### 5.1 Threat Surface Inventory

Despite rigorous cryptographic binding, the `wasm4pm-compat` type-law surface contains known theoretical gaps:

#### Gap 1: Host-Runtime Microsecond Delay
**Vector:** The time window between a WASM linear-memory state mutation and the host's OCEL log emission.

**Threat Model:** An advanced adversary controlling the host CPU scheduler could theoretically inject a false state vector into the OCEL log during this microsecond window.

**Mitigation:** Continuous Cryptographic Entanglement (CCE) — every state transition is hashed with:
- Previous state hash (chaining).
- Current host monotonic clock (nonce).
- Linear memory frame pointer (snapshot).

**Residual Risk:** Mitigated to negligible if clock resolution < 1µs.

#### Gap 2: Linear Memory Snapshot Replay
**Vector:** If WASM linear memory is snapshotted and replayed without deterministic epoch invalidation.

**Threat Model:** An attacker could replay a valid `Evidence<T, State, Witness>` block multiple times, claiming multiple process completions from a single execution.

**Mitigation:** The `epoch` field is a monotonically increasing counter. On admission, the system verifies that `new_epoch > previous_epoch`. No two evidence blocks can share the same epoch.

**Residual Risk:** If epoch counter overflows (after ~2^64 operations), epoch collision becomes possible. Mitigation: rotate epoch to a new cryptographic nonce-space after 2^32 completions.

#### Gap 3: Authority Key Compromise
**Vector:** If the private key of an auditor is compromised, an attacker can forge valid signatures on false evidence.

**Threat Model:** Attacker signs `Evidence<T, State, Witness>` blocks with stolen auditor keys.

**Mitigation:** 
- Public key rotation protocol: Keys are versioned. Revocation is broadcast immediately.
- Signature timestamp: The `epoch` field is included in the signature. Signatures with epoch < current_epoch are rejected.

**Residual Risk:** Compromise window between key theft and revocation broadcast. Mitigated by frequent key rotation (hourly or event-driven).

### 5.2 Admission-Time Vulnerability Windows

| Gap | Type | Severity | Closure Mechanism |
|---|---|---|---|
| Host-runtime drift | Cryptographic | Medium | Continuous Cryptographic Entanglement (CCE) |
| Epoch overflow | Logical | Low | Nonce rotation after 2^32 operations |
| Authority key theft | Authority | High | Public key versioning + revocation broadcast |
| Buffer overflow in deserialization | Memory | Critical | WASM sandbox bounds-checking + strict schema validation |
| Lattice join collision | Logical | Medium | Type-level enforcement of lattice axioms |

---

## 6. Integration Map: Type Laws to Receipt-Shaped Objects

### 6.1 Evidence Lifecycle Phases

```
┌──────────────────┐
│   Raw Trace      │  T (unvalidated payload)
│   (XES/OCEL)     │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  Admission Gate  │  Evidence<T, ⊥, ⊥> (initial state)
│  Cryptographic   │
│  & Temporal      │
└────────┬─────────┘
         │ (passes admission)
         ▼
┌──────────────────┐
│  Replay Phase    │  Evidence<T, M_i, W_i> (cumulative evidence)
│  Token Game      │  (i = 0, 1, ..., n)
│  Alignment       │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  Conformance     │  LossReport(fitness, compliance)
│  Evaluation      │
└────────┬─────────┘
         │ (fitness >= threshold)
         ▼
┌──────────────────┐
│  Receipt Seal    │  Evidence<T, M_f, W_f>^{auditor}
│  (Auditor Sig)   │  (receipt-shaped, immutable)
└──────────────────┘
```

### 6.2 M&A Receipt Binding

When a process claim appears in an M&A deck (e.g., "95% conformance"), the receipt binding is:

```json
{
  "slide_id": "SLIDE_42",
  "assertion": "Process conformance 95%",
  "evidence_hash": "sha256:abc123...",
  "verification_receipt": {
    "log_hash": "sha256:def456...",
    "fitness": 0.95,
    "loss_report": {...},
    "auditor_signature": "ed25519:xyz789..."
  },
  "admission_chain": [
    { "phase": "cryptographic", "status": "PASS" },
    { "phase": "temporal", "status": "PASS" },
    { "phase": "causal", "status": "PASS" },
    { "phase": "replay", "loss_policy": {...}, "status": "COMPLIANT" }
  ]
}
```

Each claim is **receipt-shaped**: immutable, cryptographically signed, and auditable.

---

## 7. Summary of Type-Law Domains

| Domain | Role | Key Type | Enforcement |
|--------|------|----------|-------------|
| **Evidence Structure** | Container for admissible artifacts | `Evidence<T, State, Witness>` | BLAKE3 hash, lattice coherence |
| **Admission Law** | Gateway enforcement | Refusal signatures | Default-deny, 6 criterion checks |
| **Loss Policy** | Conformance tolerance | `LossPolicy`, `LossReport` | Fitness threshold, move percentages |
| **Witness Lattice** | Evidence coherence algebra | `Lattice` trait, `WitnessState` enum | Monotonicity, join soundness, Top detection |
| **Structural Gaps** | Known vulnerabilities | Threat models | CCE, epoch rotation, key versioning |

---

## 8. Cross-Domain Dependencies

```
Evidence<T, State, Witness>
  │
  ├─> Cryptographic Binding (Admission Law)
  │     └─> BLAKE3 hash verification
  │
  ├─> State Transitions (Witness Lattice)
  │     └─> Conformance to process model
  │     └─> Lattice join soundness
  │
  ├─> Fitness Evaluation (Loss Policy)
  │     └─> Alignment-based conformance
  │     └─> LossReport certification
  │
  └─> Receipt Authority (Admission Law)
        └─> Ed25519 signature
        └─> Public key registry

LossReport
  │
  └─> Receipt Binding (Evidence)
        └─> Embedded in Evidence<T, State, Witness>
        └─> Auditor signature seals conformance
```

---

## 9. References & Related Standards

- **van der Aalst Constitution**: "If the code says it worked but the event log cannot prove a lawful process happened, then it did not work."
- **OCEL 2.0 Schema**: Object-Centric Event Log standard, JSON serialization.
- **Petri Net Soundness**: Structural liveness, boundedness, proper termination (WF-net constraints).
- **Alignment-Based Conformance**: Adriansyah (2014), A* shortest-path search in move space.
- **Process Tree & Inductive Miner**: Leemans (2013), block-structured process discovery.
- **Witness Lattice Theory**: Lattice algebra applied to partial evidence integration (original formulation).

---

## 10. Structural Gaps Summary

**Open Questions & Known Limitations:**

1. **Epoch Overflow**: At 2^64 epoch values, collision becomes possible. Mitigation: rotate nonce space post-2^32.
2. **Authority Key Compromise**: Compromise window exists between theft and revocation. Mitigation: frequent key rotation (hourly).
3. **WASM Sandbox Escape**: If WASM sandbox is compromised, linear memory bounds checking is defeated. Mitigation: external host-level memory enforcement.
4. **Lattice Join Undecidability**: For complex models (e.g., large BPMN with 1000+ gateways), the join operation may be NP-hard. Mitigation: sampling-based approximation.
5. **Temporal Jitter in Distributed Systems**: Multiple evidence blocks generated in different processes may have timestamp collisions. Mitigation: include process ID and sequence number in temporal ordering.

---

## Document Structure Summary

This atlas is organized as a **discovery inventory** of the type-law surface:

- **Section 1**: Core Evidence structure and admissibility axioms.
- **Section 2**: Admission criteria and refusal boundaries.
- **Section 3**: Loss Policy and Loss Report certification.
- **Section 4**: Witness Lattice algebra and model-specific rules.
- **Section 5**: Structural gaps and threat models.
- **Sections 6-10**: Integration, dependencies, and open questions.

For implementation details and code examples, refer to the supporting documents:
- [evidence-structures.md](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/evidence-structures.md)
- [admission-refusal-map.md](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/admission-refusal-map.md)
- [loss-policies.md](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/loss-policies.md)
- [witness-lattices.md](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/witness-lattices.md)
- [structural-gaps.md](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/structural-gaps.md)

---

## Section 3: Const-Generic Law Machinery (v30.1.1 Spec)

We map boolean const-generic expressions to compile-time proof obligations via:
$$\text{Assert}\langle\text{const OK: bool}\rangle; \qquad \text{IsTrue for Assert}\langle\text{true}\rangle; \qquad \text{Require}\langle\text{const OK: bool}\rangle = \text{Assert}\langle\text{OK}\rangle$$
A where-bound $\text{Require}\langle\text{EXPR}\rangle: \text{IsTrue}$ compiles successfully if and only if $\text{EXPR}$ evaluates to $\text{true}$.

The type-level rational metric bounds $\text{Between01}\langle n, d \rangle$ is defined as:
$$\text{Between01}\langle\text{const NUM: u64, const DEN: u64}\rangle \quad \text{where} \quad d > 0 \land n \leq d$$
Its soundness is stated as:
$$\forall n, d \in \mathbb{N}, \quad \text{Between01}\langle n, d \rangle \text{ is well-formed} \iff d > 0 \land n \leq d$$

**Proof of Between01 Soundness:**
The type definition carries two where-bounds: $\text{Require}\langle d > 0\rangle : \text{IsTrue}$ and $\text{Require}\langle n \leq d\rangle : \text{IsTrue}$. By the $\text{Assert}/\text{IsTrue}$ machinery, each bound reduces to $\text{Assert}\langle b \rangle : \text{IsTrue}$ where $b$ is the boolean value of the bracketed expression evaluated at compile time. The trait $\text{IsTrue}$ is implemented only for $\text{Assert}\langle\text{true}\rangle$. Therefore, well-formedness holds if and only if both $d > 0$ and $n \leq d$ evaluate to $\text{true}$, which is equivalent to $d > 0$ and $n \leq d$. $\square$

The "Need-9 means split" law enforces that a single condition cell holds at most 8 primary bits:
$$\text{ConditionCell}\langle\text{const BITS: usize}\rangle \quad \text{where} \quad \text{Require}\langle\text{BITS} \leq 8\rangle: \text{IsTrue}$$

