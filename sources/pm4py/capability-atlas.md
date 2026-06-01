# PM4Py Capability Atlas (v2.7.22.1)

**Oracle Inventory:** Comprehensive mapping of PM4Py as the baseline comparative oracle for process mining. All capabilities enumerated; all limits documented.

**Last Updated:** 2026-05-31

---

## 1. Event Log & Data Structures

### 1.1 Log Types (Core Objects)

| Type | Purpose | Canonical Format |
|------|---------|------------------|
| **EventLog** | Case-centric log (traditional) | XES (.xes) |
| **EventStream** | Event-ordered stream (no grouping) | XES, CSV |
| **OCEL** | Object-centric event log | OCEL 2.0 (.json, .xml, .sqlite, .csv) |
| **DataFrame** | Pandas dataframe representation | CSV, Parquet |

### 1.2 Log Connectors (Data Ingestion)

PM4Py can extract event logs directly from:

- **Calendar Systems:** Outlook Calendar, Outlook Mails
- **VCS:** GitHub (commits, PRs)
- **Browser History:** Chrome, Firefox
- **ERP/SAP:** Order-to-Cash, Accounting modules
- **Workflow Engines:** Camunda Workflow
- **OS Events:** Windows Event Viewer

**Constraint:** All connectors flatten to 2D [CaseID, Timestamp] unless explicitly converted to OCEL.

### 1.3 Default Schema (EventLog/EventStream)

| Column | Purpose | Mandatory |
|--------|---------|-----------|
| `case:concept:name` | Case identifier | Yes |
| `concept:name` | Activity name | Yes |
| `time:timestamp` | Event timestamp | Yes |
| `org:resource` | Resource/actor | Optional |
| Custom attributes | Domain-specific | Optional |

---

## 2. Discovery Algorithms

### 2.1 Process Model Discovery (Transitive Miners)

All discovery algorithms output complete process models (Petri Net, Process Tree, BPMN, POWL, DFG).

#### **2.1.1 Inductive Miner (Family)**
- **Core Algorithm:** `discover_process_tree_inductive` / `discover_petri_net_inductive`
- **Variants:** `im` (basic), `imd` (directly-follows), `imf` (frequency), `imc` (composite)
- **Parameters:**
  - `noise_threshold: float` — Filter traces with occurrence < threshold
  - `multi_processing: bool` — Parallel execution
  - `disable_fallthroughs: bool` — Disable greedy recovery
- **Output:** ProcessTree or (PetriNet, InitialMarking, FinalMarking)
- **Soundness Guarantee:** Yes (by construction)

#### **2.1.2 Alpha Miner**
- **Functions:**
  - `discover_petri_net_alpha` — Classic alpha algorithm
  - `discover_petri_net_alpha_plus` — Improved variant (handles some violations)
- **Output:** (PetriNet, InitialMarking, FinalMarking)
- **Soundness Guarantee:** Only for structured logs (sequential, no concurrency implicit)
- **Constraint:** Requires complete log (no missing start/end events)

#### **2.1.3 Heuristics Miner**
- **Function:** `discover_heuristics_net` (returns HeuristicsNet)
- **Parameters:**
  - `dependency_threshold: float` (0.5 default) — Min strength of dependency
  - `and_threshold: float` (0.65) — Concurrency detection threshold
  - `loop_two_threshold: float` (0.5) — Loop detection threshold
- **Output:** HeuristicsNet (can convert to Petri Net)
- **Use Case:** Tolerates noise; outputs net with quality estimates

#### **2.1.4 Genetic Algorithm Miner**
- **Function:** `discover_petri_net_genetic`
- **Parameters:**
  - `population_size: int` (500 default)
  - `generations: int` (100 default)
  - `mutation_rate: float` (0.01)
  - `crossover_rate: float` (1.0)
  - `elitism_rate: float` (0.01)
- **Output:** (PetriNet, InitialMarking, FinalMarking)
- **Constraint:** Computationally expensive; fitness optimized vs. log

#### **2.1.5 ILP Miner (Integer Linear Programming)**
- **Function:** `discover_petri_net_ilp`
- **Parameters:**
  - `alpha: float` (1.0) — Fitness-precision trade-off weight
- **Output:** (PetriNet, InitialMarking, FinalMarking)
- **Guarantee:** Sound net; precision-oriented

#### **2.1.6 Declare Miner (Constraint-Based)**
- **Function:** `discover_declare`
- **Output:** Dict[Template → Dict[Arguments → Stats]]
- **Supported Templates:** existence, response, precedence, succession, etc.
- **Parameters:**
  - `min_support_ratio: float` — Activity co-occurrence threshold
  - `min_confidence_ratio: float` — Conditional probability threshold
  - `allowed_templates: Set[str]` — Filter to specific constraint types

### 2.2 Intermediate Representations

#### **2.2.1 Directly Follows Graph (DFG)**
- **Functions:**
  - `discover_dfg` → Tuple[Dict, Dict, Dict] (adjacency, start, end)
  - `discover_dfg_typed` → DirectlyFollowsGraph (typed object)
  - `discover_performance_dfg` — DFG with timing metrics
- **Output:** Adjacency matrices (activity pairs + frequency)
- **Use:** Fast discovery for visualization; intermediate input to other miners

#### **2.2.2 Eventually Follows Graph**
- **Function:** `discover_eventually_follows_graph`
- **Output:** Dict[Tuple(activity, activity) → frequency]
- **Semantic:** All paths between A and B in trace

#### **2.2.3 Footprints**
- **Function:** `discover_footprints` (polymorphic on input: EventLog, PetriNet, ProcessTree, POWL)
- **Output:** List[Dict] or single Dict (structure varies by model type)
- **Semantic:** Successor/predecessor relationships + concurrency pairs

#### **2.2.4 Temporal Profile**
- **Function:** `discover_temporal_profile`
- **Output:** Dict[Tuple(A, B) → (min_wait, max_wait)]
- **Use Case:** Timing constraints between consecutive activities

#### **2.2.5 Log Skeleton**
- **Function:** `discover_log_skeleton`
- **Output:** Dict with equivalence classes, predecessor/successor sets
- **Use:** Constraint mining (no explicit model structure)
- **Parameters:**
  - `noise_threshold: float` — Ignore rare variants

#### **2.2.6 Minimum Self-Distance**
- **Function:** `discover_minimum_self_distances`
- **Output:** Dict[activity → [distances between consecutive occurrences]]
- **Use:** Loop detection; concurrency analysis

### 2.3 OCEL-Specific Discovery

#### **2.3.1 Object-Centric DFG (OCDFG)**
- **Function:** `discover_ocdfg`
- **Output:** Dict with event-to-object relations, object interactions, flattened DFGs per object type
- **Parameters:**
  - `business_hours: bool` — Filter to business hours
  - `business_hour_slots: List[Tuple(start, end)]` — Custom hour ranges (default: 9-17 M-F)

#### **2.3.2 Object-Centric Petri Net (OCPN)**
- **Function:** `discover_oc_petri_net`
- **Output:** Dict with petri net per object type + object life-cycle synchronization
- **Parameters:**
  - `inductive_miner_variant: str` — Choose im/imd/imf
  - `diagnostics_with_tbr: bool` — Include token-based replay diagnostics

#### **2.3.3 Event-to-Object (ETOT) Graph**
- **Function:** `discover_etot`
- **Output:** Tuple[Set(objects), Set(obj_types), Set(relations), Dict(relation_counts)]
- **Semantic:** Which objects are related by which events

#### **2.3.4 Object-to-Object (OTG) Graph**
- **Function:** `discover_otg`
- **Output:** Tuple[Set(object_types), Dict(Tuple(A, B, event) → count)]
- **Semantic:** Direct object-type interactions (labeled by event)

#### **2.3.5 Object Interactions Summary**
- **Function:** `ocel_objects_interactions_summary`
- **Output:** Summary table of object co-occurrence patterns

### 2.4 Specialized Miners

#### **2.4.1 Batches**
- **Function:** `discover_batches`
- **Output:** List[Tuple((activity_name, resource), batch_size, timing_info)]
- **Use:** Identify batch processing (grouped executions by same actor)
- **Parameters:** `merge_distance` (max time gap), `min_batch_size`

#### **2.4.2 Organizational Roles**
- **Function:** `discover_organizational_roles`
- **Output:** List[Role] (grouped resources with similar activity profiles)

#### **2.4.3 Social Network Analysis (SNA)**
- **Functions:**
  - `discover_handover_of_work_network` — Work handoffs between resources
  - `discover_working_together_network` — Concurrent collaborations
  - `discover_subcontracting_network` — N-level resource relationships
- **Output:** SNA graph (nodes: resources, edges: interaction type + weight)

#### **2.4.4 Correlation Mining**
- **Function:** Uses `correlation_miner` submodule
- **Use:** Identify correlated event patterns

#### **2.4.5 Prefix Tree / Trie**
- **Function:** `discover_prefix_tree`
- **Output:** Trie (sequence enumeration structure)
- **Parameters:** `max_path_length: int` — Limit depth

#### **2.4.6 Transition System**
- **Function:** `discover_transition_system`
- **Output:** TransitionSystem (state-based model)
- **Parameters:**
  - `window: int` (2 default) — Context window for state definition
  - `view: str` — 'sequence', 'state', or 'performance'
  - `direction: str` — 'forward', 'backward', or 'bidirectional'

#### **2.4.7 Performance Spectrum**
- **Function:** Uses `performance_spectrum` submodule
- **Output:** Time distribution across process variants

---

## 3. Conformance Checking & Replay

### 3.1 Conformance Techniques

#### **3.1.1 Token-Based Replay (TBR)**
- **Functions:**
  - `fitness_token_based_replay` → Dict[metric → float]
  - `conformance_diagnostics_token_based_replay` → List[Dict(per-trace diagnostics)]
  - `replay_prefix_tbr` → Marking (replay single prefix)
  - `generalization_tbr` → float (inverse: overfitting measure)
- **Metrics:**
  - **Fitness:** (1 - missing_tokens) / tokens
  - **Precision:** (1 - produced_tokens) / log_length
- **Input:** PetriNet + InitialMarking + FinalMarking
- **Limitation:** Binary token accounting; does not model data flow or state guards

#### **3.1.2 Alignment-Based Conformance**
- **Functions:**
  - `fitness_alignments` → Dict[metric → float]
  - `conformance_diagnostics_alignments` → List[Dict(per-trace alignment costs)]
  - `precision_alignments` → float
- **Metrics:** Edit distance (moves on log vs. model)
- **Variants:** Multiple alignment algorithms (default: A*)
- **Parameters:** `multi_processing: bool`, `variant_str` (algorithm selection)
- **Limitation:** Computationally expensive (O(log_size² × model_size)); does not scale to large logs

#### **3.1.3 Footprints-Based Conformance**
- **Functions:**
  - `fitness_footprints` → Dict
  - `conformance_diagnostics_footprints` → List[Dict]
  - `precision_footprints` → float
- **Semantic:** Compare successor/predecessor/concurrency pairs
- **Advantage:** Lightweight; tolerates incomplete logs
- **Limitation:** No full path validation; structural only

#### **3.1.4 Declare Conformance**
- **Function:** `conformance_declare`
- **Input:** Declare model (constraint dict) + log
- **Output:** List[Dict(per-trace violations)] or diagnostics dataframe
- **Semantic:** Check constraint violations (existence, response, precedence, etc.)

#### **3.1.5 Log Skeleton Conformance**
- **Function:** `conformance_log_skeleton`
- **Input:** Log skeleton (equivalence classes + relation sets)
- **Output:** List[Set] violations per trace
- **Lightweight:** No model inference; comparison-only

#### **3.1.6 Temporal Profile Conformance**
- **Function:** `conformance_temporal_profile`
- **Input:** Temporal profile (wait time bounds) + log
- **Output:** Per-trace temporal deviations (with `zeta` penalty factor)
- **Parameters:** `zeta: float` (1.0) — Severity weight for violations

#### **3.1.7 OCEL Conformance (Object-Centric)**
- **Functions:**
  - `conformance_ocdfg` — OCDFG model vs. OCEL log
  - `conformance_etot` — ETOT model vs. OCEL
  - `conformance_otg` — OTG (Object-to-Object Graph) model vs. OCEL
- **Metrics:** Behavioral fitness (event/object interaction coverage)
- **Limitation:** No token-replay semantics for OCEL; fitness limited to structure

### 3.2 Quality Dimensions

All conformance functions return a dict with:

```python
{
    'average_trace_fitness': float,  # Per-case conformance rate
    'all_activating': bool,          # Every trace passes (fitness == 1.0)
    'min_trace_fitness': float,      # Worst-fit trace
    'max_trace_fitness': float,      # Best-fit trace
    'log_fitness': float,            # Aggregate (weighted or average)
}
```

**Conformance Dimensions:**
- **Fitness:** % of log replayed without deviation
- **Precision:** % of model behavior observed in log (inverse: overfitting)
- **Generalization:** % of model behavior not in log (robustness to unseen traces)
- **Simplicity:** Structural metrics (places, transitions, arcs)

---

## 4. Format Conversions & I/O

### 4.1 Input Formats (Readers)

| Format | Function | Returns | Notes |
|--------|----------|---------|-------|
| XES | `read_xes` | EventLog | Standard; full attribute preservation |
| CSV | `read_*` (implicit via convert) | DataFrame | Requires column mappings |
| OCEL 2.0 (JSON) | `read_ocel2_json` | OCEL | Object-centric; events × objects matrix |
| OCEL 2.0 (SQLite) | `read_ocel2_sqlite` | OCEL | Relational backend |
| OCEL 2.0 (XML) | `read_ocel2_xml` | OCEL | Legacy OCEL 1.0 support |
| OCEL (custom) | `read_ocel_csv`, `read_ocel_json`, `read_ocel_xml`, `read_ocel_sqlite` | OCEL | Multiple variants |
| PNML | `read_pnml` | (PetriNet, InitialMarking, FinalMarking) | Petri Net XML standard |
| PTML | `read_ptml` | ProcessTree | Process tree XML format |
| BPMN | `read_bpmn` | BPMN | Via external standard |
| DFG | `read_dfg` | DirectlyFollowsGraph | Custom format |

### 4.2 Output Formats (Writers)

| Format | Function | Accepts | Notes |
|--------|----------|---------|-------|
| XES | `write_xes` | EventLog, DataFrame | Lossless round-trip |
| OCEL 2.0 (JSON) | `write_ocel2_json` | OCEL | Modern standard |
| OCEL 2.0 (SQLite) | `write_ocel2_sqlite` | OCEL | Database-backed |
| OCEL 2.0 (XML) | `write_ocel2_xml` | OCEL | Legacy support |
| OCEL (custom variants) | `write_ocel_*` | OCEL | CSV, JSON, XML, SQLite |
| PNML | `write_pnml` | (PetriNet, InitialMarking, FinalMarking) | Petri Net interchange |
| PTML | `write_ptml` | ProcessTree | Process tree interchange |
| BPMN | `write_bpmn` | BPMN | Via external standard |
| DFG | `write_dfg` | DirectlyFollowsGraph | Custom serialization |

### 4.3 Model Format Conversions

```
ProcessTree ↔ BPMN ↔ POWL ↔ PetriNet ↔ HeuristicsNet ↔ GeneticMatrix
                      ↓
                 Reachability Graph (TransitionSystem)
```

**Functions:**
- `convert_to_process_tree` — From any process model
- `convert_to_bpmn` — From PetriNet or ProcessTree
- `convert_to_petri_net` — From BPMN, ProcessTree, POWL, HeuristicsNet, GeneticMatrix
- `convert_to_powl` — From PetriNet, BPMN, ProcessTree
- `convert_to_reachability_graph` — From PetriNet, BPMN, ProcessTree (state explosion risk)

**Data Structure Conversions:**
- `convert_to_dataframe` — EventStream/EventLog → Pandas DataFrame
- `convert_to_event_log` — DataFrame/EventStream → EventLog
- `convert_to_event_stream` — EventLog/DataFrame → EventStream
- `convert_log_to_ocel` — EventLog/DataFrame → OCEL (with object type splitting)
- `convert_log_to_networkx` — EventLog/DataFrame → NetworkX DiGraph
- `convert_ocel_to_networkx` — OCEL → NetworkX (object-centric)
- `convert_petri_net_to_networkx` — PetriNet → NetworkX
- `convert_petri_net_type` — PetriNet variant conversion (e.g., 'classic' → 'workflow_net')
- `convert_log_to_time_intervals` — EventLog → Activity duration pairs

---

## 5. Visualization & Export

### 5.1 Visualization Functions (View & Save)

All functions have paired `view_*` (interactive/in-memory) and `save_vis_*` (PNG/HTML export) versions.

| Visualization | Models/Data | Function |
|---------------|-------------|----------|
| **Petri Net** | PetriNet | `view/save_vis_petri_net` |
| **BPMN** | BPMN | `view/save_vis_bpmn` |
| **Process Tree** | ProcessTree | `view/save_vis_process_tree` |
| **POWL** | POWL | `view/save_vis_powl` |
| **DFG** | DFG or EventLog | `view/save_vis_dfg` |
| **Performance DFG** | Timed DFG | `view/save_vis_performance_dfg` |
| **Heuristics Net** | HeuristicsNet | `view/save_vis_heuristics_net` |
| **Transition System** | TransitionSystem | `view/save_vis_transition_system` |
| **Alignments** | Alignment cost matrix | `view/save_vis_alignments` |
| **Footprints** | Footprints structure | `view/save_vis_footprints` |
| **OCDFG** | OCDFG model | `view/save_vis_ocdfg` |
| **OCPN** | Object-Centric Petri Net | `view/save_vis_ocpn` |
| **Object Graph** | OCEL object relations | `view/save_vis_object_graph` |
| **Performance Spectrum** | Variant timing | `view/save_vis_performance_spectrum` |
| **Dotted Chart** | Timeline by case | `view/save_vis_dotted_chart` |
| **Case Duration Graph** | Histogram (duration) | `view/save_vis_case_duration_graph` |
| **Events Distribution** | Histogram (event freq) | `view/save_vis_events_distribution_graph` |
| **Events per Time** | Time series (event rate) | `view/save_vis_events_per_time_graph` |
| **SNA Network** | SNA graph | `view/save_vis_sna` |
| **Network Analysis** | Custom network | `view/save_vis_network_analysis` |
| **Prefix Tree** | Trie | `view/save_vis_prefix_tree` |

### 5.2 Visualization Formats

- **Interactive:** Graphviz (dot), interactive HTML (for some)
- **Export:** PNG, PDF, SVG
- **Backend:** Graphviz, NetworkX, custom renderers

---

## 6. Statistical Analysis

### 6.1 Log Statistics (pm4py.stats)

| Statistic | Function | Returns |
|-----------|----------|---------|
| **Activity List** | `get_activity_labels` | Set[str] |
| **Start Activities** | `get_start_activities` | Dict[activity → frequency] |
| **End Activities** | `get_end_activities` | Dict[activity → frequency] |
| **Activity Position** | `get_activity_position_summary` | Dict[activity → {start%, middle%, end%}] |
| **Trace Variants** | `get_variants` | Dict[trace_tuple → frequency] |
| **Trace Attributes** | `get_trace_attribute_values` | Dict[attr → set(values)] |
| **Event Attributes** | `get_event_attribute_values` | Dict[attr → set(values)] |
| **Case Duration** | `get_case_duration` → `get_all_case_durations` | List[float] (seconds) |
| **Service Time** (A→B) | `get_service_time` | Dict[activity → {min, mean, max, median, stdev}] |
| **Cycle Time** (case end - start) | `get_cycle_time` | Dict[metric → float] |
| **Sojourn Time** | Uses `passed_time` submodule | Dict[metric → float] |
| **Rework Cases** | `get_rework_cases_per_activity` | Dict[activity → count] |
| **Case Overlap** | `get_case_overlap` | Dict[case_id → overlapping_count] |
| **Minimum Self-Distance** | `get_minimum_self_distances` | Dict[activity → [distances]] |
| **Minimum Self-Distance Witnesses** | `get_minimum_self_distances_witnesses` | Dict[activity → [trace_ids]] |
| **Frequent Trace Segments** | `get_frequent_trace_segments` | List[(segment, frequency)] |
| **Variants as Tuples** | `get_variants_as_tuples` | List[Tuple] (hashable traces) |
| **Variants with Duration** | `get_variants_paths_duration` | Dict[variant → duration_stats] |
| **Process Cube** | `get_process_cube` | Multi-dimensional DataFrame (events, traces, attributes) |
| **Stochastic Language** | `get_stochastic_language` | Dict[trace → probability] |
| **Case Arrival Average** | `get_case_arrival_average` | float (avg events/time_unit) |

### 6.2 OCEL Statistics

| Statistic | Function | Returns |
|-----------|----------|---------|
| **Object Types** | `ocel_get_object_types` | List[str] |
| **Object Attributes** | `ocel_get_attribute_names` | Dict[object_type → List[attr]] |
| **Object Count** | `ocel_objects_ot_count` | Dict[object_type → count] |
| **Objects Summary** | `ocel_objects_summary` | DataFrame (objects × attributes) |
| **Object Interactions** | `ocel_objects_interactions_summary` | DataFrame (co-occurrence matrix) |
| **Temporal Summary** | `ocel_temporal_summary` | Dict[metric → float] (event timing) |
| **Object-Type Activities** | `ocel_object_type_activities` | Dict[object_type → List[activities]] |

---

## 7. Filtering & Data Manipulation

### 7.1 Log Filtering

| Filter | Function | Semantics |
|--------|----------|-----------|
| **By Activity** | `filter_event_attribute_values` | Keep events where attribute = value |
| **By Activity Set %** | `filter_dfg_activities_percentage` | Keep top N% by frequency |
| **By Path %** | `filter_dfg_paths_percentage` | Keep top N% of directly-follows edges |
| **By Time Range** | `filter_between` | Events in [start_time, end_time] |
| **By Case Performance** | `filter_case_performance` | Cases with duration in range |
| **By Case Size** | `filter_case_size` | Cases with N events |
| **By DFG Relation** | `filter_directly_follows_relation` | Cases with edge A→B |
| **By Eventually-Follows** | `filter_eventually_follows_relation` | Cases where A precedes B |
| **By Rework** | `filter_activities_rework` | Cases with activity repetitions |
| **By Resource Attributes** | `filter_activity_done_different_resources` | Cases where activity done by >1 resource |
| **By Four-Eyes Principle** | `filter_four_eyes_principle` | Cases where activity A done by ≥2 resources |
| **By Start/End Activities** | `filter_start_activities`, `filter_end_activities` | Cases starting/ending with activity |
| **By Trace Occurrence** | `filter_log_relative_occurrence_event_attribute` | Traces with event value frequency ≥ threshold |
| **OCEL: By Connectivity** | `filter_ocel_activities_connected_object_type` | OCEL events connecting to object type |
| **OCEL: By Activity** | `filter_ocel_cc_activity` | OCEL connected components with activity |

### 7.2 Data Transformation

| Transform | Function | Output |
|-----------|----------|--------|
| **Merge Cases** | `merge_dataframe_by_* ` | Aggregate events by key |
| **Sample Cases** | `sample_cases` | Random subset of cases |
| **Sample Events** | `sample_events` | Random subset of events |
| **Split by Variant** | `split_by_process_variant` | Dict[variant → sub-log] |
| **OCEL Drop Duplicates** | `ocel_drop_duplicates` | Remove duplicate event-object pairs |
| **OCEL Merge Duplicates** | `ocel_merge_duplicates` | Consolidate identical pairs |
| **OCEL Flattening** | `ocel_flattening` | Convert OCEL → EventLog (object-per-case) |
| **OCEL Enrichment (E2O)** | `ocel_e2o_lifecycle_enrichment` | Add event lifecycle to events |
| **OCEL Enrichment (O2O)** | `ocel_o2o_enrichment` | Add object-to-object relationships |
| **OCEL Timedelta** | `ocel_add_index_based_timedelta` | Add time delta columns |
| **OCEL Sort** | `ocel_sort_by_additional_column` | Reorder by attribute |
| **OCEL Sampling** | `sample_ocel_objects`, `sample_ocel_connected_components` | Subset of objects/components |

---

## 8. Machine Learning & LLM Integration

### 8.1 Feature Extraction (ML Module)

| Feature Set | Function | Target |
|-------------|----------|--------|
| **Outcome Enrichment** | `extract_outcome_enriched_dataframe` | Add target variable (next activity, duration, etc.) |
| **Temporal Features** | `extract_temporal_features_dataframe` | Time-based attributes (hour, day, elapsed) |
| **General Features** | `extract_features_dataframe` | Activity encoding, frequency, sequence position |
| **OCEL Features** | `extract_ocel_features` | Object-centric: object count, event types, relationships |
| **Target Vector** | `extract_target_vector` | Binary/multi-class target (outcome, deviation) |
| **Trace Prefixes** | `get_prefixes_from_log` | All prefixes (for next-activity prediction) |

**Parameters:** Case ID, activity, timestamp, outcome columns; feature selection options.

### 8.2 ML Utilities

- **Train/Test Split:** `split_train_test` (temporal or random)
- **Polars/Pandas Compatibility:** `is_polars_lazyframe`
- **Data Validation:** `check_is_pandas_dataframe`

### 8.3 LLM Integration (pm4py.llm)

| Feature | Function | Backs |
|---------|----------|-------|
| **LLM Query (OpenAI)** | `openai_query` | GPT-4, GPT-3.5, etc. |
| **LLM Query (Google)** | `google_query` | Vertex AI, Gemini |
| **LLM Query (Anthropic)** | `anthropic_query` | Claude models |
| **Abstract Case** | `abstract_case` | Summarize trace to NL |
| **Abstract Log** | Inherent in above | Summarize full log |
| **Abstract DFG** | `abstract_dfg` | Describe process model in words |
| **Abstract Declare** | `abstract_declare` | Explain constraint model |
| **Abstract Petri Net** | `abstract_petri_net` | Process net explanation |
| **Abstract Temporal Profile** | `abstract_temporal_profile` | Timing constraints in NL |
| **Abstract Log Features** | `abstract_log_features` | Log characteristics summary |
| **Abstract OCEL** | `abstract_ocel` | Object-centric log summary |
| **Abstract OCDFG** | `abstract_ocel_ocdfg` | Object-centric DFG explanation |
| **Abstract Variants** | `abstract_variants` | Process variant enumeration |
| **NLP → Log Filter** | `nlp_to_log_filter` | Convert natural language → filter query |
| **NLP → Log Query** | `nlp_to_log_query` | Convert natural language → stats query |
| **Explain Visualization** | `explain_visualization` | Generate caption for plot |
| **Automated Hypotheses** | `automated_hypotheses_formulation` | Generate process hypotheses from log |
| **Clustering** | `clustering` | LLM-based trace clustering |

**Constraint:** Requires API keys (OpenAI, Google, Anthropic); rate-limited.

---

## 9. Organizational & Resource Mining

### 9.1 Resource Graphs

| Network | Function | Semantics |
|---------|----------|-----------|
| **Handover of Work (HoW)** | `discover_handover_of_work_network` | Resource A → Resource B transitions |
| **Working Together (WT)** | `discover_working_together_network` | Resources active in same case |
| **Subcontracting** | `discover_subcontracting_network` | N-level delegation chains (depth N) |
| **Activity-Based Similarity** | `discover_activity_based_resource_similarity` | Resources with similar activity profiles |
| **Network Analysis** | `discover_network_analysis` | Generic graph (custom node/edge columns) |

**Output:** SNA object (graph with nodes, edges, edge weights).

### 9.2 Role Discovery

- **Function:** `discover_organizational_roles`
- **Output:** List[Role] (clustered resources)
- **Semantics:** Group resources by behavior similarity (handover, working-together, or activity patterns)

---

## 10. Process Analysis & Quality Metrics

### 10.1 Structural Analysis (analysis Module)

| Analysis | Function | Returns |
|----------|----------|---------|
| **Soundness** | `check_is_sound`, `check_soundness` | bool (Petri net only) |
| **Workflow Net Check** | `check_is_workflow_net` | bool (single source/sink) |
| **Behavioral Similarity** | `behavioral_similarity` | float [0,1] (trace set similarity) |
| **Structural Similarity** | `structural_similarity` | float [0,1] (graph isomorphism-like) |
| **EMD (Earth Mover's Distance)** | `compute_emd` | float (trace distribution distance) |
| **Label Set Similarity** | `label_sets_similarity` | float [0,1] (activity set overlap) |
| **Embedding Similarity** | `embeddings_similarity` | float [0,1] (semantic embedding distance) |
| **Implicit Place Reduction** | `reduce_petri_net_implicit_places` | Simplified PetriNet |
| **Invisible Transition Reduction** | `reduce_petri_net_invisibles` | Simplified PetriNet |
| **Simplicity (Petri Net)** | `simplicity_petri_net` | float (structural complexity metric) |
| **Synchronous Product** | `construct_synchronous_product_net` | PetriNet (product of two nets) |
| **Marking Equation Solver** | `solve_marking_equation`, `solve_extended_marking_equation` | List[Marking] |
| **Enabled Transitions** | `get_enabled_transitions` | List[Transition] |
| **Maximal Decomposition** | `maximal_decomposition` | List[PetriNet] (sub-nets) |

### 10.2 Utility & Enrichment

| Utility | Function | Effect |
|---------|----------|--------|
| **Insert Artificial Start/End** | `insert_artificial_start_end` | Add explicit source/sink events |
| **Insert Case Metrics** | `insert_case_arrival_finish_rate`, `insert_case_service_waiting_time` | Add duration/rate columns |
| **Replace Activity Labels** | `replace_activity_labels` | Rename activities |
| **Activity Label Extraction** | `get_activity_labels` | List[str] |
| **Map Labels** | `map_labels_from_second_model` | Align activity names across logs |

---

## 11. Object-Centric Event Log (OCEL) Analysis

### 11.1 OCEL Structure

OCEL decouples events from cases. Schema:

```
Events:
  - event_id: unique identifier
  - activity: activity name
  - timestamp: when event occurred
  - ocel:type: [optional] event type
  - attributes: {key: value, ...}

Objects:
  - object_id: unique identifier
  - ocel:type: object type (order, invoice, etc.)
  - attributes: {key: value, ...}

Object-Event Relations:
  - event_id → object_id (many-to-many)
  - qualifier (optional): how object participates (created, processed, paid, etc.)
```

### 11.2 OCEL Extraction & Conversion

| Operation | Function | Semantics |
|-----------|----------|-----------|
| **Create from EventLog** | `convert_log_to_ocel` | Flatten cases → objects (configurable type extraction) |
| **Cluster by Component** | `cluster_equivalent_ocel` | Group objects by connectivity |
| **Sample Objects** | `sample_ocel_objects` | Random object subset |
| **Sample Components** | `sample_ocel_connected_components` | Random connected-component subset |
| **Object Count Per Type** | `ocel_objects_ot_count` | Dict[type → count] |

### 11.3 OCEL Analysis

| Analysis | Function | Output |
|----------|----------|--------|
| **Object Interactions** | `discover_objects_graph` | Set[(obj_type_A, obj_type_B)] edges |
| **OCDFG Discovery** | `discover_ocdfg` | Object-centric DFG (event + object interaction) |
| **OCPN Discovery** | `discover_oc_petri_net` | Petri net synchronized across object lifecycles |
| **ETOT Discovery** | `discover_etot` | Event-to-object relation structure |
| **OTG Discovery** | `discover_otg` | Object-type-to-object-type transitions |
| **Interaction Summary** | `ocel_objects_interactions_summary` | DataFrame (object co-occurrence) |

### 11.4 OCEL I/O

| Format | Read | Write |
|--------|------|-------|
| **JSON** | `read_ocel_json` / `read_ocel2_json` | `write_ocel_json` / `write_ocel2_json` |
| **XML** | `read_ocel_xml` / `read_ocel2_xml` | `write_ocel_xml` / `write_ocel2_xml` |
| **SQLite** | `read_ocel_sqlite` / `read_ocel2_sqlite` | `write_ocel_sqlite` / `write_ocel2_sqlite` |
| **CSV** | `read_ocel_csv` | `write_ocel_csv` |

---

## 12. Limitations & Constraints

### 12.1 Event Log Constraints

| Limit | Value | Implication |
|-------|-------|-------------|
| **Log Size** | ≈10^6 events (hard) | Larger logs trigger OOM or severe slowdown |
| **Case Count** | ≈10^5 cases | Beyond ~100k, discovery becomes intractable |
| **Activity Alphabet** | Unbounded (practical: ≈1k) | High cardinality degrades DFG & alignment performance |
| **Trace Variance** | Unbounded (practical: ≈10k variants) | Genetic algorithm, ILP miners choke on variance explosion |
| **Attribute Cardinality** | No hard limit | High-cardinality attributes inflate memory; weak filtering |

### 12.2 Process Model Constraints

| Model | Constraint | Impact |
|-------|-----------|--------|
| **Petri Net** | No hierarchy; unbounded concurrency | Cannot model hierarchical subprocess; state explosion |
| **Process Tree** | Forced block structure (xor, parallel, sequence, loop) | Cannot model arbitrary flows (e.g., internal loops in parallel blocks) |
| **BPMN** | Diagram semantics loose (visual, not executable) | Conversion to Petri Net lossy; multiple valid interpretations |
| **POWL** | New; partial tool support | Fewer miners; visualization limited |
| **DFG** | No control flow semantics | Cannot distinguish concurrency from choice; no loop semantics |

### 12.3 Discovery Limitations

| Miner | Limitation | Workaround |
|-------|-----------|-----------|
| **Alpha** | Requires complete, acyclic logs | Use Inductive on real-world data |
| **Inductive** | Block-structured output (no internal choice in loops) | Use ProcessTree with POWL conversion |
| **Genetic** | Slow (100+ generations × population); poor convergence on large logs | Use Inductive for speed; Genetic for precision tuning |
| **ILP** | Linear programming overhead; small log requirement | Use Inductive as baseline |
| **Heuristics** | Noise-tolerant but produces nets with quality metrics (subjective) | Combine with conformance for validation |
| **Declare** | Output is template-based, not a unified model | Use as constraint specification, not discovery |
| **All (non-object)** | Assume single case ID; case-centric view | Convert to OCEL for multi-object process |

### 12.4 Conformance Limitations

| Technique | Limitation | Impact |
|-----------|-----------|--------|
| **Token-Based Replay** | Binary token model (no data flow, guards, payloads) | Overly permissive; misses data violations |
| **Alignments** | O(log_size² × model_size) complexity | Intractable on logs > 100k events; impractical on complex nets |
| **Footprints** | Only structural; no full path validation | Lightweight but misses temporal order violations |
| **Declare** | Constraint evaluation only; no model synthesis | Cannot discover new constraints; post-hoc only |
| **Temporal Profile** | Single min/max pair per edge (no distribution quantiles) | Coarse time bounds; misses variability |
| **OCEL Conformance** | No token-replay semantics; fitness = structural coverage | Cannot detect deadlocks, object lifecycle violations |

### 12.5 Visualization Constraints

| Viz | Limit | Issue |
|-----|-------|-------|
| **Petri Net** | ≈500 transitions (visual clutter) | Beyond, graph becomes unreadable |
| **BPMN** | ≈100 elements | Diagram too dense |
| **DFG** | ≈50 activities (performance DFG worse) | Edge explosion makes layout impossible |
| **All Graphviz-backed** | Layout time O(vertices²) | Large models timeout (>60s) |

### 12.6 OCEL Limitations

| Aspect | Limitation |
|--------|-----------|
| **Object Cardinality** | No built-in sharding; if >10k objects, discovery slow |
| **Attribute Inference** | No auto-detection of object types; manual specification required |
| **Event-Object Links** | Many-to-many assumed; qualifier field largely unused |
| **Temporal Semantics** | Events are instantaneous (no duration); intervals not modeled |
| **Conformance** | No object lifecycle validation (state machine per object type) |

### 12.7 Data Type & Format Constraints

| Aspect | Constraint |
|--------|-----------|
| **XES Parsing** | Assumes well-formed XML; silent failures on malformed logs |
| **Timestamp Parsing** | Locale-dependent; ISO 8601 preferred; custom parsing via constants |
| **Case ID Inference** | Manual or default 'case:concept:name'; no auto-discovery |
| **Activity Encoding** | Case-sensitive; whitespace significant; no normalization |
| **Resource Attributes** | Optional; if absent, all events treated as resource='system' |

---

## 13. Foundational Constraints (Oracle Doctrine)

### 13.1 Epistemology

PM4Py operates under assumptions that limit its oracle authority:

1. **Retrospective Completeness:** Assumes logs represent **all possible** lawful behavior. Non-observed paths are assumed impossible, not unknown.
2. **Case-Centric Bias:** Primary model (EventLog, DFG, Petri Net) assumes single case ID drives execution. Multi-object processes force flattening or post-hoc enrichment (OCEL).
3. **Timestamp Monotonicity:** Assumes events are time-ordered and that concurrent events have distinct timestamps. Races, simultaneous distributed events, and out-of-order logging violate this.
4. **Activity Atomicity:** Each event is a discrete activity; no sub-activities, compensation, or transactional boundaries. Duration is inferred, not declared.
5. **Log Accuracy:** PM4Py **does not validate** whether the declared mining process matches the actual runtime process. A model can pass conformance checks while the real system executes differently.

### 13.2 Van der Aalst's Constitution

> **"If the code says it worked but the event log cannot prove a lawful process happened, then it did not work."**

PM4Py is the baseline oracle, but it must be **adversarially validated**:
- Inject impossible logs and confirm rejection.
- Compare discovered model against runtime telemetry.
- Use OCEL conformance to validate object lifecycle soundness.
- Cross-check alignment costs with actual deviations in the system.

---

## 14. Configuration & Extension Points

### 14.1 Parameters & Variants

Most discovery/conformance functions accept:

- `activity_key: str` — Default: `'concept:name'`
- `timestamp_key: str` — Default: `'time:timestamp'`
- `case_id_key: str` — Default: `'case:concept:name'`
- `resource_key: str` — Default: `'org:resource'`
- `opt_parameters: Dict[str, Any]` — Advanced tuning (algorithm-specific)

### 14.2 Variant Selection

- **Inductive:** `im`, `imd`, `imf`, `imc`
- **Alignment:** Multiple A*, beam search, heuristic variants
- **DFG:** Frequency vs. performance (timing) aggregation
- **OCEL Discovery:** Flattening strategies, object type inference

### 14.3 Business Hours & Calendars

- `business_hours: bool` — Filter out off-hours events
- `business_hour_slots: List[Tuple(start_sec, end_sec)]` — Custom business window
- `workcalendar` — Holiday/weekend handling (enterprise feature)

### 14.4 Multi-Processing

- `multi_processing: bool` — Enable parallelism (alignments, genetic algorithm)
- Requires picklable objects; not all analyses support it

---

## 15. Integration with SPARC Stack

### 15.1 PM4Py as Oracle

PM4Py is the **baseline comparative** for:

- **Process Discovery:** All miners are heuristic; none guarantee optimality.
- **Conformance Validation:** Token replay and alignment provide necessary but insufficient proof.
- **Temporal Analysis:** Time-aware discovery (performance DFG, temporal profile) but no compensation/backtracking.
- **Object-Centric Analysis:** OCEL support added in 2.7+; still immature (no lifecycle validation).

### 15.2 Comparative Limitations vs. WASM4PM & CodeManufactory

| Capability | PM4Py | WASM4PM | CodeManufactory |
|-----------|-------|--------|-----------------|
| **Real-time Discovery** | ✓ Batch only | ✓ Streaming | ✓ Continuous (AGI-driven) |
| **Object Lifecycle Validation** | ✗ (structural only) | ✓ (state machine) | ✓ (full causality) |
| **Predictive Conformance** | ✗ (post-hoc) | ✗ (reactive) | ✓ (proactive anomaly injection) |
| **Distributed Trace Synthesis** | ✗ (log-centric) | ✓ (OTel aware) | ✓ (AGI-coordinated) |
| **Model Mutation Tolerance** | ✗ (static net) | ✓ (variant-aware) | ✓ (adversarial learning) |
| **Receipt Anchoring** | ✗ (no proof) | ✓ (event hashing) | ✓ (proof-of-actuation) |

---

## 16. Complete Function Index

**Key Exports (Top-Level):**

- `discover_*` — 34 discovery functions
- `fitness_*`, `precision_*`, `conformance_*` — 21 conformance functions
- `read_*`, `write_*` — 29 I/O functions
- `convert_*` — 12 model conversion functions
- `view_*`, `save_vis_*` — 44 visualization pairs
- `filter_*` — 20+ filtering operations
- `stats/*` — 50+ statistical operations
- `ocel_*` — 25 OCEL-specific utilities
- `llm.{openai,google,anthropic}_query` — 3 LLM backends
- `ml.extract_*` — 5 ML feature extractors

**Submodules:** `algo`, `objects`, `util`, `stats`, `vis`, `read`, `write`, `convert`, `ocel`, `llm`, `ml`, `filtering`, `analysis`, `conformance`, `org`

---

## 17. Recommended Usage Patterns

### Pattern 1: Full Conformance Validation Pipeline

```python
# 1. Discover model
tree = pm4py.discover_process_tree_inductive(log, noise_threshold=0.1)
net, im, fm = pm4py.convert_to_petri_net(tree)

# 2. Check conformance (multi-metric)
fitness = pm4py.fitness_alignments(log, net, im, fm)
precision = pm4py.precision_alignments(log, net, im, fm)
diagnostics = pm4py.conformance_diagnostics_alignments(log, net, im, fm)

# 3. Visualize model + deviations
pm4py.view_petri_net(net, im, fm)
pm4py.view_alignments(log, net, im, fm, diagnostics=diagnostics)
```

### Pattern 2: OCEL Object-Centric Analysis

```python
# 1. Discover OCDFG
ocdfg = pm4py.discover_ocdfg(ocel)
ocpn = pm4py.discover_oc_petri_net(ocel)

# 2. Conformance (object-centric)
ocdfg_fitness = pm4py.conformance_ocdfg(ocel, ocdfg)

# 3. Object interaction graphs
objects_graph = pm4py.discover_objects_graph(ocel)
```

### Pattern 3: Iterative Noise Filtering

```python
# Start with low noise tolerance; increase if fitness too low
for threshold in [0.0, 0.05, 0.1, 0.2]:
    tree = pm4py.discover_process_tree_inductive(log, noise_threshold=threshold)
    fitness = pm4py.fitness_token_based_replay(log, *pm4py.convert_to_petri_net(tree))
    print(f"Threshold {threshold}: Fitness {fitness['log_fitness']}")
```

---

## References

- **PM4Py Docs:** https://pm4py.fit.fraunhofer.de/
- **Van der Aalst, W.M.P.:** *Process Mining: Data Science in Action* (2nd Ed.)
- **OCEL Standard:** https://www.ocel-standard.org/
- **XES Standard:** http://xes-standard.org/

---

**Document Revision:** 2026-05-31 | PM4Py v2.7.22.1 | Discovery Agent Inventory
