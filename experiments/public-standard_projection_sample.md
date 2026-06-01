# Experiment: Public-Standard Projection Validation

This experiment validates the mapping and projection of internal event log elements onto standardized public ontologies, such as W3C PROV-O (for provenance) and SHACL (for shape-based validation).

## 1. Public-Standard Projection JSON Validation Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "PublicStandardProjectionLog",
  "type": "object",
  "properties": {
    "projection_id": { "type": "string" },
    "target_standard": { "enum": ["BPMN2.0", "PROV-O", "SHACL", "ODRL"] },
    "mappings": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "source_field": { "type": "string" },
          "target_concept_uri": { "type": "string", "format": "uri" },
          "datatype": { "type": "string" },
          "validation_rule": { "type": "string" }
        },
        "required": ["source_field", "target_concept_uri", "datatype"]
      }
    }
  },
  "required": ["projection_id", "target_standard", "mappings"]
}
```

## 2. Concrete Public-Standard Mappings

The following instance defines the projection of log events to the W3C PROV-O provenance ontology:

```json
{
  "projection_id": "prov_o_projection_audit_v1",
  "target_standard": "PROV-O",
  "mappings": [
    {
      "source_field": "events/trace_id",
      "target_concept_uri": "http://www.w3.org/ns/prov#Activity",
      "datatype": "string",
      "validation_rule": "must_be_non_null"
    },
    {
      "source_field": "events/org_resource",
      "target_concept_uri": "http://www.w3.org/ns/prov#Agent",
      "datatype": "string",
      "validation_rule": "must_match_resource_directory"
    },
    {
      "source_field": "events/timestamp",
      "target_concept_uri": "http://www.w3.org/ns/prov#startedAtTime",
      "datatype": "dateTime",
      "validation_rule": "must_be_iso8601"
    }
  ]
}
```

## 3. Linkages to Standards and M&A Claims

- **Standard Crosswalk**: Aligns with standards mapped at [prov-o_provenance_placement.md](file:///Users/sac/process-intelligence/standards/prov-o_provenance_placement.md).
- **M&A Claims**: Defensibility claims are verified by mapping safety validations onto the slide-to-public-standard maps at [define_slide-to-public-standard_map.md](file:///Users/sac/process-intelligence/ma/define_slide-to-public-standard_map.md).