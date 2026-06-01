# Simulation Models for Trace Laundering & Spoofing Attacks

This document establishes the threat models, mathematical specifications, and security guarantees for detecting trace laundering and signature spoofing attacks in the `wasm4pm` execution environment. 

The accompanying executable simulation engine is located at [simulate_laundering_spoofing.py](file:///Users/sac/process-intelligence/adversarial/simulate_laundering_spoofing.py).

---

## 1. Threat Landscape & Attack Taxonomy

In process-centric audits and M&A transactions, the integrity of event logs is paramount. Adversaries (e.g., process operators or sellers attempting to artificially inflate asset valuations) deploy several techniques to manipulate logs.

```
                      ┌──────────────────────────────────────────┐
                      │            Adversarial Vectors           │
                      └────────────────────┬─────────────────────┘
         ┌─────────────────────────────────┴─────────────────────────────────┐
         ▼                                                                   ▼
┌──────────────────┐                                                ┌──────────────────┐
│ Trace Laundering │                                                │  Spoofing/Forgery│
└────────┬─────────┘                                                └────────┬─────────┘
         ├─ Timestamp Shifting (SLA Violations)                              ├─ Signature Forgery (Forged keys)
         ├─ Event Deletion (Rework/Violations)                               └─ Raw DataFrame Ingestion
         └─ Event Insertion (Fake compliance steps)
```

### A. Trace Laundering
Trace laundering is the post-hoc manipulation of event logs to present a process as more efficient, compliant, or standardized than it actually is.
1. **Timestamp Shifting**: Editing event timestamps to shorten durations, artificially satisfying Service Level Agreements (SLAs) or reducing calculated bottleneck latencies.
2. **Activity Deletion**: Removing events associated with rework loop transitions, compliance violations, or long idle cycles, which artificially inflates the conformance fitness score ($f$).
3. **Activity Insertion**: Inserting fabricated events (e.g., manual approvals or audits) to falsely satisfy compliance rules.

### B. Signature Spoofing & Ingestion Boundary Bypass
To bypass cryptographic verification, adversaries attempt to forge signatures or exploit dynamic features of the ingestion boundary.
1. **Signature Forgery**: Recalculating the hash chain using a forged key. The adversary intercepts the log, performs trace laundering, and signs the modified payloads with an unauthorized private key, hoping the verifier does not check signature authenticity against the genuine system key.
2. **Raw Ingestion Bypass**: Attempting to feed raw, mutable, in-memory structures (such as `pandas.DataFrame` or unhashed files) directly into the execution engine. This bypasses structural validation and exposes the engine to dynamic mutation (TOCTOU attacks) during processing.

---

## 2. Mathematical Formalization of Cryptographic Controls

To prevent log laundering and spoofing, the ingestion boundary enforces strict mathematical invariants on the event stream.

### A. Event Representation & Serialization
Let an event $e_j$ be represented by a payload dictionary $P_j$:
$$P_j = \{ \text{event\_id}, \text{activity\_name}, \text{timestamp\_ns}, \text{attributes} \}$$
To sign or hash the payload deterministically, it is serialized to canonical JSON:
$$\overline{P_j} = \text{canonical\_serialize}(P_j)$$

### B. System Transition Signatures
Each event is signed at execution time by the transaction authority (e.g., ERP or CRM system) using its private key $K_{\text{system}}$:
$$S_j = \operatorname{HMAC-SHA256}(K_{\text{system}}, \overline{P_j})$$

### C. Cryptographic Hash Chaining
Traces are structured as a hash chain. For a trace $\sigma$ with ID $\sigma_{\text{id}}$ and events $\langle e_0, e_1, \dots, e_n \rangle$, the hash link $H_j$ is defined recursively:
$$H_0 = \operatorname{SHA-256}(\overline{P_0} \mathbin{\Vert} \operatorname{SHA-256}(\sigma_{\text{id}}) \mathbin{\Vert} S_0)$$
$$H_j = \operatorname{SHA-256}(\overline{P_j} \mathbin{\Vert} H_{j-1} \mathbin{\Vert} S_j) \quad \text{for } j > 0$$

### D. Temporal Monotonicity and Velocity Gates
1. **Monotonicity**: The sequence of events in a trace must have monotonically non-decreasing timestamps:
   $$T_{\text{ns}}(P_j) \ge T_{\text{ns}}(P_{j-1}) \quad \forall j > 0$$
2. **Impossible Velocity**: The duration between consecutive activities must exceed the physical operational threshold $\Delta T_{\text{limit}}$:
   $$T_{\text{ns}}(P_j) - T_{\text{ns}}(P_{j-1}) \ge \Delta T_{\text{limit}} \quad \forall j > 0$$

---

## 3. Ingestion Boundary Security Architecture

The ingestion boundary acts as a gatekeeper, rejecting malformed, mutable, or unhashed logs before any analysis occurs.

```
       [ Input Log ]
             │
             ▼
┌──────────────────────────┐
│   Is pandas DataFrame?   ├── Yes ──> [ REJECT: reject_unhashed_dataframe ]
└────────────┬─────────────┘
             │ No
             ▼
┌──────────────────────────┐
│ Verify Event Signatures  ├── Invalid ──> [ REJECT: check_signature_authenticity ]
└────────────┬─────────────┘
             │ Valid
             ▼
┌──────────────────────────┐
│ Verify Hash Chain Links  ├── Broken ──> [ REJECT: check_cryptographic_chain ]
└────────────┬─────────────┘
             │ Valid
             ▼
┌──────────────────────────┐
│ Check Temp Monotonicity  ├── Backwards ──> [ REJECT: check_timestamp_monotonicity ]
└────────────┬─────────────┘
             │ Monotonic
             ▼
┌──────────────────────────┐
│  Check Velocity Limits   ├── Violation ──> [ REJECT: check_impossible_velocity ]
└────────────┬─────────────┘
             │ OK
             ▼
       [ ACCEPT LOG ]
```

### Ingestion Boundary Verdict Schema
When a log fails any guard, the engine returns a JSON verdict matching the schema defined in [raw-laundering_refusal_sample.md](file:///Users/sac/process-intelligence/experiments/raw-laundering_refusal_sample.md):
- `verdict`: `REJECTED`
- `refusal_reason.rule_failed`: Name of the failed rule (e.g. `reject_unhashed_dataframe`, `check_signature_authenticity`, etc.)
- `refusal_reason.detailed_error`: Narrative explaining the exact failure point.

---

## 4. Simulation Scenarios & Verification Outcomes

The simulation suite executes five attack scenarios against the ingestion boundary. Below are the verified outcomes:

| Scenario | Attack Type | Modus Operandi | Failed Gate | Expected Verdict |
| :--- | :--- | :--- | :--- | :--- |
| **0** | Conforming | Normal transaction log signed with genuine key. | None (Valid) | **ACCEPTED** |
| **1** | Laundering | Adversary modifies event timestamp to mask SLA delays. | `check_signature_authenticity` | **REJECTED** |
| **2** | Laundering | Adversary deletes a compliance violation/rework event. | `check_cryptographic_chain` | **REJECTED** |
| **3** | Forgery | Adversary signs modified events with a forged key. | `check_signature_authenticity` | **REJECTED** |
| **4** | Ingestion Bypass | Adversary attempts to pass mutable `pandas.DataFrame`. | `reject_unhashed_dataframe` | **REJECTED** |
| **5** | Velocity Attack | Consecutive events separated by < 1s. | `check_impossible_velocity` | **REJECTED** |

### Execution Logs of the Simulation Engine
```
=== INITIALIZING V30.1.1 ADVERSARIAL PROCESS SIMULATION MATRIX ===

--- Running Scenario 0: Conforming Log Ingestion ---
Verdict: ACCEPTED
Log Hash: aff1bfd255b5ec31f46adfad8e1867e8fc8ca945d848681a729c7cc07f30ba2e

--- Running Scenario 1: Laundering via Timestamp Shifting ---
Verdict: REJECTED
Failed Rule: check_signature_authenticity
Detailed Error: Event 'Invoice_Customer' at index 3 contains a forged or invalid signature.

--- Running Scenario 2: Laundering via Event Deletion ---
Verdict: REJECTED
Failed Rule: check_cryptographic_chain
Detailed Error: Event 'Invoice_Customer' at index 2 has an invalid transition state hash (chain broken).

--- Running Scenario 3: Signature Forgery Attempt ---
Verdict: REJECTED
Failed Rule: check_signature_authenticity
Detailed Error: Event 'Create_Order' at index 0 contains a forged or invalid signature.

--- Running Scenario 4: Unhashed DataFrame Ingestion Rejection ---
Verdict: REJECTED
Failed Rule: reject_unhashed_dataframe
Detailed Error: Rejected unhashed pandas DataFrame at ingestion boundary. Logs must be immutable, pre-hashed, and signed.

--- Running Scenario 5: Impossible Velocity Detection ---
Verdict: REJECTED
Failed Rule: check_impossible_velocity
Detailed Error: Event 'Approve_Order' at index 1 has impossible velocity: 1e-07 seconds from previous event.

=== ALL ADVERSARIAL SIMULATION SCENARIOS SUCCESSFULLY VERIFIED ===
```
