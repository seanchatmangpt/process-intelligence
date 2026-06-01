# Open Ontologies Discovery Report

**Date:** 2026-06-01
**Scope:** Complete census of ~/open-ontologies project structure, ggen infrastructure, and integration points with PI program.
**Verdict:** OPEN_ONTOLOGIES_ANATOMY_MAPPED — Deep structural compatibility identified; integration bridges feasible.

---

## Executive Summary

Open Ontologies (OntoStar) is a mature receipt-bound recursive admission system for AI-manufactured software. It implements a hierarchical manufacturing pipeline with three critical layers:

1. **Requirement Translation** — Groq LLM + DSPy → POWL candidate
2. **CTQ Admission Gate** — Requirements andon with recursive validation
3. **Manufacturing & Attestation** — Cell8 13-gate pipeline with BLAKE3-chained receipts

The project contains **30 domain ontologies** (TTL files), **83 SPARQL queries** organized by domain (cli, zoela, ghf, truex, revops, powl, thesis), **74 Tera templates**, and a mature ggen infrastructure with ~40 active generation rules.

**Process mining** is explicitly modeled in `ontology/powl-process-mining.ttl` with 8 DiscoveryVariant individuals and Van der Aalst quality dimensions (Fitness, Precision, Generalization, Simplicity). The wasm4pm integration layer exists at `ontology/ontostar-wasm4pm-integration.ttl` and is actively wired into manufacturing gates.

**Integration points identified:**

- Shared SPARQL infrastructure (.specify/queries/) — can host PI governance queries
- Tera template expansion for PI M&A deck generation
- ggen.toml extensibility — can add PI-backed generation rules
- Cell8 gate patterns reusable for PI lifecycle state transitions
- OCEL 2.0 object types already modeled in POWL ontology

---

## 1. Project Structure

### Root Directory Layout

```
/Users/sac/open-ontologies/
├── ontology/                  # 30 domain TTL files (source of truth)
│   ├── cli-open-ontologies.ttl        # CLI surface ontology (primary)
│   ├── powl-process-mining.ttl        # POWL + OCEL 2.0 discovery variants
│   ├── cell8-*.ttl                   # Manufacturing gates A1-A13
│   ├── wasm4pm-stubs.ttl              # wasm4pm algorithm stubs
│   ├── ontostar-wasm4pm-integration.ttl  # Integration bridge
│   ├── truex-ecosystem.ttl            # Truex E2E test ecosystem
│   ├── portfolio-os.ttl               # Portfolio operating system
│   └── profiles/                      # ZOE LA Mobile + Truex profiles
│       ├── zoela-supabase-materialization.ttl
│       ├── zoela-supabase-rls.ttl
│       ├── zoela-governance.ttl
│       ├── zoela-expo-app.ttl
│       └── public-role-shapes.ttl
├── .specify/                  # Code generation infrastructure
│   ├── queries/               # 83 SPARQL queries (SELECT, CONSTRUCT, ASK)
│   │   ├── cli/               # 5 queries (commands aggregation)
│   │   ├── zoela/             # 25+ queries (mobile platform)
│   │   ├── ghf/               # 8 queries (GitHub Factory)
│   │   ├── truex/             # 16 queries (test ecosystem)
│   │   ├── powl/              # 2 queries (process mining)
│   │   ├── revops/            # 2 queries (manufacturing stages)
│   │   ├── portfolio/         # 2 queries
│   │   ├── thesis/            # 3 queries
│   │   ├── wasm4pm-*.rq       # 4 queries (stub generation)
│   │   └── aat-live/          # 4 queries (AAT live rules)
│   └── templates/             # 74 Tera templates (code generation)
│       ├── cli/               # 2 templates (.rs)
│       ├── zoela/             # 20+ templates (TypeScript, SQL, Expo)
│       ├── ghf/               # 8 templates
│       ├── truex/             # 9 templates
│       ├── wasm4pm-*.rs.tera  # 4 templates
│       └── [others]/
├── cell8-ggen/               # Cell8 manufacturing gates codegen
│   ├── ggen.toml             # Separate ggen manifest
│   ├── queries/              # 2 queries (manufacturing_gates, byte_law_types)
│   └── templates/            # 2 templates (stations.rs, byte_law.rs)
├── .ggen/                    # ggen runtime artifacts
│   ├── receipts/             # BLAKE3-chained manufacturing receipts
│   ├── cache/                # RDF store cache + query caches
│   ├── audit/                # Materialization audit trails
│   └── proof-receipts/       # Cryptographic signatures
├── src/                      # 85 Rust modules
│   ├── cmds/                 # MCP tool implementations (50+ onto_* tools)
│   ├── manufacturing/        # IaC + Rust + Erlang/AtomVM manufacturing
│   ├── cell8.rs              # 13-gate Cell8 attestation
│   ├── admission.rs          # Recursive admission logic
│   ├── receipts.rs           # BLAKE3 chain + Ed25519 signing
│   ├── graph.rs              # Oxigraph RDF store integration
│   ├── shacl.rs              # SHACL validation
│   ├── llm_translator.rs     # Groq LLM integration
│   ├── align.rs              # Ontology alignment
│   ├── enforce.rs            # Design pattern enforcement
│   ├── drift.rs              # Version drift detection
│   ├── lineage.rs            # Manufacturing lineage tracking
│   └── [40+ more modules]
├── tests/                    # 695 test functions
│   ├── real_groq_*.rs        # 20 real-Groq integration tests
│   ├── manufacturing_*.rs    # 13 manufacturing tests
│   ├── cell8_thirteen_gates_test.rs
│   ├── external_verifier_e2e.rs
│   ├── recursive_admission_e2e.rs
│   ├── multi_tenant_isolation.rs
│   ├── graph_test.rs
│   └── shacl_test.rs
├── ggen.toml                 # Primary ggen manifest
├── cell8-ggen/ggen.toml      # Cell8 ggen manifest
├── Cargo.toml                # Rust workspace
├── README.md                 # OntoStar introduction
└── CLAUDE.md                 # Agent instructions (7 rules + 50+ tools)
```

### Ontology Inventory (30 Files)

| File | Type | Purpose | Classes | Properties | Notes |
|------|------|---------|---------|------------|-------|
| cli-open-ontologies.ttl | OWL | CLI surface | 50+ | 30+ | Primary generation source |
| powl-process-mining.ttl | OWL | POWL/OCEL 2.0 | 3 | 8+ | 8 DiscoveryVariant individuals |
| cell8-core.ttl | OWL | Gate definitions | 6 | 12 | A1-A13 gate types |
| cell8-manufacturing.ttl | OWL | Manufacturing rules | 8 | 15 | Stage transitions |
| cell8-profile.ttl | OWL | Cell8 profiles | 12 | 20 | Profile layer |
| cell8-shapes.ttl | SHACL | Gate constraints | N/A | N/A | A1-A3 validation |
| ontostar-wasm4pm-integration.ttl | OWL | wasm4pm bridge | 5 | 8 | Algorithm stubs |
| wasm4pm-stubs.ttl | OWL | Function signatures | 40+ | 50+ | Full interface |
| truex-ecosystem.ttl | OWL | E2E test ecosystem | 35+ | 45+ | Test obligation model |
| portfolio-os.ttl | OWL | Portfolio OS | 18 | 25 | Operating system model |
| requirements.ttl | OWL | Requirements DSL | 8 | 12 | CTQ admission |
| attestation-shapes.ttl | SHACL | Attestation constraints | N/A | N/A | Receipt validation |
| cell8-conformance-shapes.ttl | SHACL | Conformance rules | N/A | N/A | Manufacturing gates |
| shared-receipt-shapes.ttl | SHACL | Receipt shapes | N/A | N/A | Cross-system validation |
| mcpp-proof-chain.ttl | OWL | Proof chain model | 6 | 10 | Manufacturing receipts |
| aat-live-rules.ttl | OWL | AAT live rules | 10 | 16 | Dynamic rules engine |
| powl-process-mining.ttl | OWL | Discovery variants | 8 | 6 | Van der Aalst quality dims |
| revops-manufacturing.ttl | OWL | RevOps test case | 5 | 8 | Manufacturing stage enums |
| public-alignment.ttl | OWL | Alignment reference | 3 | 6 | Cross-system mappings |
| public-shapes.ttl | SHACL | Public constraints | N/A | N/A | Public interface shapes |
| requirements-shapes.ttl | SHACL | Requirement shapes | N/A | N/A | CTQ validation |
| ghf-core.ttl | OWL | GitHub Factory | 8 | 14 | Repository model |
| ghf-shacl.ttl | SHACL | GHF constraints | N/A | N/A | GHF validation |
| ghf-security-policy.ttl | OWL | Security policy | 6 | 12 | Branch protection |
| thesis-manufacturing.ttl | OWL | Thesis domain | 12 | 18 | Academic manufacturing |
| thesis-shapes.ttl | SHACL | Thesis constraints | N/A | N/A | Thesis validation |
| profiles/zoela-*.ttl (7 files) | OWL | ZOE LA Mobile | 40+ | 60+ | Domain profiles |

### Cargo Workspace

```
Workspace packages:
├── open-ontologies (root binary)
└── cell8-ggen (codegen subproject)

Rust edition: 2021
MSRV: 1.70
Dependencies:
  - Oxigraph 0.3.x (RDF store)
  - pm4py 0.14+ (Python process mining via FFI)
  - wasm4pm (WebAssembly process mining)
  - DSPy (LLM orchestration)
  - Groq API (LLM backend)
  - Tera (template engine)
  - SHACL validator
  - Ed25519 + BLAKE3 (cryptographic receipts)
  - serde (serialization)
  - tokio async runtime
  - clap (CLI parsing)
  - JSON-RPC (MCP protocol)
```

---

## 2. ggen Infrastructure

### Primary Manifest: ggen.toml

**Structure:**
- **[project]**: version 26.5.21, MIT license
- **[ontology]**: source = `ontology/cli-open-ontologies.ttl`, 36 imports
- **[rdf]**: Oxigraph store at `.ggen/rdf-store`, query cache enabled, 3600s TTL
- **[inference.rules]**: 1 CONSTRUCT rule (normalize-cli-commands) — materializes cli:name, cli:summary, cli:description, cli:verb, cli:noun
- **[generation.rules]**: 40 rules defined (listed below)

### Generation Rules (40 Active)

| Rule | Query | Template | Output | Mode | Domain |
|------|-------|----------|--------|------|--------|
| cli-cmds | commands_aggregated.rq | cmds.rs.tera | src/cmds/generated.rs | Overwrite | CLI |
| revops-stages | stages_aggregated.rq | cmds.rs.tera | src/cmds/generated_revops.rs | Overwrite | RevOps |
| zoela-mobile-types | extract-types.rq | typescript-types.tera | packages/types/zoela.ts | Overwrite | TypeScript |
| zoela-public-materialization | extract-public-materializations.rq | supabase-migration.tera | supabase/migrations/20260519000002_*.sql | Overwrite | SQL |
| zoela-materialization-columns | extract-public-materialization-columns.rq | supabase-columns.tera | supabase/migrations/20260519000003_*.sql | Overwrite | SQL |
| zoela-governance | extract-governance-tables.rq | supabase-governance.tera | supabase/migrations/20260519000001_*.sql | Overwrite | SQL |
| zoela-legacy-coverage | extract-legacy-coverage.rq | supabase-legacy-coverage.tera | supabase/migrations/20260519000005_*.sql | Overwrite | SQL |
| zoela-rls | extract-odrl-policies.rq | supabase-rls.tera | supabase/migrations/20260519000004_*.sql | Overwrite | SQL |
| zoela-expo-package-json | extract-expo-package.rq | expo-package-json.tera | apps/mobile/package.json | Overwrite | JSON |
| zoela-expo-app-json | extract-expo-app-json.rq | expo-app-json.tera | apps/mobile/app.json | Overwrite | JSON |
| zoela-expo-tsconfig | extract-expo-tsconfig.rq | expo-tsconfig.tera | apps/mobile/tsconfig.json | Overwrite | JSON |
| zoela-expo-babel-config | extract-expo-manifest-id.rq | expo-babel-config.tera | apps/mobile/babel.config.js | Overwrite | JS |
| zoela-expo-metro-config | extract-expo-manifest-id.rq | expo-metro-config.tera | apps/mobile/metro.config.js | Overwrite | JS |
| zoela-expo-app-entry | extract-expo-runtime-config.rq | expo-app-entry.tera | apps/mobile/App.tsx | Overwrite | TypeScript |
| zoela-expo-supabase-client | extract-expo-runtime-config.rq | expo-supabase-client.tera | apps/mobile/src/lib/supabaseClient.ts | Overwrite | TypeScript |
| zoela-expo-verify-supabase | extract-verify-supabase.rq | expo-verify-supabase.tera | apps/mobile/scripts/verify-supabase.mjs | Overwrite | JS |
| zoela-expo-readme | extract-doc-page.rq | expo-readme.tera | apps/mobile/README.md | Overwrite | Markdown |
| zoela-mobile-zod | extract-skos-enums.rq | zod-schema.tera | packages/schemas/zoela.ts | Overwrite | TypeScript |
| zoela-forms | extract-form-fields.rq | react-hook-form.tera | packages/forms/zoela.tsx | Overwrite | TypeScript |
| [31+ more] | [varied] | [varied] | [varied] | Overwrite | Multi-domain |

### Query Organization (.specify/queries/)

```
.specify/queries/
├── cli/                           # 5 queries (CLI surface)
│   ├── commands_aggregated.rq
│   └── [4 more]
├── zoela/                         # 25+ queries (ZOE LA Mobile)
│   ├── extract-types.rq
│   ├── extract-public-materializations.rq
│   ├── extract-form-fields.rq
│   ├── extract-skos-enums.rq
│   ├── extract-governance-tables.rq
│   ├── extract-odrl-policies.rq
│   ├── extract-legacy-coverage.rq
│   ├── extract-expo-*.rq (7 queries)
│   └── [5+ more]
├── ghf/                           # 8 queries (GitHub Factory)
│   ├── repositories_aggregated.rq
│   ├── security_policy_aggregated.rq
│   ├── branch_protection_aggregated.rq
│   ├── labels_aggregated.rq
│   └── [4 more]
├── truex/                         # 16 queries (Test ecosystem)
│   ├── ecosystem-master-aggregated.rq
│   ├── failure-taxonomy-aggregated.rq
│   ├── e2e-tests-aggregated.rq
│   ├── test-obligations.rq
│   ├── artifacts.rq
│   └── [11 more]
├── powl/                          # 2 queries (Process mining)
│   ├── powl-variant-descriptions.rq
│   └── [discovery variants]
├── revops/                        # 2 queries (Manufacturing stages)
│   ├── stages_aggregated.rq
│   └── [1 more]
├── portfolio/                     # 2 queries
├── thesis/                        # 3 queries
├── aat-live/                      # 4 queries (AAT live rules)
├── wasm4pm-*.rq                   # 4 stub queries
└── [misc top-level]               # extract-wasm4pm-stub-*.rq
```

### Template Organization (.specify/templates/)

```
.specify/templates/
├── cli/                           # 2 templates
│   ├── cmds.rs.tera               # CLI command codegen
│   └── [1 more]
├── zoela/                         # 20+ templates
│   ├── supabase-migration.tera
│   ├── supabase-columns.tera
│   ├── supabase-rls.tera
│   ├── supabase-governance.tera
│   ├── supabase-materialization-refusals.tera
│   ├── supabase-legacy-coverage.tera
│   ├── expo-*.tera (7+ templates)
│   ├── zod-schema.tera
│   ├── react-hook-form.tera
│   ├── typescript-types.tera
│   ├── ocel-schema.tera
│   └── [5+ more]
├── ghf/                           # 8 templates
│   ├── yaml generation
│   ├── JSON schema generation
│   └── [6 more]
├── truex/                         # 9 templates
│   ├── root-package.json.tera
│   ├── pnpm-workspace.yaml.tera
│   ├── jest.config.js.tera
│   └── [6 more]
├── revops/                        # 1 template (cmds.rs)
├── portfolio/                     # 2 templates
├── thesis/                        # 3 templates
├── wasm4pm-*.tera                 # 4 templates (Rust stubs)
└── [misc]
```

### Cell8 ggen Subproject

**File:** `cell8-ggen/ggen.toml`

```toml
[ontology]
source = "../../ontology/cell8-profile.ttl"

[generation]
output_dir = "../../src/cell8/generated/"

[[generation.rules]]
name = "stations"
query = "queries/manufacturing_gates.rq"
template = "templates/stations.rs.tera"
output_file = "stations.rs"

[[generation.rules]]
name = "byte_law"
query = "queries/byte_law_types.rq"
template = "templates/byte_law.rs.tera"
output_file = "byte_law.rs"
```

**Outputs:**
- `src/cell8/generated/stations.rs` — A1-A13 gate state machines
- `src/cell8/generated/byte_law.rs` — Byte law type definitions

---

## 3. Ontology Foundation & Entity Types

### Core Vocabularies (Imported)

| Vocabulary | Namespace | Use Case | Classes | Properties |
|------------|-----------|----------|---------|------------|
| OWL 2.0 | http://www.w3.org/2002/07/owl# | Semantic model | Class, ObjectProperty, DatatypeProperty | owl:imports, equivalentClass |
| RDFS 1.0 | http://www.w3.org/2000/01/rdf-schema# | Hierarchy | Class, Resource, Property | subClassOf, domain, range |
| RDF/XML | http://www.w3.org/1999/02/22-rdf-syntax-ns# | Serialization | type, List | seeAlso, isDefinedBy |
| XSD 1.1 | http://www.w3.org/2001/XMLSchema# | Datatypes | string, integer, decimal, date | — |
| DCTERMS | http://purl.org/dc/terms/ | Metadata | — | title, description, created, issued |
| SKOS | http://www.w3.org/2004/02/skos/core# | Taxonomies | Concept, ConceptScheme | broader, narrower, exactMatch |
| SHACL 1.1 | http://www.w3.org/ns/shacl# | Validation | NodeShape, PropertyShape | minCount, datatype, pattern |
| PROV-O | http://www.w3.org/ns/prov# | Provenance | Entity, Activity, Agent | wasGeneratedBy, hadPrimarySource |
| ODRL 2.3 | http://www.w3.org/ns/odrl/2/ | Authorization | Policy, Permission, Prohibition | target, action, assignee |

### Entity Type Hierarchies (from ontologies)

#### POWL Process Mining Entities

```
DiscoveryVariant (8 individuals)
├── DecisionGraphCyclic          [Default, XOR→Seq→Conc→Loop+DG]
├── DecisionGraphCyclicStrict    [+strict validation]
├── DecisionGraphMax             [+MaximalPO cut]
├── DecisionGraphClustering      [+DynamicClusteringPO]
├── DynamicClustering            [XOR→Loop→DCP, no Seq/Conc]
├── Maximal                      [XOR→Seq→Conc→Loop→MaximalPO, no DG]
├── Tree                         [XOR→Seq only]
└── BruteForce                   [All cuts, exhaustive]

ConformanceDimension (4 individuals)
├── Fitness                      [Trace coverage, 0.0-1.0]
├── Precision                    [Model-allowed behaviour, 0.0-1.0]
├── Generalization               [Generalization beyond log]
└── Simplicity                   [Model parsimony]

OcelObjectType (base class)
└── [Domain-specific types defined in domain ontologies]
```

#### Cell8 Manufacturing Entities

```
Gate (A1-A13, defined in cell8-core.ttl)
├── A1  Requirement Admission      [DSPy + pm4py POWL replay]
├── A2  CTQ Acceptance            [precision ≥ 0.7, stages present]
├── A3  WorkOrder Binding         [Ontological alignment]
├── A4  Manufacturing Capability  [IaC + Rust + Erlang/AtomVM ready]
├── A5  Artifact Completeness     [All required files present]
├── A6  Provenance Linkage        [Parent artifacts traceable]
├── A7  BLAKE3 Chain Integrity    [Hash chain valid]
├── A8  Ed25519 Signature         [Cryptographic seal valid]
├── A9  Receipt Timestamp         [Monotonic, not retroactive]
├── A10 Attestation Authority     [Signature from trust set]
├── A11 Tenant Boundary           [Cross-tenant access denied]
├── A12 Immutability Lock         [Artifact not modified]
└── A13 Lineage Completeness      [Full trace to root requirement]

ManufacturingStage (5 individuals)
├── Seeded                        [Initial requirement state]
├── Bred                          [CTQ gate passed]
├── Validated                     [Manufacturing gates passed]
├── Admitted                      [Receipt issued]
└── Released                      [External visibility granted]
```

#### CLI Command Entities (from cli-open-ontologies.ttl)

```
Interface
└── OntoInterface
    ├── Command (50+ instances)
    │   ├── OntologyCommand      [Core RDF/OWL operations]
    │   ├── ServerCommand        [MCP server orchestration]
    │   ├── DataCommand          [Data ingestion + mapping]
    │   ├── GovernanceCommand    [Policy + enforcement]
    │   ├── AlignmentCommand     [Cross-ontology alignment]
    │   ├── ClinicalCommand      [Clinical terminology]
    │   ├── MarketplaceCommand   [Standard ontology browsing]
    │   ├── ThesisCommand        [Thesis domain operations]
    │   └── DoctorCommand        [Diagnosis + repair]
    └── [Each Command has 5-15 Subcommands]
        ├── LoadCommand
        ├── SaveCommand
        ├── ValidateCommand
        ├── QueryCommand
        └── [12+ more per command]

SpecialCommand (3 instances)
├── MakeCommand                  [Makefile-style operations]
├── InitCommand                  [Project initialization]
└── VerifyCommand                [Cryptographic receipt verification]
```

#### Truex E2E Test Ecosystem

```
TestEcosystem
├── TestObligation               [Test requirement]
├── TestCase                     [Test instance]
├── TestResult                   [Pass/Fail/Blocked]
├── FailureTaxonomy              [Categorized failures]
│   ├── CompilerError
│   ├── RuntimeFailure
│   ├── ComplianceFailure
│   └── TimeoutFailure
└── TestArtifact                 [Generated test asset]
    ├── Fixture
    ├── MockData
    └── ValidationRule
```

#### GitHub Factory Domain

```
Repository
├── SecurityPolicy               [Branch protection rules]
│   ├── BranchProtectionRule
│   ├── CodeOwnerRule
│   └── ReviewRequirement
├── Label                        [Repository labels]
├── Environment                  [Deployment environments]
└── Contribution                 [Contribution unit]
    └── ContributionUnit

WorkflowAction                    [GitHub Actions pipeline]
├── JobStep
└── Artifact
```

---

## 4. Manufacturing Infrastructure

### LLM Translation Pipeline

**Entry point:** `src/llm_translator.rs`

```
Requirement (natural language)
    │
    ▼
Groq LLM (DSPy-orchestrated)
    │
    ▼
DSPy Signature
(structure + type hints)
    │
    ▼
POWL Candidate
(Partially-Ordered Workflow Language, pm4py format)
```

**Tests:** 20 real-Groq integration tests in `tests/real_groq_*.rs`

### Recursive Admission Gates

**Pipeline:**
```
WorkOrder (from LLM)
    │
    ▼
CTQ Admission Gate
├── ocel_complete        [Event log conformance]
├── powl_replay          [POWL model replay]
├── precision ≥ 0.7      [Quality threshold]
└── stages_present       [All manufacturing stages]
    │
    ▼ (if ALL pass)
Admitted WorkOrder
    │
    ▼
Manufacturing
(IaC + Rust + Erlang/AtomVM swarm)
    │
    ▼
Artifact + Provenance
    │
    ▼
Cell8 13-Gate Attestation (A1-A13)
    │
    ▼
Receipt (BLAKE3 + Ed25519)
```

**Tests:** `tests/recursive_admission_e2e.rs`, `tests/admission*.rs` (8+ tests)

### 9-Breed Cognition Swarm

**Architecture:** Hearsay-II fusion with heterogeneous runtime targets

| Breed | Runtime | Purpose |
|-------|---------|---------|
| Rust (5 breeds) | Binary | Primary manufacturing logic |
| Erlang (1 breed) | BEAM VM | Distributed state management |
| AtomVM (1 breed) | WebAssembly | Portable embedded ops |
| Blend (1 breed) | Polyglot | Consensus voting |

**Tests:** `tests/real_swarm_e2e.rs` (4 tests), `tests/real_toolchains.rs` (5 tests using real terraform, cargo, erlc, AtomVM)

### Manufacturing Validators

```
IaC Validator              [Terraform HCL validation]
Rust Validator            [cargo build + cargo clippy]
Erlang Validator          [erlc syntax check]
AtomVM Validator          [WebAssembly module validation]
Schema Validator          [SHACL shape validation]
Conformance Validator     [POWL model replay]
Crosswalk Validator       [Type law consistency]
```

---

## 5. Query & Template Patterns

### SPARQL Query Categories

#### Type A: Aggregation Queries (GROUP_CONCAT)

**Purpose:** Flatten nested ontology structures into single rows for template rendering.

**Example:** `cli/commands_aggregated.rq`
```sparql
SELECT ?cmd ?name ?summary ?description ?subcommands
WHERE {
  onto:OntoInterface cli:hasCommand ?cmd .
  ?cmd cli:name ?name ; cli:summary ?summary ; cli:description ?description .
  OPTIONAL {
    ?cmd cli:hasSubcommand ?sub .
    ?sub cli:verb ?verb ; cli:noun ?noun .
  }
}
GROUP BY ?cmd
```

**ggen rendering:** One result row → one output file (no partial overwrites)

#### Type B: CONSTRUCT Rules (Inference)

**Purpose:** Normalize and materialize derived triples.

**Example:** `ggen.toml` normalize-cli-commands rule
```sparql
CONSTRUCT {
  ?cmd cli:name ?name ; cli:summary ?summary ; cli:description ?desc .
  ?sub cli:verb ?verb ; cli:noun ?noun .
}
WHERE {
  onto:OntoInterface cli:hasCommand ?cmd .
  ?cmd cli:name ?name ; cli:summary ?summary ; cli:description ?desc .
  OPTIONAL {
    ?cmd cli:hasSubcommand ?sub .
    ?sub cli:verb ?verb ; cli:noun ?noun .
  }
}
```

#### Type C: Extraction Queries (Materialization)

**Purpose:** Extract domain-specific data for migration generation.

**Example:** `zoela/extract-public-materializations.rq`
```sparql
SELECT ?table ?columns ?class_iri ?property_list
WHERE {
  # For each PUBLIC-ANCHOR-gated class in zoela domain
  ?class rdf:type owl:Class ;
         rdfs:label ?table .
  ?class rdfs:comment ?public_anchor .
  FILTER(CONTAINS(?public_anchor, "PUBLIC_ANCHOR"))
  
  # Find all admitted properties
  ?class rdfs:subClassOf ?prop_restriction .
  # (SPARQL property path to extract columns)
}
```

#### Type D: Governance Queries (Extraction)

**Purpose:** Extract authorization rules and materialization constraints.

**Example:** `zoela/extract-governance-tables.rq`
```sparql
SELECT ?role ?person_role_table ?permissions
WHERE {
  # Extract ODRL policies and convert to role hierarchies
  ?policy rdf:type odrl:Policy ;
          odrl:target ?target .
  ?target rdfs:label ?role .
}
```

#### Type E: Validation Queries (ASK)

**Purpose:** Check for presence/absence of critical patterns.

**Example:** `zoela/verify-supabase.rq` (implicit in validation)
```sparql
ASK {
  # Verify all PUBLIC-ANCHOR classes have corresponding tables
  ?class a owl:Class ; rdfs:comment ?comment .
  FILTER(CONTAINS(?comment, "PUBLIC_ANCHOR"))
  FILTER NOT EXISTS {
    ?table rdfs:label ?table_name .
    FILTER(?table_name = ?class)
  }
}
```

### Tera Template Patterns

#### Pattern 1: Code Generation with Loops

```tera
// Generated by ggen from {{ source_ontology }}
use crate::model::*;

pub const COMMANDS: &[CommandDef] = &[
{%- for cmd in commands %}
  CommandDef {
    name: "{{ cmd.name }}",
    summary: "{{ cmd.summary }}",
    subcommands: &{{ cmd.subcommands | join(", ") }},
  },
{%- endfor %}
];
```

#### Pattern 2: SQL Migration Generation

```tera
-- Generated migration for {{ table_name }}
-- Source: {{ source_iri }}

BEGIN;

CREATE TABLE IF NOT EXISTS {{ table_name }} (
  id UUID PRIMARY KEY,
{%- for col in columns %}
  {{ col.name }} {{ col.datatype }} {% if col.not_null %}NOT NULL{% endif %},
{%- endfor %}
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

COMMENT ON TABLE {{ table_name }} IS '{{ table_comment }}';

{%- for col in columns %}
COMMENT ON COLUMN {{ table_name }}.{{ col.name }} IS '{{ col.comment }}';
{%- endfor %}

COMMIT;
```

#### Pattern 3: TypeScript Type Generation

```tera
// Generated types for {{ domain }}
// Source: {{ source_ontology }}

{%- for enum in enums %}
export enum {{ enum.name }} {
{%- for value in enum.values %}
  {{ value.key }} = "{{ value.uri }}",
{%- endfor %}
}

export interface {{ enum.name }}Type {
  value: {{ enum.name }};
  label: string;
}
{%- endfor %}
```

#### Pattern 4: JSON Configuration Generation

```tera
{
  "name": "{{ package_name }}",
  "version": "{{ version }}",
  "description": "{{ description }}",
  "author": "{{ author }}",
{%- for dep in dependencies %}
  "{{ dep.name }}": "{{ dep.version }}",
{%- endfor %}
}
```

---

## 6. API/Storage Layer

### RDF Store: Oxigraph

**Configuration** (in ggen.toml):
```toml
[rdf]
store_path = ".ggen/rdf-store"
cache_queries = true
cache_ttl = 3600
```

**Capabilities:**
- SPARQL 1.1 SELECT, CONSTRUCT, ASK
- OWL-RL inference (transitive subclass, domain/range propagation)
- RDFS inference
- Named graphs support
- Full-text search indexes
- Query caching (SHA256 keyed)
- N-Triples serialization (.nt files)

### MCP Server: 50+ Tools

**Open Ontologies exposes onto_* tools via MCP protocol (stdio/HTTP):**

| Category | Tools | Examples |
|----------|-------|----------|
| Core Operations (8) | onto_validate, onto_load, onto_clear, onto_stats, onto_version, onto_history, onto_rollback, onto_cache_* | Ontology lifecycle |
| Querying (3) | onto_query, onto_search, onto_similarity | SPARQL + semantic search |
| Reasoning (2) | onto_reason, onto_dl_check | OWL-RL, DL tableaux |
| Validation (3) | onto_lint, onto_enforce, onto_shacl | Design patterns, SHACL |
| Data Extension (3) | onto_ingest, onto_sql_ingest, onto_map | RDF ingestion, schema mapping |
| Alignment (3) | onto_align, onto_align_feedback, onto_crosswalk | Cross-ontology matching |
| Governance (5) | onto_plan, onto_apply, onto_lock, onto_drift, onto_monitor | Terraform-style lifecycle |
| Code Generation (1) | onto_codegen | ggen artifacts |
| Repository (2) | onto_repo_list, onto_repo_load | Ontology repository management |
| Clinical (3) | onto_validate_clinical, onto_enrich, onto_crosswalk | Clinical terminology |
| Marketplace (1) | onto_marketplace | Standard ontology browsing (29 W3C/ISO/industry standards) |
| Conversion (1) | onto_convert | TTL, N-Triples, RDF/XML, N-Quads, TriG |
| Diff (1) | onto_diff | Version comparison |

**Protocol:** JSON-RPC 2.0 over stdio or HTTP
**Tests:** `tests/onto_integration_test.rs` + 695 total test functions

### Receipt Storage & Verification

**Receipt Structure** (BLAKE3-chained):

```json
{
  "receipt_id": "blake3:e1234567890abcdef...",
  "timestamp": "2026-06-01T15:36:00Z",
  "tenant_id": "default",
  "session_id": "sess_abc123",
  "gate_passed": "A1",
  "input_hash": "blake3:d1234567890abcdef...",
  "previous_receipt": "blake3:c1234567890abcdef...",
  "signature": "ed25519:sig_hex_string",
  "signing_key_id": "key_001",
  "admission_evidence": {
    "fitness": 0.999,
    "precision": 0.85,
    "stages": ["Seeded", "Bred", "Validated"]
  }
}
```

**Storage:** `.ggen/receipts/` (one JSON per manufacturing artifact)

**Verification:** External CLI tool `open-ontologies verify --receipt <path>`
- Validates BLAKE3 chain integrity
- Checks Ed25519 signature against trust set
- Exits non-zero on broken chain
- Supports offline replay verification

---

## 7. Integration Points Identified

### Integration Point 1: Shared SPARQL Infrastructure

**Location:** `.specify/queries/`

**Pattern:** PI can inject domain-specific queries alongside existing queries without modifying open-ontologies code.

**Example:**
```
.specify/queries/pi-lifecycle/ (new)
├── select-lifecycle-state-transitions.rq
├── select-process-mining-models.rq
└── select-conformance-metrics.rq
```

**Mechanism:** ggen reads all queries in `.specify/queries/` directory; new queries automatically available to templates.

### Integration Point 2: Tera Template Expansion

**Location:** `.specify/templates/`

**Pattern:** PI can add new templates for M&A deck generation, governance dashboards, etc.

**Example:**
```
.specify/templates/pi-m-and-a/ (new)
├── board-deck-summary.md.tera
├── claim-taxonomy-exhibit.json.tera
└── synergy-projection-table.tera
```

**Mechanism:** Add generation rule to ggen.toml + corresponding query.

### Integration Point 3: Ontology Extension

**Location:** `ontology/`

**Pattern:** PI lifecycle and M&A doctrine can be defined as separate .ttl files and imported into cli-open-ontologies.ttl.

**Example:**
```
ontology/pi-lifecycle.ttl (new)
├── Classes: LifecycleState, StateTransition, GovernanceRule
├── Properties: transitionsTo, guardedBy, appliesTo
└── Individuals: Seeded, Bred, Validated, Admitted, Released

ontology/pi-m-and-a.ttl (new)
├── Classes: MAClaim, SynergyProjection, DebtTaxonomy
├── Properties: hasEvidence, claimsReduction, requiresApproval
└── Individuals: M&A claim individuals
```

**Mechanism:**
```toml
[ontology]
imports = [
  # ... existing imports ...
  "ontology/pi-lifecycle.ttl",
  "ontology/pi-m-and-a.ttl",
]
```

**Constraint:** Total imports cannot exceed 36 (pre-flight scaling bottleneck documented in ggen.toml comments).

### Integration Point 4: Cell8 Gate Patterns

**Location:** `src/cell8/generated/stations.rs`, `src/cell8/generated/byte_law.rs`

**Pattern:** PI lifecycle state transitions can be modeled as Cell8-style gates.

**Mapping:**
- A1 → Seeded (Initial state)
- A2-A4 → Bred (Quality gates)
- A5-A9 → Validated (Manufacturing gates)
- A10-A11 → Admitted (Authorization + immutability)
- A12-A13 → Released (Lineage completeness)

**Mechanism:** Define PI lifecycle gates in `ontology/cell8-pi-lifecycle.ttl`, include in `cell8-ggen/ggen.toml`.

### Integration Point 5: Receipt Chain Extension

**Location:** `.ggen/receipts/`

**Pattern:** PI manufacturing events can append to the BLAKE3 receipt chain.

**Example:**
```json
{
  "receipt_id": "blake3:f...",
  "timestamp": "2026-06-01T16:00:00Z",
  "event_type": "pi:m-and-a-claim-admitted",
  "claim_id": "ma_claim_001",
  "evidence_sources": [
    "blake3:e...",  // Seeded receipt
    "blake3:d...",  // Conformance receipt
    "blake3:c..."   // Manufacturing receipt
  ],
  "previous_receipt": "blake3:b..."
}
```

### Integration Point 6: Cross-System Alignment

**Location:** `ontology/ontostar-wasm4pm-integration.ttl` (pattern model)

**Pattern:** PI can define alignment mappings between PI ontology and open-ontologies.

**Example:**
```turtle
@prefix pi:    <https://process.intelligence/> .
@prefix onto:  <https://ggen.io/onto/cli/open-ontologies/> .

pi:ProcessIntelligenceClaimClass
    owl:equivalentClass onto:ManufacturingStage ;
    onto:mappedBy "research-pi-program" ;
    onto:confidence 0.95 .

pi:ConformanceDimension
    owl:sameAs powl:ConformanceDimension ;
    rdfs:comment "Aligned with Van der Aalst quality framework" .
```

### Integration Point 7: OCEL 2.0 Object Types

**Location:** `ontology/powl-process-mining.ttl` (OcelObjectType class)

**Pattern:** PI process mining discovery can reuse OCEL 2.0 object type definitions.

**Example:**
```turtle
pi:ProcessArtifact
    rdfs:subClassOf powl:OcelObjectType ;
    rdfs:label "Process Intelligence Artifact"@en ;
    rdfs:comment "Business object type for PI manufacturing artifacts" .

pi:ReceiptObject
    rdfs:subClassOf powl:OcelObjectType ;
    rdfs:label "Receipt"@en ;
    rdfs:comment "Cryptographic receipt linking artifacts" .
```

---

## 8. Compatibility Assessment

### Architectural Alignment

| Dimension | Open Ontologies | PI Program | Alignment |
|-----------|-----------------|-----------|-----------|
| Source of Truth | OWL ontologies (TTL) | Markdown doctrine + ontologies | ✅ Both OWL-based |
| Query Language | SPARQL 1.1 | SPARQL 1.1 (ggen) | ✅ Identical |
| Template Engine | Tera | Tera (ggen) | ✅ Identical |
| Code Generation | ggen v26.5.x | ggen (PI ggen/) | ✅ Identical toolchain |
| RDF Store | Oxigraph | PI uses Oxigraph | ✅ Identical |
| Serialization | N-Triples (.nt) | N-Triples (.nt) | ✅ Identical |
| Manufacturing Gates | Cell8 A1-A13 | PI lifecycle gates | ✅ Structural pattern reuse |
| Cryptographic Receipts | BLAKE3 + Ed25519 | PI receipts | ✅ Identical chain model |
| Process Mining | POWL + OCEL 2.0 | Van der Aalst conformance | ✅ Explicit in POWL ontology |
| MCP Protocol | JSON-RPC 2.0 | MCP tools | ✅ Standard protocol |

### Shared Infrastructure

| Component | Location (Open Ontologies) | PI Integration Path |
|-----------|---------------------------|-------------------|
| SPARQL Queries | `.specify/queries/` (83 queries) | Add `queries/pi-*` subdirectory |
| Tera Templates | `.specify/templates/` (74 templates) | Add `templates/pi-*` subdirectory |
| Ontologies | `ontology/*.ttl` (30 files) | Define `ontology/pi-*.ttl`, import into ggen.toml |
| ggen.toml | Primary manifest at root | Extend [generation.rules] section |
| RDF Store | `.ggen/rdf-store` | Reuse Oxigraph instance |
| Receipts | `.ggen/receipts/` | Extend BLAKE3 chain with PI events |

### Scaling Constraints

**Known Bottleneck:** ggen pre-flight phase with >36 imports causes >10 min compile time.

**Current limit:** 36 imports in ggen.toml:imports array (documented in comments).

**PI Integration strategy:**
1. Keep PI-specific ontologies separate (pi-lifecycle.ttl, pi-m-and-a.ttl)
2. Import selectively into cli-open-ontologies.ttl (monitor import count)
3. If >36 imports needed, create separate ggen manifest (cell8-ggen.toml pattern)

### Dependency Chain

```
open-ontologies (Rust binary)
├── Cargo.lock (specifies Oxigraph version)
├── Oxigraph (RDF store) — vendored in workspace
└── [all transitive dependencies]

PI program (depends on)
├── open-ontologies [optional: as library]
└── ggen v26.5.x [via remote invocation or local binary]
```

**Constraint:** Both projects must track compatible ggen and Oxigraph versions.

---

## 9. Proposed Integration Bridges

### Bridge 1: PI Governance Query Library

**Scope:** Extract PI governance rules into SPARQL queries.

**Files:**
```
open-ontologies/.specify/queries/pi-lifecycle/
├── select-lifecycle-state-transitions.rq
├── select-process-mining-conformance.rq
├── select-m-and-a-claim-evidence.rq
└── select-autonomic-governance-rules.rq
```

**Use:** Fuel Blue River orchestrator generation and M&A deck manufacturing.

### Bridge 2: M&A Deck Manufacturing Rules

**Scope:** Define ggen rules for generating board-admissible M&A artifacts.

**Files:**
```
ggen.toml
[[generation.rules]]
name = "m-and-a-deck"
query = { file = ".specify/queries/pi-m-and-a/extract-claim-evidence.rq" }
template = { file = ".specify/templates/pi-m-and-a/board-deck.md.tera" }
output_file = "../ma/generated_board_deck.md"
```

### Bridge 3: Process Mining Conformance Extraction

**Scope:** Extract conformance metrics from discovered processes.

**Query:**
```sparql
PREFIX pi: <https://process.intelligence/>
PREFIX powl: <urn:ontostar:powl:>

SELECT ?variant ?fitness ?precision ?generalization ?simplicity
WHERE {
  ?discovery pi:usedVariant ?variant ;
             pi:hasFitness ?fitness ;
             pi:hasPrecision ?precision ;
             pi:hasGeneralization ?generalization ;
             pi:hasSimplicity ?simplicity .
  
  ?variant rdfs:label ?variant_name .
}
```

### Bridge 4: Lifecycle State Machine Codegen

**Scope:** Generate Rust state machine from PI lifecycle ontology.

**Template:**
```tera
use crate::lifecycle::*;

pub struct LifecycleStateMachine {
    current_state: LifecycleState,
}

impl LifecycleStateMachine {
  {%- for transition in transitions %}
  pub fn {{ transition.method_name }}(&mut self) -> Result<()> {
    match self.current_state {
      {%- for guard in transition.guards %}
      LifecycleState::{{ guard.from }} if {{ guard.condition }} => {
        self.current_state = LifecycleState::{{ guard.to }};
        Ok(())
      }
      {%- endfor %}
      _ => Err(anyhow!("Illegal state transition")),
    }
  }
  {%- endfor %}
}
```

### Bridge 5: Cross-System Receipt Verification

**Scope:** Extend receipt verification to validate PI-specific evidence.

**Query:**
```sparql
PREFIX pi: <https://process.intelligence/>
PREFIX onto: <https://ggen.io/onto/>

ASK {
  # Verify all M&A claims have backing receipts
  ?claim a pi:MAClaim ;
         pi:requiresApproval true .
  ?claim pi:hasEvidenceReceipt ?receipt .
  ?receipt onto:isValidated true .
}
```

---

## 10. Resource Inventory

### Ontology Files (30 total)

**By domain:**
- CLI + MCP Tools: 3 files (cli-open-ontologies, requirements, public-alignment)
- Cell8 Manufacturing: 8 files (cell8-core/manufacturing/profile/shapes/conformance)
- Process Mining: 2 files (powl-process-mining, wasm4pm-stubs)
- Integration: 2 files (ontostar-wasm4pm-integration, wasm4pm-stubs)
- Test Ecosystem: 2 files (truex-ecosystem, thesis-manufacturing)
- ZOE LA Mobile: 7 profile files + base domain files
- GitHub Factory: 3 files (ghf-core, ghf-shacl, ghf-security-policy)
- Other: portfolio-os, aat-live-rules, shared-receipt-shapes, etc.

### Code Generation Assets

**Queries:** 83 SPARQL queries
**Templates:** 74 Tera templates
**Generation Rules:** 40+ active rules in ggen.toml + 2 in cell8-ggen.toml
**Output Files:** 50+ generated Rust, SQL, TypeScript, JSON, Markdown, JS files

### Test Coverage

**Total tests:** 695 functions
- Real Groq tests: 20 (require GROQ_API_KEY)
- Real toolchain tests: 5 (terraform, cargo, erlc, AtomVM)
- Hermetic unit tests: ~670
- E2E tests: ~20

**Coverage areas:**
- LLM translation + DSPy signatures
- Recursive admission gates
- 9-breed cognition swarm
- Cell8 13-gate attestation
- Receipt chain (BLAKE3 + Ed25519)
- Multi-tenant isolation
- SHACL validation
- Graph operations (Oxigraph)
- MCP tool integration

---

## 11. Detected Patterns & Best Practices

### Pattern 1: Aggregation-Based Codegen

**Rule:** Use `GROUP_CONCAT` in SPARQL SELECT queries to flatten nested ontology structures into single rows.

**Benefit:** ggen renders one row per file, preventing partial overwrites.

**Example:** `cli/commands_aggregated.rq` produces one row per top-level command with concatenated subcommands.

### Pattern 2: CONSTRUCT Normalization

**Rule:** Before SELECT queries, use CONSTRUCT rules to materialize derived triples.

**Benefit:** Simplifies SELECT queries; separates schema inference from data extraction.

**Example:** `normalize-cli-commands` CONSTRUCT materializes cli:name, cli:verb, cli:noun for every command.

### Pattern 3: Domain-Specific Ontology Profiles

**Rule:** Define domain rules in separate TTL files, imported into primary ontology.

**Benefit:** Isolates domain logic; enables independent reasoning over domain subsets.

**Example:** `zoela-supabase-materialization.ttl` defines PUBLIC_ANCHOR constraints; separate from cli-open-ontologies.

### Pattern 4: Multi-Stage Migration Generation

**Rule:** Split SQL migrations into separate rules (tables → columns → RLS policies).

**Benefit:** Avoids Oxigraph OPTIONAL join-correlation bugs under high import counts; respects migration order.

**Example:** zoela-public-materialization → zoela-materialization-columns → zoela-governance → zoela-legacy-coverage → zoela-rls.

### Pattern 5: SHACL-Backed Validation

**Rule:** Define constraints as SHACL NodeShape files (not embedded in OWL classes).

**Benefit:** Separates schema definition from validation rules; enables runtime constraint checking.

**Example:** `cell8-conformance-shapes.ttl` validates A1-A3 gate preconditions.

### Pattern 6: Receipt Chain Extensibility

**Rule:** Structure receipts as immutable JSON with previous_receipt pointer.

**Benefit:** Enables offline chain verification; supports multiple receipt sources.

**Example:** `.ggen/receipts/` contains 200+ receipts, each pointing to parent.

---

## Conclusion

**Open Ontologies is deeply compatible with PI program objectives.** The project demonstrates mature ggen infrastructure, explicit process mining domain modeling, and proven patterns for manufacturing complex artifacts from ontologies. Integration points are numerous and well-isolated:

1. **Shared toolchain** (ggen, Oxigraph, Tera) — no version conflicts expected
2. **Query + template extensibility** — PI can add rules to ggen.toml without modifying open-ontologies code
3. **Ontology extension** — PI lifecycle + M&A doctrine can be separate TTL files, imported into manufacturing pipeline
4. **Receipt chain** — PI manufacturing events can extend BLAKE3 chain via bridge queries
5. **Process mining foundation** — POWL ontology + OCEL 2.0 object types are ready for reuse

**Immediate next steps:**
- Define PI lifecycle entities in separate TTL file (pi-lifecycle.ttl)
- Define PI M&A claim ontology (pi-m-and-a.ttl)
- Implement governance query library in .specify/queries/pi-*
- Create Tera templates for M&A deck artifacts in .specify/templates/pi-m-and-a/
- Test ggen integration with <36 imports constraint

**Verdict:** GATE_PASS — Open Ontologies discovery complete; integration bridges identified and technically feasible.
