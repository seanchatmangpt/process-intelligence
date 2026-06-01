# Experiment: XES Loss-Policy Validation

This experiment validates corporate event logs against the XES (eXtensible Event Stream) standard, focusing specifically on compliance with an insurance or financial Loss Policy process (claims, appraisals, payouts, and compliance audits).

## 1. XES Loss-Policy JSON Validation Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "XesLossPolicyLog",
  "type": "object",
  "properties": {
    "log_attributes": {
      "type": "object",
      "properties": {
        "concept_name": { "type": "string" },
        "lifecycle_transition": { "type": "string" }
      },
      "required": ["concept_name"]
    },
    "traces": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "trace_id": { "type": "string" },
          "policy_holder_id": { "type": "string" },
          "events": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "concept_name": { "type": "string" },
                "time_timestamp": { "type": "string", "format": "date-time" },
                "org_resource": { "type": "string" },
                "loss_amount_usd": { "type": "number", "minimum": 0.0 },
                "claim_approved": { "type": "boolean" }
              },
              "required": ["concept_name", "time_timestamp", "org_resource"]
            }
          }
        },
        "required": ["trace_id", "policy_holder_id", "events"]
      }
    }
  },
  "required": ["log_attributes", "traces"]
}
```

## 2. Concrete Loss-Policy Event Stream (XES JSON)

The following instance defines a compliant policy appraisal sequence followed by an approved payout:

```json
{
  "log_attributes": {
    "concept_name": "Loss_Policy_Claims_Log"
  },
  "traces": [
    {
      "trace_id": "claim_90210_auto",
      "policy_holder_id": "holder_77a9",
      "events": [
        {
          "concept_name": "File_Claim",
          "time_timestamp": "2026-05-31T22:44:00Z",
          "org_resource": "web_portal_api",
          "loss_amount_usd": 14500.00
        },
        {
          "concept_name": "Assess_Loss",
          "time_timestamp": "2026-05-31T22:45:10Z",
          "org_resource": "assessor_smith_22",
          "loss_amount_usd": 12800.00
        },
        {
          "concept_name": "Approve_Payout",
          "time_timestamp": "2026-05-31T22:46:00Z",
          "org_resource": "compliance_mgr_01",
          "loss_amount_usd": 12800.00,
          "claim_approved": true
        },
        {
          "concept_name": "Disburse_Funds",
          "time_timestamp": "2026-05-31T22:47:00Z",
          "org_resource": "automated_ach_gateway",
          "loss_amount_usd": 12800.00
        }
      ]
    }
  ]
}
```

## 3. Linkages to Standards and M&A Claims

- **Standard Crosswalk**: Fits into XES serialization definitions mapped at file:///Users/sac/process-intelligence/standards/xes_process-intelligence_placement.md.
- **M&A Claims**: Defensibility claims of loss-policy conformance are mapped to process liabilities at file:///Users/sac/process-intelligence/ma/define_process_liability_claim_taxonomy.md.