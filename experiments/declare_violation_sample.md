# Experiment: DECLARE Constraint Violation Validation

This experiment validates violations of declarative process rules defined using the DECLARE language (based on Linear Temporal Logic). DECLARE specifies policies like `Precedence(A, B)` (B cannot occur unless A occurred first) or `Succession(A, B)` (A must be followed by B, and B must be preceded by A).

## 1. DECLARE Violation JSON Validation Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "DeclareViolationLog",
  "type": "object",
  "properties": {
    "log_id": { "type": "string" },
    "violations": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "trace_id": { "type": "string" },
          "rule_name": { "type": "string" },
          "rule_type": { "enum": ["Precedence", "Response", "Succession", "CoExistence"] },
          "param_a": { "type": "string" },
          "param_b": { "type": "string" },
          "violation_details": {
            "type": "object",
            "properties": {
              "violation_index": { "type": "integer" },
              "trigger_event_id": { "type": "string" },
              "error_message": { "type": "string" }
            },
            "required": ["violation_index", "trigger_event_id", "error_message"]
          }
        },
        "required": ["trace_id", "rule_name", "rule_type", "param_a", "param_b", "violation_details"]
      }
    }
  },
  "required": ["log_id", "violations"]
}
```

## 2. Concrete DECLARE Violation Instance

The following instance records a violation of the rule `Precedence("Audit_Invoice", "Approve_Payment")` where payment occurred without auditing:

```json
{
  "log_id": "procurement_audit_2026_q2",
  "violations": [
    {
      "trace_id": "trace_corp_9021",
      "rule_name": "Audited_Before_Payment",
      "rule_type": "Precedence",
      "param_a": "Audit_Invoice",
      "param_b": "Approve_Payment",
      "violation_details": {
        "violation_index": 3,
        "trigger_event_id": "evt_approve_payment_881b",
        "error_message": "Activity 'Approve_Payment' fired at index 3, but preceding activity 'Audit_Invoice' was never executed in trace."
      }
    }
  ]
}
```

### 2.1 Step-by-Step DFA Replay Verification
For the constraint $\phi = \Box(\neg \text{Approve\_Payment}) \lor (\neg \text{Approve\_Payment} \mathbin{\mathcal{U}} \text{Audit\_Invoice})$:
1.  **DFA Definition**: The automaton $\mathcal{A}_{\phi}$ has states $Q = \{q_0, q_1, q_{\text{viol}}\}$, with initial state $q_0$ and accepting states $F = \{q_0, q_1\}$.
    *   $q_0$: State where neither `Audit_Invoice` nor `Approve_Payment` has occurred. (Accepting)
    *   $q_1$: State where `Audit_Invoice` has occurred. (Accepting, and once in $q_1$, any subsequent `Approve_Payment` is allowed and remains in $q_1$).
    *   $q_{\text{viol}}$: Sink state reached if `Approve_Payment` occurs without `Audit_Invoice`. (Non-accepting, $\delta(q_{\text{viol}}, e) = q_{\text{viol}}$).
2.  **Trace Sequence**: Let $\sigma = \langle e_1, e_2, e_3 \rangle$ where $e_1 = \text{Create\_Order}$, $e_2 = \text{Receive\_Goods}$, $e_3 = \text{Approve\_Payment}$.
3.  **State Transitions**:
    *   $\delta(q_0, \text{Create\_Order}) = q_0$
    *   $\delta(q_0, \text{Receive\_Goods}) = q_0$
    *   $\delta(q_0, \text{Approve\_Payment}) = q_{\text{viol}}$
4.  **Verdict**: Since the final state after replaying $\sigma$ is $q_{\text{viol}} \notin F$, the trace violates the constraint. The violation is logged with activation trigger `evt_approve_payment_881b` at trace index 3.

### 2.2 Vacuous Satisfaction Verification
Consider the trace $\sigma_{\text{vacuous}} = \langle \text{Create\_Order}, \text{Receive\_Goods} \rangle$.
1.  **DFA Replay**:
    *   $\delta(q_0, \text{Create\_Order}) = q_0$
    *   $\delta(q_0, \text{Receive\_Goods}) = q_0$
2.  **Verdict**: The final state is $q_0 \in F$, so the trace satisfies $\phi$.
3.  **Vacuity Check**: The activation condition is the occurrence of $B = \text{Approve\_Payment}$. The activation set is:
    $$\text{Acts}(\sigma_{\text{vacuous}}, B) = \emptyset$$
    Since the trace is accepted but has 0 activations, the constraint is satisfied **vacuously**. The validation system outputs a compliance report indicating this status:

```json
{
  "log_id": "procurement_audit_2026_q2_compliance",
  "evaluations": [
    {
      "trace_id": "trace_corp_9022",
      "rule_name": "Audited_Before_Payment",
      "rule_type": "Precedence",
      "param_a": "Audit_Invoice",
      "param_b": "Approve_Payment",
      "status": "VacuouslySatisfied",
      "activation_count": 0,
      "fulfillment_count": 0,
      "violation_count": 0,
      "is_vacuously_satisfied": true
    }
  ]
}
```

---

## 3. Linkages to Standards and M&A Claims

*   **Standard Crosswalk**: Aligns with standards mapped at [Declare Placement Standard](file:///Users/sac/process-intelligence/standards/declare_placement.md).
*   **M&A Claims**: Defensibility claims are verified by mapping these rule violation proofs to operational risk assessments at [Operational Debt Taxonomy](file:///Users/sac/process-intelligence/ma/define_operational_debt_taxonomy.md).