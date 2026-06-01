# Experiment: OCEL Lifecycle Validation

This experiment validates object-centric event logs against the OCEL 2.0 standard. Unlike traditional XES logs that assume a single case identifier, OCEL logs track events that interact with multiple objects of different types, capturing complex process topologies.

## 1. OCEL 2.0 Lifecycle JSON Validation Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Ocel2LifecycleLog",
  "type": "object",
  "properties": {
    "eventTypes": {
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
    "objectTypes": {
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
          "time": { "type": "string", "format": "date-time" },
          "attributes": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "name": { "type": "string" },
                "value": { "type": "string" }
              },
              "required": ["name", "value"]
            }
          },
          "relationships": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "objectId": { "type": "string" },
                "qualifier": { "type": "string" }
              },
              "required": ["objectId", "qualifier"]
            }
          }
        },
        "required": ["id", "type", "time"]
      }
    },
    "objects": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": { "type": "string" },
          "type": { "type": "string" },
          "relationships": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "objectId": { "type": "string" },
                "qualifier": { "type": "string" }
              },
              "required": ["objectId", "qualifier"]
            }
          },
          "attributes": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "name": { "type": "string" },
                "value": { "type": "string" },
                "time": { "type": "string", "format": "date-time" }
              },
              "required": ["name", "value", "time"]
            }
          }
        },
        "required": ["id", "type"]
      }
    }
  },
  "required": ["eventTypes", "objectTypes", "events", "objects"]
}
```

## 2. Concrete OCEL 2.0 Lifecycle Log

This log captures the lifecycle of a purchase order (`po_4001`), its associated component parts (`part_a`, `part_b`), and the corresponding shipment (`shipment_9901_west`).

```json
{
  "eventTypes": [
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
  "objectTypes": [
    {
      "name": "purchase_order",
      "attributes": [
        { "name": "total_cost", "type": "string" },
        { "name": "supplier", "type": "string" }
      ]
    },
    {
      "name": "part",
      "attributes": [
        { "name": "sku", "type": "string" },
        { "name": "weight_kg", "type": "string" }
      ]
    },
    {
      "name": "shipment",
      "attributes": [
        { "name": "destination", "type": "string" },
        { "name": "carrier", "type": "string" }
      ]
    }
  ],
  "events": [
    {
      "id": "e_create_01",
      "type": "create_order",
      "time": "2026-05-31T22:44:00Z",
      "relationships": [
        { "objectId": "po_4001", "qualifier": "order" }
      ],
      "attributes": [
        { "name": "payment_terms", "value": "Net30" }
      ]
    },
    {
      "id": "e_allocate_01",
      "type": "allocate_part",
      "time": "2026-05-31T22:45:00Z",
      "relationships": [
        { "objectId": "po_4001", "qualifier": "order" },
        { "objectId": "part_a", "qualifier": "item" }
      ],
      "attributes": []
    },
    {
      "id": "e_allocate_02",
      "type": "allocate_part",
      "time": "2026-05-31T22:45:15Z",
      "relationships": [
        { "objectId": "po_4001", "qualifier": "order" },
        { "objectId": "part_b", "qualifier": "item" }
      ],
      "attributes": []
    },
    {
      "id": "e_ship_01",
      "type": "ship_order",
      "time": "2026-05-31T22:46:00Z",
      "relationships": [
        { "objectId": "po_4001", "qualifier": "order" },
        { "objectId": "part_a", "qualifier": "item" },
        { "objectId": "part_b", "qualifier": "item" },
        { "objectId": "shipment_9901_west", "qualifier": "shipment" }
      ],
      "attributes": [
        { "name": "carrier", "value": "GlobalExpress" }
      ]
    }
  ],
  "objects": [
    {
      "id": "po_4001",
      "type": "purchase_order",
      "relationships": [],
      "attributes": [
        { "name": "total_cost", "value": "85000.00", "time": "2026-05-31T22:44:00Z" },
        { "name": "supplier", "value": "Acme Industrial", "time": "2026-05-31T22:44:00Z" }
      ]
    },
    {
      "id": "part_a",
      "type": "part",
      "relationships": [
        { "objectId": "po_4001", "qualifier": "part_of_order" }
      ],
      "attributes": [
        { "name": "sku", "value": "SKU-990-A", "time": "2026-05-31T22:44:00Z" },
        { "name": "weight_kg", "value": "154.2", "time": "2026-05-31T22:44:00Z" }
      ]
    },
    {
      "id": "part_b",
      "type": "part",
      "relationships": [
        { "objectId": "po_4001", "qualifier": "part_of_order" }
      ],
      "attributes": [
        { "name": "sku", "value": "SKU-990-B", "time": "2026-05-31T22:44:00Z" },
        { "name": "weight_kg", "value": "12.8", "time": "2026-05-31T22:44:00Z" }
      ]
    },
    {
      "id": "shipment_9901_west",
      "type": "shipment",
      "relationships": [
        { "objectId": "po_4001", "qualifier": "shipment_for_order" }
      ],
      "attributes": [
        { "name": "destination", "value": "West Warehouse", "time": "2026-05-31T22:46:00Z" },
        { "name": "carrier", "value": "GlobalExpress", "time": "2026-05-31T22:46:00Z" }
      ]
    }
  ]
}
```

## 3. Linkages to Standards and M&A Claims

- **Standard Crosswalk**: Fits into OCEL validation constraints mapped at [OCEL Standard Ledger Placement](file:///Users/sac/process-intelligence/standards/ocel_process-intelligence_placement.md).
- **M&A Claims**: Defensibility claims of multi-object lifecycle flows are mapped at [Slide-to-Public-Standard Map](file:///Users/sac/process-intelligence/ma/define_slide-to-public-standard_map.md).