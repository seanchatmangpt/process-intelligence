# Lifecycle: Checkpoint Lifecycle Model Complete

This document defines the automated validation checkpoints and verification test assertions required to certify that a process model is structurally, behaviorally, and operationally complete.

## Automated Verification Assertions

To declare a model lifecycle-complete, the verification test suite must execute and pass the following assertions:

### 1. Soundness Check (`assert_soundness`)
* **Input**: Petri Net $N = (P, T, F)$
* **Logic**:
  1. Parse the Petri Net and assert it is a Workflow Net (WF-net) with a single source place $i$ and a single sink place $o$.
  2. Construct the coverability tree.
  3. Assert that the maximum token count in any place is 1 (1-boundedness/safety).
  4. Assert that for every reachable marking $M$, there is a path to the final marking $[o]$.
  5. Assert that no transition $t \in T$ is dead.
* **Assertion Code Blueprint**:
  ```python
  def assert_soundness(net):
      assert is_wf_net(net), "Error: Not a valid Workflow Net structure"
      assert is_bounded(net, limit=1), "Error: Petri net is not 1-bounded"
      assert is_live(net), "Error: Deadlock detected in state space"
      assert no_dead_transitions(net), "Error: Dead transitions exist"
      return True
  ```

### 2. Conformance Fitness Check (`assert_fitness`)
* **Input**: Petri Net $N$, Event Log $L$, threshold $\theta_{\text{fit}}$ (default = 0.95)
* **Logic**:
  1. Compute the optimal alignment $\gamma_{\text{opt}}$ for each trace in $L$ against $N$.
  2. Calculate the alignment fitness $\operatorname{fitness}(L, N)$.
  3. Assert that the fitness exceeds the threshold.
* **Assertion Code Blueprint**:
  ```python
  def assert_fitness(net, log, threshold=0.95):
      fit_score = calculate_alignment_fitness(net, log)
      assert fit_score >= threshold, f"Error: Conformance fitness {fit_score} is below threshold {threshold}"
      return True
  ```

### 3. Ghost Transitions Check (`assert_no_ghost_transitions`)
* **Input**: Petri Net $N$, Event Log $L$
* **Logic**:
  1. Identify all transitions $t \in T$ that have labels.
  2. Map the set of active transitions in the log $A_L$.
  3. Assert that every labeled transition $t$ is fired at least once in the log, or is classified as an explicit routing transition (tau transition).
  ```python
  def assert_no_ghost_transitions(net, log):
      active_labels = get_log_activities(log)
      for transition in net.transitions:
          if transition.label and not transition.is_silent:
              assert transition.label in active_labels, f"Error: Ghost transition {transition.label} has no matching log events"
      return True
  ```

### 4. Decommission Receipt Verification (`assert_decommission_receipt`)
* **Input**: Decommissioning Receipt $R_d$, public key $K_{pub}$
* **Logic**:
  1. Re-calculate the SHA-256 hashes of the retired model $N$ and final log $L_{final}$.
  2. Verify that the hashes match the fields in $R_d$.
  3. Decrypt the signature using $K_{pub}$ and verify authenticity.
  ```python
  def assert_decommission_receipt(receipt, pub_key):
      computed_model_hash = sha256(receipt.model_structure)
      computed_log_hash = sha256(receipt.final_log)
      assert computed_model_hash == receipt.model_hash, "Error: Model structure hash mismatch"
      assert computed_log_hash == receipt.log_hash, "Error: Log hash mismatch"
      assert verify_signature(receipt.signature, receipt.payload, pub_key), "Error: Invalid receipt signature"
      return True
  ```

---

## Checkpoint Verdict Rules

A process model receives the status **ALIVE** (authorized for operational execution) if and only if:
$$\operatorname{status}(N) = \text{ALIVE} \iff \operatorname{assert\_soundness}(N) \land \operatorname{assert\_fitness}(N, L, 0.95) \land \operatorname{assert\_no\_ghost\_transitions}(N, L)$$

If the process is retired, it receives the status **DECOMMISSIONED** if and only if:
$$\operatorname{status}(N) = \text{DECOMMISSIONED} \iff \operatorname{assert\_decommission\_receipt}(R_d, K_{pub})$$

---

## Related Documents
* Review the completeness checklist in [Audit Lifecycle Completeness](file:///Users/sac/process-intelligence/lifecycle/audit__lifecycle_completeness.md).
* Back to [Lifecycle README](file:///Users/sac/process-intelligence/lifecycle/docs-law__lifecycle_readme.md).

---

## Section 19: New Modules (v30.1.1 Spec)

We formalize three additional modules:
1. **Process Cube**: Dimensions are encoded as distinct types using const-strings:
$$\text{CubeDimension}\langle\text{const NAME: \&'static str}\rangle$$
2. **Temporal Ordering**: Four-valued temporal ordering relation:
$$\text{TemporalOrder} = \{\texttt{Before}, \texttt{After}, \texttt{Concurrent}, \texttt{Unknown}\}$$
3. **Object Lifecycle**: Phase transitions enforced through typestate methods:
$$\text{PHASE} \in \{\texttt{Created}, \texttt{Active}, \texttt{Modified}, \texttt{Archived}, \texttt{Deleted}\}$$
To prevent the nightly Rust compiler E0391 variance cycle when using enums in const generic bounds, intermediate type aliases are introduced:
$$\text{type ActiveToModified}\langle T \rangle = \text{LifecycledObject}\langle T, \texttt{Modified} \rangle$$
This preserves readability and compiles correctly under nightly rules by avoiding direct evaluation of where-bound constraints during variance inference.