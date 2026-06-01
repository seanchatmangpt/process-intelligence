# ggen Integration Guide

**How the Governance Generation Engine connects to wasm4pm, compat, and Blue River**

---

## Data Flow: From Evidence to Board Assets

```
┌─────────────────────────────────────────────────────────────────┐
│ EXECUTION LAYER (Process Runtime)                               │
│ ┌───────────────────────────────────────────────────────────┐   │
│ │ OTel Traces → OCEL Log → wasm4pm Conformance Engine      │   │
│ │                          (fitness, precision, receipt)    │   │
│ └───────────────────────────────────────────────────────────┘   │
└──────────────────────┬──────────────────────────────────────────┘
                       │ ConformanceVerdict + CryptographicReceipt
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│ KNOWLEDGE LAYER (RDF Ontology)                                  │
│ ┌───────────────────────────────────────────────────────────┐   │
│ │ Process Intelligence Ontology (facts)                     │   │
│ │  - ma:SynergyProjection backedBy ConformanceVerdict      │   │
│ │  - lifecycle:DesignState transitions to SimulationState  │   │
│ │  - compat:Evidence typed with claim, state, witness      │   │
│ └───────────────────────────────────────────────────────────┘   │
└──────────────────────┬──────────────────────────────────────────┘
                       │ RDF facts loaded from receipts/, checkpoints/
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│ GENERATION LAYER (ggen)                                          │
│ ┌───────────────────────────────────────────────────────────┐   │
│ │ 1. SPARQL Query (extract-*.rq)                            │   │
│ │    → Select board claims, diligence facts, governance     │   │
│ │                                                            │   │
│ │ 2. Tera Template (*.tera)                                 │   │
│ │    → Render slides, sheets, Rust code                    │   │
│ │                                                            │   │
│ │ 3. Output Artifact                                        │   │
│ │    → .pptx, .xlsx, .rs file                              │   │
│ └───────────────────────────────────────────────────────────┘   │
└──────────────────────┬──────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│ OUTPUT LAYER (Board-Ready Assets)                               │
│ ├─ ma/acquisition_ready_deck_FINAL.pptx                        │
│ ├─ ma/diligence_workbook.xlsx                                  │
│ └─ blue_river_dam/src/lib.rs                                   │
└─────────────────────────────────────────────────────────────────┘
```

---

## Component Integration

### 1. **wasm4pm** → ggen

wasm4pm (the process intelligence execution core) produces:
- **ConformanceVerdict** — fitness, precision, generalization, simplicity
- **CryptographicReceipt** — BLAKE3 hash chain with system signatures
- **ReplayTrace** — deviations, gas-to-return, optimal alignment cost

These flow into the RDF ontology as facts:

```sparql
# Fact in RDF store (sourced from wasm4pm output)
ex:SynergyProjection_001
  a ma:SynergyProjection ;
  rdfs:label "Accounts Payable Process Automation" ;
  ma:quantifies ex:Metric_5M_Savings ;
  ma:backedBy ex:Verdict_AP_Fitness_0.96 ;
  wasm4pm:supportedBy ex:ReplayTrace_AP_123 .

ex:Verdict_AP_Fitness_0.96
  a wasm4pm:ConformanceVerdict ;
  wasm4pm:fitness 0.96 ;
  wasm4pm:precision 0.92 ;
  wasm4pm:receipt ex:Receipt_AP_hash_123abc .
```

**ggen consumes these facts** via SPARQL queries (extract-board-claims.rq, etc.)

### 2. **compat** → ggen

wasm4pm-compat provides type-safe Evidence structures:

```rust
pub struct Evidence<T, State, W> {
    pub claim: T,           // The claim (ma:BoardClaim)
    pub state: State,       // Process state (lifecycle:ProcessState)
    pub witness: W,         // Cryptographic witness
}
```

ggen's ontology defines:

```ttl
ma:BoardClaim
  rdfs:subClassOf [
    a owl:Restriction ;
    owl:onProperty ma:evidenceContainer ;
    owl:minCardinality 1
  ] .

ma:evidenceContainer
  a owl:ObjectProperty ;
  rdfs:range compat:Evidence ;
  rdfs:comment "Board claim is contained in Evidence<BoardClaim, ProcessState, CryptographicWitness>" .
```

This ensures every board claim is wrapped in a typed, state-marked, witness-proven evidence container.

### 3. **Blue River Dam** → ggen

Blue River is the autonomic process orchestrator. ggen generates its initialization code from the lifecycle state machine.

**Template (blue-river.tera) generates:**
- LifecycleState enum with all 7 states
- StateDefinition with MAPE-K rules per state
- BlueRiverOrchestrator.new() with state definitions loaded from SPARQL results

**Integration point:**
```rust
pub struct BlueRiverOrchestrator {
    pub states: HashMap<LifecycleState, StateDefinition>,
    pub current_context: GovernanceContext,
}

impl BlueRiverOrchestrator {
    pub fn monitor(&mut self, metric_name: &str, value: f64) { ... }
    pub fn analyze(&self) -> Vec<String> { ... }
    pub fn plan(&self) -> Option<String> { ... }
    pub fn execute(&mut self, action: &str) -> bool { ... }
    pub fn transition(&mut self, target: LifecycleState) -> bool { ... }
}
```

Every MAPE-K loop execution:
1. **Monitor** collects metrics
2. **Analyze** matches against rules extracted by ggen
3. **Plan** applies policies extracted by ggen
4. **Execute** fires actions extracted by ggen
5. **Transition** checks guards extracted by ggen

---

## Concrete Example: Accounts Payable Synergy

### Step 1: wasm4pm Produces Verdict

```
OTel Traces (AP process execution)
  → OCEL Log (event log with objects: invoices, payments, approvals)
  → wasm4pm Conformance Check (against AP Petri net model)
  → Verdict: fitness=0.96, precision=0.92
  → Receipt: hash=abc123def456, timestamp=2026-06-01T10:30:00Z, sig=system_sign(...)
```

### Step 2: Facts Loaded into RDF Store

```ttl
# Receipt store exports facts
ex:SynergyAP
  a ma:SynergyProjection ;
  rdfs:label "AP Automation Synergy" ;
  ma:quantifies ex:MetricAP ;
  ma:backedBy ex:VerdictAP ;
  wasm4pm:supportedBy ex:TraceAP .

ex:VerdictAP
  a wasm4pm:ConformanceVerdict ;
  wasm4pm:fitness 0.96 ;
  wasm4pm:precision 0.92 ;
  wasm4pm:receipt ex:ReceiptAP .

ex:ReceiptAP
  a wasm4pm:CryptographicReceipt ;
  wasm4pm:receiptHash "abc123def456..." ;
  wasm4pm:timestamp "2026-06-01T10:30:00Z" .
```

### Step 3: ggen SPARQL Query Extracts

`extract-board-claims.rq` runs:

```sparql
SELECT ?claim ?claimLabel ?metric ?metricValue 
       ?verdict ?verdictFitness ?verdictPrecision
       ?receipt ?receiptHash ?receiptTimestamp
WHERE {
  ?claim a ma:SynergyProjection .
  ?claim rdfs:label ?claimLabel .
  ?claim ma:quantifies ?metric .
  ?metric ma:value ?metricValue .
  ?claim ma:backedBy ?verdict .
  ?verdict a wasm4pm:ConformanceVerdict .
  ?verdict wasm4pm:fitness ?verdictFitness .
  ?verdict wasm4pm:precision ?verdictPrecision .
  ?verdict wasm4pm:receipt ?receipt .
  ?receipt wasm4pm:receiptHash ?receiptHash .
  ?receipt wasm4pm:timestamp ?receiptTimestamp .
  FILTER (?verdictFitness >= 0.95 && ?verdictPrecision >= 0.90)
}
```

**Result:** One row with claim=ex:SynergyAP, fitness=0.96, precision=0.92, receipt hash, timestamp

### Step 4: Tera Template Renders Slide

`ma-deck.tera` gets the SPARQL result and renders:

```json
{
  "type": "claim",
  "claim_id": "SynergyAP",
  "title": "AP Automation Synergy",
  "category": "SynergyProjection",
  "metric": {
    "value": 5000000,
    "unit": "USD"
  },
  "conformance": {
    "fitness": 96.0,
    "precision": 92.0,
    "fitness_threshold": 95,
    "status": "ADMISSIBLE"
  },
  "receipt": {
    "hash": "abc123def456...",
    "timestamp": "2026-06-01T10:30:00Z",
    "verify_url": "https://proof.intelligence/verify/ex:ReceiptAP"
  }
}
```

### Step 5: PowerPoint Rendered

A slide appears in acquisition_ready_deck_FINAL.pptx with:
- **Title:** "AP Automation Synergy"
- **Metric:** $5M USD
- **Fitness:** 96% (green, above 95% threshold)
- **Receipt:** Hash (truncated) + link to verification
- **Legal caveat:** "Board Admissible. Verify receipt at https://proof.intelligence/verify/..."

**Board can verify:** Load receipt, re-run SPARQL query, confirm fitness still 0.96.

---

## Board Admissibility Guarantees

ggen enforces all four pillars:

### Pillar 1: Event Log Integrity
**Implementation in SPARQL:**
```sparql
FILTER (?logFormat IN ("ocel:2.0", "xes:1849-2016"))
```
Every selected fact must reference an OCEL 2.0 or XES log.

### Pillar 2: Mathematical Bounds
**Implementation in SPARQL:**
```sparql
FILTER (?verdictFitness >= 0.95 && ?verdictPrecision >= 0.90)
```
Only verdicts passing thresholds are selected; no exceptions.

### Pillar 3: Model Soundness
**Implementation in template comments:**
```jinja
"soundness_proof": "Workflow Net proven sound: option-to-complete, liveness, boundedness"
```
Templates document the soundness framework; Blue River code enforces transition guards.

### Pillar 4: Cryptographic Slide-to-Receipt
**Implementation in template:**
```jinja
"verify_url": "https://proof.intelligence/verify/" ~ claim.receipt
```
Every slide includes a verification URL; auditors can repeat the query with the receipt.

---

## Workflow: From ggen Output Back to Governance

```
1. Board sees acquisition_ready_deck_FINAL.pptx
   ├─ Slide claims $5M AP Synergy (ADMISSIBLE)
   └─ Includes receipt hash and verification URL

2. Board queries: "Verify this claim"
   ├─ Looks up receipt in data room
   ├─ Repeats ggen conformance query with receipt
   └─ Confirms fitness still 0.96

3. Due diligence team uses diligence_workbook.xlsx
   ├─ Drill into replay traces (see deviations)
   ├─ Check remediation paths (AP process redesign: 200 hours)
   └─ Calculate post-close integration risk

4. Operations team loads blue_river_dam/src/lib.rs
   ├─ Compiles orchestrator with MAPE-K loop
   ├─ Links to Monitoring state (observes AP process metrics)
   ├─ Analyze rules detect fitness deviation (< 0.95)
   ├─ Plan rules recommend escalation
   └─ Execute actions trigger manual review

5. Post-close governance
   ├─ Blue River continuously monitors AP process
   ├─ If fitness drifts, autonomic loop escalates
   ├─ Every escalation logged as OCEL event
   └─ Audit can mine the log to verify conformance held
```

---

## File Interconnections

### ggen.toml References
- `queries/extract-board-claims.rq` — Board slide generation
- `queries/extract-diligence-claims.rq` — Diligence workbook generation
- `queries/extract-lifecycle-governance.rq` — Blue River orchestrator generation
- `templates/ma-deck.tera` — PowerPoint rendering
- `templates/ma-diligence.tera` — Excel rendering
- `templates/blue-river.tera` — Rust code generation
- `ontology-extensions.ttl` — RDF schema for all queries

### SPARQL Queries Reference
- `ontology-extensions.ttl` — Define ma:, lifecycle:, wasm4pm:, compat: namespaces and classes
- `ggen.toml` [metadata.ontology_source] — Points to wasm4pm-compat/compat/src/ontology.rs
- `ggen.toml` [metadata.evidence_sources] — Points to receipts/, checkpoints/ directories

### Tera Templates Reference
- `ggen.toml` — Specifies template engine (Tera), base_dir, autoescape, strict_variables
- SPARQL results — Passed as template context (claims array, states array, etc.)
- `ontology-extensions.ttl` — Informs template structure (e.g., lifecycle states enum)

### Ontology Extensions Reference
- wasm4pm-compat types (Evidence, ConformanceVerdict, Receipt) — Extended with ma:, lifecycle: properties
- Blue River (blue_river_dam/src/lib.rs) — Uses MAPE-K structures defined in ontology

---

## Extending ggen

### Add a New M&A Claim Type

1. **Update ontology-extensions.ttl:**
   ```ttl
   ma:MyNewClaimType
     a owl:Class ;
     rdfs:subClassOf ma:BoardClaim ;
     rdfs:label "My New Claim Type" .
   ```

2. **Update SPARQL query (extract-board-claims.rq):**
   ```sparql
   FILTER (?claimType IN (
     ma:SynergyProjection,
     ...,
     ma:MyNewClaimType
   ))
   ```

3. **Update Tera template (ma-deck.tera) if needed:**
   ```jinja
   {% if claim.claimType == "ma:MyNewClaimType" %}
     [render custom slide]
   {% endif %}
   ```

4. **Add RDF facts** to receipt store (or checkpoint export)

5. **Run ggen** to regenerate artifacts

### Add a New Lifecycle State

1. **Update ontology-extensions.ttl:**
   ```ttl
   lifecycle:MyNewState a owl:Class ;
     rdfs:subClassOf lifecycle:ProcessState ;
     rdfs:label "My New State" .
   ```

2. **Update SPARQL query (extract-lifecycle-governance.rq):**
   ```sparql
   FILTER (?statePhase IN (
     lifecycle:DesignPhase,
     ...,
     lifecycle:MyNewPhase
   ))
   ```

3. **Add RDF facts** with transitions, rules, actions for the state

4. **Run ggen** to regenerate blue-river.tera

---

## Testing & Validation

ggen includes built-in validations:

```bash
# Verify all artifacts meet board admissibility contract
ggen --config ggen.toml --verify-all

# Checks performed:
# 1. All SPARQL queries return results with required fields
# 2. All fitness scores >= 0.95, all precision scores >= 0.90
# 3. All claims have cryptographic receipts
# 4. All templates render without syntax errors
# 5. Output artifacts are well-formed (valid PPTX, XLSX, Rust)
```

---

## References

- **ggen.toml** — Generation rules and config
- **queries/** — SPARQL queries for fact extraction
- **templates/** — Tera templates for artifact rendering
- **ontology-extensions.ttl** — RDF schema binding all components
- **README.md** — Detailed architecture and usage
- **../ma/define_board-admissible_claim_requirements.md** — Board admissibility framework
- **../standards/MAPE_K_INTEGRATION.md** — MAPE-K autonomic loop
- **../doctrine/blue-river-dam.md** — Blue River orchestrator specification
