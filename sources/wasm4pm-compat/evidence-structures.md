# Evidence Type Structures in WASM4PM-Compat

The `Evidence<T, State, Witness>` type is the primary wrapper for securing and certifying process mining executions. It is designed to ensure that raw event data cannot be laundered to make false compliance claims and that every state transition is accompanied by a cryptographically verifiable proof of correctness.

---

## 1. Generic Structure Definition

In `wasm4pm-compat`, evidence is defined as a generic struct bound by the properties of its type parameters:

```rust
pub struct Evidence<T, State, Witness> {
    /// The event log payload (e.g., XesTrace, OcelEventLog, or BpmnInstanceData)
    pub payload: T,

    /// The verified process execution state (e.g., PetriNetMarking, BpmnTokenConfiguration)
    pub state: State,

    /// The cryptographic alignment or replay proof (e.g., WitnessState, ReplayReceipt)
    pub witness: Witness,

    /// The epoch identifier, preventing replay attacks and defining execution context
    pub epoch: u64,

    /// The cryptographic signature of the verifying authority (runner or auditor)
    pub signature: IdentitySignature,

    /// The BLAKE3 cryptographic hash binding all fields together
    pub hash: Blake3Hash,
}
```

### 1.1 Field Constraints
- **Payload (`T`)**: Must implement serialization (`Serialize`) and represent a valid event log structure (such as XES or OCEL).
- **State (`State`)**: Must represent a state in the process model. It must be deterministic and serializable.
- **Witness (`Witness`)**: Must represent the alignment or token game replay progress. It must implement the `Lattice` trait, allowing combining and ordering.

---

## 2. Evidence Invariants and Axioms

To be admissible in the corporate process ledger, any instance of `Evidence<T, State, Witness>` must satisfy three strict axioms:

### 2.1 Cryptographic Binding (Non-Forgeability)
The hash must be a deterministic BLAKE3 digest of the serialized payload, state, witness, epoch, and signature:
$$\mathcal{H} = \operatorname{BLAKE3}(\operatorname{Serialize}(payload) \parallel \operatorname{Serialize}(state) \parallel \operatorname{Serialize}(witness) \parallel \operatorname{Serialize}(epoch) \parallel \operatorname{Serialize}(signature))$$
Any alteration of the trace or execution state invalidates the hash.

### 2.2 Replay Soundness
Given two sequential evidence blocks $E_1 = \langle T, S_1, W_1 \rangle$ and $E_2 = \langle T, S_2, W_2 \rangle$ and a transition activity $t$, the transition must be valid under the process execution law:
$$S_1 \xrightarrow{t} S_2 \quad \text{and} \quad W_1 \sqsubseteq W_2$$
A witness state can only move monotonically upward in the lattice: $W_1 \sqsubseteq W_2$.

### 2.3 Signature Admissibility
The `signature` must be verified using the public key corresponding to the authorized runner or auditor role. The signature covers the entire hash:
$$\operatorname{VerifySignature}(\text{PublicKey}_{\text{Auditor}}, \text{Signature}, \mathcal{H}) \equiv \text{True}$$

---

## 3. Rust Implementation Interface

```rust
use blake3::Hasher;
use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct IdentitySignature {
    pub public_key: Vec<u8>,
    pub signature_bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blake3Hash([u8; 32]);

impl Blake3Hash {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Evidence<T, State, Witness>
where
    T: Serialize,
    State: Serialize,
    Witness: Serialize,
{
    pub payload: T,
    pub state: State,
    pub witness: Witness,
    pub epoch: u64,
    pub signature: IdentitySignature,
    pub hash: Blake3Hash,
}

impl<T, State, Witness> Evidence<T, State, Witness>
where
    T: Serialize,
    State: Serialize,
    Witness: Serialize,
{
    /// Calculate the BLAKE3 hash of the Evidence fields (payload, state, witness, epoch, signature)
    pub fn calculate_hash(&self) -> Blake3Hash {
        let mut hasher = Hasher::new();
        
        let payload_bytes = serde_json::to_vec(&self.payload).unwrap_or_default();
        let state_bytes = serde_json::to_vec(&self.state).unwrap_or_default();
        let witness_bytes = serde_json::to_vec(&self.witness).unwrap_or_default();
        let epoch_bytes = self.epoch.to_le_bytes();
        let sig_bytes = serde_json::to_vec(&self.signature).unwrap_or_default();

        hasher.update(&payload_bytes);
        hasher.update(&state_bytes);
        hasher.update(&witness_bytes);
        hasher.update(&epoch_bytes);
        hasher.update(&sig_bytes);

        let hash_result = hasher.finalize();
        Blake3Hash(*hash_result.as_bytes())
    }

    /// Perform full self-validation checks on the evidence block
    pub fn validate(&self) -> Result<(), EvidenceError> {
        // 1. Verify cryptographic binding
        let computed_hash = self.calculate_hash();
        if computed_hash != self.hash {
            return Err(EvidenceError::HashMismatch);
        }
        
        // 2. Verify authority signature using ed25519-dalek
        let public_key_bytes: &[u8] = &self.signature.public_key;
        let signature_bytes: &[u8] = &self.signature.signature_bytes;
        
        let verifying_key = match ed25519_dalek::VerifyingKey::try_from(public_key_bytes) {
            Ok(key) => key,
            Err(_) => return Err(EvidenceError::InvalidSignature),
        };
        
        let signature = match ed25519_dalek::Signature::from_slice(signature_bytes) {
            Ok(sig) => sig,
            Err(_) => return Err(EvidenceError::InvalidSignature),
        };
        
        use ed25519_dalek::Verifier;
        if verifying_key.verify(computed_hash.as_bytes(), &signature).is_err() {
            return Err(EvidenceError::InvalidSignature);
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum EvidenceError {
    HashMismatch,
    InvalidSignature,
    InvalidTransition,
    LatticeViolation,
}
```

---

## 4. References & Related Documents

*   For details on witness lattices and their join properties, see [Witness Lattices](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/witness-lattices.md).
*   For the alignment loss policies and fitness checks, see [Loss Policies](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/loss-policies.md).
*   To review the master type-law mapping, see [Type Law Atlas](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/type-law-atlas.md).
