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

## 3. Linkages to Standards and M&A Claims

- **Standard Crosswalk**: Aligns with standards mapped at file:///Users/sac/process-intelligence/standards/declare_placement.md.
- **M&A Claims**: Defensibility claims are verified by mapping these rule violation proofs to operational risk assessments at file:///Users/sac/process-intelligence/ma/define_operational_debt_taxonomy.md.