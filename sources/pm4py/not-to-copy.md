# PM4Py Capabilities: NOT-TO-COPY for wasm4pm

**Purpose:** Identify PM4Py features that must NOT be ported to wasm4pm due to semantic misalignment, architectural unfitness, or conformance risk.

**Last Updated:** 2026-05-31

**Epistemology:** If the code works in PM4Py but cannot be ported to wasm4pm without losing proof-of-execution guarantees, then it is unsafe to copy. — Projection Doctrine

---

## 1. Dynamic Typing & Permissive Coercion

### 1.1 Why NOT to Copy: Pandas NaN Coercion

**PM4Py Behavior:**
- Silently converts missing numeric values to `np.float64(NaN)`.
- Operations on NaN cascade silently (e.g., `sum([1, 2, NaN, 3])` → `NaN`).
- No error raised; downstream code may compute meaningless results.

**Example:**
```python
# PM4Py permits this:
log = {"case_id": [1, 1, 2, 2],
       "timestamp": ["2024-01-01", "2024-01-02", "2024-01-03", None],  # None → NaT
       "cost": [100.0, None, 200.0, 150.0]}  # None → NaN
df = pd.DataFrame(log)
mean_cost = df["cost"].mean()  # Returns 150.0, silently ignoring NaN
```

**Why wasm4pm CANNOT Copy:**
1. **No silent failure.** wasm4pm must reject malformed inputs at the admission boundary.
2. **Type integrity.** `cost` is numeric; `null` is not a valid number. The type system forbids implicit coercion.
3. **Proof chain breaks.** If a fitness metric aggregates over NaN values, the conformance certificate is void.

**Compat Rule:**
```
✗ FORBIDDEN: Implicit NaN propagation
✓ REQUIRED: Reject on admission with RefusalReport
  {
    "violation": "TypeBoundaryError",
    "field": "cost",
    "expected_type": "number",
    "found": "null",
    "location": "event_id: evt_004"
  }
```

---

### 1.2 Why NOT to Copy: Dict Mutation & Attribute Injection

**PM4Py Behavior:**
- Event attributes are arbitrary Python dicts; keys can be added, modified, or deleted at runtime.
- No schema enforcement; attributes grow unbounded.

**Example:**
```python
# PM4Py permits this:
event = {
    "concept:name": "Approve",
    "time:timestamp": "2024-01-01T10:00:00",
    "custom_field": "value1"
}
# Later, another function adds:
event["new_field"] = "value2"
# Or overwrites:
event["concept:name"] = "Reject"  # Original meaning lost
```

**Why wasm4pm CANNOT Copy:**
1. **Immutability required.** Events are evidence; they cannot be mutated after creation.
2. **Schema lock.** Attributes must be declared upfront; no late injection.
3. **Proof invalidation.** If an event is retroactively modified, all downstream conformance checks become unreliable.

**Compat Rule:**
```
✗ FORBIDDEN: Mutable event attributes
✓ REQUIRED: Immutable record with sealed schema
  {
    "event_id": "evt_001",
    "activity": "Approve",
    "timestamp_ns": 1704110400000000000,
    "attributes": {
      "cost": 500.0,
      "resource_id": "agent_42"
    }
    // No additional fields permitted
  }
```

---

## 2. Batch Operations Without Atomicity

### 2.1 Why NOT to Copy: Lazy DataFrames & Vectorized Mutations

**PM4Py Behavior:**
- Pandas operations are lazy (not executed immediately).
- Vectorized mutations (e.g., `df["cost"] = df["cost"] * 1.1`) modify in-place with no transaction semantics.
- Exceptions mid-operation leave the log in an inconsistent state.

**Example:**
```python
# PM4Py permits this:
df["duration"] = df["end_time"] - df["start_time"]  # May fail partway
df.loc[df["cost"] > 1000, "risk_flag"] = True  # Exception leaves NaN entries
# No rollback; log is corrupted.
```

**Why wasm4pm CANNOT Copy:**
1. **Atomicity required.** All-or-nothing semantics for data transformations.
2. **No partial state.** Either a filter/transformation succeeds completely, or the log remains unchanged.
3. **Audit trail.** Each operation must be logged as an atomic event; no in-flight states.

**Compat Rule:**
```
✗ FORBIDDEN: Lazy/in-place vectorized mutations
✓ REQUIRED: Atomic transformation with rollback on failure
  transform(log, filter_fn) → Evidence<EventLog, Filtered, TransformationReceipt>
  // Entire transformation succeeds or entire log reverts
```

---

## 3. Non-Deterministic Algorithm Outputs

### 3.1 Why NOT to Copy: Genetic Algorithm Non-Seeded Variance

**PM4Py Behavior:**
- Genetic algorithm miner uses random initialization; without a fixed seed, outputs vary between runs.
- Parameters like `population_size`, `generations`, `mutation_rate` are heuristic; no guarantee on optimality.

**Example:**
```python
# PM4Py permits this:
net1 = pm4py.discover_petri_net_genetic(log1)
net2 = pm4py.discover_petri_net_genetic(log1)  # Different net, same log
assert net1 == net2  # FAILS!
```

**Why wasm4pm CANNOT Copy (As-Is):**
1. **Determinism required.** Same log + same seed → same output.
2. **Fitness proof.** A conformance certificate claims fitness *for this specific model*; variance breaks that claim.
3. **M&A defensibility.** If a model used in a diligence claim is non-deterministic, auditors reject it.

**Compat Rule:**
```
✗ FORBIDDEN: Unseeded randomness in discovery
✓ REQUIRED: Explicit seed parameter; deterministic output
  discover_petri_net_genetic(log, seed=42) → Evidence<PetriNet, OptimizedByGA, FitnessCert>
  // EVERY call with seed=42 returns IDENTICAL net
```

---

### 3.2 Why NOT to Copy: Heuristic Quality Metrics

**PM4Py Behavior:**
- Heuristics miner produces `HeuristicsNet` with quality metrics (thresholds, confidence).
- These metrics are heuristic estimates, not formal proofs.

**Example:**
```python
# PM4Py permits this:
hnet = pm4py.discover_heuristics_net(log)
print(hnet.dependency_threshold)  # 0.5 (arbitrary default)
print(hnet.and_threshold)  # 0.65 (arbitrary)
# No formal semantics; values are tuning parameters, not proofs.
```

**Why wasm4pm CANNOT Copy (Without Anchoring):**
1. **Proof liability.** Quality metrics must be anchored to formal measures (e.g., fitness, precision).
2. **Defensibility.** An M&A claim backed by "confidence = 0.75 (heuristic)" is rejected; formal evidence required.
3. **Type integrity.** Heuristic metrics are advisory (`Evidence<HeuristicsNet, AnalyzedWithMetrics, ConfidenceVector>`); cannot be treated as ground truth.

**Compat Rule:**
```
✗ FORBIDDEN: Heuristic metrics treated as proof
✓ REQUIRED: Separate heuristic estimates from formal proofs
  HeuristicsNet → Evidence<HeuristicsNet, AnalyzedWithMetrics, ConfidenceVector>
                      [Advisory; not suitable for conformance]
  
  ALSO REQUIRED: Formal validation (e.g., token-based replay fitness)
  Evidence<PetriNet, ValidatedSound, FitnessDiagnostics>
                      [Formal proof; suitable for audit]
```

---

## 4. Unvalidated External Data Integrations

### 4.1 Why NOT to Copy: Calendar Connectors & VCS Integrations

**PM4Py Capability:**
- Can ingest logs directly from Outlook Calendar, GitHub, Chrome history, Camunda workflows, Windows Event Viewer.
- No built-in validation; raw data is trusted implicitly.

**Example:**
```python
# PM4Py permits this:
log = pm4py.read_log_from_github_repo("my_org/my_repo", activity_key="commit_type")
# No verification: repo could be hijacked, commits forged, timestamps tampered.
```

**Why wasm4pm CANNOT Copy:**
1. **Untrusted source.** External systems are not under wasm4pm control; data must be authenticated.
2. **Proof chain broken.** A conformance proof is only valid if the log has cryptographic attestation (e.g., commit signature, API token validation).
3. **Liability exposure.** If a log is later found to be forged, any claim based on it is void.

**Compat Rule:**
```
✗ FORBIDDEN: Direct ingestion from external systems
✓ REQUIRED: Explicit authentication + cryptographic receipt
  read_log_from_github(repo, auth_token=token, signature_check=True)
    → Evidence<EventLog, ParsedWithAttestation, GitHubSignature>
  
  Or: Require pre-exported + signature-verified export file
```

---

### 4.2 Why NOT to Copy: LLM Abstractions

**PM4Py Capability:**
- LLM integration (`pm4py.llm.openai_query`, `anthropic_query`, etc.) for NL abstractions (summarize logs, explain models, generate hypotheses).
- Outputs are non-deterministic and subject to model versioning, API rate limits, pricing changes.

**Example:**
```python
# PM4Py permits this (may vary):
summary = pm4py.llm.anthropic_query(
    prompt="Summarize this log:",
    model="claude-3-sonnet",  # Version changes without notice
    api_key=os.getenv("ANTHROPIC_API_KEY")  # Key may be compromised
)
# Output varies by model version; not reproducible.
```

**Why wasm4pm CANNOT Copy (Into Conformance):**
1. **Non-deterministic.** LLM outputs are not reproducible.
2. **Proof-incompatible.** A claim backed by "the LLM said so" is not defensible in audit.
3. **External dependency.** If the LLM service changes pricing or access, the proof becomes unreliable.

**Compat Rule:**
```
✗ FORBIDDEN: LLM outputs in conformance proof chains
✓ PERMITTED: LLM outputs as advisory annotations only
  Evidence<EventLog, Parsed, XESValidationReport>
    + LLMAnnotation["This log looks like an order-to-cash process"]  [Advisory]
  
  NOT: Evidence<EventLog, ValidatedSound, LLMConcurrence>  [INVALID]
```

---

## 5. Lossy Format Conversions Without Auditing

### 5.1 Why NOT to Copy: Silent Flattening (OCEL → XES)

**PM4Py Behavior:**
- OCEL (object-centric) → EventLog (case-centric) flattening is supported but silent.
- Objects are converted to trace attributes; relationships are lost without a loss report.

**Example:**
```python
# PM4Py permits this:
ocel = pm4py.read_ocel2_json("events.json")  # Multi-object process
event_log = pm4py.convert_ocel_to_event_log(ocel)  # Silent flattening
# Original object relations are gone; no loss report generated.
# Downstream conformance checks are invalid.
```

**Why wasm4pm CANNOT Copy (Without Auditing):**
1. **Semantic loss untracked.** Flattening discards inter-object relationships; conformance checks become unsound.
2. **Audit failure.** An M&A claim based on flattened data is indefensible (original source was multi-object).
3. **Type incompatibility.** OCEL and EventLog are fundamentally different types; conversion is lossy and requires explicit LossPolicy.

**Compat Rule:**
```
✗ FORBIDDEN: Silent flattening
✓ REQUIRED: Explicit LossPolicy + LossReport
  convert_ocel_to_event_log(ocel, loss_policy="flatten_objects") 
    → (EventLog, LossReport{
         "relations_lost": 156,
         "object_types_flattened": 3,
         "impact": "Object lifecycle constraints unvalidatable"
       })
```

---

### 5.2 Why NOT to Copy: BPMN → Petri Net With Ambiguous Semantics

**PM4Py Behavior:**
- BPMN constructs (OR-joins, complex gateways) have multiple valid interpretations when converted to Petri Nets.
- PM4Py picks one interpretation without documenting the choice.

**Example:**
```python
# PM4Py permits this:
bpmn = pm4py.read_bpmn("process.bpmn")
net, im, fm = pm4py.convert_to_petri_net(bpmn)
# If BPMN contains OR-join, there are 2^n possible Petri Net translations.
# PM4Py picks one silently; no documentation of which.
```

**Why wasm4pm CANNOT Copy:**
1. **Non-canonical.** BPMN → Petri Net is one-to-many; wasm4pm must choose and declare the semantics.
2. **Proof liability.** A conformance check on the wrong net is invalid.
3. **Interoperability risk.** Another tool may interpret the same BPMN differently; proofs don't transfer.

**Compat Rule:**
```
✗ FORBIDDEN: Ambiguous BPMN conversion
✓ REQUIRED: Declare semantic profile
  convert_to_petri_net(bpmn, bpmn_semantics="standard_iso")
    → Evidence<PetriNet, Converted, BPMNSemanticProfile{
         "or_join_interpretation": "separate_choice_per_token",
         "loop_semantics": "do-while"
       }>
```

---

## 6. Unlimited Streaming & Out-of-Core Processing

### 6.1 Why NOT to Copy: Unbounded Event Streams

**PM4Py Capability:**
- `EventStream` is designed for streaming input; no requirement that logs fit in memory.
- Algorithms may operate on partial streams, producing incremental outputs.

**Example:**
```python
# PM4Py permits this:
stream = pm4py.read_event_stream("events.csv")
for batch in stream.iter_batches(size=1000):
    partial_dfg = pm4py.discover_dfg(batch)
    # DFG changes with each batch; no final model guaranteed.
```

**Why wasm4pm CANNOT Copy (Without Semantic Lock):**
1. **Non-converging.** Streaming discovery may never reach a stable model.
2. **Proof snapshot required.** A conformance proof is for a *specific model at a specific time*; streaming breaks this.
3. **Memory bounds.** wasm4pm has hard memory limits; unbounded streaming is unsafe.

**Compat Rule:**
```
✗ FORBIDDEN: Incremental streaming without convergence guarantee
✓ REQUIRED: Batch or streaming with explicit convergence check
  discover_dfg_streaming(stream, convergence_check=True)
    → Evidence<DirectlyFollowsGraph, Discovered, DFGSnapshot{
         "timestamp": "2026-05-31T12:00:00Z",
         "final": true,  // Confirms convergence
         "events_processed": 1000000
       }>
```

---

### 6.2 Why NOT to Copy: Out-of-Core Algorithms

**PM4Py Behavior:**
- Some algorithms (e.g., genetic algorithm on large logs) may swap to disk.
- No explicit bounds on memory or compute time.

**Example:**
```python
# PM4Py permits this (may swap to disk / OOM):
net = pm4py.discover_petri_net_genetic(
    huge_log,
    population_size=10000,  # 10k solutions × huge log = OOM risk
    generations=1000
)
# May hang, crash, or produce incorrect result.
```

**Why wasm4pm CANNOT Copy:**
1. **Bounded execution required.** wasm4pm operates in sandboxed memory; no disk swapping.
2. **Proof requires bounded resources.** A fitness certificate is only valid if resources were bounded and deterministic.
3. **Real-time constraints.** wasm4pm is event-driven; algorithms must complete in bounded time per receipt.

**Compat Rule:**
```
✗ FORBIDDEN: Unbounded memory/compute discovery
✓ REQUIRED: Explicit resource bounds
  discover_petri_net_genetic(
    log,
    population_size=500,  // Bounded
    max_memory_mb=256,    // Explicit limit
    timeout_seconds=60    // Explicit timeout
  ) → Evidence<PetriNet, OptimizedByGA, ResourceBound{
        "memory_used_mb": 145,
        "time_seconds": 23.5,
        "generations_completed": 45
      }>
```

---

## 7. Post-Hoc Validation & Explanation

### 7.1 Why NOT to Copy: Behavioral/Structural Similarity (Without Proof)

**PM4Py Capability:**
- `behavioral_similarity`, `structural_similarity`, `embeddings_similarity` are computed after-the-fact.
- Scores are heuristic estimates; no formal semantics.

**Example:**
```python
# PM4Py permits this:
similarity = pm4py.behavioral_similarity(log1, log2)
print(f"Similarity: {similarity:.3f}")  # 0.847 (what does this mean?)
# No formal definition; just a number.
```

**Why wasm4pm CANNOT Copy (Into Proof Chains):**
1. **Undefined semantics.** What does "0.847 similar" mean formally?
2. **Non-canonical.** Different similarity metrics yield different values; no universal ground truth.
3. **Proof liability.** An audit claim "Models are similar (similarity=0.847)" is rejected without formal definition.

**Compat Rule:**
```
✗ FORBIDDEN: Similarity scores in conformance proofs
✓ REQUIRED: Explicit metric definition + bounds
  compute_similarity(log1, log2, metric="jaccard")
    → Evidence<SimilarityScore, AnalyzedBehavior, JaccardSimilarity{
         "value": 0.847,
         "metric": "trace_jaccard_index",
         "definition": "https://en.wikipedia.org/wiki/Jaccard_index"
       }>
         [Advisory; not proof; may be used for filtering/ranking]
```

---

## 8. Configuration & Parameterization Without Bounds

### 8.1 Why NOT to Copy: Unconstrained opt_parameters Dict

**PM4Py Behavior:**
- Most functions accept `opt_parameters: Dict[str, Any]` for advanced tuning.
- Interpretation varies by function; no validation or bounds.

**Example:**
```python
# PM4Py permits this:
tree = pm4py.discover_process_tree_inductive(
    log,
    opt_parameters={
        "noise_threshold": -0.5,  # Invalid? Allowed anyway.
        "unknown_param": "xyz",   # Ignored silently.
        "memory_limit_gb": 1e10   # Huge; no bounds check.
    }
)
```

**Why wasm4pm CANNOT Copy (Directly):**
1. **Type safety.** Every parameter must be typed and validated.
2. **Bounds checking.** Parameters must be within sensible ranges (e.g., 0 ≤ noise_threshold ≤ 1).
3. **Discoverability.** Parameter semantics must be documented and enforced at compile time.

**Compat Rule:**
```
✗ FORBIDDEN: Unconstrained opt_parameters dict
✓ REQUIRED: Strongly typed parameter struct
  struct InductiveParams {
    noise_threshold: f64,      // [0.0, 1.0]
    multi_processing: bool,
    disable_fallthroughs: bool
  }
  
  discover_process_tree_inductive(log, params: InductiveParams)
    → Evidence<ProcessTree, Discovered, TreeStructure>
    // Type checker rejects invalid params at compile time
```

---

## 9. Visualization & Rendering

### 9.1 Why NOT to Copy: Graphviz-Based Rendering

**PM4Py Behavior:**
- All Petri Net, BPMN, DFG visualizations use Graphviz (dot language).
- Rendering depends on system Graphviz installation; output layout non-deterministic across platforms.

**Example:**
```python
# PM4Py permits this:
pm4py.view_petri_net(net, im, fm)
# Rendering depends on: Graphviz version, OS, available fonts.
# Layout is non-deterministic; two runs produce visually different (but semantically identical) results.
```

**Why wasm4pm CANNOT Copy (Directly Into WASM):**
1. **WASM sandbox.** No system Graphviz available in WASM; external call required.
2. **Non-determinism.** Rendering depends on host environment; not reproducible in sandbox.
3. **Proof irrelevance.** Visualization is post-hoc; not part of conformance proof chain.

**Compat Rule:**
```
✗ FORBIDDEN: WASM-native Graphviz rendering
✓ PERMITTED: Export data; render on host
  Evidence<PetriNet, Discovered, PetriNetStructure>
    + export_as_dot() → "digraph {...}"
    // Host system renders dot → PNG
  // Visualization is advisory; not proof
```

---

## 10. Version-Dependent & Deprecated Features

### 10.1 Why NOT to Copy: Legacy EventLog Format & XES Variants

**PM4Py Behavior:**
- Supports both modern (XES 2.0) and legacy (XES 1.x) formats.
- Attribute names vary (e.g., `'concept:name'` vs custom naming).
- Some attributes are optional in XES, leading to implicit defaults.

**Example:**
```python
# PM4Py permits this (legacy):
log = pm4py.read_xes("old_log.xes")
# May have: case:concept:name, concept:name, time:timestamp
# OR: case_id, activity, timestamp (custom naming)
# OR: Missing time:timestamp; inferred from event order
# Parser must guess intent; fragile.
```

**Why wasm4pm CANNOT Copy (Without Modernization):**
1. **Schema lock.** wasm4pm supports only XES 2.0 (canonical standard).
2. **No guessing.** Missing fields → admission rejection, not implicit defaults.
3. **Audit trail.** If a legacy log is normalized, the normalization strategy must be explicit.

**Compat Rule:**
```
✗ FORBIDDEN: Legacy XES 1.x parsing with implicit defaults
✓ REQUIRED: XES 2.0 strict schema; reject non-conformant logs
  read_xes("log.xes", version="2.0", strict=true)
    → Evidence<EventLog, Parsed, XES2ValidationReport>
    // OR: RefusalReport if XES 1.x detected
```

---

## 11. Operational Checklist: Features NOT to Port

For each PM4Py feature considered for wasm4pm:

- [ ] **Is it permissively typed?** (NaN coercion, dict mutation) → DO NOT COPY. Enforce strict types.
- [ ] **Is it non-deterministic without seeding?** (GA, randomized heuristics) → DO NOT COPY as-is. Add mandatory seed parameter.
- [ ] **Is it heuristic without formal semantics?** (quality metrics, similarity scores) → DO NOT COPY into proof chains. Tag as advisory.
- [ ] **Does it integrate with untrusted external systems?** (calendars, GitHub, LLMs) → DO NOT COPY. Require explicit authentication + attestation.
- [ ] **Does it silently lose information?** (OCEL flattening, BPMN conversion) → DO NOT COPY. Require LossPolicy + LossReport.
- [ ] **Does it use unbounded memory/compute?** (streaming, genetic with huge populations) → DO NOT COPY. Add explicit bounds.
- [ ] **Does it depend on system state?** (Graphviz, OS events, locale) → DO NOT COPY into WASM. Export data only; render on host.
- [ ] **Is it deprecated or version-dependent?** (legacy XES, old OCEL) → DO NOT COPY. Modernize to canonical standard first.

If ANY of these apply → **Feature is NOT suitable for wasm4pm without major architectural changes.**

---

## 12. Summary: What NOT to Port

### High Risk (NEVER without redesign)
1. Dynamic typing & NaN coercion
2. Mutable event attributes
3. Batch operations without atomicity
4. Non-deterministic discovery (unseeded GA)
5. Unvalidated external integrations
6. LLM outputs in proof chains
7. Lossy conversion without loss reports
8. Unbounded streaming/compute

### Medium Risk (ONLY with explicit safeguards)
1. Heuristic quality metrics (tag as advisory)
2. Similarity scores (tag as advisory)
3. Unconstrained parameter dicts (add bounds & typing)
4. Legacy format variants (modernize first)

### Low Risk (Safe to port with minor adaptation)
1. Visualization & rendering (export data; render on host)
2. Post-hoc analysis & explanation (tag as advisory)

---

## 13. References

- Divergence Matrix: `sources/pm4py/divergence-matrix.md`
- PM4Py Capability Atlas: `sources/pm4py/capability-atlas.md`
- Type Boundary Matrix: `experiments/pm4py_vs_compat_type_boundary_matrix.md`
- wasm4pm Conformance Authority: `sources/wasm4pm/conformance-authority-map.md`

---

**Document Revision:** 2026-05-31 | Projection Agent
