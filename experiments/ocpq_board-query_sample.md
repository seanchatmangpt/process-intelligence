# Experiment: OCPQ Board-Level Query Validation

This experiment validates Object-Centric Process Queries (OCPQ) designed to audit interactions across heterogeneous objects (e.g., matching shipments to individual purchase items and corresponding financial logs).

## 1. OCPQ Board-Level Query JSON Validation Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "OcpqBoardQueryLog",
  "type": "object",
  "properties": {
    "query_id": { "type": "string" },
    "execution_timestamp": { "type": "string", "format": "date-time" },
    "query_logic": {
      "type": "object",
      "properties": {
        "primary_object_type": { "type": "string" },
        "interacting_object_types": {
          "type": "array",
          "items": { "type": "string" }
        },
        "temporal_constraints": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "relation": { "enum": ["before", "after", "concurrent"] },
              "activity_a": { "type": "string" },
              "activity_b": { "type": "string" }
            },
            "required": ["relation", "activity_a", "activity_b"]
          }
        }
      },
      "required": ["primary_object_type", "interacting_object_types"]
    },
    "query_results": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "matching_object_id": { "type": "string" },
          "related_object_ids": {
            "type": "array",
            "items": { "type": "string" }
          },
          "compliance_verdict": { "enum": ["COMPLIANT", "VIOLATION"] }
        },
        "required": ["matching_object_id", "related_object_ids", "compliance_verdict"]
      }
    }
  },
  "required": ["query_id", "execution_timestamp", "query_logic", "query_results"]
}
```

## 2. Concrete OCPQ Board-Level Query Execution

The following instance records a board-level query finding all high-value purchase orders that interacted with delayed parts:

```json
{
  "query_id": "ocpq_diligence_q1_delayed_synergy",
  "execution_timestamp": "2026-05-31T22:50:00Z",
  "query_logic": {
    "primary_object_type": "purchase_order",
    "interacting_object_types": ["part", "shipment"],
    "temporal_constraints": [
      {
        "relation": "before",
        "activity_a": "allocate_part",
        "activity_b": "ship_order"
      }
    ]
  },
  "query_results": [
    {
      "matching_object_id": "po_4001",
      "related_object_ids": ["part_a", "part_b", "shipment_9901_west"],
      "compliance_verdict": "COMPLIANT"
    }
  ]
}
```

## 3. Linkages to Standards and M&A Claims

- **Standard Crosswalk**: Aligns with standards mapped at [OCPQ Standard Ledger Placement](file:///Users/sac/process-intelligence/standards/ocpq_placement.md).
- **M&A Claims**: Defensibility claims are verified by mapping these queries directly to board-level claims at [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).