# Integration Risk Assessment Through Process Intelligence

**Doctrine:** Integration risk is process risk. The question is not whether the two companies use
compatible systems. The question is whether the two companies' process execution patterns are
compatible, quantifiably, from evidence that survives audit.

> "We can quantify integration risk from event logs within 2 weeks of data access."
> This is the integration claim backed by process intelligence.

---

## The Conventional Integration Risk Assessment

Standard integration risk assessments produce:
- Technology stack compatibility matrices (ERP versions, API capabilities)
- Org chart overlap analysis (role duplication, management span)
- Cultural fit assessments (survey-based, interview-derived)
- IT integration complexity estimates (based on system count and integration points)
- Synergy estimates (revenue uplift and cost reduction projections from consultant models)

What these assessments do not produce:
- Evidence that the two companies' processes are compatible at the execution level
- Quantification of process variant entropy (how many different ways each company executes the same process)
- Object-centric conflict analysis (where the two companies' object lifecycles are structurally incompatible)
- Conformance gap measurement (how far each company deviates from the declared post-merger target process)

The conventional assessment tells you whether the systems can be connected.
Process intelligence tells you whether the processes can be merged.

---

## With Process Intelligence: Five Analyses

### 1. Process Model Similarity

**What it measures:**
Structural similarity between the seller's discovered process model and the buyer's declared post-merger process model.

**How produced:**
1. Buyer provides their declared post-merger process model (WF-net or process tree)
2. Seller OCEL log is admitted and Inductive Miner is run against full population
3. wasm4pm computes process tree edit distance between seller's discovered model and buyer's target model
4. Similarity score: 1 - (edit_distance / max_edit_distance_for_tree_size)

**Interpretation:**
- Similarity ≥ 0.85: High compatibility — seller process is close to buyer's target model
- Similarity 0.65–0.84: Moderate compatibility — specific divergences require mapping
- Similarity <0.65: Low compatibility — process redesign required, not just integration

**wasm4pm-compat grounding:**
- Seller model admitted through `Admission<ProcessTree, InductiveMinerWitness>`
- Buyer model admitted through `Admission<ProcessTree, BuyerTargetProcessWitness>`
- Similarity metric: `Metric<ProcessModelSimilarity, NUM, DEN>` with `Between01` bound

---

### 2. Log Overlap

**What it measures:**
The fraction of seller trace variants that also appear in the buyer's historical logs (shared process patterns).

**How produced:**
1. Buyer provides historical event log for the same process domain
2. Seller provides OCEL log
3. Both logs are projected to comparable XES format with documented loss policies
4. wasm4pm identifies trace variants present in both logs (shared variants) vs. seller-only variants
5. Log overlap score: |shared_variants| / |seller_variants|

**Interpretation:**
- Overlap ≥ 0.70: High overlap — seller and buyer execute most process patterns identically
- Overlap 0.40–0.69: Moderate overlap — significant variant mapping required
- Overlap <0.40: Low overlap — fundamentally different process execution styles

**Integration implication of low overlap:**
Low log overlap means that a large fraction of the seller's process patterns are unfamiliar to the buyer's
operational teams. The integration plan must account for knowledge transfer across all non-overlapping variants,
not just the dominant variant.

**wasm4pm-compat grounding:**
- Both logs admitted as `Admission<OcelLog, Ocel20Witness>`
- XES projection with named loss policy: `AllowNamedProjection("OCEL-to-XES for variant comparison")`
- Loss report on projection: `LossReport<OcelLog, XesLog, DroppedObjectRelations>`

---

### 3. Object-Centric Conflict

**What it measures:**
Structural incompatibilities between the seller's object types and their relationships, and the buyer's
declared object model for the merged entity.

**Why this matters:**
Two companies can have nominally identical processes (Order-to-Cash) but structurally incompatible
object models. For example:
- Seller: Order → multiple LineItems; each LineItem has independent Shipment and Invoice
- Buyer: Order → single Shipment → multiple Invoices

Merging these two processes is not a configuration change. It requires redesigning the object model
for one company before integration can proceed.

**How produced:**
1. Seller OCEL log provides implicit object type schema (object types, E2O relations, O2O relations)
2. Buyer provides their object model for the merged entity
3. wasm4pm maps seller object types to buyer object types
4. Conflict analysis identifies:
   - Object type mismatches (seller has object types with no buyer analog)
   - Relation arity conflicts (one-to-many in seller vs. many-to-many in buyer)
   - Temporal precedence conflicts (A before B in seller vs. B before A in buyer)
   - Missing object types (buyer process requires object types the seller does not track)

**Conflict severity classification:**
- Structural conflict: object type exists in one model but not the other → requires ETL redesign
- Arity conflict: same object pair but different cardinality → requires normalization
- Temporal conflict: same activities but different precedence → requires process redesign

**wasm4pm-compat grounding:**
- `Admission<OcelLog, Ocel20Witness>` — seller log admitted with full E2O/O2O relations
- Named conflict violations: `ObjectTypeConflict<SellerType, BuyerType>`, `ArityConflict<ObjA, ObjB>`, `TemporalPrecedenceConflict<ActA, ActB>`
- Conflict report: `Evidence<ObjCentricConflict, Receipted, Ocel20Witness>`

---

### 4. Conformance Gap Analysis

**What it measures:**
For each company individually: how far does their current process execution deviate from the
declared post-merger target process?

**How produced:**
1. Buyer provides post-merger target process model
2. Seller OCEL log is replayed against the target model (cross-model conformance)
3. Buyer's own historical log is replayed against the target model
4. Conformance gap = (buyer_fitness_against_target) - (seller_fitness_against_target)

**Interpretation:**
- Small gap (≤0.05): Both companies are close to the target — integration is configuration, not redesign
- Moderate gap (0.05–0.20): One company requires more process change — asymmetric integration burden
- Large gap (>0.20): Fundamental process redesign required for the low-fitness company

**Integration budget implication:**
The conformance gap is an input to integration budget estimation.
Higher gap = higher process change management cost = higher integration budget = lower synergy realization.
This is a quantified, log-derived input to integration financial modeling.

**wasm4pm-compat grounding:**
- Target model admitted through `Admission<WfNet, PostMergerTargetWitness>`
- Cross-model replay: seller log against target model, buyer log against target model
- Gap metric: `Metric<ConformanceGap, NUM, DEN>` — difference in fitness scores

---

### 5. Exception Rate Analysis

**What it measures:**
The frequency of process exceptions (deviations from the dominant process variant) in each company's log,
and whether exception patterns are compatible.

**Why it matters for integration:**
Two companies with similar dominant variants can have radically different exception handling.
If the seller handles 15% of orders as exceptions (custom pricing, expedited fulfillment, partial shipments)
and the buyer handles 2%, the post-merger integration must absorb the seller's exception volume into the buyer's
exception handling infrastructure — which may be sized for 2%, not 15%.

**How produced:**
1. wasm4pm runs variant analysis on seller log: (dominant_variant_frequency, exception_rate, exception_variant_count)
2. wasm4pm runs variant analysis on buyer log: same metrics
3. Exception rate delta: |seller_exception_rate - buyer_exception_rate|
4. Exception variant compatibility: for each seller exception variant, is there a corresponding buyer exception variant?

**Integration complexity from exception mismatch:**
Each seller exception variant with no buyer analog is an integration gap requiring:
- Process documentation
- Exception handling training for buyer's operations team
- System configuration to handle the seller's exception pattern
- Potentially, process redesign to eliminate or standardize the exception

**wasm4pm-compat grounding:**
- Exception variants admitted through `Evidence<ProcessVariant, Admitted, InductiveMinerWitness>`
- Exception rate metric: `Metric<ExceptionRate, NUM, DEN>`
- Compatibility gap: per-variant `Evidence<ExceptionVariant, Receipted, VariantWitness>`

---

## The Integration Claim

> "We can quantify integration risk from event logs within 2 weeks of data access."

**What "2 weeks" requires:**
- Day 1–3: OCEL log extraction from seller's system of record (requires seller cooperation and extraction pipeline)
- Day 3–5: Log admission and validation (wasm4pm admission layer verifies log completeness and structure)
- Day 5–8: Five analyses run in parallel (wasm4pm manufacturing pipeline)
- Day 8–10: Integration risk report with quantified scores for all five analyses
- Day 10–14: Buyer review and challenge process (seller may dispute specific findings)

**What the 2-week claim requires on the seller side:**
- An OCEL 2.0 extraction pipeline (or XES with documented object-centric relations)
- A cooperating seller who provides full log access during due diligence
- A post-merger target process model from the buyer (required for conformance gap analysis)

**What the 2-week claim produces:**
A quantified integration risk report with five scores, each grounded in admitted evidence, each buyer-reproducible.
This is not a consultant's estimate. It is a manufacturing artifact.

---

## Integration Risk Score

The five analyses combine into a single Integration Risk Score:

```
IRS = w1 × (1 - model_similarity) +
      w2 × (1 - log_overlap) +
      w3 × conflict_severity_index +
      w4 × conformance_gap +
      w5 × exception_rate_delta
```

Default weights (adjustable by deal type):
- w1 = 0.25 (model similarity most important for process-led integration)
- w2 = 0.20 (log overlap indicates operational compatibility)
- w3 = 0.25 (object-centric conflicts are deep structural issues)
- w4 = 0.15 (conformance gap drives process change management cost)
- w5 = 0.15 (exception rate drives operational capacity requirement)

**IRS interpretation:**
- IRS ≤ 0.20: Low integration risk — process integration is configuration-dominant
- IRS 0.20–0.40: Moderate integration risk — process redesign required in specific areas
- IRS > 0.40: High integration risk — fundamental process architecture divergence

---

## Integration Risk Report Format

Each report produced by wasm4pm includes:

| Section | Content | Receipt Type |
|---|---|---|
| Log completeness | Case count, time range, object type coverage | `Admission<OcelLog, Ocel20Witness>` |
| Process model similarity | Score, edit distance, divergence map | `Metric<ProcessModelSimilarity, N, D>` |
| Log overlap | Overlap score, shared/unique variant list | `Evidence<VariantOverlap, Receipted, W>` |
| Object-centric conflicts | Conflict count by type, severity, named conflicts | `Evidence<ObjCentricConflict, Receipted, W>` |
| Conformance gap | Per-company fitness against target, gap score | `Metric<ConformanceGap, N, D>` |
| Exception rate | Per-company exception rate, delta, compatibility | `Metric<ExceptionRate, N, D>` |
| Integration Risk Score | Weighted composite with component breakdown | `Evidence<IntegrationRisk, Receipted, W>` |

---

*Grounded in: Blue River Dam doctrine, wasm4pm execution authority, wasm4pm-compat evidence types, OCEL 2.0, Van der Aalst process mining conformance and variant analysis.*
