# Downstream Directive: Public Standards Expansion

This document outlines the mandatory design and implementation requirements for validating, parsing, and expanding support for public process mining standards, including XES, OCEL 2.0, BPMN, and POWL.

## 1. Schema Validation and Strict Parsing
- **XML Schema Enforcement**: Ensure all XES inputs are validated against the official IEEE 1849-2016 XES schema before ingestion. Any unrecognized tag structures must be rejected unless they belong to standard extensions.
- **OCEL 2.0 Relational Integrity**: Implement structural validators for OCEL 2.0 JSON and SQLite formats. Ensure that every event references a valid event type, that every object references a valid object type, and that all type schemas are adhered to.
- **BPMN 2.0 Execution Mapping**: Enforce correct parsing of BPMN 2.0 process definitions. Check for valid gateways (parallel, exclusive, inclusive) and sound connections. Reject models containing dead gateways or orphaned flows.

## 2. Cross-Standard Conversion Loss Controls
When importing or exporting data across standards, implement conversion filters that generate a `LossReport` based on a predefined `LossPolicy` logging all structural changes:
- **XES to OCEL**: Flattened trace concepts must be reconstructed into separate object tables using object heuristics. If reconstruction is partial, output a detailed warning report.
- **OCEL to XES**: Track semantic loss when object-to-object relations and multi-object events are flattened into single-perspective XES traces.
  - The `LossReport` must quantify the cardinality of lost relations and attribute pruning.
- **POWL to Petri Net**: Ensure that the block structure is strictly translated to sound places and transitions. Any non-mappable block hierarchy must trigger conversion refusal.
- **Process Tree to DFG Conversion**: Concurrency information and loop structures are flattened. The DFG must report the loss of block-structured hierarchy in the `LossReport`.

## 3. Downstream Integration and Traceability
All implementation details must align with:
- [public-standards-gravity.md](file:///Users/sac/process-intelligence/doctrine/public-standards-gravity.md)
- [public-standard_projection_sample.md](file:///Users/sac/process-intelligence/experiments/public-standard_projection_sample.md)