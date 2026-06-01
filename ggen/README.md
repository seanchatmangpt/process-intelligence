# ggen — Governance Generation Engine

**Substrate for board-admissible M&A deck and Blue River orchestrator**

ggen is a configuration-driven artifact generation system built on SPARQL queries and Tera templates. It transforms process intelligence ontology into three critical outputs:

1. **M&A Deck (PowerPoint)** — Board-admissible acquisition presentation
2. **Diligence Workbook (Excel)** — Detailed claims with replay evidence  
3. **Blue River Orchestrator (Rust)** — Autonomic MAPE-K governance engine

---

## Architecture

```
ggen/
├── ggen.toml              # Generation rules config
├── queries/
│   ├── extract-board-claims.rq           # Select board-admissible M&A claims
│   ├── extract-diligence-claims.rq       # Select synergy/debt/risk claims
│   └── extract-lifecycle-governance.rq   # Select MAPE-K rules and state transitions
├── templates/
│   ├── ma-deck.tera                      # PowerPoint JSON structure
│   ├── ma-diligence.tera                 # Excel workbook structure
│   └── blue-river.tera                   # Rust orchestrator with MAPE-K loop
├── ontology-extensions.ttl               # RDF extensions for board claims & lifecycle states
└── README.md
```

---

## Configuration: ggen.toml

The root configuration file defines generation rules that map SPARQL queries → Tera templates → output artifacts.

```toml
[[generation.rules]]
name = "ma-deck-powerpoint"
query = { file = "queries/extract-board-claims.rq" }
template = { file = "templates/ma-deck.tera" }
output_file = "../ma/acquisition_ready_deck_FINAL.pptx"
mode = "Overwrite"
```

Each rule:
- **name** — Unique rule identifier
- **query** — SPARQL file that extracts data from process intelligence ontology
- **template** — Tera file that renders the artifact
- **output_file** — Where to write the artifact (relative to ggen/)
- **mode** — Overwrite, Append, or Merge

---

## SPARQL Queries

### extract-board-claims.rq

Selects all board-admissible claims backed by wasm4pm conformance verdicts.

**Filters:**
- Claim type: SynergyProjection, OperationalDebtClaim, IntegrationRiskClaim, ProcessAssetClaim, ControlClaim
- Fitness >= 0.95, Precision >= 0.90 (board admissibility thresholds)
- Must have cryptographic receipt for audit trail
- Log format: OCEL 2.0 or XES (IEEE 1849-2016)

**Outputs:**
- claimLabel, claimType, metricValue, metricThreshold
- verdictFitness, verdictPrecision
- receiptHash, receiptTimestamp
- backedByLog (with format)

### extract-diligence-claims.rq

Selects detailed claims across synergy/debt/risk categories with replay traces and remediation paths.

**Extends board query with:**
- Synergy category (CostReduction, RevenueUplift, EfficiencyGain)
- Operational debt category and remediation effort hours
- Risk severity (High/Medium/Low)
- Replay trace deviations and gas-to-return costs
- Affected activities and bottleneck indicators

**Outputs:**
- quantifiedMetric, metricValue, metricUnit
- replayTrace, traceDeviations, traceGasToReturn
- remediationPath, remediationEffortHours
- relatedActivity, activityBottleneck

### extract-lifecycle-governance.rq

Selects MAPE-K rules and state transitions for the Blue River orchestrator.

**Covers lifecycle spectrum:**
- DesignPhase, SimulationPhase, ValidationPhase, MonitoringPhase, OptimizationPhase, RepairPhase, DecommissionPhase

**Extracts per state:**
- Allowed transitions with guard conditions and actions
- MAPE-K Monitor rules: what observations trigger analysis
- MAPE-K Analyze rules: what patterns to detect and thresholds
- MAPE-K Plan rules: remediation policies and output shapes
- MAPE-K Execute actions: allowed interventions
- MAPE-K Knowledge assets: learned models and strategies

**Outputs:**
- state, stateName, statePhase, stateDescription
- transitionTarget, transitionGuardCondition, transitionGuardExpression
- monitorRule, analyzeRule, planRule, executeAction, knowledgeAsset

---

## Tera Templates

### ma-deck.tera

Renders PowerPoint-ready JSON structure (for pptx-rs library).

**Slides generated:**
1. **Title Slide** — Presentation metadata and confidentiality marking
2. **Executive Summary** — High-level metrics (claim count, total value, average fitness)
3. **Claim Detail Slides** — One slide per board claim:
   - Claim label and category
   - Quantified metric (value, unit, threshold)
   - Conformance section (fitness %, precision %, status: ADMISSIBLE/REJECTED)
   - Cryptographic receipt (hash, timestamp, verification URL)
   - Event log backing (format, conformance verdict ID)
4. **Operational Debt Slide** — Aggregated debt inventory with remediation hours
5. **Synergy Waterfall Slide** — Waterfall chart of synergy value realization
6. **Conformance Model Summary** — Framework (soundness, provenance, alignment, aggregates)
7. **Board Sign-Off Slide** — Five declarations of admissibility (with timestamp and UUID)

**Each slide references:**
- wasm4pm ConformanceVerdicts (fitness, precision)
- Cryptographic receipts for verification
- Event log provenance (OCEL/XES format)
- Board admissibility framework (Adriansyah optimal alignments, van der Aalst soundness)

### ma-diligence.tera

Renders Excel workbook with multiple sheets (JSON structure for xlsx library).

**Sheets generated:**
1. **Executive_Summary** — Aggregate metrics (total claims, total value, average fitness/precision)
2. **Synergy_Claims** — Synergy projections with category, annual value, realization phase, fitness scores, receipts
3. **Operational_Debt** — Debt inventory with remediation hours, cost estimates, affected activities, priorities
4. **Integration_Risks** — Risk assessment with severity, probability, mitigation strategy, timeline, responsible function
5. **Replay_Traces** — Detailed conformance evidence (deviations, gas-to-return, fitness/precision calculated, repeatable status)
6. **Activity_Impact** — Per-activity rollup (related claims, bottleneck status, total impact, optimization opportunities)
7. **Governance** — Compliance checklist (event log standards, cryptographic chaining, fitness thresholds, soundness proofs, receipts in data room)

Each row is traceable back to SPARQL query results and linked to conformance verdicts.

### blue-river.tera

Generates Rust orchestrator with MAPE-K loop and lifecycle state machine.

**Code structure:**
- **LifecycleState enum** — States: Design, Simulation, Validation, Monitoring, Optimization, Repair, Decommission
- **StateDefinition struct** — Container for transitions, monitor/analyze/plan/execute/knowledge rules
- **MAPE-K phases as methods:**
  - `monitor()` — Record metric observation
  - `analyze()` — Evaluate analyze rules, detect anomalies
  - `plan()` — Apply plan rules to formulate remediation
  - `execute()` — Fire allowed action in current state, record as OCEL event
  - `transition()` — Check guard and move to target state
- **BlueRiverOrchestrator struct** — Main loop that orchestrates MAPE-K cycle
- **Testing module** — Basic MAPE-K cycle verification

**Generated rules per state:**
- TransitionGuard with condition name and SPARQL expression
- MonitorRule with observed metric and trigger expression
- AnalyzeRule with pattern and threshold
- PlanRule with policy and output shape
- ExecuteAction with name, description, audit log reference
- KnowledgeAsset (ProcessModel, ConformancePattern, RemediationStrategy, PredictiveModel, AutonomicPolicy)

---

## Ontology Extensions: ontology-extensions.ttl

RDF/Turtle ontology that extends the wasm4pm-compat type system with:

### Board Claims (ma: namespace)
- `ma:BoardClaim` — Abstract base for all M&A claims
  - `ma:SynergyProjection` — Cost reduction, revenue uplift, efficiency gain
  - `ma:OperationalDebtClaim` — Process debt, architectural debt, manual intervention debt
  - `ma:IntegrationRiskClaim` — Post-close process hazards (High/Medium/Low severity)
  - `ma:ProcessAssetClaim` — Positive capabilities and automation
  - `ma:ControlClaim` — Effective control operation

**Properties:**
- `ma:backedBy` — Links to wasm4pm ConformanceVerdict (fitness >= 0.95, precision >= 0.90)
- `ma:supportedBy` — Links to wasm4pm ReplayTrace (deviations, alignment cost)
- `ma:evidencedBy` — Links to conformance verdict
- `ma:quantifies` — Links to Metric (value, unit, threshold)
- `ma:hasRemediationPath` — Optional RemediationStrategy with estimated hours

### Lifecycle States (lifecycle: namespace)
- `lifecycle:ProcessState` — Disjoint union of 7 states
  - `lifecycle:DesignState`, `SimulationState`, `ValidationState`, etc.

**Properties:**
- `lifecycle:transitions` — Allowed state transitions with guards
- `lifecycle:monitorRule` — Monitor rules (observe metric, expression)
- `lifecycle:analyzeRule` — Analyze rules (pattern, threshold)
- `lifecycle:planRule` — Plan rules (policy, output shape)
- `lifecycle:canExecute` — Allowed execute actions
- `lifecycle:knowledgeSource` — Knowledge assets (models, patterns, strategies)

### Conformance Evidence (compat: namespace)
- `compat:Evidence` — Typed container Evidence<T, State, W>
  - claimContent (typed claim)
  - stateMarker (process state)
  - witness (cryptographic proof)

---

## Execution Flow

```
1. Load ggen.toml
   ↓
2. For each generation rule:
   ├─ Load SPARQL query from file
   ├─ Execute against process intelligence ontology
   │  (or in-memory RDF store with pre-loaded facts)
   ├─ Get result set (claims, metrics, verdicts, receipts)
   │
   ├─ Load Tera template from file
   ├─ Render template with SPARQL results
   │  (Tera iterates over claims, builds slides/sheets/code)
   │
   └─ Write artifact to output_file
      (ma/acquisition_ready_deck_FINAL.pptx, etc.)
```

---

## Board Admissibility Contract

Every artifact produced by ggen enforces the board admissibility framework:

### Pillar 1: Event Log Integrity & Provenance
- Logs conform to IEEE 1849-2016 (XES) or OCEL 2.0
- BLAKE3 cryptographic hash chain with system signatures at transaction commit
- Provenance model (W3C PROV-O) traces each event back to source transaction

### Pillar 2: Mathematical Conformance Bounds
- **Fitness >= 0.95** — Optimal alignment-based measure of how much log behavior the model replays
- **Precision >= 0.90** — How much model-allowed behavior is observed in the log
- **Generalization >= 0.85** (optional)
- **Simplicity >= 0.80** (optional)

### Pillar 3: Structural Model Soundness
- Workflow Net proven sound under van der Aalst 1998:
  - Option to Complete: from any reachable marking, can reach final marking
  - Liveness: no dead transitions
  - Boundedness: no infinite token accumulation

### Pillar 4: Cryptographic Slide-to-Receipt Mapping
- Every slide in the deck maps to a unique receipt
- Receipt generated by wasm4pm execution core after running validation query
- Receipt contains BLAKE3 hash, timestamp, and system signature
- Board can verify claim by repeating query and comparing fitness score

---

## Integration Points

### Input: Process Intelligence Ontology
- Assumes RDF/Turtle facts already loaded in knowledge base
- Facts sourced from:
  - wasm4pm conformance verdicts (fitness, precision, receipts)
  - Receipt store (`../receipts/`)
  - Checkpoint exports (`../checkpoints/`)
  - M&A taxonomy definitions (`../ma/define_*.md` encoded as RDF)

### Output Artifacts
- **ma/acquisition_ready_deck_FINAL.pptx** — PowerPoint ready for board
- **ma/diligence_workbook.xlsx** — Excel with detailed breakdowns
- **blue_river_dam/src/lib.rs** — Rust orchestrator (compile with `cargo make`)

### Downstream Consumers
- Investment committee uses deck for board approval
- Financial advisors use workbook for due diligence
- Process intelligence runtime links blue-river orchestrator to lifecycle controller

---

## Usage Example

```bash
# Assuming process intelligence ontology is pre-loaded in RDF store
cd /Users/sac/process-intelligence/ggen

# Generate all artifacts
ggen --config ggen.toml --execute-all

# Or generate single artifact
ggen --config ggen.toml --rule ma-deck-powerpoint
ggen --config ggen.toml --rule ma-diligence-workbook
ggen --config ggen.toml --rule blue-river-orchestrator

# Verify board admissibility
ggen --config ggen.toml --verify-all
  # Runs conformance query on each receipt
  # Compares re-calculated fitness to slide assertion
  # Reports any audit failures
```

---

## Governance Framework

ggen operates within the board admissibility contract:
- **No unsubstantiated claims** — Every assertion must be backed by a conformance verdict
- **Repeatable verification** — Every receipt can be used to re-run the query and verify the claim
- **Cryptographic proof** — Receipts are signed and hash-chained, preventing tampering
- **Transparency** — Claims, queries, and verdicts are all open for inspection
- **Liability** — False claims are mathematically provable (fitness mismatch is audit failure)

---

## References

- **Board Admissibility Framework** — `/Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md`
- **M&A Taxonomy** — `/Users/sac/process-intelligence/ma/define_*_claim_taxonomy.md`
- **MAPE-K Integration** — `/Users/sac/process-intelligence/standards/MAPE_K_INTEGRATION.md`
- **Lifecycle Process** — `/Users/sac/process-intelligence/doctrine/full-lifecycle-process.md`
- **Blue River Dam** — `/Users/sac/process-intelligence/doctrine/blue-river-dam.md`
