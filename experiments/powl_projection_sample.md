# Experiment: POWL Projection Validation

This experiment validates process projections defined using the POWL (Process Trees with Loop-like operators and partial orders) notation. POWL is a generalization of Process Trees that supports partial order execution and explicit loop structures.

## 1. POWL Projection JSON Validation Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "PowlProjectionOutput",
  "type": "object",
  "properties": {
    "powl_id": { "type": "string" },
    "activities": {
      "type": "array",
      "items": { "type": "string" }
    },
    "structure": {
      "type": "object",
      "properties": {
        "operator": { "enum": ["sequence", "choice", "loop", "partial_order"] },
        "children": {
          "type": "array",
          "items": { "type": "object" }
        },
        "partial_order_edges": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "from_index": { "type": "integer" },
              "to_index": { "type": "integer" }
            },
            "required": ["from_index", "to_index"]
          }
        }
      },
      "required": ["operator", "children"]
    }
  },
  "required": ["powl_id", "activities", "structure"]
}
```

## 2. Concrete POWL Projection Output

The following instance defines a loop-based POWL tree where `Review` can run concurrently or partially ordered with `Approve` inside a loop context:

```json
{
  "powl_id": "powl_procure_to_pay_2026",
  "activities": ["Register", "Review", "Approve", "Pay"],
  "structure": {
    "operator": "sequence",
    "children": [
      {
        "activity": "Register"
      },
      {
        "operator": "loop",
        "children": [
          {
            "operator": "partial_order",
            "children": [
              { "activity": "Review" },
              { "activity": "Approve" }
            ],
            "partial_order_edges": [
              { "from_index": 0, "to_index": 1 }
            ]
          }
        ]
      },
      {
        "activity": "Pay"
      }
    ]
  }
}
```

## 3. Linkages to Standards and M&A Claims

- **Standard Crosswalk**: Aligns with standards mapped at [powl_placement.md](file:///Users/sac/process-intelligence/standards/powl_placement.md).
- **M&A Claims**: Defensibility claims of POWL process structures are mapped to the synergy taxonomy at [define_synergy_claim_taxonomy.md](file:///Users/sac/process-intelligence/ma/define_synergy_claim_taxonomy.md).