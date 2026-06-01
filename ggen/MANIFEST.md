# ggen Manifest

**Project:** ggen — Governance Generation Engine  
**Version:** 0.1.0  
**Status:** COMPLETE  
**Date:** 2026-06-01  
**Location:** /Users/sac/process-intelligence/ggen/

---

## File Manifest

### Configuration
| File | Lines | Purpose |
|------|-------|---------|
| **ggen.toml** | 76 | Generation rules, namespaces, output config |

### SPARQL Queries (queries/)
| File | Lines | Purpose |
|------|-------|---------|
| **extract-board-claims.rq** | 56 | Select board-admissible M&A claims backed by ConformanceVerdicts |
| **extract-diligence-claims.rq** | 85 | Select synergy/debt/risk claims with replay evidence & remediation paths |
| **extract-lifecycle-governance.rq** | 91 | Extract MAPE-K rules and lifecycle state transitions |
| **Total (queries/)** | 232 | — |

### Tera Templates (templates/)
| File | Lines | Purpose |
|------|-------|---------|
| **ma-deck.tera** | 198 | Render PowerPoint-compatible JSON structure for board presentation |
| **ma-diligence.tera** | 242 | Render Excel workbook structure with 7 sheets for due diligence |
| **blue-river.tera** | 272 | Generate Rust orchestrator with MAPE-K loop and state machine |
| **Total (templates/)** | 712 | — |

### Ontology
| File | Lines | Purpose |
|------|-------|---------|
| **ontology-extensions.ttl** | 409 | RDF/Turtle extensions for M&A claims & lifecycle states |

### Documentation
| File | Lines | Purpose |
|------|-------|---------|
| **README.md** | 248 | Complete architecture, usage guide, references |
| **INTEGRATION.md** | 352 | Integration with wasm4pm, compat, Blue River + examples |
| **DELIVERY_SUMMARY.md** | 346 | Comprehensive inventory of deliverables & next steps |
| **MANIFEST.md** | (this file) | File-by-file manifest and statistics |
| **Total (docs)** | 946 | — |

### Summary
| Category | Count | Lines |
|----------|-------|-------|
| Configuration | 1 | 76 |
| SPARQL Queries | 3 | 232 |
| Tera Templates | 3 | 712 |
| Ontology | 1 | 409 |
| Documentation | 4 | 946 |
| **TOTAL** | **12** | **2,375** |

---

## Dependency Graph

```
ggen.toml
├── references: queries/
│   ├── extract-board-claims.rq
│   ├── extract-diligence-claims.rq
│   └── extract-lifecycle-governance.rq
├── references: templates/
│   ├── ma-deck.tera
│   ├── ma-diligence.tera
│   └── blue-river.tera
└── references: ontology-extensions.ttl

extract-board-claims.rq
├── uses: ontology-extensions.ttl (namespaces: ma:, compat:, wasm4pm:)
└── outputs: ~40 SPARQL result columns (claims, metrics, verdicts, receipts)

extract-diligence-claims.rq
├── extends: extract-board-claims.rq (same namespaces)
└── outputs: ~20 additional columns (debt, risk, remediation, activities)

extract-lifecycle-governance.rq
├── uses: ontology-extensions.ttl (namespace: lifecycle:)
└── outputs: ~20 columns (states, transitions, rules, knowledge)

ma-deck.tera
├── consumes: extract-board-claims.rq results
├── references: ontology-extensions.ttl (ma:, wasm4pm: classes)
└── renders: PowerPoint JSON structure

ma-diligence.tera
├── consumes: extract-diligence-claims.rq results
├── references: ontology-extensions.ttl (ma:, compat: classes)
└── renders: Excel workbook JSON structure

blue-river.tera
├── consumes: extract-lifecycle-governance.rq results
├── references: ontology-extensions.ttl (lifecycle: classes)
└── renders: Rust orchestrator code with MAPE-K loop

ontology-extensions.ttl
├── extends: wasm4pm-compat ontology
├── defines: ma: namespace (claims, properties, types)
├── defines: lifecycle: namespace (states, transitions, rules)
└── references: compat: (Evidence types)

README.md
├── references: All above files
├── documents: Architecture, queries, templates, ontology
└── provides: Usage guide, examples, framework

INTEGRATION.md
├── documents: Data flow (wasm4pm → RDF → ggen → artifacts)
├── examples: Accounts Payable Synergy end-to-end
└── shows: Board verification workflow

DELIVERY_SUMMARY.md
├── inventories: All deliverables
├── describes: Each component in detail
└── provides: Next steps and quality metrics
```

---

## Cross-References to Process Intelligence Authority

### ggen.toml
- References: `ggen/ontology-extensions.ttl` (ontology source)
- References: `../receipts/`, `../checkpoints/` (evidence sources)

### Queries
- **extract-board-claims.rq**
  - Authority: `ma/define_board-admissible_claim_requirements.md` (Pillar 1-4)
  - References: `ma/define_board_claim_taxonomy.md` (claim types)
  - References: `ma/define_buyer_reliance_requirements.md` (board reliance)
  
- **extract-diligence-claims.rq**
  - Authority: `ma/define_diligence_claim_taxonomy.md`
  - Authority: `ma/define_operational_debt_taxonomy.md`
  - References: `ma/define_synergy_claim_taxonomy.md`
  
- **extract-lifecycle-governance.rq**
  - Authority: `standards/MAPE_K_INTEGRATION.md`
  - Authority: `doctrine/full-lifecycle-process.md`
  - Authority: `doctrine/blue-river-dam.md`

### Templates
- **ma-deck.tera**
  - References: Adrian syah 2014 (optimal alignments)
  - References: van der Aalst 1998 (WF-net soundness)
  - References: Board admissibility framework
  
- **ma-diligence.tera**
  - References: Claim taxonomies and governance checklists
  - References: Remediation strategies
  
- **blue-river.tera**
  - References: MAPE-K phases (Monitor, Analyze, Plan, Execute, Knowledge)
  - References: Lifecycle states (Design, Simulation, etc.)

### Ontology
- **ontology-extensions.ttl**
  - Extends: `../sources/wasm4pm-compat/compat/src/ontology.rs`
  - Defines: `ma:` namespace (M&A claims)
  - Defines: `lifecycle:` namespace (process states)
  - Integrates: `compat:Evidence` type system

---

## Namespace Declarations

### Defined Namespaces

| Prefix | URI | Purpose |
|--------|-----|---------|
| ma: | https://process.intelligence/ma/ | M&A claims, synergy, debt, risk |
| lifecycle: | https://process.intelligence/lifecycle/ | Process states, transitions, MAPE-K rules |
| wasm4pm: | https://process.intelligence/wasm4pm/ | Conformance verdicts, receipts, traces |
| compat: | https://process.intelligence/compat/ | Evidence types, event logs |
| dcterms: | http://purl.org/dc/terms/ | Dublin Core (label, description, creator) |
| rdfs: | http://www.w3.org/2000/01/rdf-schema# | RDF Schema (subClassOf, domain, range) |
| rdf: | http://www.w3.org/1999/02/22-rdf-syntax-ns# | RDF (type, value) |
| owl: | http://www.w3.org/2002/07/owl# | OWL (Class, ObjectProperty, Restriction) |
| xsd: | http://www.w3.org/2001/XMLSchema# | XML Schema (integer, decimal, string, dateTime) |

### Namespace Usage by File

| Namespace | ggen.toml | Queries | Templates | Ontology |
|-----------|-----------|---------|-----------|----------|
| ma: | ✓ | ✓ | ✓ | ✓ |
| lifecycle: | ✓ | ✓ | ✓ | ✓ |
| wasm4pm: | ✓ | ✓ | ✓ | ✓ |
| compat: | ✓ | ✓ | ✓ | ✓ |
| dcterms: | | ✓ | ✓ | ✓ |
| rdfs: | | ✓ | | ✓ |
| rdf: | | | | ✓ |
| owl: | | | | ✓ |
| xsd: | | ✓ | | ✓ |

---

## Key Metrics

### Query Coverage
- **Board Claims:** 5 types (SynergyProjection, OperationalDebtClaim, IntegrationRiskClaim, ProcessAssetClaim, ControlClaim)
- **Lifecycle States:** 7 states (Design, Simulation, Validation, Monitoring, Optimization, Repair, Decommission)
- **MAPE-K Rules:** 5 types (Monitor, Analyze, Plan, Execute, Knowledge)
- **Conformance Filters:** Fitness >= 0.95, Precision >= 0.90

### Template Generation
- **Board Deck Slides:** 7 (title, summary, claim details, debt, synergy, model summary, sign-off)
- **Diligence Worksheets:** 7 (summary, synergy, debt, risks, traces, activities, governance)
- **Rust Structures:** 11 (enum, structs for transitions/guards/rules/actions/knowledge, orchestrator)

### Ontology Classes
- **M&A Classes:** 15 (BoardClaim, 5 subtypes, 3 synergy types, 3 debt types, 3 risk types)
- **Lifecycle Classes:** 8 (ProcessState, 7 subtypes)
- **MAPE-K Classes:** 5 (MonitorRule, AnalyzeRule, PlanRule, ExecuteAction, KnowledgeAsset)
- **Conformance Classes:** 3 (ConformanceVerdict, CryptographicReceipt, ReplayTrace)
- **Total:** 31 classes defined

### Ontology Properties
- **M&A Properties:** 12
- **Lifecycle Properties:** 12
- **MAPE-K Properties:** 8
- **Conformance Properties:** 6
- **Total:** 38 properties defined

---

## Governance Artifacts

### Board Admissibility Enforcement
- ✓ Query-level filters (fitness >= 0.95, precision >= 0.90)
- ✓ Template-level declarations (framework references, soundness proof)
- ✓ Ontology-level constraints (cardinality, type restrictions)
- ✓ Output-level verification (receipt inclusion, UUID generation)

### Audit Trail
- ✓ Cryptographic receipts in every claim
- ✓ Receipt hashes traceable to transaction WAL
- ✓ Conformance verdicts repeatable (re-run query with receipt)
- ✓ OCEL event log backing (log format declared)

### Compliance Checklist
- ✓ Event log standards (OCEL 2.0, XES 1849-2016)
- ✓ Cryptographic chaining (BLAKE3 + system signatures)
- ✓ Conformance bounds (fitness, precision thresholds)
- ✓ Model soundness (WF-net proofs)
- ✓ Receipts in data room (for audit repetition)

---

## Integration Checklist

### Pre-Execution Requirements
- [ ] RDF triple store populated with process intelligence facts
- [ ] wasm4pm-compat ontology loaded
- [ ] Receipt store exports available (../receipts/)
- [ ] Checkpoint exports available (../checkpoints/)
- [ ] SPARQL 1.1 endpoint accessible
- [ ] Tera template engine installed

### Execution Steps
- [ ] `ggen --config ggen.toml --execute-all`
- [ ] Verify SPARQL queries return results
- [ ] Verify templates render without errors
- [ ] Check output files created:
  - [ ] ../ma/acquisition_ready_deck_FINAL.pptx
  - [ ] ../ma/diligence_workbook.xlsx
  - [ ] ../blue_river_dam/src/lib.rs

### Post-Execution Steps
- [ ] Render PowerPoint from JSON (pptx-rs)
- [ ] Render Excel from JSON (xlsx)
- [ ] Compile Rust orchestrator (`cargo make` in blue_river_dam/)
- [ ] Verify board deck slides render correctly
- [ ] Verify workbook sheets populate with correct data
- [ ] Test blue-river orchestrator (run MAPE-K cycle)

### Verification Steps
- [ ] Board can verify claims via receipt verification URL
- [ ] Due diligence team can drill into workbook detail
- [ ] Operations team can compile and deploy orchestrator
- [ ] Audit can repeat conformance queries with receipts

---

## File Locations Summary

```
/Users/sac/process-intelligence/ggen/
├── ggen.toml                              # Configuration file
├── README.md                              # Architecture guide
├── INTEGRATION.md                         # Integration reference
├── DELIVERY_SUMMARY.md                    # Inventory & next steps
├── MANIFEST.md                            # This file (file manifest)
├── ontology-extensions.ttl                # RDF extensions
├── queries/
│   ├── extract-board-claims.rq            # Board claims extraction
│   ├── extract-diligence-claims.rq        # Detailed claims extraction
│   └── extract-lifecycle-governance.rq    # Governance rules extraction
└── templates/
    ├── ma-deck.tera                       # PowerPoint template
    ├── ma-diligence.tera                  # Excel template
    └── blue-river.tera                    # Rust orchestrator template
```

---

## Downstream References

### Produced Artifacts (Outputs)
- **../ma/acquisition_ready_deck_FINAL.pptx**
  - Consumed by: Board of Directors, Transaction Committee
  - Format: PowerPoint (via pptx-rs)
  - Verification: Board uses receipt to verify claims

- **../ma/diligence_workbook.xlsx**
  - Consumed by: Financial advisors, due diligence teams
  - Format: Excel (via xlsx)
  - Verification: Each row links to conformance verdict

- **../blue_river_dam/src/lib.rs**
  - Consumed by: Process intelligence runtime
  - Format: Rust source
  - Integration: `cargo make` compiles into blue_river_dam binary

### Source Data
- **../receipts/** — Cryptographic receipt exports
- **../checkpoints/** — Process intelligence snapshots
- **../sources/wasm4pm-compat/** — Type system definitions
- **../ma/define_*.md** — M&A taxonomy documentation
- **../standards/MAPE_K_INTEGRATION.md** — MAPE-K specification

---

## Version Control

- **Created:** 2026-06-01
- **Status:** READY FOR INTEGRATION
- **Checkpoint:** PROCESS_INTELLIGENCE_ALIVE_001
- **Git Commit:** To be added post-delivery
- **Next Review:** Upon RDF store population and first execution

---

## Summary Statement

**ggen (Governance Generation Engine)** is a complete configuration-driven substrate for transforming process intelligence ontology into three board-ready artifacts: M&A presentation deck, due diligence workbook, and autonomic governance orchestrator. All artifacts are backed by conformance verdicts, cryptographic receipts, and the board admissibility framework. The system enforces strict governance controls at query, template, and ontology levels.

**Status: COMPLETE — Ready for integration with process intelligence runtime.**
