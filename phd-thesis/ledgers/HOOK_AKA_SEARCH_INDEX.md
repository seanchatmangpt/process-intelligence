# Knowledge Hooks / AKA Unified Search Index

**Generated:** 2026-06-01
**Sources:** Three search agents — Agent 1 (KNHK Primary), Agent 2 (Research Cluster), Agent 3 (Extended Corpus)
**Total sources indexed:** 68
**HIGH relevance:** 34  MEDIUM relevance:** 23  LOW relevance:** 11

---

## Correct Direction (Frame Law — Load-Bearing Spine)

```
attempt → hook → admission/refusal → durable motion → receipt → replay → accounting → promotion
```

Compression (verbatim from `/Users/sac/truex/docs/MANIFESTO.md`):

```
No hook, no consequence.
No receipt, no authority.
No replay, no substrate.
No accounting, no promotion.
```

---

## Primary Sources (from ~/knhk and ~/gitvan)

### ~/knhk — Primary KNHK Doctrine

**`/Users/sac/knhk/KNHK_PHD_THESIS.md`** — HIGH
Verbatim: "KNHK (Knowledge Hot Path Engine) ... eliminates false positives through schema-first development." Receipt: "Cryptographic proof of execution. Hash of inputs and outputs, prevents tampering." Covenant 3: "Feedback Loops Run at Machine Speed (MAPE-K ⊨ Autonomy)." A = μ(O): "Actions (A) = μ (Execution) applied to Observations (O)."

**`/Users/sac/knhk/DOCTRINE_2027.md`** — HIGH
Verbatim: "MAPE-K embedded as knowledge hooks is the only way to keep the discipline while closing the loop at machine speed." Era table row: "2027 → | Autonomous evolution | MAPE-K autonomic hooks | Sub-nanosecond decisions." Chatman Constant Q3: "max_run_length ≤ 8 ticks."

**`/Users/sac/knhk/GENESIS_CORE_SPECIFICATION.md`** — HIGH
Primary structural definitions for Receipt (Rust struct with BLAKE3 chain), Refusal (struct with RefusalReason enum), Construct8 (bounded 8-lane act), Pair2, AdmissionGate (SPARQL ASK predicate). Kernel/membrane separation doctrine: "Genesis is the consequence law kernel... IO-free, pure, and testable in isolation."

**`/Users/sac/knhk/V30_1_1_MANIFESTO.md`** — HIGH
Verbatim final manifesto: "origin before motion | canon before actuation | admission before consequence | refusal before corruption | receipt before claim | replay before trust | substitution before scale | continuity before growth." First Law: "No movable part executes without canon_basis[]." Refusal Is Integrity: "A system that cannot refuse cannot preserve consequence." Receipts Replace Logs: "Logs are observation. Receipts are institutional memory."

**`/Users/sac/knhk/PART_SPEC.md`** — HIGH
Truex 6-phase lifecycle verbatim: Phase 1 Attempt, Phase 2 Hook (SPARQL ASK admission gate), Phase 3 Projection (Receipt generated), Phase 4 Admission (lockchain), Phase 5 Refusal/Accounting, Phase 6 Promotion (Corpus → Authority). "Refusals are first-class evidence, not errors to hide."

**`/Users/sac/knhk/DOCTRINE_COVENANT.md`** — HIGH
Six binding covenants. Covenant 1: "Turtle ontologies are the single source of truth." Covenant 2: "Q invariants are not suggestions; they are enforceable constraints." Covenant 3: "Every workflow has embedded monitoring, analysis, planning, execution, and learning." Covenant 5: "8 ticks (nanoseconds) is the hard latency bound for all critical path operations." Covenant 6: "Observations (O) are not logs; they are first-class data with the same status as code."

**`/Users/sac/knhk/MAPE-K_AUTONOMIC_INTEGRATION.md`** — HIGH
Ten MAPE-K autonomic lifecycle hooks: mape:PreMonitor, mape:PostMonitor, mape:PreAnalyze, mape:PostAnalyze, mape:PrePlan, mape:PostPlan, mape:PreExecute, mape:PostExecute, mape:PreFeedback, mape:PostFeedback. These are autonomic lifecycle control points (not knowledge hooks in the Reflex sense).

**`/Users/sac/knhk/CHATMAN_CONSTANT_ENFORCEMENT.md`** — MEDIUM
Enforcement mechanism for ≤8 tick bound. Violation handling.

**`/Users/sac/knhk/DOCTRINE_INDEX.md`** — MEDIUM
Navigation map, file mapping, audience routing, covenant hierarchy.

**`/Users/sac/knhk/ARCHITECTURE.md`** — MEDIUM
Three-tier architecture overview. "KNHK (Knowledge Graph Kernel) is a real-time, distributed system."

**`/Users/sac/knhk/MAPE-K_AUTONOMIC_INTEGRATION.md`** (chatmangpt copy) — MEDIUM
See: `/Users/sac/chatmangpt/knhk/MAPE-K_AUTONOMIC_INTEGRATION.md` — confirmed identical in doctrine content.

### ~/gitvan — GitVan Knowledge Hooks Implementation

**`/Users/sac/gitvan/@UNRDF_HOOKS_ARCHITECTURE_DIAGRAMS.md`** — HIGH
Knowledge Hook Engine, HookOrchestrator, predicate evaluator flow. Turtle hook definition pattern:
```turtle
:preCommitValidator a gh:Hook ;
  gv:title "Pre-commit File Validator" ;
  gh:hasPredicate :fileCheckPredicate ;
  gh:orderedPipelines :validationPipeline .
```
KnowledgeHookRegistry as Turtle-defined hook predicates.

**`/Users/sac/gitvan/@UNRDF_HOOKS_QUICK_REFERENCE.md`** — MEDIUM
Predicate types, component statuses for the GitVan knowledge hooks system.

### ~/chatmangpt/knhk — ChatGPT Conversation Transcripts (Primary Definition Source)

**`/Users/sac/chatmangpt/knhk/yawl.txt`** — HIGH
CRITICAL — formal definition of knowledge hook:
> "In Reflex, knowledge hooks are the compiled interfaces between ontological laws and runtime reconciliation. They are neither functions nor listeners—they are embedded invariants that bind semantic constraints (Σ, Q) directly to data movement and execution."
>
> "A knowledge hook = (predicate, guard, action) triple generated from Σ. It enforces an invariant Q on every Δ admitted into μ(O). Formally: hook(p, q, a): Δ ⊨ Qp ⇒ μ(O ⊔ Δ) = μ(O) ⊔ μ(Δ). Each hook is compiled ahead of time into a branchless kernel that runs within the eight-tick beat."

**`/Users/sac/chatmangpt/knhk/DOCTRINE_2027.md`** — MEDIUM
Confirmed identical copy of `/Users/sac/knhk/DOCTRINE_2027.md`.

### ~/gitvan-backup and gitvan-recent-changes-backup (Implementation Suite)

**`/Users/sac/gitvan-recent-changes-backup-20250919-091930/KNOWLEDGE-HOOKS-END-TO-END-VERIFICATION-REPORT.md`** — HIGH
17/17 tests passed; 8 predicate types fully implemented; 21/21 Git operations covered. Binary admission/refusal per ASK predicate confirmed.

**`/Users/sac/gitvan-recent-changes-backup-20250919-091930/GITVAN-HOOKS-ARCHITECTURE-AUDIT-REPORT.md`** — HIGH
Explicit separation: traditional Git hooks (bypass KH system) vs. Knowledge Hook system (SPARQL predicates). Goal: pure Knowledge Hook architecture.

**`/Users/sac/gitvan-recent-changes-backup-20250919-091930/src/hooks/HookOrchestrator.mjs`** — HIGH
Production source. Complete lifecycle: initialize RDF → load previous state → parse hooks → evaluate predicate → plan DAG → execute steps → persist via GitNativeIO. This is attempt → hook → admission/refusal → durable motion → receipt in working code.

**`/Users/sac/gitvan-backup-20250918-164245/GIT-HOOKS-SIGNALS-KNOWLEDGE-HOOKS-ARCHITECTURE.md`** — HIGH
Two-layer architecture: Git hooks = signals (WHEN); Knowledge hooks = intelligence (SPARQL, WHAT). Architecture: `Git Operation → Git Hook Signal → Knowledge Hook Evaluation → SPARQL Predicate → Workflow Execution`.

**`/Users/sac/gitvan-backup-20250918-164245/knowledge-hooks-suite/README.md`** — MEDIUM
12/21 Git lifecycle operations; Turtle-defined hook predicates; 9 hook implementation files.

**`/Users/sac/gitvan-recent-changes-backup-20250919-091930/KNOWLEDGE-HOOKS-GAPS-IMPLEMENTATION-COMPLETE-REPORT.md`** — MEDIUM
All 8 predicate types completed: ASK, SELECTThreshold, ResultDelta, SHACLAllConform, CONSTRUCT, DESCRIBE, Federated, Temporal.

**`/Users/sac/gitvan-recent-changes-backup-20250919-091930/DEVELOPER-CENTRIC-KNOWLEDGE-HOOKS-ARCHITECTURE.md`** — MEDIUM
Scrum-at-Scale cadence encoded as SPARQL predicates in TTL hooks.

---

## Research Cluster Sources

### process-intelligence — Primary AKA Doctrine Layer

**`/Users/sac/process-intelligence/doctrine/AUTONOMIC_KNOWLEDGE_ACTUATION.md`** — HIGH
Primary AKA doctrine. Defines MAPE-K with typed observations and receipt-bearing actuation. Receipt structure: `BLAKE3(action || pre_state || post_state || timestamp || elastic_subnet_proof)`. "Every execution action emits a receipt. No silent actuation. Executions that do not emit receipts are not closures. They are narration." Distinguishes elastic subnet (autonomous authority) from compliance subnet (executive authority — FROZEN).

**`/Users/sac/process-intelligence/doctrine/autonomic-knowledge-actuation.md`** — HIGH
Mathematical AKA formalization. Typestate transitions: `transition : State(s1, Proof(s1)) → Transition(t) → Option(State(s2, Proof(s2)))` — output is None (compiler failure) if safety invariants violated. T_elastic / T_compliance partition. Commits-as-receipts schema: "A git commit is treated as a manufacturing transition τ = (c, ρ) where c is the commit hash and ρ is a receipt class."

**`/Users/sac/process-intelligence/doctrine/spr_thesis_actuation.md`** — HIGH
SPR thesis for full-lifecycle AKA. Defines the actuation pipeline: `knowledge → actuation boundary → typed transition → admissible condition → refusal rule → repair route → receipt → replay → decommissioning law`.

**`/Users/sac/process-intelligence/doctrine/PROCESS_INTELLIGENCE_SPR_THESIS.md`** — HIGH
Operator chain semantics: `kappa(rho(alpha(mu(O*)))) → ALIVE | PARTIAL | REFUSED`. Defines: μ(O*) = manufacture; α(μ(O*)) = actuate knowledge; ρ(α(μ(O*))) = emit evidence; κ(ρ(α(μ(O*)))) = gate evidence.

**`/Users/sac/process-intelligence/phd-thesis/research/knowledge-hooks/KNOWLEDGE_HOOKS_AND_AKA_DOCTRINE.md`** — HIGH
Primary synthesis document. Key definitions:
- "A knowledge hook is a deterministic admission/refusal boundary that consumes an attempted motion, evaluates it against admissible process law, emits a decision, and manufactures the first durable proof that motion occurred or was refused."
- "Every knowledge hook produces exactly one of three typed outcomes: ADMIT(R), REFUSE(F), or PARTIAL(X)."
- "A hook that produces no decision is not a hook — it is narration."
- "A lifecycle transition without a hook firing is not a lawful transition; it is noise."
- "Knowledge that cannot actuate is documentation. Documentation is PARTIAL."
Full GALL test battery: GALL-H (hook fires / does not fire), GALL-R (valid/invalid receipt), GALL-RP (replay match/divergence), GALL-S (sabotage refusal).

**`/Users/sac/process-intelligence/phd-thesis/research/knowledge-hooks/01_hook_definition_map.md`** — HIGH
Hook definition cartography.

**`/Users/sac/process-intelligence/phd-thesis/research/knowledge-hooks/02_aka_definition_map.md`** — HIGH
AKA definition cartography. "Autonomic Knowledge Actuation = Knowledge → Proof → Consequence → Knowledge."

**`/Users/sac/process-intelligence/phd-thesis/research/knowledge-hooks/03_autoinstinct_lineage_map.md`** — HIGH
AutoInstinct lineage map.

**`/Users/sac/process-intelligence/phd-thesis/research/knowledge-hooks/04_construct8_motion_boundary_map.md`** — HIGH
CONSTRUCT8 motion boundary map. "No receipt, no authority" verified at line 167.

**`/Users/sac/process-intelligence/phd-thesis/research/knowledge-hooks/05_frame_preservation_audit.md`** — HIGH
Frame law audit. Confirms: AKA ≠ automation; Report ≠ proof; No receipt, no authority. Hooks are "Claude Code Andon gates — deterministic enforcement points that enforce admission/refusal conditions at named lifecycle boundaries."

**`/Users/sac/process-intelligence/audits/05_frame_preservation_audit.md`** — HIGH
Same frame law audit, repo-level copy. Validates Frame Laws 1–7 against corpus.

**`/Users/sac/process-intelligence/doctrine/PROCESS_TRUTH_AUTHORITY.md`** — MEDIUM
Direction chain: `→ 8-bit bounded state → typed admission/refusal → external-witness mapping → GALL growth → receipts/replay`.

**`/Users/sac/process-intelligence/doctrine/BLUE_RIVER_DAM.md`** — MEDIUM
Upstream closure law. AKA as the actuation mechanism bounded by the Dam.

**`/Users/sac/process-intelligence/doctrine/DOWNSTREAM_AUTHORIZATION_LAW.md`** — MEDIUM
Gate requirement: kappa operator chain with receipt requirement for each lifecycle stage.

**`/Users/sac/process-intelligence/standards/public_standards_to_lifecycle_actuation.md`** — MEDIUM
Maps public standards (XES, OCEL, BPMN) to MAPE-K autonomic control loops.

**`/Users/sac/process-intelligence/standards/MAPE_K_INTEGRATION.md`** — HIGH
Standard integration specification for MAPE-K in the research program.

**`/Users/sac/process-intelligence/COVENANT.md`** — MEDIUM
"Autonomic knowledge actuation: MAPE-K closed-loop control doctrine" listed as canonical claim.

### ggen — Rust/Python Implementation Layer

**`/Users/sac/ggen/crates/ggen-graph/src/graph/dataset.rs`** — HIGH
Rust `KnowledgeHook` struct definition:
```rust
pub struct KnowledgeHook {
    pub name: String,
    pub sparql_query: String,  // SPARQL ASK or SELECT query defining the constraint
}
```
Implements `execute()` against a `DeterministicGraph`.

**`/Users/sac/ggen/crates/ggen-graph/src/receipt/mod.rs`** — HIGH
`HookReceipt` struct: "Cryptographic receipt for the execution and validation of a specific knowledge hook." Uses BLAKE3.

**`/Users/sac/ggen/crates/ggen-graph/src/prelude.rs`** — HIGH
Exports `KnowledgeHook`, `TransitionReceipt`, `HookReceipt` as core public API.

**`/Users/sac/ggen/ORIGINAL_REQUEST.md`** — HIGH
GALL protocol definition: Witnessed Agent Truthfulness using "Witnessed Code Evaluation / Knowledge Hook Actuation" over OCEL/PROV/RDF evidence. Specifies hook-pack triggers, receipt-bound decision deltas, sabotage fixtures.

**`/Users/sac/ggen/crates/ggen-cli/tests/gall_sync_actuation.rs`** — HIGH
Live GALL actuation test — file name directly encodes hook → actuation direction.

**`/Users/sac/ggen/examples/chatman-equation-paper.rdf`** — HIGH
Formal paper: "The core mechanism is the knowledge hook: a policy-bound program that detects changes in a knowledge graph, evaluates invariants, and triggers workflow actions with cryptographic receipts." Defines: "A knowledge hook h is the atomic unit of knowledge work."

**`/Users/sac/ggen/examples/chatman-equation-paper-ENHANCED-v3.rdf`** — MEDIUM
Quantifies dark matter eliminated by knowledge hook deployment.

**`/Users/sac/ggen/fusion-thesis/chapters/ch06_autonomic_swarms.tex`** — HIGH
Chapter on autonomic swarms — directly references KNHK/AKA doctrine.

**`/Users/sac/ggen/docs/PUBLIC_ONTOLOGY_GOVERNANCE.md`** — MEDIUM
"Knowledge hook governance system implemented in the ggen-graph substrate."

**`/Users/sac/ggen/docs/diataxis/tutorials.md`** — MEDIUM
"Enforce safety checks via knowledge hook validation, and verify the resulting state transitions using cryptographic receipts."

### truex — Law Manifesto + Kernel

**`/Users/sac/truex/docs/MANIFESTO.md`** — HIGH
Primary source for the "No hook, no consequence" compression frame law verbatim (appears twice):
```
No hook, no consequence.
No receipt, no authority.
No replay, no substrate.
No accounting, no promotion.
```
"The receipt is not decoration. The receipt is the authority surface." Defines Truex as "a system of conserved consequence."

**`/Users/sac/truex/docs/reports/project-status/KNHK.md`** — HIGH
"Hook-based architecture triggers an autonomic line-stop if graph invariants are violated or if a cyclic hook storm is detected." Confirms hooks as Jidoka/Andon stops.

**`/Users/sac/truex/docs/vision2030/EXPERIMENTAL-CORPUS-LESSONS.md`** — HIGH
"knowd proved the need for policy hooks and cryptographic receipts at the database layer. knhk proved the necessity of ≤2ns latency for hot-path operations."

**`/Users/sac/truex/docs/vision2030/projects/KNHK.md`** — HIGH
"KNHK is a projection of the receipted substrate; it is forbidden from mutating the raw state directly." `knhk.reconstruct(receipts)` as replay interface.

**`/Users/sac/truex/docs/vision2030/VISION-2030.md`** — MEDIUM
"An agent proposes a move. The Membrane intercepts it. KNHK checks permissions. Prolog8 checks policy. [...] UNIBIT stamps the receipt."

**`/Users/sac/truex/docs/vision2030/TRUEX-EVOLUTION-MATRIX.md`** — MEDIUM
"KNHK: Will become the backing store for the Virtual Knowledge Graph Runtime (VKG-HR), calculating avatar-relative projections dynamically in nanoseconds."

### wasm4pm — KNHK Execution Port

**`/Users/sac/wasm4pm/docs_quarantine/ARCHIVE/docs/vision-2030-hyperthesis.md`** — HIGH
"Knowledge Hooks thesis" as one of 7 convergent pillars. "μ decomposes it (Knowledge Hooks) — 8 information-theoretic operators, each reducing intent entropy by ~6.1 nats." "Remove Knowledge Hooks and the transformation is monolithic."

**`/Users/sac/wasm4pm/apps/wasm4pm/src/commands/doctor.ts`** — HIGH
Warning: "No hooks wired in .claude/settings.json — TPS enforcement gates inactive." Confirms hooks are enforcement gates, not automations.

**`/Users/sac/wasm4pm/docs_quarantine/ARCHIVE/docs/thesis-operational-autonomy-wasm.md`** — MEDIUM
"Porting algorithms from the knhk knowledge-graph engine... embedding operational autonomy directly into the algorithm substrate."

### pcp — TypeScript Hook-OTP Actor System

**`/Users/sac/pcp/src/lib/truex/hook-otp/`** — HIGH
Complete hook actor system (8 files): `types.ts` (HookMessageType: graph_delta | receipt_event | replay_request | supervisor_signal), `receipts.ts`, `replay.ts`, `supervisor.ts`, `registry.ts`, `mailbox.ts`, `behavior.ts`.

**`/Users/sac/pcp/src/framework/membrane/membrane.ts`** — HIGH
`ReceiptManager` with `emitRefusal()` and chained receipt generation. Every action returns `{ success, result, receipt }`.

**`/Users/sac/pcp/src/framework/README.md`** — HIGH
"Every action (A) must be accompanied by a cryptographic receipt (R) proving safety, provenance, and conformance."

**`/Users/sac/pcp/src/lib/truex/evidence/ocel.ts`** — HIGH
`exportToOcel(receipts: HookReceipt[])` — converts hook receipts to OCEL event log. Confirms receipts → OCEL evidence pipeline.

### clap-noun-verb — Research Suite Mirror

**`/Users/sac/clap-noun-verb/phd-thesis/research/knowledge-hooks/02_aka_definition_map.md`** — HIGH
Mirror of AKA definition cartography. "Autonomic Knowledge Actuation = Knowledge → Proof → Consequence → Knowledge." Explicitly distinguishes AKA from automation and lifecycle management.

**`/Users/sac/clap-noun-verb/phd-thesis/research/knowledge-hooks/01_hook_definition_map.md`** — HIGH
Mirror hook definition map.

**`/Users/sac/clap-noun-verb/evidence_graph.json`** — HIGH
Evidence graph nodes: `src/kernel/execution_receipts.rs` — "Attested execution receipts: proof of invocation, capability attestation, quota tracking, hash linkage." `src/autonomic/receipts.rs` — "Receipt generation: timing data, evidence collection, linked to parent receipts via hash (Γ)."

### open-ontologies — Actuation Membrane

**`/Users/sac/open-ontologies/src/actuation.rs`** — HIGH
Defines `ActuationPlan`, `ActuationResult`, `capture_observed_ocel()`. Confirms actuation → OCEL evidence pipeline.

**`/Users/sac/open-ontologies/GEMINI.md`** — HIGH
"Core Mandates for System Actuation and Proof." Defines `UnboundedActuation` as a defect. "Gemini CLI must act as the actuation membrane, executing only what is defined in the ontology."

**`/Users/sac/open-ontologies/src/ocel_store.rs`** — MEDIUM
`// Mirrors admission::A13_BETWEEN_SNAPSHOT_HOOK (R5 WB-1)` — admission hooks mapped to OCEL store.

### insa — Byte-Speed Substrate

**`/Users/sac/insa/CLAUDE.md`** — HIGH
"Byte-speed system manufacturing lawful instinct from closed operational fields (TCPS + INSA Byte Law)." A = μ(O*) formally stated. Byte-lane family: INST8, KAPPA8, Family8, POWL8, CONSTRUCT8.

**`/Users/sac/insa/AGENTS.md`** — HIGH
"Do not emit without proof: Unproofed emission is structurally forbidden." "Do not report without replay: Board/security reports are derived from POWL64 replayable evidence, not generated prose." "Do not let projection results mutate state: They must re-enter as Observation → CONSTRUCT8 → O*."

### mcpp — Receipt Chain Management

**`/Users/sac/mcpp/CLAUDE.md`** — HIGH
Full receipt lifecycle: `mcpp receipt list/show/verify/replay`. `mcpp ggen verify` — verifies BLAKE3 chain continuity; Andon pull on break. "Accept emits exactly 1 receipt; Refuse emits 0. LIVE-03 enforces."

### zoeapp — Truex Hooks Bridge

**`/Users/sac/zoeapp/docs/vision2030/truex-collaborative-intelligence.md`** — HIGH
"Knowledge hooks become Truex Hooks." Truex Autonomics: "lifecycle reflexes: threshold incoming reality, classify actors, construct O*, compile route obligations, enforce Field8 surfaces, require OCEL evidence, emit receipts, trigger replay, and evolve future route law."

---

## Extended Corpus Sources

### compiled-cognition-hub

**`/Users/sac/compiled-cognition-hub/governance/PHILOSOPHY.md`** — MEDIUM
Oracle vs. Angel framing: "Intelligence stops being a service and becomes a deterministic, zero-dependency property of the binary." Confirms: LLM output is NOT runtime authority.

**`/Users/sac/compiled-cognition-hub/src/main.rs`** — LOW
`use unibit_graph::construct8_to_oxigraph` — CONSTRUCT8 bridge to Oxigraph RDF store.

### chatmangpt (non-knhk)

**`/Users/sac/chatmangpt/knhk/KNHK_2027_PRESS_RELEASE.md`** — MEDIUM
"A = µ(O) in Production: How Rust Became the Control Plane for Fortune 500 Ontologies." Constraints: "No dynamic configuration inside the kernel, No unbounded recursion, No allocation on the hot path." Confirms LLM output is NOT runtime authority.

### Academic Papers

**`/Users/sac/Documents/Papers/workflow/`** — MEDIUM (academic citations only)
20+ process mining / YAWL papers. Relevant titles: YAWL (Van der Aalst), workflow-patterns-the-definitive-guide, OCPQ, PM4Py, Object-Centric Analysis of XES, PMAx, Compliance-Aware Predictive Process Monitoring. No direct "knowledge hook" content — these are academic backing for the conformance checking layer.

### Null Roots (searched, no signal)

The following roots were searched by Agent 3 and contain no knowledge hook or AKA signal:
- `/Users/sac/cell8`, `/Users/sac/coordination`, `/Users/sac/obsr`, `/Users/sac/seth`
- `/Users/sac/SparsePrimingRepresentations`, `/Users/sac/claude-desktop-context`, `/Users/sac/memory`
- `/Users/sac/knowd`, `/Users/sac/knowtro`, `/Users/sac/kgc-sidecar`, `/Users/sac/kgn`
- `/Users/sac/ultrathink-bpm-engine`, `/Users/sac/claude`, `/Users/sac/clawd`, `/Users/sac/clawdbot`

Repos with no signal from Agent 2:
- `/Users/sac/speckit-ralph`, `/Users/sac/universe-chain`, `/Users/sac/A2A`, `/Users/sac/zoela`, `/Users/sac/bytestar`, `/Users/sac/cns`

---

## Top 20 High-Relevance Sources (Ranked)

Ranked by: authoritative definition density, unique evidence contribution, and citation value for thesis.

| Rank | File | Project | Reason |
|------|------|---------|--------|
| 1 | `/Users/sac/chatmangpt/knhk/yawl.txt` | knhk | ONLY source with formal mathematical definition of knowledge hook as (predicate, guard, action) triple with Δ ⊨ Qp formula |
| 2 | `/Users/sac/process-intelligence/phd-thesis/research/knowledge-hooks/KNOWLEDGE_HOOKS_AND_AKA_DOCTRINE.md` | process-intelligence | Full hook definition, AKA lifecycle, GALL test battery, "narration" anti-law |
| 3 | `/Users/sac/knhk/V30_1_1_MANIFESTO.md` | knhk | Final Manifesto verbatim: all eight precedence laws + First Law + Refusal Is Integrity + Receipts Replace Logs |
| 4 | `/Users/sac/truex/docs/MANIFESTO.md` | truex | "No hook, no consequence" compression law — verbatim, twice |
| 5 | `/Users/sac/knhk/DOCTRINE_2027.md` | knhk | Canonical: "MAPE-K embedded as knowledge hooks" — ONLY verbatim use of "knowledge hooks" in doctrine |
| 6 | `/Users/sac/knhk/GENESIS_CORE_SPECIFICATION.md` | knhk | Primary Rust struct definitions: Receipt, Refusal, Construct8, Pair2, AdmissionGate |
| 7 | `/Users/sac/process-intelligence/doctrine/AUTONOMIC_KNOWLEDGE_ACTUATION.md` | process-intelligence | Primary AKA doctrine: MAPE-K, receipt structure, elastic/compliance partition |
| 8 | `/Users/sac/ggen/examples/chatman-equation-paper.rdf` | ggen | Formal paper: "knowledge hook h is the atomic unit of knowledge work" |
| 9 | `/Users/sac/knhk/PART_SPEC.md` | knhk | Full Truex 6-phase lifecycle with admission gate detail |
| 10 | `/Users/sac/process-intelligence/doctrine/spr_thesis_actuation.md` | process-intelligence | Full AKA actuation pipeline verbatim |
| 11 | `/Users/sac/process-intelligence/doctrine/PROCESS_INTELLIGENCE_SPR_THESIS.md` | process-intelligence | kappa(rho(alpha(mu(O*)))) operator chain |
| 12 | `/Users/sac/gitvan-recent-changes-backup-20250919-091930/src/hooks/HookOrchestrator.mjs` | gitvan | Production source: complete lifecycle in working code |
| 13 | `/Users/sac/gitvan-backup-20250918-164245/GIT-HOOKS-SIGNALS-KNOWLEDGE-HOOKS-ARCHITECTURE.md` | gitvan | Two-layer architecture: signals vs. intelligence |
| 14 | `/Users/sac/ggen/crates/ggen-graph/src/graph/dataset.rs` | ggen | KnowledgeHook Rust implementation with execute() |
| 15 | `/Users/sac/ggen/crates/ggen-graph/src/receipt/mod.rs` | ggen | HookReceipt BLAKE3 implementation |
| 16 | `/Users/sac/knhk/KNHK_PHD_THESIS.md` | knhk | Primary academic dissertation — MAPE-K, Chatman Equation, Covenant 3/6, Receipt glossary |
| 17 | `/Users/sac/pcp/src/lib/truex/hook-otp/` | pcp | Hook-OTP actor system: full TypeScript implementation (8 files) |
| 18 | `/Users/sac/process-intelligence/doctrine/autonomic-knowledge-actuation.md` | process-intelligence | Mathematical typestate formalization; commits-as-receipts |
| 19 | `/Users/sac/knhk/DOCTRINE_COVENANT.md` | knhk | Six binding covenants with anti-patterns and validation |
| 20 | `/Users/sac/insa/AGENTS.md` | insa | "Unproofed emission is structurally forbidden" — operating contract |

---

## Key Definitions (Verbatim)

### Knowledge Hook — Formal Definition
SOURCE: `/Users/sac/chatmangpt/knhk/yawl.txt`
> "A knowledge hook = (predicate, guard, action) triple generated from Σ. It enforces an invariant Q on every Δ admitted into μ(O). Formally: hook(p, q, a): Δ ⊨ Qp ⇒ μ(O ⊔ Δ) = μ(O) ⊔ μ(Δ). Each hook is compiled ahead of time into a branchless kernel that runs within the eight-tick beat."

### Knowledge Hook — Process Lifecycle Definition
SOURCE: `/Users/sac/process-intelligence/phd-thesis/research/knowledge-hooks/KNOWLEDGE_HOOKS_AND_AKA_DOCTRINE.md`
> "A knowledge hook is a deterministic admission/refusal boundary that consumes an attempted motion, evaluates it against admissible process law, emits a decision, and manufactures the first durable proof that motion occurred or was refused."

### Autonomic Knowledge Actuation (AKA)
SOURCE: `/Users/sac/process-intelligence/doctrine/AUTONOMIC_KNOWLEDGE_ACTUATION.md`
> "AKA is the principle that verified truth must actuate itself instantly, manifesting system state changes without intermediary validation."
> "AKA is not monitoring. It is not dashboards. It is not alerting."
> "Knowledge retrieval is looking up what you know. Knowledge actuation is making what you know consequential."

### Receipt
SOURCE: `/Users/sac/knhk/V30_1_1_MANIFESTO.md`
> "Logs are observation. Receipts are institutional memory."

SOURCE: `/Users/sac/knhk/GENESIS_CORE_SPECIFICATION.md`
> "Receipt: Deterministic proof of consequence (not telemetry)"

### The Frame Law Chain
SOURCE: `/Users/sac/truex/docs/MANIFESTO.md`
> "No hook, no consequence. No receipt, no authority. No replay, no substrate. No accounting, no promotion."
