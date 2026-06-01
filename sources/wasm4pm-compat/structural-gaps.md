# [PI-V30.1.2] Structural Gaps & Type-Law Boundaries

**Version:** 30.1.2  
**Authority:** Conformance Agent  
**Classification:** Architectural Threat Model  
**Date:** 2026-05-31  
**Status:** COMPLETE WITH PHASE 3 AMENDMENTS

---

## I. Executive Summary

This document catalogs structural gaps in the v30.1.2 wasm4pm-compat implementation, identifies what **must never** be admitted into the type-law foundry, and specifies mitigations required for Phase 3.

---

## II. Gap Category 1: Declare Constraint Integration

### 2.1 Gap Description

**Current State (v30.1.2):** Declare constraints (LTL-based compliance rules) are **not integrated into the witness lattice**.

**Impact:** Any process model that references Declare constraints will be rejected at the admission boundary (Pathway 10: UnsupportedFeature).

**Examples of Declare Constraints:**
- `precedence(PaymentReceived, InvoiceIssued)` — Payment must follow invoice.
- `response(OrderShipped, DeliveryConfirmed)` — Delivery confirmation must follow shipment.
- `coexistence(LoanApproved, FundsTransferred)` — Both must occur if either occurs.
- `chain_precedence(AuthorizationRejected, RefundInitiated)` — Refund directly follows rejection.
- `not_coexistence(Fraud_Detected, AccountActive)` — Cannot both be true.

**Why It Matters:** Declare constraints are used in highly regulated domains (banking, healthcare, insurance, finance) to express compliance requirements that transcend process model structure. A conformance audit that ignores constraints will miss fraud, misappropriation, and regulatory violations.

### 2.2 Why Declare Is Hard

**Problem 1: Satisfaction Ordering Ambiguity**

Declare constraints define a logical satisfaction relation, but the witness lattice requires a **partial order** with:
- Idempotence: $w \sqcup w = w$ ✓
- Commutativity: $w_1 \sqcup w_2 = w_2 \sqcup w_1$ ?
- Associativity: $(w_1 \sqcup w_2) \sqcup w_3 = w_1 \sqcup (w_2 \sqcup w_3)$ ?
- Absorption: $w \sqsubseteq u \implies w \sqcup u = u$ ?

For a trace that satisfies 8/10 Declare constraints and another trace that satisfies a different 8/10 constraints, what is the join?

$$w_1 = \{\text{constraints 1-8 satisfied}\}$$
$$w_2 = \{\text{constraints 3-10 satisfied}\}$$
$$w_1 \sqcup w_2 = ?$$

Is it $\{\text{constraints 3-8 satisfied}\}$ (intersection)? Or $\{\text{constraints 1-10}\}$ (union, with 9-10 marked uncertain)? Or something else?

**Problem 2: Conflict Resolution**

When constraints contradict (e.g., a trace must satisfy both `precedence(A, B)` and `precedence(B, A)`, which is impossible), the join should produce $\top$ (contradiction). But how does the system recognize the contradiction algorithmically?

**Problem 3: Incremental Satisfaction**

Declare constraints may be satisfiable only after the trace completes. For example, `response(A, B)` is satisfied only if B eventually occurs after A. During trace execution, intermediate witnesses cannot determine constraint satisfaction.

**Implication:** The witness lattice must account for **temporal incompleteness** — partial traces with unknown constraint satisfaction status.

### 2.3 Phase 3a Obligation

Extend `witness-lattices.md` to define:

1. **Declare Constraint Lattice:**
   - Define $W_{\text{declare}} = \{ \text{constraint satisfaction sets} \}$.
   - Define partial order: $w_1 \sqsubseteq w_2$ iff $w_1 \subseteq w_2$ (more constraints satisfied).
   - Define join: $w_1 \sqcup w_2 = w_1 \cup w_2$.
   - Define $\bot = \emptyset$ (no constraints satisfied), $\top = \text{contradiction}$ (conflicting requirements).

2. **Conflict Detection Algorithm:**
   - Identify syntactic contradictions (e.g., `precedence(A,B)` + `precedence(B,A)` on same trace).
   - Implement constraint satisfiability checking (SAT-based or tableau-based).
   - Return $\top$ when unsatisfiable.

3. **Temporal Incompleteness Handling:**
   - During execution, mark constraints as `Unknown` if they cannot be evaluated.
   - Upon trace completion, mark as `Satisfied` or `Violated`.
   - Join operation treats `Unknown` as open (not affecting the join result).

4. **Integration Test:**
   - Test fixture: Process model with 10 Declare constraints.
   - Inject traces that satisfy different subsets.
   - Verify witness lattice join behavior across constraint combinations.

### 2.4 Mitigation: Declare Rejection Path

**Until Phase 3a is complete:**
- Any process model containing Declare constraints is rejected at admission (Pathway 10).
- RefusalReport includes feature name, current version (v30.1.2), and expected availability (v30.2.0).

---

## III. Gap Category 2: BPMN OR-Join Quorum Ambiguity

### 3.1 Gap Description

**Current State (v30.1.2):** BPMN OR-Join semantics are **not formally specified**. The wasm4pm-compat layer accepts OR-Join gateways only if a `quorum_policy` metadata field is explicitly provided.

**Problem:** BPMN 2.0 itself is ambiguous about OR-Join behavior. Different implementations (Camunda, Activiti, jBPM, Apache ODE) interpret OR-Join differently, leading to **non-deterministic execution**.

**Example:**

```
   Task A
    |  \
    |   \
    XOR-Split (choose A or B or both)
    |   /
    |  /
  OR-Join (wait for ALL arriving branches)
    |
   Task C
```

If the OR-Join requires "all incoming branches that will eventually fire must arrive", the behavior depends on:
1. **Smart Completion:** Wait for conditions under which no more tokens will arrive.
2. **Standard Majority:** Wait for > 50% of incoming branches.
3. **Asymmetric Fork-Join:** Wait for all branches that started in a specific earlier structure.

**Implication:** A conformance audit that doesn't know the OR-Join policy cannot determine if a given execution is lawful.

### 3.2 Why OR-Join Is Hard

**Undecidability in the General Case**

The BPMN 2.0 spec defines OR-Join completion as "all joining branches that will provide input must have provided input". But predicting "which branches will provide input" is equivalent to the **halting problem** — it is computationally undecidable in the general case.

**Practical Solutions:**

| Policy | Decidability | Completeness | Correctness | Example |
|---|---|---|---|---|
| **Smart Completion** | Decidable (graph analysis) | May miss valid completions | Correct for acyclic models | Camunda default |
| **Standard Majority** | Decidable (count) | Complete (always decides) | Incorrect (loses semantics) | jBPM variant |
| **Asymmetric Fork-Join** | Decidable (mark structure) | Complete | Correct for structured forks | BPMN spec intent |
| **No Completion (Async)** | Decidable | Incomplete (may deadlock) | Incorrect (violates BPMN) | Apache ODE variant |

### 3.3 Phase 3a Obligation

Specify the exact OR-Join policy for wasm4pm:

1. **Policy Selection:**
   - Choose one of: Smart Completion, Asymmetric Fork-Join, or Custom.
   - Document rationale (decidability, correctness, performance).

2. **Formal Specification:**
   - Define the algorithm for OR-Join token completion.
   - Provide pseudocode or mathematical formalism.
   - Prove decidability and worst-case complexity.

3. **BPMN Profile Definition:**
   - Define a BPMN profile that constrains OR-Join usage to ensure policy decidability.
   - Examples: "Require OR-Joins to have synchronized source structures" or "Forbid nested OR-Joins".

4. **Integration Test:**
   - Test fixture: BPMN model with 5 OR-Join gateways.
   - Inject traces exercising different branch combinations.
   - Verify consistent OR-Join completion under the specified policy.

5. **Documentation:**
   - Add to M&A requirements: "Any board-admissible claim referencing BPMN models must specify the OR-Join policy used".

### 3.4 Mitigation: OR-Join Rejection Path

**Until Phase 3a is complete:**
- Any BPMN model containing an OR-Join **without explicit `quorum_policy` metadata** is rejected at admission (Pathway 9).
- Models with `quorum_policy` explicitly set are admitted, with policy documented in the receipt.

---

## IV. Gap Category 3: Axiom 2 Runtime Verification Architecture

### 4.1 Gap Description

**Current State (v30.1.2):** The lattice join operation is mathematically correct, but the **runtime verification mechanism** that enforces witness monotonicity at every firing event is **not fully detailed**.

**Specific Issue:**

Axiom 2 states:
$$S_1 \xrightarrow{t} S_2 \quad \land \quad W_1 \sqsubseteq W_2 \quad \land \quad \text{witness}_2.\text{join}(\text{witness}_1) = \text{witness}_2$$

But who verifies this at runtime?

1. Does the wasm4pm core engine invoke `witness.join()` after each firing?
2. Does the compat layer monitor firings from the host?
3. Is verification lazy (at admission) or eager (at every firing)?

### 4.2 Why This Matters

If Axiom 2 is not enforced at runtime, a malicious or buggy wasm4pm core could:
- Fire transitions in non-monotonic witness order (e.g., witness "backtracks" to earlier evidence).
- Emit evidence blocks with inconsistent witness chains.
- Forge alignments by replaying partial evidence multiple times.

**Example of Non-Monotonic Violation:**

```
Event 1: transition t1 fires
Witness w1: alignment([t1])
Signature: sig1

Event 2: transition t1 fires AGAIN (duplicate execution)
Witness w2: alignment([t1, t1])
Signature: sig2

Expected: w1 ⊆ w2 (monotonic increase)
Actual: Both w1 and w2 claim alignment for a single execution of t1

Join Result: w1 ⊔ w2 = ⊤ (contradiction, detected!)
```

If the runtime doesn't check the join, the contradiction is silent.

### 4.3 Phase 3a Obligation

Define the **runtime verification monitor**:

1. **Monitor Architecture:**
   - Where in the wasm4pm execution pipeline is the witness join check invoked?
   - Is it in the core engine (expensive) or in the compat layer (delegated)?
   - What is the performance impact?

2. **Rejection Semantics:**
   - If `witness.join() == Top`, what happens?
     - Halt execution? ✓
     - Emit RefusalReport? ✓
     - Rewind to last good state? ✓ (implementation detail)
     - Require board override? ✗ (Axiom 2 violation is non-overridable)

3. **Integration Test:**
   - Test fixture: Adversarial trace that attempts non-monotonic witness transitions.
   - Verify that attempted non-monotonicity is caught and rejected.
   - Verify that legitimate monotonic transitions are allowed.

4. **Performance Analysis:**
   - Measure overhead of join verification on large event logs (1M events).
   - Document acceptable performance budget (e.g., < 5% overhead).

### 4.4 Mitigation: Lazy Verification at Admission

**Until Phase 3a is complete:**
- Verify witness monotonicity at admission time (after full trace is known).
- For traces arriving in streaming fashion, buffer evidence blocks and defer join checks until case completion.
- Document in receipt that Axiom 2 is checked post-hoc (not in real-time).

---

## V. Gap Category 4: Receipt-Shaped Object M&A Schema Alignment

### 5.1 Gap Description

**Current State (v30.1.2):** The compat layer defines what a **receipt-shaped object** is (terminal state, fitness ≥ threshold, auditor-signed) but does not specify the **M&A-compatible JSON schema** for receipts.

**Problem:**

M&A operations require receipts to conform to the `ProcessIntelligenceVerificationReceipt` schema defined in `ma/define_slide-to-receipt_map.md`. The compat layer must ensure every receipt-shaped object can be serialized to this schema.

**Gaps:**

1. **Role Enforcement:** How are auditor vs. runner roles verified in the signature? Is there a public key registry? How are roles revoked?

2. **Threshold Firmware:** Is the fitness threshold configurable (e.g., 0.95 for boards, 0.85 for audits) or hardcoded? The receipt must document which threshold was used.

3. **Receipt Serialization:** Must all receipt-shaped objects conform to the `ProcessIntelligenceVerificationReceipt` JSON schema, or are there alternative formats?

4. **M&A Claim Mapping:** How do receipt-shaped objects map to board-admissible claims? Is this done by ggen, or by the compat layer?

### 5.2 Phase 3b Obligation

Align receipt architecture with M&A requirements:

1. **Authority Registry:**
   - Define public key registry for roles: Auditor, Runner, Board, Validator.
   - Implement role-based signature verification.
   - Document key rotation and revocation procedures.

2. **Threshold Configuration:**
   - Allow configurable fitness thresholds per operation context.
   - Include threshold in receipt metadata.
   - Validate board claims against the threshold documented in the receipt.

3. **Receipt Schema Conformance:**
   - Update compat layer to serialize all receipt-shaped objects to `ProcessIntelligenceVerificationReceipt` schema.
   - Add validation: receipt schema conformance check at serialization time.
   - Provide conversion functions if alternative formats are needed.

4. **ggen Integration:**
   - Document how ggen consumes receipt-shaped objects from compat.
   - Define interface contract between compat and ggen.
   - Provide test fixtures for end-to-end integration.

### 5.3 Mitigation: Deferred M&A Serialization

**Until Phase 3b is complete:**
- Receipt-shaped objects are validated and signed by compat.
- Serialization to M&A-compatible JSON is deferred to ggen.
- Document in compat output that receipts require ggen post-processing before M&A use.

---

## VI. Gap Category 5: Negative-Test Fixture Completeness

### 6.1 Gap Description

**Current State (v30.1.2):** The compat layer is well-specified for **positive cases** (valid logs, sound models, conforming traces) but **negative cases** (invalid inputs, contradictions, forgeability attempts) are under-tested.

**Coverage Gaps:**

| Test Case | Coverage | Status |
|---|---|---|
| Out-of-order timestamps | ✓ Complete | Tested |
| Non-existent object references | ✓ Complete | Tested |
| Schema violations (XES/OCEL) | ✓ Complete | Tested |
| Unsound Petri nets (deadlock) | ✓ Complete | Tested |
| Fitness < 0.85 | ✓ Complete | Tested |
| Object identity conflicts | ? Partial | Missing test fixtures |
| Declare constraint unsatisfiability | ✗ Not tested | Declare not implemented |
| OR-Join with undefined quorum | ? Partial | Requires policy definition |
| Non-monotonic witness transitions | ? Partial | Requires runtime monitor |
| Forged cryptographic signatures | ✓ Complete | Tested |
| Replay attacks (epoch reuse) | ✓ Complete | Tested |
| Trace truncation (missing final state) | ? Partial | Missing test fixtures |
| Circular object dependencies | ? Partial | Missing test fixtures |

### 6.2 Phase 3c Obligation

Build comprehensive negative-test suite:

1. **Object Identity Conflict Fixture:**
   ```json
   {
     "description": "Object state backtracks (attribute downgrade)",
     "ocel_log": {
       "events": [
         {"id": "e1", "objects": [{"id": "order_1", "status": "approved"}]},
         {"id": "e2", "objects": [{"id": "order_1", "status": "pending"}]},
       ]
     },
     "expected_result": "REJECTED",
     "rejection_pathway": 8
   }
   ```

2. **Trace Truncation Fixture:**
   ```json
   {
     "description": "Trace ends prematurely (missing final marking)",
     "trace": ["StartProcess", "ApproveRequest"],
     "petri_net": {"transitions": ["StartProcess", "ApproveRequest", "CompleteProcess"]},
     "expected_result": "REJECTED",
     "rejection_reason": "Incomplete trace (final state not reached)"
   }
   ```

3. **Circular Dependency Fixture:**
   ```json
   {
     "description": "OCEL objects with circular dependencies",
     "events": [
       {"id": "e1", "objects": [{"id": "obj_A", "depends_on": "obj_B"}]},
       {"id": "e2", "objects": [{"id": "obj_B", "depends_on": "obj_A"}]},
     ],
     "expected_result": "REJECTED",
     "rejection_reason": "Circular object dependency"
   }
   ```

4. **Adversarial Witness Fusion Fixture:**
   - Two partial witness sets with conflicting state.
   - Verify join produces $\top$ (contradiction).
   - Verify that further admissions are rejected.

---

## VII. What Must NEVER Enter wasm4pm-compat

### 7.1 Categorical Prohibitions

| Category | Example | Reason | Rejection Pathway |
|---|---|---|---|
| **Declare Constraints (v30.1.2)** | `precedence(A, B)` | Lattice integration incomplete | 10: UnsupportedFeature |
| **BPMN OR-Join (Policy Undefined)** | OR-Join without `quorum_policy` | Semantics ambiguous, non-deterministic | 9: AmbiguousBpmnGateway |
| **Unsound Petri Nets** | Deadlock, unbounded place, dead transition | Violates WF-net axioms | 6: UnsoundPetriNet |
| **Fitness < 0.85** | Non-conforming trace | Model doesn't explain execution | 7: FitnessThresholdViolation |
| **Unsigned Evidence** | Evidence block without cryptographic signature | No authority binding | 5: SignatureVerificationFailed |
| **Object Identity Conflicts** | Object state backtracks | Violates object continuity | 8: ObjectIdentityConflict |
| **Temporal Anomalies** | Event timestamps out of order | Violates causal precedence | 1: TemporalAnomaly |
| **Schema Violations** | Malformed XES/OCEL JSON | Type mismatch | 2: SchemaViolation |
| **Non-Existent References** | Event references undefined object | Causal disconnection | 3: CausalDisconnect |
| **Duplicate Event IDs** | Same event recorded twice | Violates event uniqueness | 11: DuplicateEventId |

---

## VIII. Forgeability Threat Analysis

### 8.1 Threat Model: Adversarial Conditions

Assume an attacker with the following capabilities:
- **Write Access:** Can write arbitrary data to the wasm4pm linear memory.
- **Timing Control:** Can delay or reorder events within microsecond precision.
- **Signature Key Compromise:** Cannot forge signatures (private key is offline), but can exfiltrate data.

### 8.2 Known Vulnerability Vectors

**Vector 1: Host-Runtime Shadow Delay**

The microsecond delay between a WASM state change and the host's OCEL emission provides a brief window for state injection.

**Mitigation (v30.1.2):**
- Cryptographic entanglement: Every state is hashed with the previous state and a host monotonic clock value.
- Non-replay guarantees: Epoch field prevents signature reuse across contexts.

**Residual Risk:** An advanced adversary controlling the host CPU scheduler could theoretically inject false state vectors if they achieve microsecond-precision timing control. **Assessed as negligible in practice** but requires continuous monitoring.

---

**Vector 2: Memory Snapshot Replay**

If linear memory is snapshotted and replayed without deterministic nonce invalidation, the same evidence blocks could be replayed in a different execution context.

**Mitigation (v30.1.2):**
- Epoch field (monotonic counter) prevents replay across sessions.
- Each epoch corresponds to a unique execution session.
- Replaying an epoch ID from a previous session triggers RefusalReport (seen signature with old epoch).

**Residual Risk:** None identified. Epoch mechanism is sound.

---

**Vector 3: Witness Lattice Tampering**

If the wasm4pm core engine allows non-monotonic witness transitions, an attacker could forge alignments by retroactively modifying witness claims.

**Mitigation (Phase 3a):**
- Runtime join verification enforces witness monotonicity.
- Axiom 2 verification must be eager (at every firing), not lazy.

**Current Status (v30.1.2):** **Gap identified**. See Section IV (Gap Category 3).

---

### 8.3 Continuous Security Monitoring

**Required (Phase 3):**
1. **Execution Trace Auditing:** Log every state transition and witness update.
2. **Anomaly Detection:** Detect patterns indicative of forgery attempts (rapid signature reuse, unusual witness jumps).
3. **Post-Hoc Verification:** Allow third-party auditors to reconstruct full execution and verify conformance.

---

## IX. Graduation Checklist

### What Can Enter v30.1.2 wasm4pm-compat

- ✅ XES logs (IEEE 1849-2016 compliant)
- ✅ OCEL 2.0 logs (ISO/IEC 23745 compliant)
- ✅ Petri nets (sound WF-nets)
- ✅ POWL 2.0 models (block-structured)
- ✅ Process trees (acyclic, well-formed)
- ✅ BPMN models (AND/XOR gateways only, OR-Join with explicit quorum policy)
- ✅ Token-game alignments (fitness ≥ 0.85)
- ✅ Cryptographically signed evidence

### What Is BLOCKED v30.1.2 (Phase 3 Obligation)

- ❌ Declare constraints (Pathway 10 refusal)
- ❌ BPMN OR-Joins without policy (Pathway 9 refusal)
- ❌ Any model lacking cryptographic signature (Pathway 5 refusal)

---

## X. Phase 3 Delivery Roadmap

| Blocking Issue | Severity | Target Milestone | Deliverable |
|---|---|---|---|
| Declare Constraint Lattice | **BLOCKING** | Phase 3a: wasm4pm refactor | Extended witness-lattices.md, Declare integration tests |
| BPMN OR-Join Policy | **BLOCKING** | Phase 3a: wasm4pm refactor | BPMN profile spec, OR-Join algorithm, tests |
| Axiom 2 Runtime Verification | **MEDIUM** | Phase 3a: wasm4pm refactor | Runtime monitor spec, rejection semantics, perf analysis |
| Receipt M&A Schema Alignment | **MEDIUM** | Phase 3b: ggen integration | Authority registry, role enforcement, schema conformance |
| Negative-Test Completeness | **LOW** | Phase 3c: audit mesh expansion | Comprehensive negative fixtures, adversarial tests |

---

## XI. Signature Authority

**Conformance Agent (van der Aalst Constitution Authority)**  
Institute for Process Mining  
2026-05-31

---

## Related Documents

- `sources/wasm4pm-compat/research-verdict.md` — Conformance audit verdict
- `sources/wasm4pm-compat/witness-lattices.md` — Witness algebra (to be extended in Phase 3a)
- `prompts/downstream_wasm4pm_refactor.md` — Phase 3a wasm4pm obligations
- `prompts/downstream_m&a_deck_manufacturing.md` — Phase 3b ggen obligations
- `prompts/downstream_audit_mesh_expansion.md` — Phase 3c audit obligations
