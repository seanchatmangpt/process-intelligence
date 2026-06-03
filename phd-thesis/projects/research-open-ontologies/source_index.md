# Source Index — research-open-ontologies Thesis Files

All source files read during authoring of the 8 TeX thesis files for this project.

## Primary Evidence Files

| File | Description |
|------|-------------|
| `/Users/sac/process-intelligence/research/open-ontologies/federated/CROSS_PROJECT_DEPENDENCY_GRAPH_20260601.yaml` | Phase 4 federated dependency analysis: 24 TTL files, 4622 triples, 871 entities, ACYCLIC/CONSISTENT, gate_pass: true |
| `/Users/sac/process-intelligence/research/pi-program/emitted/ggen-validator-recovery/open-ontologies-roundtrip-report.md` | Phase 7 TTL roundtrip check: 14/14 files loaded, 3684 triples, 4/4 SPARQL smoke queries passing, classification AVAILABLE |
| `/Users/sac/process-intelligence/checkpoints/PI_RESEARCH_PROGRAM_ALIVE_001.md` | Parent program ALIVE checkpoint: 12/12 audit gates PASS, BLAKE3 seal, authorizes downstream manufacturing |
| `/Users/sac/process-intelligence/research/pi-program/checkpoints/PI_GGEN_UNIFIED_RUN_CONFORMANCE_AUDIT_001.md` | GGEN conformance audit PARTIAL checkpoint: 9/15 gates PASS, 101 violations, Gates 3/4/9 blocking ALIVE reissue |
| `/Users/sac/process-intelligence/receipts/RECEIPT_REGISTRY.md` | 7 canonical research program receipts: PAPER_CANON, PM4PY_ORACLE, WASM4PM_GAP, LIFECYCLE, MA, STANDARDS, ADVERSARIAL |
| `/Users/sac/process-intelligence/COVENANT.md` | Program governance covenant: admissibility law, ALIVE/PARTIAL/FAILED verdict rules, downstream authorization law |

## Ontology Source Files (TTL)

| File | Description |
|------|-------------|
| `/Users/sac/process-intelligence/research/pi-program/ggen/ontology/pi-program.ttl` | Top-level PI program structural ontology: pi:ProgramRole taxonomy (PROGRAM, PROOF_CELL, ENGINE, etc.), imports PROV-O/SKOS/DCTERMS/DCAT/SHACL/OWL |
| `/Users/sac/process-intelligence/research/pi-program/ggen/ontology/autonomic-law.ttl` | MAPE-K loop ontology: mape:MAPEKLoop subclasses (Monitor/Analyze/Plan/Execute/Knowledge), aka:ElasticTransition and aka:ComplianceTransition |
| `/Users/sac/process-intelligence/ggen/wasm4pm-compat.ttl` | Witness lattice and compat type law: 8 compat:WitnessMarker instances, compat:ProcessForm, compat:TypeLawAtom, compat:BoundaryCrossing |

## Manufacturing Pipeline

| File | Description |
|------|-------------|
| `/Users/sac/process-intelligence/ggen/ggen.toml` | ggen pipeline manifest: 3 SPARQL-to-Tera generation rules (witness-markers.rs, process-forms.md, boundary-law.wit), BLAKE3 checksums |

## Output Directory

`/Users/sac/process-intelligence/phd-thesis/projects/research-open-ontologies/`
