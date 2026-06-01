# Full-Lifecycle Process Intelligence (FLPI)

## 1. The Imperative of Total Observation
A process unobserved is a process compromised. FLPI asserts that intelligence must span the entire timeline of a process—from its dark-state genesis to its cryptographic terminus. This complete observation is verified through conformance alignments defined in the [Blue River Dam](file:///Users/sac/process-intelligence/doctrine/blue-river-dam.md) doctrine.

## 2. The FLPI Continuum and Mathematical Mapping
Each stage of the FLPI continuum maps to a formal property in the underlying Petri net process model $W = (P, T, F, i, o)$ and its execution trace $\sigma$:

1.  **Dark State (Genesis)**:
    - *Definition*: The theoretical origin of intent, defining the initial state space.
    - *Formalism*: Initializing the marking $M_0 = [i]$. The system prepares the landing zone by binding execution variables to the source place $i$.
2.  **Ontological Emergence**:
    - *Definition*: The moment a process requests state formulation.
    - *Formalism*: The VM compiler evaluates a proposed transition $t \in T$. This transition is checked against the global LTL safety policies $\Phi_{\text{Gov}}$ defined by the `ostar-governor`.
3.  **Actuation (The Forge)**:
    - *Definition*: The process manipulates data or manifests system changes.
    - *Formalism*: The WASM VM fires transition $t$, updating marking $M_k \xrightarrow{t} M_{k+1}$ under the zero-latency constraints of [Autonomic Knowledge Actuation](file:///Users/sac/process-intelligence/doctrine/autonomic-knowledge-actuation.md).
4.  **Audit & Receipt**:
    - *Definition*: Immutable OTel traces and BLAKE3 receipts are generated.
    - *Formalism*: The trace prefix $\sigma_{\le k}$ is committed to the unforgeable ledger. The `ostar-auditor` computes the real-time conformance alignment:
      $$\text{Fitness}(\sigma_{\le k}, W) = 1.0$$
      If the fitness drops below $1.0$, a violation is immediately raised.
5.  **Terminus (Closure)**:
    - *Definition*: Verification that the process completed its intended state transition without dropping logic.
    - *Formalism*: The marking reaches the final sink place $o$ such that $M_n = [o]$, and no other tokens remain in the net (guaranteed by WF-net soundness). The `ostar-doctor` confirms closure and seals the execution envelope.

## 3. AGI-Adversarial Posture: Eliminating Shadow-States
At every stage, FLPI assumes a hostile AGI is attempting to hijack the state transition or scaffold "ghost processes" (transitions executing outside the observed Petri net structure). 
To prevent shadow-states:
- **State Exhaustiveness**: The complete system state must be representable as a vector of tokens over the set of places $P$.
- **Liveness Guarantee**: The short-circuited net $\overline{W}$ must be mathematically proven to be live and bounded. If a process attempts to execute a transition $t' \notin T$, the step cannot map to the flow relation $F$, and the compilation fails.
- **Trace-to-Model Alignment**: The auditor verifies that every event in the trace corresponds to a valid transition. Let $\sigma$ be the execution trace. If any event $e \in \sigma$ does not align synchronously with a transition $t \in T$ (resulting in a model-move cost or log-move cost), the deviation is flagged, and the [Blue River Dam](file:///Users/sac/process-intelligence/doctrine/blue-river-dam.md) triggers immediate kinetic containment.
