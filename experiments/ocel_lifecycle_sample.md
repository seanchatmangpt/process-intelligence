# Experiment: OCEL Lifecycle Validation

This experiment validates object-centric event logs against the OCEL 2.0 standard. Unlike traditional XES logs that assume a single case identifier, OCEL logs track events that interact with multiple objects of different types, capturing complex process topologies.

## 1. OCEL 2.0 Lifecycle JSON Validation Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Ocel2LifecycleLog",
  "type": "object",
  "properties": {
    "object_types": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "name": { "type": "string" },
          "attributes": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "name": { "type": "string" },
                "type": { "type": "string" }
              },
              "required": ["name", "type"]
            }
          }
        },
        "required": ["name", "attributes"]
      }
    },
    "event_types": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "name": { "type": "string" },
          "attributes": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "name": { "type": "string" },
                "type": { "type": "string" }
              },
              "required": ["name", "type"]
            }
          }
        },
        "required": ["name", "attributes"]
      }
    },
    "events": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": { "type": "string" },
          "type": { "type": "string" },
          "timestamp": { "type": "string", "format": "date-time" },
          "objects": {
            "type": "array",
            "items": { "type": "string" }
          },
          "attributes": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "name": { "type": "string" },
                "value": { "type": ["string", "number", "boolean"] }
              },
              "required": ["name", "value"]
            }
          }
        },
        "required": ["id", "type", "timestamp", "objects", "attributes"]
      }
    },
    "objects": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": { "type": "string" },
          "type": { "type": "string" },
          "attributes": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "name": { "type": "string" },
                "value": { "type": ["string", "number", "boolean"] }
              },
              "required": ["name", "value"]
            }
          }
        },
        "required": ["id", "type", "attributes"]
      }
    }
  },
  "required": ["object_types", "event_types", "events", "objects"]
}
```

## 2. Concrete OCEL 2.0 Lifecycle Log

This log captures the lifecycle of a purchase order (`po_4001`) and its associated component parts (`part_a`, `part_b`).

```json
{
  "object_types": [
    {
      "name": "purchase_order",
      "attributes": [
        { "name": "total_cost", "type": "float" },
        { "name": "supplier", "type": "string" }
      ]
    },
    {
      "name": "part",
      "attributes": [
        { "name": "sku", "type": "string" },
        { "name": "weight_kg", "type": "float" }
      ]
    }
  ],
  "event_types": [
    {
      "name": "create_order",
      "attributes": [
        { "name": "payment_terms", "type": "string" }
      ]
    },
    {
      "name": "allocate_part",
      "attributes": []
    },
    {
      "name": "ship_order",
      "attributes": [
        { "name": "carrier", "type": "string" }
      ]
    }
  ],
  "events": [
    {
      "id": "e_create_01",
      "type": "create_order",
      "timestamp": "2026-05-31T22:44:00Z",
      "objects": ["po_4001"],
      "attributes": [
        { "name": "payment_terms", "value": "Net30" }
      ]
    },
    {
      "id": "e_allocate_01",
      "type": "allocate_part",
      "timestamp": "2026-05-31T22:45:00Z",
      "objects": ["po_4001", "part_a"],
      "attributes": []
    },
    {
      "id": "e_allocate_02",
      "type": "allocate_part",
      "timestamp": "2026-05-31T22:45:15Z",
      "objects": ["po_4001", "part_b"],
      "attributes": []
    },
    {
      "id": "e_ship_01",
      "type": "ship_order",
      "timestamp": "2026-05-31T22:46:00Z",
      "objects": ["po_4001", "part_a", "part_b"],
      "attributes": [
        { "name": "carrier", "value": "GlobalExpress" }
      ]
    }
  ],
  "objects": [
    {
      "id": "po_4001",
      "type": "purchase_order",
      "attributes": [
        { "name": "total_cost", "value": 85000.00 },
        { "name": "supplier", "value": "Acme Industrial" }
      ]
    },
    {
      "id": "part_a",
      "type": "part",
      "attributes": [
        { "name": "sku", "value": "SKU-990-A" },
        { "name": "weight_kg", "value": 154.2 }
      ]
    },
    {
      "id": "part_b",
      "type": "part",
      "attributes": [
        { "name": "sku", "value": "SKU-990-B" },
        { "name": "weight_kg", "value": 12.8 }
      ]
    }
  ]
}
```

## 3. Linkages to Standards and M&A Claims

- **Standard Crosswalk**: Fits into OCEL validation constraints mapped at file:///Users/sac/process-intelligence/standards/ocel_process-intelligence_placement.md.
- **M&A Claims**: Defensibility claims of multi-object lifecycle flows are mapped at file:///Users/sac/process-intelligence/ma/define_slide-to-public-standard_map.md.