# Declare Standard Ledger Placement

The **Declare** framework (Pesic and van der Aalst 2006) represents a declarative process modeling paradigm. Instead of specifying the exact flow of activities (imperative modeling), Declare defines a set of constraints that the process execution must satisfy. This document establishes how Declare constraints, templates, and violations are formally mapped, checked, and recorded on the ledger.

---

## 1. Ontological Mapping to the Ledger

Declare constraints are based on Linear Temporal Logic (LTL) templates. The ledger represents these rules using template patterns:

| Declare Template Group | LTL Expression | Ledger Class | Description |
| :--- | :--- | :--- | :--- |
| **Existence(n, A)** | $\lozenge A$ | `MinOccurrenceConstraint` | Activity $A$ must occur at least $n$ times in the case. |
| **Absence(n, A)** | $\neg(\lozenge A)$ | `MaxOccurrenceConstraint` | Activity $A$ can occur at most $n-1$ times in the case. |
| **Response(A, B)** | $\Box(A \implies \lozenge B)$ | `ResponseConstraint` | If activity $A$ occurs, activity $B$ must occur after it. |
| **Precedence(A, B)**| $\neg B \mathbin{\mathcal{U}} A$ | `PrecedenceConstraint` | Activity $B$ cannot occur unless activity $A$ has occurred. |
| **Succession(A, B)**| $\text{Response}(A,B) \land \text{Precedence}(A,B)$ | `SuccessionConstraint` | Strict ordered succession of activities $A$ and $B$. |

The ledger registers each constraint assertion in a validation table:

```json
{
  "constraint_id": "dec-770e8400-e29b-41d4-a716-446655441111",
  "template": "Response",
  "source_activity": "Create Invoice",
  "target_activity": "Approve Invoice",
  "activation_count": 1205,
  "violation_count": 3
}
```

---

## 2. Type Laws and Automata Verification

Every Declare constraint is compiled into a Finite State Automaton (FSA) to evaluate event stream compliance:

1.  **FSA Execution**: Ingested events are replayed on the FSA. A state is "accepting" if the LTL formula is satisfied at that point in the trace.
2.  **State Verification**: A trace $\sigma$ violates a Declare constraint if the final state of the FSA after replaying $\sigma$ is non-accepting:
    $$\delta^*(s_0, \sigma) \notin F_{\text{accept}}$$
3.  **Cryptographic Violations**: When a constraint violation occurs, the validator writes a signed violation receipt containing the trace UUID, the violated constraint ID, and the event timestamp:
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