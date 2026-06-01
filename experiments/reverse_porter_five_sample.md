# Experiment: Reverse Porter's Five Forces Process Validation

This experiment validates the calculation of Porter's Five Forces metrics from event logs, converting tactical process execution data into strategic competitive indicators.

## 1. Porter's Five Forces JSON Validation Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "PorterFiveForcesProcessLog",
  "type": "object",
  "properties": {
    "evaluation_id": { "type": "string" },
    "target_enterprise": { "type": "string" },
    "metrics": {
      "type": "object",
      "properties": {
        "supplier_power": {
          "type": "object",
          "properties": {
            "supplier_concentration_ratio": { "type": "number", "minimum": 0.0, "maximum": 1.0 },
            "switching_cost_conformance_penalty": { "type": "number" }
          },
          "required": ["supplier_concentration_ratio", "switching_cost_conformance_penalty"]
        },
        "buyer_power": {
          "type": "object",
          "properties": {
            "order_cancellation_rate": { "type": "number", "minimum": 0.0, "maximum": 1.0 },
            "average_lead_time_days": { "type": "number" }
          },
          "required": ["order_cancellation_rate", "average_lead_time_days"]
        },
        "substitution_threat": {
          "type": "object",
          "properties": {
            "alternative_route_utilization": { "type": "number", "minimum": 0.0, "maximum": 1.0 }
          },
          "required": ["alternative_route_utilization"]
        }
      },
      "required": ["supplier_power", "buyer_power", "substitution_threat"]
    }
  },
  "required": ["evaluation_id", "target_enterprise", "metrics"]
}
```

## 2. Concrete Porter's Five Forces Metrics

The following instance maps real supply chain log metrics to competitive forces for an target firm:

```json
{
  "evaluation_id": "porter5_diligence_fulfillment_2026",
  "target_enterprise": "Acme Manufacturing Group",
  "metrics": {
    "supplier_power": {
      "supplier_concentration_ratio": 0.85,
      "switching_cost_conformance_penalty": 45000.00
    },
    "buyer_power": {
      "order_cancellation_rate": 0.042,
      "average_lead_time_days": 12.8
    },
    "substitution_threat": {
      "alternative_route_utilization": 0.18
    }
  }
}
```

## 3. Linkages to Standards and M&A Claims

- **Standard Crosswalk**: Aligns with standards mapped at file:///Users/sac/process-intelligence/standards/public_standards_to_m&a_claims.md.
- **M&A Claims**: Defensibility claims are verified by mapping safety validations onto the porter maps at file:///Users/sac/process-intelligence/experiments/reverse_porter_five_sample.md.