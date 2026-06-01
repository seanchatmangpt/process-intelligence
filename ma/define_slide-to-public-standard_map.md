# Slide-to-Public-Standard Map

To protect buyers from proprietary vendor lock-in and ensure the long-term maintainability of process models, all diligence assertions must be backed by data that conforms to open, public process mining standards. This document defines the Slide-to-Public-Standard Map, ensuring alignment with IEEE XES, OCEL 2.0, BPMN, and POWL.

## 1. Public Standards Coverage

Every slide claim must declare compliance with the relevant public process standard:

```
┌──────────────────┐      requires conformance to      ┌────────────────────────┐
│ M&A Slide Claim  │ ────────────────────────────────> │ IEEE XES / OCEL 2.0    │
│ (Asset/Synergy)  │                                   │ BPMN 2.0 / POWL        │
└──────────────────┘                                   └────────────────────────┘
```

| Standard | Target Domain | Conformance Verification | Reference Link |
| :--- | :--- | :--- | :--- |
| **IEEE 1849-2016 (XES)** | Single-perspective sequential event logs. | XSD schema validation, lifecycle transition checks (schedule, start, complete). | [XES Placement](file:///Users/sac/process-intelligence/standards/xes_process-intelligence_placement.md) |
| **OCEL 2.0** | Object-Centric Event Logs (multi-object interactions). | JSON/XML schema compliance, object-to-event relation validation. | [OCEL Placement](file:///Users/sac/process-intelligence/standards/ocel_process-intelligence_placement.md) |
| **BPMN 2.0** | Visual process modeling and collaboration. | Semantic validation, token-game execution mapping. | [BPMN Placement](file:///Users/sac/process-intelligence/standards/bpmn_process-intelligence_placement.md) |
| **POWL / Process Trees** | Block-structured process models. | Direct tree parsing, guarantee of structural soundness. | [POWL Placement](file:///Users/sac/process-intelligence/standards/powl_placement.md) |

## 2. Standards Mapping Specification

To map a slide claim to a public standard, the seller must provide a standard validation mapping block in the VDR:

### A. XES Conformance Validation
If a slide claims performance metrics (e.g., cycle times), the underlying XES log must pass the schema validator:
```bash
# Example verification of XES schema compliance
xes-validator --log /path/to/log.xes --schema ieee-1849-2016.xsd
```
The log must preserve the standard event lifecycle extensions (e.g., `concept:name`, `time:timestamp`, `lifecycle:transition`).

### B. OCEL 2.0 Object-Centric Validation
For multi-object claims (e.g., P2P processes with multiple items, orders, and invoices), the log must conform to the OCEL 2.0 standard:
* Every event must link to at least one object.
* Object-to-object and event-to-object relations must be explicitly defined without orphan elements.

## 3. Related M&A Validation Documents

* For the general slide-to-receipt architecture, see [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md).
* For defining acquisition readiness criteria, see [Acquisition-Ready Process Intelligence](file:///Users/sac/process-intelligence/ma/define_acquisition-ready_process_intelligence.md).
* For standard placement in the repository, see [BPMN Placement](file:///Users/sac/process-intelligence/standards/bpmn_process-intelligence_placement.md).
* For standard lifecycle mappings, see [XES Placement](file:///Users/sac/process-intelligence/standards/xes_process-intelligence_placement.md).