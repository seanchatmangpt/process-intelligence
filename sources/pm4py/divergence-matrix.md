# PM4Py ↔ wasm4pm Divergence Matrix v2.0

**Purpose:** Identify which PM4Py capabilities must align with wasm4pm and which can safely diverge. Maps each algorithm/capability to its conformance requirements and typing implications.

**Last Updated:** 2026-05-31

**Doctrine:** If the code says it worked but the event log cannot prove a lawful process happened, then it did not work. — Van der Aalst Constitution

---

## 1. Discovery Algorithms — Alignment Requirements

### 1.1 Petri Net Miners (Transitive Discovery)

| Miner | Must Match wasm4pm? | Divergence Safety | Compat Type Binding |
|-------|----|----|---|
| **Inductive Miner (im/imd/imf/imc)** | YES — Core algorithm | ✗ Cannot diverge. Output model soundness must be identical. Both must guarantee sound workflow nets by construction. | `Evidence<PetriNet, ValidatedSound, SoundnessCertificate>` — wasm4pm must emit cryptographic soundness proof. |
| **Alpha Miner** | YES — Baseline reference | ✗ Cannot diverge. If both claim `discover_petri_net_alpha`, they must produce identical nets on identical logs. | `Evidence<PetriNet, ParsedStructured, AlphaInvariant>` — wasm4pm must verify α-miner preconditions (complete, acyclic logs). |
| **Heuristics Miner** | YES — Reference for noisy logs | ✓ **Safe divergence**: Quality metrics may differ (PM4Py outputs thresholds; wasm4pm may output confidence intervals). Structural output must match. | `Evidence<HeuristicsNet, ValidatedWithMetrics, ConfidenceVector>` — wasm4pm type binds to a stricter confidence model. |
| **Genetic Algorithm Miner** | PARTIAL — Heuristic optimization | ✓ **Safe divergence**: Different initial populations, crossover strategies, and termination criteria allowed. Final model structure may differ slightly. Output fitness scores must be comparable. | `Evidence<PetriNet, OptimizedByGA, FitnessVector>` — wasm4pm must report final population metrics. |
| **ILP Miner** | PARTIAL — Computational heuristic | ✓ **Safe divergence**: Different LP solvers (Gurobi vs. COIN-OR), timeout thresholds, and precision tolerances allowed. Semantic output (net + precision/fitness metrics) must match within epsilon. | `Evidence<PetriNet, OptimizedByILP, PrecisionBound>` — wasm4pm must track solver termination status. |
| **Declare Miner** | YES — Constraint extraction | ✓ **Limited divergence**: Template set must match (existence, response, precedence, etc.). Thresholds (min_support, min_confidence) may differ for filtering, but mined constraints must be semantically equivalent on shared inputs. | `Evidence<DeclareModel, ValidatedConstraints, ConstraintSet>` — types bind to immutable constraint lattice. |

### 1.2 Intermediate Representations (DFG, Footprints, Temporal)

| Representation | Must Match wasm4pm? | Divergence Safety | Compat Type Binding |
|-------|----|----|---|
| **Directly Follows Graph (DFG)** | YES — Fundamental | ✗ Cannot diverge. Adjacency structure (A→B edges + frequencies) must be identical. This is the input to all downstream miners and conformance checks. | `Evidence<DirectlyFollowsGraph, Discovered, EdgeFrequencyMap>` — wasm4pm binds to immutable edge frequency assertions. |
| **Performance DFG** | YES — Time-aware variant | ✗ Cannot diverge on structure; timing metrics must match within precision (seconds to microseconds). | `Evidence<PerformanceDFG, Discovered, TimingStatistics>` — includes min/max/mean/stddev per edge. |
| **Eventually Follows Graph** | YES — Structural semantics | ✗ Cannot diverge. All indirect paths must be enumerated identically. | `Evidence<EventuallyFollowsGraph, Discovered, TransitiveClosureSet>` |
| **Footprints** | YES — Conformance input | ✗ Cannot diverge. Successor/predecessor/concurrency pairs define the footprint structure that conformance checks validate against. | `Evidence<Footprints, Discovered, ConcurrencyRelations>` |
| **Temporal Profile** | PARTIAL — Statistical aggregate | ✓ **Safe divergence**: Different aggregation windows (min/max/median/quantiles) allowed. Core temporal bounds (min_wait, max_wait per edge) must align. | `Evidence<TemporalProfile, Discovered, TimeBoundVector>` |
| **Log Skeleton** | YES — Lightweight conformance | ✗ Cannot diverge. Equivalence classes and predecessor/successor relation sets are deterministic from log. | `Evidence<LogSkeleton, Discovered, RelationSet>` |
| **Minimum Self-Distance** | YES — Loop detection | ✗ Cannot diverge. Self-distance histogram must be identical (array of consecutive activity distances). | `Evidence<MinSelfDistance, Discovered, DistanceHistogram>` |

### 1.3 OCEL (Object-Centric) Discovery

| Capability | Must Match wasm4pm? | Divergence Safety | Compat Type Binding |
|-------|----|----|---|
| **Object-Centric DFG (OCDFG)** | YES — Core OCEL discovery | ✗ Cannot diverge. Event-to-object relations and flattened DFGs per object type are structural. | `Evidence<OCDFG, Discovered, EventObjectRelationMap>` |
| **Object-Centric Petri Net (OCPN)** | YES — Object lifecycle model | ✗ Cannot diverge. Per-object-type nets and synchronization edges must be identical. | `Evidence<OCPN, ValidatedSound, ObjectLifecycleSynchronization>` |
| **Event-to-Object (ETOT) Graph** | YES — Relation structure | ✗ Cannot diverge. Sets of objects and relation counts are deterministic. | `Evidence<ETOT, Discovered, ObjectTypeSet>` |
| **Object-to-Object (OTG) Graph** | YES — Object interaction | ✗ Cannot diverge. Inter-object transitions labeled by events must match. | `Evidence<OTG, Discovered, InteractionMap>` |

### 1.4 Specialized Miners

| Miner | Must Match wasm4pm? | Divergence Safety | Compat Type Binding |
|-------|----|----|---|
| **Batches (Activity Clustering)** | PARTIAL — Statistical pattern | ✓ **Safe divergence**: Merge distance thresholds, batch size minima, and clustering heuristics may differ. Output format (batch tuples) must be consistent. | `Evidence<BatchList, Discovered, ClusteringParams>` |
| **Organizational Roles** | PARTIAL — Clustering | ✓ **Safe divergence**: Different clustering algorithms (k-means vs. hierarchical) allowed if roles (resource groupings) are semantically equivalent. | `Evidence<RoleList, Discovered, RoleMembership>` |
| **Social Network Analysis (HoW, WT, Subcontracting)** | PARTIAL — Graph aggregation | ✓ **Safe divergence**: Edge weight aggregation and centrality calculations may use different formulas. Underlying graph structure must be identical. | `Evidence<SNAGraph, Discovered, CentralityMetrics>` — wasm4pm may omit centrality; core graph must match. |
| **Correlation Mining** | PARTIAL — Pattern discovery | ✓ **Safe divergence**: Different correlation metrics (Pearson, Spearman, MI) allowed. Common patterns must be identified if thresholds match. | `Evidence<CorrelationSet, Discovered, CorrelationMetric>` |
| **Prefix Tree / Trie** | YES — Sequence structure | ✗ Cannot diverge. Trie structure is deterministic from trace sequences. | `Evidence<PrefixTree, Discovered, SequenceEnumeration>` |
| **Transition System** | YES — State space | ✗ Cannot diverge. State enumeration and transitions must be identical for same window/view parameters. | `Evidence<TransitionSystem, Discovered, StateSpaceGraph>` |

---

## 2. Conformance Checking — Alignment Requirements

### 2.1 Deterministic Conformance (Token-Based Replay, Alignments)

| Technique | Must Match wasm4pm? | Divergence Safety | Compat Type Binding |
|-------|----|----|---|
| **Token-Based Replay (TBR)** | YES — Deterministic | ✗ Cannot diverge. Fitness/precision/generalization metrics must be identical (binary token model is fully specified). | `Evidence<ConformanceResult, Replayed, FitnessDiagnostics>` — wasm4pm must emit per-trace token diagnostics. |
| **Alignment-Based Conformance** | PARTIAL — Search heuristic | ✓ **Safe divergence**: Different A* implementations, beam search depths, and heuristic functions allowed if final alignment costs match within rounding. Output alignment paths may differ (multiple optimal alignments exist). | `Evidence<AlignmentSet, Replayed, AlignmentCostMatrix>` — types bind to cost guarantees, not path uniqueness. |
| **Prefix Replay (TBR variant)** | YES — Deterministic | ✗ Cannot diverge. Replaying a trace prefix to a marking is deterministic; output marking must be identical. | `Evidence<Marking, Replayed, TokenState>` |

### 2.2 Structural/Lightweight Conformance

| Technique | Must Match wasm4pm? | Divergence Safety | Compat Type Binding |
|-------|----|----|---|
| **Footprints Conformance** | YES — Deterministic structure | ✗ Cannot diverge. Violations (successor/predecessor/concurrency mismatches) are deterministic. | `Evidence<FootprintViolations, ValidatedStructure, ViolationSet>` |
| **Log Skeleton Conformance** | YES — Deterministic structure | ✗ Cannot diverge. Equivalence class and relation set violations are deterministic. | `Evidence<LogSkeletonViolations, ValidatedStructure, ViolationSet>` |
| **Temporal Profile Conformance** | PARTIAL — Time bound semantics | ✓ **Safe divergence**: Penalty functions (zeta factor, quantile thresholds) may differ. Core temporal violations (event outside bounds) must match. | `Evidence<TemporalViolations, ValidatedTiming, ViolationVector>` |
| **Declare Conformance** | YES — Deterministic constraints | ✗ Cannot diverge. Constraint violation counts per trace must be identical. | `Evidence<DeclareViolations, ValidatedConstraints, ViolationSet>` |

### 2.3 Object-Centric Conformance

| Technique | Must Match wasm4pm? | Divergence Safety | Compat Type Binding |
|-------|----|----|---|
| **OCDFG Conformance** | YES — Deterministic structure | ✗ Cannot diverge. Event-object relation coverage is deterministic. | `Evidence<OCDFGFitness, ValidatedObjectCentric, CoverageTuple>` |
| **OCPN Conformance** | PARTIAL — Object lifecycle | ✓ **Safe divergence**: Per-object-type replay strategies may differ. Aggregate fitness across object types must match within semantically equivalent calculations. | `Evidence<OCPNFitness, ValidatedObjectCentric, ObjectLifecycleTrace>` |

---

## 3. Format Conversions & Model Transformations

### 3.1 Deterministic Conversions

| Conversion | Must Match wasm4pm? | Divergence Safety | Compat Type Binding |
|-------|----|----|---|
| **ProcessTree ↔ Petri Net** | YES — Deterministic | ✗ Cannot diverge. Structural isomorphism (places, transitions, arcs) must be identical. | `Evidence<PetriNet, ConvertedStructural, IsomorphismMap>` |
| **DFG → Process Tree** | YES — Deterministic reconstruction | ✗ Cannot diverge. Block structure inferred from DFG must be identical. | `Evidence<ProcessTree, ConvertedStructural, BlockStructure>` |
| **Log → OCEL (object type extraction)** | PARTIAL — Heuristic flattening | ✓ **Safe divergence**: Different object type inference strategies allowed if they result in semantically equivalent OCEL. Configuration must be explicit. | `Evidence<OCEL, ConvertedStructural, ObjectTypeInferenceParams>` |
| **EventLog → EventStream ↔ DataFrame** | YES — Deterministic | ✗ Cannot diverge. Row ordering, column mapping must be identical. | `Evidence<DataFrame, ConvertedStructural, SchemaMapping>` |
| **OCEL → EventLog (flattening)** | PARTIAL — Lossy flattening | ✓ **Safe divergence**: Different flattening strategies (which object becomes case_id) allowed if they're explicit. Semantically equivalent traces must be achievable. | `Evidence<EventLog, ConvertedWithLoss, LossPolicy>` |
| **Petri Net → Reachability Graph** | YES — Deterministic | ✗ Cannot diverge (with state explosion caveat). Full reachability enumeration must be identical up to marking canonicalization. | `Evidence<ReachabilityGraph, ConvertedStructural, StateEnumeration>` |

---

## 4. I/O & Data Ingestion

### 4.1 Parsing & Validation

| Format | Must Match wasm4pm? | Divergence Safety | Compat Type Binding |
|-------|----|----|---|
| **XES Parser** | YES — Standard compliance | ✗ Cannot diverge. Parsing must preserve all standard attributes; non-standard attributes may be handled differently but core case/activity/timestamp must match. | `Evidence<EventLog, Parsed, XESValidationReport>` |
| **OCEL 2.0 JSON** | YES — Standard compliance | ✗ Cannot diverge. Schema validation and relational integrity (events, objects, relations) must match. | `Evidence<OCEL, Parsed, OCEL2ValidationReport>` |
| **OCEL SQLite** | YES — Relational integrity | ✗ Cannot diverge. Queries extracting events/objects/relations must be identical. | `Evidence<OCEL, Parsed, SQLiteConstraintCheck>` |
| **CSV (custom log)** | PARTIAL — Column-mapped | ✓ **Safe divergence**: Different column mappings allowed if transformation is explicit. Output EventLog/DataFrame must be semantically equivalent. | `Evidence<EventLog, ParsedWithMapping, ColumnMap>` |
| **PNML (Petri Net)** | YES — Standard compliance | ✗ Cannot diverge. Places, transitions, arcs, markings must be preserved exactly. | `Evidence<PetriNet, Parsed, PNMLValidationReport>` |
| **BPMN** | PARTIAL — Semantics variation | ✓ **Safe divergence**: Different BPMN → Petri Net conversion strategies allowed (OR-joins, loops may be interpreted differently). Structural safety must be guaranteed. | `Evidence<BPMN, Parsed, BPMNSemanticProfile>` |

---

## 5. Visualization & Export

| Capability | Must Match wasm4pm? | Divergence Safety | Compat Type Binding |
|-------|----|----|---|
| **Petri Net Visualization** | NO — Rendering only | ✓ **Fully safe divergence**. Layout algorithms, colors, fonts may differ. Underlying model must match. | No Evidence type binding; visualization is post-hoc. |
| **DFG Visualization** | NO — Rendering only | ✓ **Fully safe divergence**. Graph layout may differ; underlying edge/frequency data must match. | No Evidence type binding. |
| **All Graphviz-backed Vis** | NO — Rendering only | ✓ **Fully safe divergence**. Different renderers allowed; model data must match. | No Evidence type binding. |
| **PNG/SVG/HTML Export** | NO — Format only | ✓ **Fully safe divergence**. File format, compression, encoding may vary. | No Evidence type binding. |

---

## 6. Statistical Analysis & Aggregations

| Statistic | Must Match wasm4pm? | Divergence Safety | Compat Type Binding |
|-------|----|----|---|
| **Start/End Activities** | YES — Deterministic | ✗ Cannot diverge. Frequency counts must be identical. | `Evidence<ActivityFrequencyMap, Discovered, ActivitySet>` |
| **Trace Variants** | YES — Deterministic | ✗ Cannot diverge. Variant enumeration and occurrence counts must be identical. | `Evidence<VariantMap, Discovered, VariantCounts>` |
| **Activity Position** | YES — Deterministic | ✗ Cannot diverge. Position percentages (start/middle/end) must be identical. | `Evidence<PositionStatistics, Discovered, Percentiles>` |
| **Case Duration** | YES — Deterministic | ✗ Cannot diverge (within microsecond precision). Duration values must match. | `Evidence<DurationVector, Discovered, TimeValues>` |
| **Service Time (A→B)** | YES — Deterministic | ✗ Cannot diverge (within precision). Min/mean/max/median/stdev must match. | `Evidence<TimingStatistics, Discovered, StatisticsTuple>` |
| **Rework Cases** | YES — Deterministic | ✗ Cannot diverge. Activity repetition counts must be identical. | `Evidence<ReworkMap, Discovered, RepetitionCounts>` |
| **Case Overlap** | YES — Deterministic | ✗ Cannot diverge. Concurrency counts must be identical. | `Evidence<OverlapMap, Discovered, ConcurrencyCounts>` |
| **Minimum Self-Distance** | YES — Deterministic | ✗ Cannot diverge. Self-distance arrays must be identical. | `Evidence<SelfDistanceMap, Discovered, DistanceArrays>` |
| **Process Cube** | PARTIAL — Multi-dim aggregation | ✓ **Safe divergence**: Different aggregation dimensions and formulas allowed if queries match within semantics. | `Evidence<ProcessCube, Discovered, AggregationParams>` |
| **Stochastic Language** | YES — Deterministic (frequency-based) | ✗ Cannot diverge. Trace probability distribution must match. | `Evidence<StochasticLanguage, Discovered, ProbabilityDistribution>` |

### 6.1 OCEL Statistics

| Statistic | Must Match wasm4pm? | Divergence Safety | Compat Type Binding |
|-------|----|----|---|
| **Object Types** | YES — Deterministic | ✗ Cannot diverge. Type set must be identical. | `Evidence<ObjectTypeSet, Discovered, TypeList>` |
| **Object Attributes** | YES — Deterministic | ✗ Cannot diverge. Attribute set per type must be identical. | `Evidence<ObjectTypeAttributes, Discovered, AttributeMap>` |
| **Object Count Per Type** | YES — Deterministic | ✗ Cannot diverge. Counts must be identical. | `Evidence<ObjectCountMap, Discovered, CountVector>` |
| **Object Interactions** | YES — Deterministic | ✗ Cannot diverge. Co-occurrence matrix must be identical. | `Evidence<InteractionMatrix, Discovered, CooccurrenceData>` |

---

## 7. Filtering & Data Manipulation

| Operation | Must Match wasm4pm? | Divergence Safety | Compat Type Binding |
|-------|----|----|---|
| **Filter by Activity** | YES — Deterministic | ✗ Cannot diverge. Subset predicate is deterministic; output rows must match exactly. | `Evidence<EventLog, Filtered, FilterPredicate>` |
| **Filter by DFG Activities %** | PARTIAL — Relative ranking | ✓ **Safe divergence**: Ties in frequency ranking may break differently. Output must be semantically equivalent (top N% by occurrence). | `Evidence<EventLog, Filtered, FilterThreshold>` |
| **Filter by Time Range** | YES — Deterministic | ✗ Cannot diverge. Timestamp comparison is deterministic. | `Evidence<EventLog, Filtered, TimeRange>` |
| **Filter by Case Performance** | YES — Deterministic | ✗ Cannot diverge. Duration comparison is deterministic. | `Evidence<EventLog, Filtered, DurationRange>` |
| **Filter by Eventually-Follows** | YES — Deterministic | ✗ Cannot diverge. Path existence is deterministic. | `Evidence<EventLog, Filtered, PathConstraint>` |
| **Filter by Rework** | YES — Deterministic | ✗ Cannot diverge. Activity repetition detection is deterministic. | `Evidence<EventLog, Filtered, ReworkConstraint>` |
| **OCEL Filter by Connectivity** | YES — Deterministic | ✗ Cannot diverge. Object-type relation lookup is deterministic. | `Evidence<OCEL, Filtered, ConnectivityConstraint>` |

---

## 8. Machine Learning & LLM Integration

| Feature | Must Match wasm4pm? | Divergence Safety | Compat Type Binding |
|-------|----|----|---|
| **Feature Extraction (ML module)** | PARTIAL — Data transformation | ✓ **Safe divergence**: Different encoding schemes (one-hot vs. embedding), feature selection allowed. Output feature vectors may differ in representation but must be semantically transformable. | `Evidence<FeatureMatrix, Extracted, EncodingScheme>` |
| **Train/Test Split** | YES — Deterministic (temporal/random) | ✗ Cannot diverge on seed. Same RNG seed must produce identical splits. | `Evidence<DataSplit, Partitioned, SplitIndices>` |
| **LLM Integration (OpenAI, Google, Anthropic)** | NO — External service | ✓ **Fully safe divergence**. LLM outputs are non-deterministic. Must not be used in conformance-critical paths. | No Evidence type binding; LLM outputs are advisory only. |

---

## 9. Analysis & Structural Validation

| Operation | Must Match wasm4pm? | Divergence Safety | Compat Type Binding |
|-------|----|----|---|
| **Soundness Check (WF-net)** | YES — Deterministic | ✗ Cannot diverge. Binary result (sound/unsound) must be identical. | `Evidence<SoundnessResult, ValidatedStructural, SoundnessCertificate>` |
| **Workflow Net Check** | YES — Deterministic | ✗ Cannot diverge. Binary result (is WFN/is not) must be identical. | `Evidence<WorkflowNetResult, ValidatedStructural, StructuralCertificate>` |
| **Behavioral Similarity** | PARTIAL — Distance metric | ✓ **Safe divergence**: Different similarity formulas allowed. Results must be comparable (e.g., Jaccard vs. EMD; both valid). | `Evidence<SimilarityScore, AnalyzedBehavior, SimilarityMetric>` |
| **Structural Similarity** | PARTIAL — Distance metric | ✓ **Safe divergence**: Different graph matching algorithms allowed. Results comparable. | `Evidence<SimilarityScore, AnalyzedStructure, StructuralMetric>` |
| **EMD (Earth Mover's Distance)** | PARTIAL — Computational metric | ✓ **Safe divergence**: Different LP solvers allowed. Results must match within numerical precision. | `Evidence<EMDDistance, AnalyzedBehavior, WassersteinDistance>` |
| **Implicit Place Reduction** | YES — Deterministic structural simplification | ✗ Cannot diverge. Output net (reduced places) must be identical. | `Evidence<PetriNet, Simplified, ImplicitPlaceMap>` |
| **Invisible Transition Reduction** | YES — Deterministic | ✗ Cannot diverge. Output net must be identical. | `Evidence<PetriNet, Simplified, InvisibleTransitionMap>` |
| **Synchronous Product** | YES — Deterministic | ✗ Cannot diverge. Product net structure must be identical. | `Evidence<PetriNet, Composed, ProductStructure>` |
| **Marking Equation Solver** | PARTIAL — Mathematical solver | ✓ **Safe divergence**: Different solving strategies allowed. All solutions found must be semantically identical (up to reordering). | `Evidence<MarkingSet, Solved, MarkingEquation>` |

---

## 10. Type System & Evidence Lattice

### 10.1 Evidence State Machine

All compat types must flow through this state lattice (monotonic ascent):

```
┌─────────────┐
│   Parsed    │  ← Raw data admitted; schema validated
└──────┬──────┘
       │
       ↓
┌─────────────────────┐
│  Validated* (Safe)  │  ← Semantic constraints checked (soundness, consistency, integrity)
│  - ValidatedSound
│  - ValidatedStructure
│  - ValidatedConstraints
│  - ValidatedObjectCentric
│  - ValidatedTiming
└──────┬──────────────┘
       │
       ↓
┌──────────────────┐
│  Replayed/       │  ← Execution evidence collected (conformance, fitness, alignment)
│  Analyzed        │
└──────────────────┘
```

### 10.2 Evidence Binding Rules

| State Transition | Rule | Compat Implication |
|---|---|---|
| `Parsed` → `ValidatedSound` | Must have soundness certificate (WF-net proof) | wasm4pm required to emit cryptographic proof token. |
| `Parsed` → `ValidatedStructure` | Must verify structural invariants (no duplicate IDs, timestamp monotonicity) | wasm4pm must validate admission predicates. |
| `ValidatedSound` → `Replayed` | Token count, alignment cost, or conformance metric must be derived and attached | wasm4pm must generate witness (cost matrix, token vector). |

---

## 11. Synthesis: Divergence Summary

### 11.1 Must-Match Categories (Core Integrity)

**These cannot diverge between PM4Py and wasm4pm:**

1. **Deterministic discovery:** Alpha, Inductive, DFG, eventually-follows, footprints, log skeleton, OCEL discovery.
2. **Deterministic conformance:** Token-based replay, footprints conformance, log skeleton conformance, Declare conformance.
3. **Deterministic statistics:** All frequency/count aggregations (activities, traces, rework, case overlap, self-distance, etc.).
4. **Deterministic I/O:** XES/OCEL/PNML parsing (schema compliance).
5. **Deterministic filtering:** All activity/time/path-based filters.
6. **Deterministic structural analysis:** Soundness check, WF-net check, implicit place reduction, invisible transition reduction.

**Risk If Diverged:** Conformance proofs fail; audit chain breaks; M&A claims become indefensible.

---

### 11.2 Safe Divergence Categories (Heuristic Optimization)

**These can diverge if semantics are preserved:**

1. **Heuristic optimization:** Genetic algorithm, ILP mining (different solvers/parameters allowed).
2. **Alignment-based conformance:** Different A* or search heuristics allowed if costs match.
3. **Clustering & statistical:** Roles discovery, batches, SNA (different clustering allowed).
4. **Feature extraction:** ML feature engineering (different encodings allowed).
5. **Similarity metrics:** Different formulas (Jaccard, EMD, etc.) allowed if labeled and comparable.
6. **Visualization:** All rendering and layout (PNG, SVG, Graphviz).
7. **LLM integration:** Non-deterministic; advisory only; forbidden from conformance paths.

**Safe Because:** Multiple correct solutions exist; semantic equivalence is verifiable; outputs not part of proof chain.

---

### 11.3 Partially Divergent Categories (Explicit Config)

**These can diverge only with explicit, documented configuration:**

1. **Log flattening:** OCEL → EventLog requires declaring which object becomes case_id.
2. **Object type inference:** Log → OCEL requires declaring type extraction strategy.
3. **Temporal profile:** Aggregation windows (quantiles, mean/median mix) must be explicit.
4. **Filter thresholds:** Top-N%-by-frequency may have tie-breaking rules; must document.
5. **Model conversions with loss:** BPMN → Petri Net, ProcessTree → DFG must report LossPolicy.

**Safe Because:** Configuration is captured in Evidence metadata; traces are auditable.

---

## 12. Compat Type Binding

### 12.1 Core Type Signature

All conformance-critical functions must return:

```
Evidence<T, State, Witness> where
  T ∈ {EventLog, OCEL, PetriNet, ProcessTree, DFG, HeuristicsNet, Declare, ...}
  State ∈ {Parsed, Validated*, Replayed, Analyzed, Optimized, Simplified, Composed, Filtered}
  Witness ∈ {SoundnessCertificate, FitnessDiagnostics, AlignmentCostMatrix, ...}
```

### 12.2 Type Enforcement Rules

1. **No implicit coercion:** PM4Py's permissive typing (Pandas NaN, dynamic dicts) forbidden in wasm4pm.
2. **Schema validation on admission:** All parsed objects must validate against formal schema (XES, OCEL, JSON Schema).
3. **Monotonic state flow:** No State downgrade (e.g., `ValidatedSound` cannot revert to `Parsed`).
4. **Witness generation on state change:** Every state transition must produce auditable evidence (hash, proof, diagnostic).

---

## 13. Operational Checklist for Compat Layer

For each PM4Py algorithm ported to wasm4pm:

- [ ] **Is it deterministic?** If yes → must-match category; output must be bitwise identical.
- [ ] **Is it heuristic?** If yes → check for safe divergence (multi-solution allowed?) or partial divergence (config explicit?).
- [ ] **Does it produce a conformance artifact?** If yes → must emit Evidence<T, *, Witness>.
- [ ] **Is it used downstream by other algorithms?** If yes → output must be stable (schema-locked).
- [ ] **Does it call external services (LLM, solver)?** If yes → results forbidden from conformance paths; tag as advisory.
- [ ] **Does it handle lossy conversion?** If yes → emit LossReport with policy documented.

---

## 14. References

- Van der Aalst, W.M.P. *Process Mining: Data Science in Action.* Springer, 2022.
- PM4Py Capability Atlas: `sources/pm4py/capability-atlas.md`
- wasm4pm Conformance Authority: `sources/wasm4pm/conformance-authority-map.md`
- Type Boundary Matrix: `experiments/pm4py_vs_compat_type_boundary_matrix.md`

---

**Document Revision:** 2026-05-31 | Projection Agent
