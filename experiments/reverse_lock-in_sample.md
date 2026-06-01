# Experiment: Reverse Lock-in Migration Validation

This experiment validates migration patterns that extract process assets from proprietary vendor environments and map them to standard specifications (BPMN 2.0 and XES) to prevent vendor lock-in.

## 1. Reverse Lock-in Migration JSON Validation Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "ReverseLockinMigrationLog",
  "type": "object",
  "properties": {
    "migration_id": { "type": "string" },
    "source_proprietary_system": { "type": "string" },
    "target_standards": {
      "type": "array",
      "items": { "enum": ["XES", "BPMN2.0", "OCEL2.0", "POWL"] }
    },
    "migration_map": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "proprietary_construct": { "type": "string" },
          "public_standard_concept": { "type": "string" },
          "translation_status": { "enum": ["SUCCESS", "PARTIAL", "FAILED"] },
          "notes": { "type": "string" }
        },
        "required": ["proprietary_construct", "public_standard_concept", "translation_status"]
      }
    }
  },
  "required": ["migration_id", "source_proprietary_system", "target_standards", "migration_map"]
}
```

## 2. Concrete Reverse Lock-in Migration

The following instance maps custom workflows from a proprietary system to public specifications:

```json
{
  "migration_id": "mig_rev_lockin_fulfillment_2026",
  "source_proprietary_system": "Celonis_IBC_v9",
  "target_standards": ["BPMN2.0", "XES"],
  "migration_map": [
    {
      "proprietary_construct": "Celonis_PQL_ProcessFlow",
      "public_standard_concept": "BPMN_Process_Diagram",
      "translation_status": "SUCCESS",
      "notes": "Direct structural translation of activities to BPMN Task nodes."
    },
    {
      "proprietary_construct": "Celonis_Case_Table",
      "public_standard_concept": "XES_Trace_Identity",
      "translation_status": "SUCCESS",
      "notes": "Mapped proprietary case key directly to standard XES trace id."
    },
    {
      "proprietary_construct": "Celonis_Activity_Table",
      "public_standard_concept": "XES_Event_Log",
      "translation_status": "SUCCESS",
      "notes": "Translated proprietary timestamp column to XES time:timestamp ISO 8601 attribute."
    }
  ]
}
```

## 3. Linkages to Standards and M&A Claims

- **Standard Crosswalk**: Aligns with standards mapped at [public_ontology_reverse-lock-in_map.md](file:///Users/sac/process-intelligence/standards/public_ontology_reverse-lock-in_map.md).
- **M&A Claims**: Defensibility claims are verified by mapping safety validations onto the reverse lock-in maps at [reverse_lock-in_sample.md](file:///Users/sac/process-intelligence/experiments/reverse_lock-in_sample.md).