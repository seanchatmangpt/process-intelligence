# Declare Standard Ledger Placement

The **Declare** framework (Pesic and van der Aalst 2006) represents a declarative process modeling paradigm. Instead of specifying the exact flow of activities (imperative modeling), Declare defines a set of constraints that the process execution must satisfy. This document establishes how Declare constraints, templates, and violations are formally mapped, checked, and recorded on the ledger.

---

## 1. Ontological Mapping to the Ledger

Declare constraints are defined using Linear Temporal Logic over Finite Traces ($\text{LTL}_f$). The ledger represents and stores these rules using structured templates. Below is the complete mapping of all 21 Declare templates, including their future-only $\text{LTL}_f$ semantic definitions and vacuous satisfaction activation conditions:

| Declare Template Group | $\text{LTL}_f$ Formal Formula | Ledger Class | Activation Condition | Description |
| :--- | :--- | :--- | :--- | :--- |
| **Existence(A)** | $\lozenge A$ | `MinOccurrenceConstraint` | $\text{True}$ (Always Active) | Activity $A$ must occur at least once. |
| **Existence2(A)** | $\lozenge(A \land \bigcirc \lozenge A)$ | `MinOccurrenceConstraint` | $\text{True}$ (Always Active) | Activity $A$ must occur at least twice. |
| **Existence3(A)** | $\lozenge(A \land \bigcirc \lozenge(A \land \bigcirc \lozenge A))$ | `MinOccurrenceConstraint` | $\text{True}$ (Always Active) | Activity $A$ must occur at least three times. |
| **Absence(A)** | $\Box \neg A$ | `MaxOccurrenceConstraint` | $\text{True}$ (Always Active) | Activity $A$ must not occur. |
| **Absence2(A)** | $\Box \neg A \lor \lozenge(A \land \bigcirc \Box \neg A)$ | `MaxOccurrenceConstraint` | $\text{True}$ (Always Active) | Activity $A$ can occur at most once. |
| **Absence3(A)** | $\Box \neg A \lor \lozenge(A \land \bigcirc (\Box \neg A \lor \lozenge(A \land \bigcirc \Box \neg A)))$ | `MaxOccurrenceConstraint` | $\text{True}$ (Always Active) | Activity $A$ can occur at most twice. |
| **Init(A)** | $A \lor \text{empty\_trace}$ | `InitConstraint` | $\text{True}$ (Always Active) | If any activity occurs, $A$ must be the first. |
| **RespondedExistence(A, B)** | $\lozenge A \implies \lozenge B$ | `RespondedExistenceConstraint` | Occurrence of $A$ | If activity $A$ occurs, activity $B$ must also occur (before or after). |
| **CoExistence(A, B)** | $\lozenge A \iff \lozenge B$ | `CoExistenceConstraint` | Occurrence of $A$ or $B$ | If either activity occurs, the other must also occur. |
| **Response(A, B)** | $\Box(A \implies \lozenge B)$ | `ResponseConstraint` | Occurrence of $A$ | If activity $A$ occurs, activity $B$ must occur at or after it. |
| **Precedence(A, B)** | $\Box(\neg B) \lor (\neg B \mathbin{\mathcal{U}} A)$ | `PrecedenceConstraint` | Occurrence of $B$ | Activity $B$ cannot occur unless activity $A$ has occurred before it. |
| **Succession(A, B)** | $\text{Response}(A, B) \land \text{Precedence}(A, B)$ | `SuccessionConstraint` | Occurrence of $A$ or $B$ | Strict ordered succession of activities $A$ and $B$. |
| **AlternateResponse(A, B)** | $\Box(A \implies \widetilde{\bigcirc}(\neg A \mathbin{\mathcal{W}} B))$ | `AlternateResponseConstraint` | Occurrence of $A$ | Between any two consecutive occurrences of $A$, activity $B$ must occur. |
| **AlternatePrecedence(A, B)** | $\text{Precedence}(A, B) \land \Box(B \implies \widetilde{\bigcirc}(\neg B \mathbin{\mathcal{W}} A))$ | `AlternatePrecedenceConstraint` | Occurrence of $B$ | Between any two consecutive occurrences of $B$, activity $A$ must occur. |
| **AlternateSuccession(A, B)** | $\text{AlternateResponse}(A, B) \land \text{AlternatePrecedence}(A, B)$ | `AlternateSuccessionConstraint` | Occurrence of $A$ or $B$ | Both AlternateResponse and AlternatePrecedence must hold. |
| **ChainResponse(A, B)** | $\Box(A \implies \bigcirc B)$ | `ChainResponseConstraint` | Occurrence of $A$ | Activity $B$ must immediately follow activity $A$. |
| **ChainPrecedence(A, B)** | $\neg B \land \Box(\bigcirc B \implies A)$ | `ChainPrecedenceConstraint` | Occurrence of $B$ | Activity $A$ must immediately precede activity $B$. |
| **ChainSuccession(A, B)** | $\text{ChainResponse}(A, B) \land \text{ChainPrecedence}(A, B)$ | `ChainSuccessionConstraint` | Occurrence of $A$ or $B$ | Both ChainResponse and ChainPrecedence must hold. |
| **NotCoExistence(A, B)** | $\neg(\lozenge A \land \lozenge B)$ | `NotCoExistenceConstraint` | Occurrence of $A$ or $B$ | Activities $A$ and $B$ cannot both occur in the same trace. |
| **NotSuccession(A, B)** | $\Box(A \implies \Box \neg B)$ | `NotSuccessionConstraint` | Occurrence of $A$ | Activity $A$ cannot be eventually followed by activity $B$. |
| **NotChainSuccession(A, B)** | $\Box(A \implies \neg \bigcirc B)$ | `NotChainSuccessionConstraint` | Occurrence of $A$ | Activity $B$ cannot immediately follow activity $A$. |
| **ExclusiveChoice(A, B)** | $(\lozenge A \lor \lozenge B) \land \neg(\lozenge A \land \lozenge B)$ | `ExclusiveChoiceConstraint` | $\text{True}$ (Always Active) | Exactly one of $A$ or $B$ must occur. |

The ledger registers each constraint verification in a validation table, explicitly tracking fulfillment, violation, and vacuous satisfaction counts to check for vacuous truth:

```json
{
  "constraint_id": "dec-770e8400-e29b-41d4-a716-446655441111",
  "template": "Response",
  "source_activity": "Create Invoice",
  "target_activity": "Approve Invoice",
  "activation_count": 1205,
  "fulfillment_count": 1202,
  "violation_count": 3,
  "is_vacuously_satisfied": false
}
```

---

## 2. Finite Trace Semantics ($\text{LTL}_f$) and Vacuous Truth Verification

### 2.1 Formal Semantics of $\text{LTL}_f$
Let $\sigma = e_1 e_2 \dots e_m$ be a finite trace of length $|\sigma| = m$ over alphabet $\Sigma$, where each event $e_i \in \Sigma$. Let $\sigma, i \models \phi$ denote that the formula $\phi$ holds at index $i \in \{1, \dots, m\}$:
*   $\sigma, i \models p \iff p = e_i$ (for atomic proposition/activity $p$)
*   $\sigma, i \models \neg \phi \iff \sigma, i \not\models \phi$
*   $\sigma, i \models \phi_1 \lor \phi_2 \iff \sigma, i \models \phi_1 \text{ or } \sigma, i \models \phi_2$
*   $\sigma, i \models \bigcirc \phi \iff i < m \text{ and } \sigma, i+1 \models \phi$ (Strong Next: false at the end of the trace)
*   $\sigma, i \models \widetilde{\bigcirc} \phi \iff i = m \text{ or } \sigma, i+1 \models \phi$ (Weak Next: true at the end of the trace)
*   $\sigma, i \models \phi_1 \mathbin{\mathcal{U}} \phi_2 \iff \exists j \in \{i, \dots, m\} \text{ s.t. } \sigma, j \models \phi_2 \text{ and } \forall k \in \{i, \dots, j-1\}, \sigma, k \models \phi_1$
*   $\sigma, i \models \lozenge \phi \iff \exists j \in \{i, \dots, m\} \text{ s.t. } \sigma, j \models \phi$
*   $\sigma, i \models \Box \phi \iff \forall j \in \{i, \dots, m\}, \sigma, j \models \phi$
*   $\sigma, i \models \phi_1 \mathbin{\mathcal{W}} \phi_2 \iff (\sigma, i \models \phi_1 \mathbin{\mathcal{U}} \phi_2) \lor (\sigma, i \models \Box \phi_1)$

### 2.2 Activation and Vacuous Satisfaction
A relation constraint $\phi$ has a defined activation condition $\alpha_{\phi}$ (a state or event formula) and a target condition $\beta_{\phi}$. Let $\text{Acts}(\sigma, \alpha_{\phi}) \subseteq \{1, \dots, m\}$ be the set of indices where the activation condition is satisfied:
$$\text{Acts}(\sigma, \alpha_{\phi}) = \{ i \in \{1, \dots, m\} \mid \sigma, i \models \alpha_{\phi} \}$$

*   **Fulfillment**: An activation at index $i \in \text{Acts}(\sigma, \alpha_{\phi})$ is fulfilled if the temporal target condition $\beta_{\phi}$ is satisfied from index $i$ onward.
*   **Violation**: An activation at index $i \in \text{Acts}(\sigma, \alpha_{\phi})$ is violated if $\beta_{\phi}$ is not satisfied.
*   **Vacuous Satisfaction**: A trace $\sigma$ satisfies a constraint $\phi$ (i.e., $\sigma, 1 \models \phi$) vacuously if the activation set is empty:
    $$\text{Acts}(\sigma, \alpha_{\phi}) = \emptyset$$

To prevent vacuous truths from masquerading as actual process execution coverage, the validation ledger logs `is_vacuously_satisfied: true` when $\sigma, 1 \models \phi$ and $|\text{Acts}(\sigma, \alpha_{\phi})| = 0$.

### 2.3 Automata Compilation and State Verification
Every $\text{LTL}_f$ constraint $\phi$ is compiled into a Deterministic Finite Automaton (DFA) $\mathcal{A}_{\phi} = (\Sigma, Q, q_0, \delta, F)$ where:
1.  **Trace Replay**: Ingested events are replayed on $\mathcal{A}_{\phi}$ starting from initial state $q_0$.
2.  **Acceptance**: A trace $\sigma = e_1 \dots e_m$ satisfies the constraint if and only if:
    $$\delta^*(q_0, \sigma) \in F$$
3.  **Runtime Violation Detection**: If during replay the automaton reaches a sink state $q_{\text{sink}} \notin F$ from which no accepting state is reachable ($\forall s \in \Sigma^*, \delta^*(q_{\text{sink}}, s) \notin F$), a **Permanent Violation** is declared immediately.
4.  **Cryptographic Receipts**: When a permanent or final violation occurs, the validator writes a signed violation receipt to the ledger:
    $$\text{ViolationReceipt} = \operatorname{BLAKE3}\left( \text{TraceUUID} \parallel \text{ConstraintID} \parallel t_{\text{violation}} \right)$$

---

## 3. Academic Foundations and Conformance

*   Declare is used for flexible processes where strict ordering is not required but compliance gates must be enforced.
*   For experimental validation, see the [Declare Violation Sample](file:///Users/sac/process-intelligence/experiments/declare_violation_sample.md).
*   For paper mappings, see the [Paper Canon](file:///Users/sac/process-intelligence/sources/papers/paper-canon.md).

---

## 4. M&A Slide-to-Receipt Bridge

To verify compliance claims for unstructured processes (e.g., "all financial audits satisfy segregation of duty templates"):
1.  The segregation rules are modeled as Declare precedence/response constraints.
2.  Every violation-free state must be backed by a wasm4pm execution proof.
3.  The results are linked in [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md) and verified against the rules in [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).

---

## 5. Trans-Standard Conversions and Loss Policy

### 5.1 Conversion: Declare to Finite State Automaton (FSA)
When evaluating declarative Linear Temporal Logic ($\text{LTL}_f$) constraints, each rule is compiled into a Deterministic Finite Automaton (DFA). The system composes these individual automata into a single execution monitor.

*   **Structural Loss Policy**:
    1.  **Composition State Space Refusal**: State-space composition can lead to state explosion. If the combined automaton exceeds 10,000 states, the compiler refuses the conversion.
    2.  **Redundancy Pruning**: Overlapping or logically redundant constraints are simplified. The structural loss policy permits the removal of redundant constraints if they are proven to be subsumed (e.g., `Succession(A, B)` subsumes `Response(A, B)`).
*   **Signed LossReport Output Schema**:
    Every compilation generates a `LossReport` signed by the compiler witness:
    ```json
    {
      "loss_report_id": "lr-declare-fsa-uuid",
      "timestamp": "2026-06-01T00:00:00Z",
      "source_format": "Declare",
      "target_format": "FSA",
      "structural_changes": {
        "compiled_constraints_count": 6,
        "total_fsa_states": 128,
        "pruned_redundant_constraints": 1
      },
      "witness_signature": "SIG_ED25519_..."
    }
    ```