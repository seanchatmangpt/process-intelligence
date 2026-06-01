# Experiment: Blue River Dam Gate Control Process Validation

This experiment validates cyber-physical process telemetry logs for the Blue River Dam gate operations. The process enforces strict safety gates (e.g. water levels must be checked before opening the dam gates, gate open angles must match actuator signals).

## 1. Blue River Dam Gate Log JSON Validation Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "BlueRiverDamGateLog",
  "type": "object",
  "properties": {
    "dam_id": { "type": "string" },
    "events": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "event_id": { "type": "string" },
          "timestamp": { "type": "string", "format": "date-time" },
          "activity": { "enum": ["Check_Water_Level", "Signal_Gate_Open", "Confirm_Gate_Angle", "Alert_Overpressure"] },
          "metrics": {
            "type": "object",
            "properties": {
              "water_level_meters": { "type": "number" },
              "gate_angle_degrees": { "type": "number", "minimum": 0.0, "maximum": 90.0 },
              "flow_rate_cms": { "type": "number", "minimum": 0.0 }
            },
            "required": ["water_level_meters"]
          },
          "operator_id": { "type": "string" }
        },
        "required": ["event_id", "timestamp", "activity", "metrics", "operator_id"]
      }
    }
  },
  "required": ["dam_id", "events"]
}
```

## 2. Concrete Blue River Dam Telemetry Log

The following event sequence demonstrates a compliant check-and-release operation on the gate:

```json
{
  "dam_id": "blue_river_dam_node_01",
  "events": [
    {
      "event_id": "evt_dam_001",
      "timestamp": "2026-05-31T22:44:00Z",
      "activity": "Check_Water_Level",
      "metrics": {
        "water_level_meters": 44.18,
        "gate_angle_degrees": 0.0,
        "flow_rate_cms": 0.0
      },
      "operator_id": "sys_auto_gov_0"
    },
    {
      "event_id": "evt_dam_002",
      "timestamp": "2026-05-31T22:44:15Z",
      "activity": "Signal_Gate_Open",
      "metrics": {
        "water_level_meters": 44.18,
        "gate_angle_degrees": 15.0,
        "flow_rate_cms": 120.5
      },
      "operator_id": "operator_chief_weske"
    },
    {
      "event_id": "evt_dam_003",
      "timestamp": "2026-05-31T22:44:30Z",
      "activity": "Confirm_Gate_Angle",
      "metrics": {
        "water_level_meters": 44.12,
        "gate_angle_degrees": 15.0,
        "flow_rate_cms": 120.5
      },
      "operator_id": "sys_auto_gov_0"
    }
  ]
}
```

## 3. Linkages to Standards and M&A Claims

- **Standard Crosswalk**: Aligns with standards mapped at [Public Standards to Blue River Dam](file:///Users/sac/process-intelligence/standards/public_standards_to_blue_river_dam.md).
- **M&A Claims**: Defensibility claims are verified by mapping safety validations onto the lifecycle gate maps at [Blue River Dam Lifecycle Gate Map](file:///Users/sac/process-intelligence/lifecycle/define_blue_river_dam_lifecycle_gate_map.md).