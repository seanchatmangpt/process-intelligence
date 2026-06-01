# Adversarial Type-Law Research Map (v30.1.1)
**Target:** `wasm4pm-compat`
**Focus:** Witness Lattice Forgeability, Absence-Proof Failures, Cross-Witness Contamination Prevention

## 1. Witness Lattice Forgeability

### 1.1. Morphological Vulnerabilities in Witness Encoding
The primary risk in `wasm4pm-compat` arises from the non-injective mapping of runtime state transitions to their representation in the witness lattice. If the serialization of a witness allows for padding or canonicalization ambiguities, an adversary can forge computationally indistinguishable but semantically distinct witness states. 

**Vulnerability Vector:**
*   **State-Space Aliasing:** When a valid target state $S_t$ and a forged state $S_f$ produce identical lattice coordinates $C(S_t) = C(S_f)$, the verification function $V$ cannot discern the invalid transition.
*   **Malleability of Unconstrained Inputs:** Parameters omitted from the constraints logic but retained in the data payload enable adversaries to graft arbitrary semantic load onto verified transitions.

### 1.2. Forgery Mechanics
An attacker exploiting witness lattice forgeability typically targets the boundary where the WebAssembly linear memory intersects with the proof system's constraint environment. By manipulating unmapped memory regions, the adversary can induce a state where the cryptographic receipt validates, but the WASM execution context has been silently altered.

## 2. Absence-Proof Failures

### 2.1. The Epistemic Gap in Non-Inclusion
Absence-proofs in `wasm4pm-compat` are designed to assert that a specific illicit state transition or data structure does *not* exist within a given epoch. Failure occurs when the Merkle-tree (or equivalent accumulator) construction relies on sparse representations without strict cryptographic enforcement of the "empty" leaves.

**Failure Modes:**
*   **Default-State Exploitation:** If the absence-proof evaluates default values (e.g., zeroed leaves) as valid "absence", an attacker can selectively zero out malicious operations post-execution but pre-accumulation.
*   **Hash Collision Traps:** Utilizing weak hash functions for the sparse tree allows for crafting nodes that evaluate to the empty-leaf hash despite containing latent, malicious structure.

## 3. Structural Laws for Preventing Cross-Witness Contamination

To ensure robust typestate enforcement and prevent the blending of adversarial state with valid witness structures (Cross-Witness Contamination), the following structural laws must be enforced at the compiler/scaffolding level.

### Law I: Strict Injectivity of State Mapping (The Chatman Constraint)
Every distinct semantic state in the WASM linear memory MUST map to a unique, non-colliding coordinate in the witness lattice.
$$ \forall S_a, S_b \in StateSpace, S_a \neq S_b \implies C(S_a) \neq C(S_b) $$
**Implementation:** Enforce rigid schema definitions using unpadded, canonical serialization formats (e.g., deterministic bincode) prior to constraint generation.

### Law II: Exhaustive Typestate Closure
The state machine defining `wasm4pm-compat` must be closed. There must be no reachable state $S_{unknown}$ that lacks a defined transition or explicit panic/rejection condition within the ontology.
**Implementation:** All boundary interactions must pass through a strict type-guard function that validates the structural integrity and semantic legality of the input before any transition is accumulated in the witness.

### Law III: Cryptographic Isolation of Witness Contexts
Witnesses generated across different execution contexts or epochs must not share underlying mutable references or accumulator roots unless cryptographically bound.
**Implementation:** Introduce a context-specific domain separator into the hash function for every witness generation step. 
$$ Hash_{domain}(Witness) = H(DomainID_{epoch} || WitnessData) $$
This ensures that a witness valid in Epoch $N$ cannot be replayed or cross-contaminated into Epoch $N+1$.