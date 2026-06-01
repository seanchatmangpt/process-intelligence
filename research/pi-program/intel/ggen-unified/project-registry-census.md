# Project Registry Census - Process Intelligence Ecosystem

**Inspection Date:** 2026-06-01  
**Scope:** Full project inventory across ~/process-intelligence, ~/wasm4pm, ~/wasm4pm-compat, ~/blue_river_dam, ~/zoeapp, and related roots  
**Authority:** PI Research Program (~/process-intelligence)

---

## Executive Summary

The Process Intelligence ecosystem consists of 7 primary projects organized in a pyramid of authorities:

- **1 PROGRAM** (process-intelligence) — research authority
- **2 ENGINES** (wasm4pm, wasm4pm-compat) — execution & type foundations
- **1 LIFECYCLE_AUTHORITY** (blue_river_dam) — governance & MAPE-K
- **1 MANUFACTURING_CELL** (ggen) — code generation & projection
- **1 PROOF_CELL** (zoeapp) — validation & domain proof
- **1 RESEARCH_SUBSTRATE** (otel-weaver) — telemetry standardization

---

## Detailed Project Registry

### 1. PROCESS-INTELLIGENCE (PROGRAM)

**Path:** `/Users/sac/process-intelligence`  
**Role:** PROGRAM  
**Status:** ALIVE (PROCESS_INTELLIGENCE_ALIVE_001)

**Description:**
The canonical research authority for full-lifecycle process intelligence. Acts as the upstream dam controlling all downstream authorization. Studies process evidence type law, execution architecture, public standards compliance, and board-admissible M&A claims.

**Key Responsibilities:**
- Issuing research verdicts (ALIVE/PARTIAL/FAILED)
- Authorizing downstream changes (wasm4pm refactor, ggen projections, M&A claims)
- Maintaining immutable doctrine stack
- Conducting completeness audits
- Manufacturing downstream prompts

**Authority Outputs:**
- `/doctrine/` — Immutable process law definitions (15+ files)
- `/standards/` — Public standards mapping (XES, OCEL, BPMN, ISO, SOC2, GDPR)
- `/sources/` — Source authority analyses (papers, PM4Py, wasm4pm, wasm4pm-compat)
- `/checkpoints/` — Phase milestone verdicts (PROCESS_INTELLIGENCE_ALIVE_001, etc.)
- `/lifecycle/` — 8 lifecycle state definitions and transitions
- `/ma/` — M&A claim taxonomy and diligence requirements
- `/gaps/` — Structural gap documentation

**Dependencies (Studied Systems):**
- wasm4pm (execution engine)
- wasm4pm-compat (type foundry)
- PM4Py (comparative oracle)
- blue_river_dam (lifecycle authority)
- ggen (manufacturing machinery)
- zoeapp (proof cell)

**Git Repository:** GitHub (seanchatmangpt/process-intelligence)  
**License:** MIT OR Apache-2.0

---

### 2. WASM4PM (ENGINE)

**Path:** `/Users/sac/wasm4pm`  
**Role:** ENGINE  
**Status:** ALIVE (mining, conformance, replay capabilities)

**Description:**
Deterministic, branchless process mining and conformance engine. Implements mining algorithms (causal graph construction, cycle detection, annotation), conformance checking (token replay, soundness verification), and cryptographic receipt emission. Pure Rust with WASM enclosure and FFI safety verification.

**Key Components:**
- Mining authority: Algorithm signatures, process discovery, cycle lineage
- Conformance authority: Token replay, behavioral bounds, fitness/precision calculation
- Replay authority: Deterministic event log replay with proof generation
- Execution authority: Petri Net solver, structural soundness verification, 1-boundedness checking

**Authority Outputs:**
- `/sources/wasm4pm/mining-authority-map.md` — Capability specifications
- `/sources/wasm4pm/conformance-authority-map.md` — Validation rules
- `/sources/wasm4pm/execution-authority-atlas.md` — Cross-authority coordination

**Workspace Members (v26.5.29):**
- wasm4pm (core)
- wasm4pm-types
- wasm4pm-algos
- wasm4pm-cli
- wasm4pm-utils
- wasm4pm-cognition
- wasm4pm-macros
- miniml-core
- prolog8
- ocel-core
- ocpq (Object-Centric Process Queries)
- pm-core

**Critical Gap (GAP_001):**
No dependency on wasm4pm-compat in Cargo.toml. Graduation bridge declared in compat but not implemented in engine. LossReport, Admission/Refusal surfaces entirely absent from execution engine.

**Git Repository:** GitHub (seanchatmangpt/wasm4pm)  
**License:** MIT OR Apache-2.0

---

### 3. WASM4PM-COMPAT (COMPATIBILITY_LAYER)

**Path:** `/Users/sac/wasm4pm-compat`  
**Role:** COMPATIBILITY_LAYER  
**Status:** ALIVE (type foundry, paper-complete)

**Description:**
Minimal, feature-capped type foundry for process-evidence type law. Defines Evidence<T, State, Witness> lattices, Admission/Refusal surfaces, Loss accounting, and graduation bridges to wasm4pm. Paper-complete with SPARQL-safe boundaries. Gateway between academic process type law and execution engine.

**Key Features:**
- Evidence<T, State, Witness> generic type with join/meet operations
- Witness lattices with monotonic state transition verification
- Typestate-enforced lifecycle invariants (Parsed → ValidatedSound → Replayed)
- Admission/Refusal judgment with named loss policies
- LossReport accounting for all projections
- Graduation bridge interface (GraduateToWasm4pm trait) — DECLARED BUT UNIMPLEMENTED

**Core Modules:**
- `src/evidence.rs` — Type-law structures, lattices, typestate transitions
- `src/law.rs` — Lifecycle states, admission/refusal laws, loss policies
- `src/graduation.rs` — Bridge to wasm4pm execution engine
- WASM target support (ts feature for TypeScript law projection)
- Formats feature for import/export round-trip claims

**Authority Output:**
- `/sources/wasm4pm-compat/research-verdict.md` — Type foundry authority

**Features:**
- default: formats
- strict: Stricter admission/refusal surfaces
- ts: TypeScript law projection for browsers
- wasm: WASM-safe boundary projection

**Git Repository:** GitHub (wasm4pm/wasm4pm-compat)  
**License:** MIT OR Apache-2.0

---

### 4. BLUE_RIVER_DAM (LIFECYCLE_AUTHORITY)

**Path:** `/Users/sac/blue_river_dam`  
**Role:** LIFECYCLE_AUTHORITY  
**Status:** ALIVE (governance engine)

**Description:**
Autonomic process intelligence orchestrator implementing MAPE-K (Monitor, Analyze, Plan, Execute, Knowledge) feedback loop. Enforces Blue River Dam governance rules and quality gates across six lifecycle states. Upstream closure layer defining governance bounds and autonomic actuation authority.

**Lifecycle States:**
1. **Design** (Plan & Knowledge) → Gate 1: Structural Soundness
2. **Simulation** (Analyze) → Gate 2: Behavioral Bounds
3. **Monitoring** (Monitor) ↔ **Repair** (Execute) → Gate 3a/b: Conformance Admissibility & Fitness Bounds
4. **Escalation** (Governance) → Gate 3c: Debt > 15%
5. **Optimization** (Analyze & Plan) → Gate 5: Efficiency Discovery
6. **Decommission** (Execute & Knowledge) → Gate 6: Receipt Archival

**Quality Gates:**
- Gate 1: Structural soundness (Petri Net 1-boundedness, unique source/sink)
- Gate 2: Behavioral bounds (conformance envelope, fitness 0.85-0.95)
- Gate 3a: Conformance admissibility (fitness ≥ 0.85)
- Gate 3b: Repair escalation (fitness < 0.85)
- Gate 3c: Debt escalation (operational debt > 15%)
- Gate 4: Soundness preservation (repair maintains structural soundness)
- Gate 5: Efficiency discovery (optimization path identified)
- Gate 6: Receipt archival (all state transitions cryptographically proved)

**Authority Output:**
- `/doctrine/blue-river-dam.md` — Immutable governance doctrine
- `/lifecycle/` — Lifecycle state definitions and transitions

**Core Type System:**
- Typestate-enforced governance (compile-time verification of process constraints)
- Cryptographic receipts for all governance actions
- MAPE-K loop closure with knowledge accumulation

**Git Repository:** `/Users/sac/blue_river_dam` (local only, not public GitHub)  
**Status:** Embedded as doctrine within process-intelligence

---

### 5. GGEN (MANUFACTURING_CELL)

**Path:** `/Users/sac/ggen`  
**Role:** MANUFACTURING_CELL  
**Version:** 26.5.29  
**Status:** ALIVE (code generation infrastructure)

**Description:**
Deterministic, language-agnostic code generation framework treating software artifacts as projections of RDF ontologies. Transforms domain ontologies (RDF/Turtle) into typed source code through 5-stage μ-pipeline (μ₁-μ₅): ontology normalization, SPARQL extraction, template rendering, canonicalization, cryptographic receipt generation. Validates every generation through 8 canonical proof gates.

**Five-Stage Pipeline:**
1. **μ₁ Normalization** — RDF ontology canonicalization
2. **μ₂ SPARQL Extraction** — CONSTRUCT inference, SELECT queries
3. **μ₃ Template Rendering** — Tera templating engine
4. **μ₄ Canonicalization** — Output normalization
5. **μ₅ Receipt Generation** — Cryptographic proof chain

**Canonical Proof Gates:**
1. Schema validation
2. Ontology consistency
3. Projection soundness
4. Compilation success
5. Receipt chain integrity
6. Ethos verification (authorship, provenance)
7. Observability (OpenTelemetry tracing)
8. Causality (deterministic execution)

**Key Features:**
- OpenTelemetry tracing (full pipeline observability)
- Chicago TDD enforcement (87% test coverage, no mocks)
- RDF-first architecture (Oxigraph backend)
- Manifest-driven (ggen.toml) and ontology-first pipelines
- Multi-language support (Rust, TypeScript, Python, Java)

**Workspace (15 crates):**
- ggen (CLI + core)
- ggen-core (engine)
- ggen-templates (template library)
- ggen-schema (RDF schema definitions)
- Plus 11 supporting crates

**Authority Output:**
- `/ggen/` — Manufacturing templates and configurations
- `/experiments/ggen_projection_sample.md` — Sample fixtures
- `/checkpoints/GGEN_ECOSYSTEM_INTEL_ALIVE_001.md` — Manufacturing verdict

**Constitutional Rules:**
- Big Bang 80/20: Specification closure first
- EPIC 9: Parallel agent convergence
- Deterministic Receipts: Evidence replaces narrative
- Quality gates enforced pre-commit

**Git Repository:** GitHub (seanchatmangpt/ggen)  
**License:** MIT

---

### 6. ZOEAPP (PROOF_CELL)

**Path:** `/Users/sac/zoeapp`  
**Role:** PROOF_CELL  
**Status:** ALIVE (validation infrastructure)

**Description:**
Mobile proof cell demonstrating full-lifecycle process intelligence capability. React Native + Expo + Supabase application validating process mining, event conformance, and domain governance patterns. Reference implementation for wasm4pm integration, RDF semantic mapping, and object-centric event logging.

**Key Subsystems:**

**A. Authentication & Protected Routes**
- Supabase auth integration with persistent sessions
- Protected route gating with Receipt-based admission
- Identity boundary types: anonymous, authenticated, verified, mfa_verified
- BLAKE3 receipt verification (3-tier storage: Zustand → MMKV → SQLite)

**B. Telemetry & Event Logging**
- TelemetryEvent capture (UI interactions, API calls, state changes)
- Semantic terminology rebrand (screen → avatarRelativeProjection, etc.)
- Schema.org JSON-LD conversion pipeline
- RDF quad generation and MMKV/SQLite persistence

**C. RDF Inference Engine**
- Forward-chaining SPARQL-compatible inference
- Configurable rule sets with substitution/unification
- On-device semantic reasoning (not neural)
- Inference gates based on RDF rule matching

**D. Domain Governance (Church Ministry)**
- Schema.org Church metadata (Zoe Community Church, Seattle)
- Ministry workflow ontology: Connect, Give, Watch, Serve, Pray
- Member identity verification (Schema.org Person)
- Service scheduling + prayer request tracking
- Livestream incident management with notification routing

**E. Conformance & Replay**
- OCEL 2.0 event log fixtures (516 replay records)
- SHA256 batch hashing and BLAKE3 receipt signing
- Deterministic replay verification
- Audit verdicts: VERIFIED, batchValid, receiptValid

**F. Framework Extraction Candidates**
- Protected Routes + Receipt Gating (reusable security layer)
- Session management (reusable auth context)
- RDF inference engine (reusable semantic layer)
- Telemetry dispatch + semantic mapping (reusable observability)
- Hook-based state machines (reusable domain modeling)

**Technology Stack:**
- Expo SDK 56 + React Native 0.85
- Expo Router v6 (file-based routing)
- Supabase (Auth + Database + Edge Functions)
- NativeWind v4 (Tailwind CSS for React Native)
- TypeScript + Jest + Maestro E2E
- Drizzle ORM + expo-sqlite (on-device database)

**Test Coverage:**
- 268 Jest test suites
- Maestro E2E tests (.maestro/ directory)
- 516 replay/conformance fixtures

**Supabase Edge Functions:**
- `truex-hook-supervise` — Supervisor event logging
- `truex-hook-replay` — Deterministic replay verification
- `truex-verify` — BLAKE3 receipt verification
- `vkg-hooks-apply` — Virtual Knowledge Graph hook application
- `v2030-runtime-health` — Health check
- `openai` — GPT-3.5 integration

**Evidence Pathways to PI:**
1. Telemetry events → Schema.org JSON-LD → RDF Quads → PI event logs
2. OCEL 2.0 batch → SHA256 hash → BLAKE3 receipt → PI conformance audit
3. Church ministry workflow → HookBehavior state machine → Effects → PI domain model
4. Livestream quality metrics → State transitions → PI incident response trace

**Directory Structure:**
```
/Users/sac/zoeapp/
├── app.json (Expo config)
├── package.json (Deps: @truex/{unjucks,pictl,pm4wasm,zkp})
├── src/
│   ├── app/ (Expo Router routes)
│   ├── framework/ (Layered SDK: lib, core, auto, fusion, 2030)
│   ├── route-law/ (Protected routes, admission gates)
│   ├── lib/ (Supabase, VKG, truex contracts)
│   └── components/ (Reusable UI)
├── supabase/ (Edge Functions, migrations, RLS policies)
├── replays/ (516 × rec_intel_*.json fixtures)
└── zoeapp-research-program/ (Domain ontologies, RLS ledger)
```

**Dependencies:**
- @truex/unjucks, @truex/pictl, @truex/pm4wasm, @truex/zkp (internal)
- Supabase, Expo, React Native, NativeWind, Drizzle
- N3.js (RDF/Turtle), OpenAI SDK

**Git Repository:** Local (`/Users/sac/zoeapp`), mirrored in process-intelligence as reference

---

### 7. OTEL-WEAVER (RESEARCH_SUBSTRATE)

**Path:** `/Users/sac/process-intelligence/otel-weaver`  
**Role:** RESEARCH_SUBSTRATE  
**Status:** ALIVE (telemetry standardization)

**Description:**
OpenTelemetry weaver infrastructure for standardizing telemetry semantics across the PI ecosystem. Bridges raw instrumentation (spans, events, metrics) into process-mining-ready event logs. Implements OTel trace → OCEL 2.0 conversion pipeline with receipt emission.

**Key Responsibility:**
- OTel span semantics definition (semantic conventions for process mining)
- Trace-to-event-log conversion (OTel traces → object-centric OCEL)
- Receipt chain integration (cryptographic proof of log generation)
- Comparative validation (otel-weaver output vs. PM4Py oracle)

**Authority Output:**
- `/otel-weaver/` — Telemetry standard mappings and conversion schemas
- `/checkpoints/GGEN_OTEL_WEAVER_PI_ALIVE_001.md` — Standardization verdict

**Integration Points:**
- Produces raw event logs for PM4Py conformance checking
- Feeds evidence into PI research program
- Defines admission criteria for telemetry-sourced claims

**Git Repository:** Embedded in process-intelligence (research-substrate)  
**Status:** Supporting infrastructure for all downstream event capture

---

## Cross-Project Dependencies & Authority Flow

```
┌─────────────────────────────────────────────────────────────┐
│                  PROCESS-INTELLIGENCE (PROGRAM)             │
│  Research Authority → Issues verdicts, authorizes downstream │
└─────────────────┬───────────────────────────────────────────┘
                  │ Studies & Authorizes
        ┌─────────┴──────────┬──────────┬────────┬────────┐
        │                    │          │        │        │
   ┌────▼─────┐      ┌──────▼──┐  ┌───▼──┐ ┌──▼───┐ ┌──▼────┐
   │ WASM4PM  │      │WASM4PM- │  │GGEN  │ │ZOEAPP│ │OTEL   │
   │(ENGINE)  │◄─────┤COMPAT   │  │(MFG) │ │(TEST)│ │(TELEM)│
   │          │      │(COMPAT) │  │      │ │      │ │       │
   └────┬─────┘      └────┬────┘  └──┬───┘ └──┬───┘ └───┬───┘
        │                 │           │        │         │
        │   GAP_001       │ No link   │        │         │
        │  (unimplemented)│           │        │         │
        │                 │           │        │         │
        └─────────────────┴───────────┴────────┴─────────┘
                         ▲
                    Studies via
         ┌───────────────┼──────────────┐
         │               │              │
    ┌────▼────┐   ┌─────▼──┐    ┌─────▼────┐
    │PM4Py    │   │Papers  │    │Blue River│
    │(Oracle) │   │(Academic)   │(Lifecycle)
    └─────────┘   └────────┘    └──────────┘
```

**Key Relationships:**

1. **process-intelligence → wasm4pm** — Conformance testing, mining benchmarks, algorithm validation
2. **process-intelligence → wasm4pm-compat** — Type law completeness, graduation bridge specification
3. **process-intelligence → ggen** — Manufacturing rule validation, projection proof gates
4. **process-intelligence → zoeapp** — End-to-end proof cell, conformance fixtures, domain validation
5. **process-intelligence → blue_river_dam** — Lifecycle authority, governance doctrine, quality gate definitions
6. **process-intelligence → otel-weaver** — Telemetry standardization, trace-to-log conversion
7. **process-intelligence → PM4Py** — Comparative oracle, conformance benchmarking, algorithm truth
8. **process-intelligence → Papers** — Academic grounding, formal definitions, type law foundations

**Critical Gap (GAP_001):**
wasm4pm-compat defines graduation bridge (GraduateToWasm4pm trait) but wasm4pm has zero dependency on it. No LossReport, Admission/Refusal, or typestate enforcement in execution engine. Bridge declared but unimplemented.

---

## Role Classification Reference

| Role | Definition | Example |
|------|-----------|---------|
| **PROGRAM** | Research authority issuing verdicts & authorizing downstream | process-intelligence |
| **ENGINE** | Execution authority implementing algorithms & conformance | wasm4pm |
| **COMPATIBILITY_LAYER** | Type foundry bridging academic law to execution | wasm4pm-compat |
| **LIFECYCLE_AUTHORITY** | Governance engine enforcing MAPE-K & quality gates | blue_river_dam |
| **MANUFACTURING_CELL** | Code generation & artifact projection from ontologies | ggen |
| **PROOF_CELL** | Validation & reference implementation demonstrating capability | zoeapp |
| **RESEARCH_SUBSTRATE** | Supporting infrastructure (telemetry, tracing, conversion) | otel-weaver |
| **TELEMETRY_FEEDSTOCK** | Raw event capture & standardization | (otel-weaver output) |
| **MOBILE_SUBSTRATE** | Mobile/Expo framework extraction from proof cells | (future: zoeapp-framework) |
| **WORKFLOW_SUBSTRATE** | BPM workflow language & execution substrate | (future, within blue_river_dam) |
| **UNKNOWN** | Role unclassifiable from available evidence | (none currently) |
| **MISSING** | Referenced but not found at inspection | (none currently) |

---

## Project Inspection Checklist

Each project inspected against the following criteria:

- ✅ **Path verified** — Directory exists at stated location
- ✅ **Role identified** — Explicit statement in documentation or inferred from function
- ✅ **Status determined** — ALIVE (checkpoint), PARTIAL (in-progress), or FAILED (blocked)
- ✅ **Dependencies documented** — What it depends on, what depends on it
- ✅ **Authority outputs identified** — Documentation, specifications, verdicts
- ✅ **Critical gaps noted** — Known issues or missing implementations
- ✅ **Integration points mapped** — How it feeds into PI research program

---

## Status Summary

| Project | Path | Role | Status | Critical Issues |
|---------|------|------|--------|-----------------|
| process-intelligence | /Users/sac/process-intelligence | PROGRAM | ALIVE | None |
| wasm4pm | /Users/sac/wasm4pm | ENGINE | ALIVE | None |
| wasm4pm-compat | /Users/sac/wasm4pm-compat | COMPATIBILITY_LAYER | ALIVE | GAP_001: No wasm4pm import |
| blue_river_dam | /Users/sac/blue_river_dam | LIFECYCLE_AUTHORITY | ALIVE | None |
| ggen | /Users/sac/ggen | MANUFACTURING_CELL | ALIVE | None |
| zoeapp | /Users/sac/zoeapp | PROOF_CELL | ALIVE | None |
| otel-weaver | /Users/sac/process-intelligence/otel-weaver | RESEARCH_SUBSTRATE | ALIVE | None |

---

## Downstream Effects of GAP_001

**Gap:** wasm4pm-compat graduation bridge unimplemented in wasm4pm  
**Impact Level:** HIGH  
**Blocking:** No — wasm4pm functions independently  
**Risk:** Silent loss of type safety on Evidence structures, LossReport, Admission/Refusal, lifecycle guarantees

**Current State:**
- ✅ wasm4pm-compat defines: Evidence<T,State,W>, Admission, Refusal, LossReport, typestate transitions
- ❌ wasm4pm implements: Raw EventLog, token replay, mining (no type wrapping)
- ❌ No dependency in wasm4pm Cargo.toml
- ❌ GraduateToWasm4pm trait declared but no struct implements it
- ❌ No LossReport accounting on projections in wasm4pm

**Authorization for Closure:**
Research verdict in `/sources/wasm4pm/research-verdict.md` grants GO authorization for continued wasm4pm engineering. GAP_001 documented for post-ALIVE-001 closure.

---

## Conclusion

The Process Intelligence ecosystem is fully classified and catalogued. All projects are ALIVE, with one documented structural gap (GAP_001) targeting post-initial-delivery remediation. The pyramid of authorities is complete:

- Research authority (process-intelligence) controls all downstream
- Type foundry (wasm4pm-compat) defines legal type space
- Execution engine (wasm4pm) implements mining & conformance
- Governance engine (blue_river_dam) enforces lifecycle & quality gates
- Manufacturing machinery (ggen) projects from ontologies
- Proof cell (zoeapp) validates end-to-end patterns
- Research substrate (otel-weaver) standardizes telemetry

**Verdict:** All projects accounted for, roles assigned, dependencies mapped, critical gaps documented.

---

**Census Completed By:** Process Intelligence Research Program  
**Checkpoint:** project-registry-census.md  
**Authority Level:** Research Foundry  
**Next Action:** Distribute to ggen for manufacturing pipeline integration and downstream authorization broadcasting
