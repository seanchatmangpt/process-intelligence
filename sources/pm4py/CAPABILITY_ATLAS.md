# PM4Py Capability Atlas

**Source:** pm4py fork at ~/chatmangpt/pm4py — Process Intelligence Solutions GmbH, 2026
**WASM bridge:** pm4wasm.d.ts (TypeScript type declarations, Rust/wasm-bindgen compiled)
**Purpose:** Complete map of PM4Py capabilities; each entry notes whether the WASM bridge exposes it.

WASM_BRIDGED = YES means the function appears in pm4wasm.d.ts with a "Mirrors pm4py.*" docstring.
WASM_BRIDGED = NO means the capability exists only in the Python library.
WASM_BRIDGED = PARTIAL means a related but not identical function exists in WASM.

---

## 1. Event Log I/O

| Capability | Key Function | Input | Output | WASM Bridged |
|---|---|---|---|---|
| Read XES | `pm4py.read_xes()` | .xes file path | EventLog | YES — `parse_xes_log(xml: string)` |
| Write XES | `pm4py.write_xes()` | EventLog, path | .xes file | YES — `write_xes_log(log_json)` |
| Read CSV | `pm4py.read_csv()` | .csv path | DataFrame | YES — `parse_csv_log(csv: string)` |
| Write CSV | `pm4py.write_csv()` | EventLog/DataFrame | .csv file | YES — `write_csv_log(log_json)` |
| Read PNML | `pm4py.read_pnml()` | .pnml path | PetriNet+Markings | YES — `from_pnml(xml)` / `from_pnml_string(xml)` |
| Write PNML | `pm4py.write_pnml()` | PetriNet+Markings | .pnml file | YES — `to_pnml(pn_json)` / `to_pnml_json(pn_json)` |
| Read BPMN | `pm4py.read_bpmn()` | .bpmn path | BPMN object | YES — `read_bpmn(bpmn_xml)` |
| Read DFG | `pm4py.read_dfg()` | .dfg path | DFG dict | YES — `read_dfg(dfg_json)` |
| Write DFG | `pm4py.write_dfg()` | DFG dict | .dfg file | YES — `write_dfg(dfg_json)` |
| Read PTML (process tree) | file-based | .ptml path | ProcessTree | YES — `from_ptml_string(xml)` / `to_ptml_json(tree_json)` |
| OCEL 2.0 import | `pm4py.read_ocel2_*()` | .json/.xml/.sqlite | OCEL | NO |
| OCEL 2.0 export | `pm4py.write_ocel2_*()` | OCEL | file | NO |
| SAP/ERP connectors | `pm4py.connectors.*` | live system | DataFrame | NO |
| Browser/calendar connectors | `pm4py.connectors.*` | live system | DataFrame | NO |
| Sort log | `pm4py.sort_log()` | EventLog | EventLog (sorted) | YES — `sort_log(log_json)` |
| Project log (attribute subset) | `pm4py.project_log()` | EventLog, attrs | EventLog | YES — `project_log(log_json, attributes_json)` |

---

## 2. Process Discovery

| Capability | Key Function | Input | Output | WASM Bridged |
|---|---|---|---|---|
| Alpha Miner | `pm4py.discover_petri_net_alpha()` | EventLog | PetriNet+Markings | YES — `discover_petri_net_alpha(log_json)` |
| Inductive Miner (Petri net) | `pm4py.discover_petri_net_inductive()` | EventLog | PetriNet+Markings | YES — `discover_petri_net_inductive(log_json)` |
| Inductive Miner (process tree) | `pm4py.discover_process_tree_inductive()` | EventLog | ProcessTree | YES — `discover_process_tree_inductive(log_json)` |
| Inductive Miner (BPMN) | `pm4py.discover_bpmn_inductive()` | EventLog | BPMN XML | YES — `discover_bpmn_inductive(log_json)` |
| Heuristics Miner | `pm4py.discover_heuristics_net()` | EventLog, threshold | HeuristicsNet | YES — `discover_heuristics_miner(log_json, dep_threshold)` |
| Heuristics Net → Petri Net | `pm4py.convert_to_petri_net(hnet)` | HeuristicsNet | PetriNet+Markings | YES — `heuristics_to_petri_net(net_json)` |
| Genetic Miner | `pm4py.discover_petri_net_genetic()` | EventLog, config | PetriNet+Markings | YES — `discover_petri_net_genetic(log_json, config_json)` |
| POWL Discovery (Inductive) | `pm4py.discover_powl()` | EventLog, variant | POWL | PARTIAL — `parse_powl(s)` parses; no discover_powl() in WASM |
| POWL 2.0 ChoiceGraph / mineDG | `InductiveMinerChoiceGraph` | EventLog | POWL (DecisionGraph) | NO |
| DFG Discovery | `pm4py.discover_dfg()` | EventLog | (dfg, starts, ends) | YES — `discover_dfg(log_json)` |
| Performance DFG | `pm4py.discover_performance_dfg()` | EventLog | DFG+durations | YES — `discover_performance_dfg(log_json)` |
| Eventually-Follows Graph | `pm4py.discover_eventually_follows_graph()` | EventLog | EFG dict | YES — `discover_eventually_follows_graph(log_json)` |
| Correlation Mining (case-less) | `pm4py.discover_correlation()` | EventLog, threshold | DFG-like | YES — `discover_correlation(log_json, threshold)` |
| DECLARE Discovery | `pm4py.discover_declare()` | EventLog | DECLARE model | YES — `discover_declare(log_json)` |
| Log Skeleton Discovery | `pm4py.discover_log_skeleton()` | EventLog | LogSkeleton | YES — `discover_log_skeleton(log_json)` |
| Temporal Profile Discovery | `pm4py.discover_temporal_profile()` | EventLog | TemporalProfile | YES — `discover_temporal_profile(log_json)` |
| Batch Processing Detection | `pm4py.discover_batches()` | EventLog | BatchResult | YES — `discover_batches(log_json)` |
| OC-DFG Discovery | `pm4py.discover_ocdfg()` | OCEL | OC-DFG | NO |
| OCPQ Discovery | via `pm4py.ocel.*` | OCEL | OCPQ | NO |

---

## 3. Conformance Checking

| Capability | Key Function | Input | Output | WASM Bridged |
|---|---|---|---|---|
| Token-based Replay | `pm4py.conformance_diagnostics_token_based_replay()` | log, PetriNet+M | per-trace diagnostics | YES — `token_replay_fitness(pn_json, log_json)` |
| Alignment-based Fitness | `pm4py.conformance_diagnostics_alignments()` | log, PetriNet+M | per-trace alignments | YES — `align_log(pn_json, log_json)` |
| Single-trace Alignment | `pm4py.fitness_alignments()` (trace variant) | trace, PetriNet+M | alignment | YES — `align_trace(pn_json, trace_json)` |
| ETConformance Precision | `pm4py.precision_etconformance()` | log, PetriNet+M | precision score | YES — `precision_etconformance(pn_json, log_json)` |
| Footprints Conformance | `pm4py.conformance_diagnostics_footprints()` | log, model | footprint compare | YES — `conformance_footprints(log_json, model_str)` |
| Temporal Conformance | `pm4py.conformance_temporal_profile()` | log, profile, zeta | deviations | YES — `check_temporal_conformance(log_json, profile_json, zeta)` |
| DECLARE Conformance | `pm4py.conformance_declare()` | log, declare | violations | NO |
| Log Skeleton Conformance | `pm4py.conformance_log_skeleton()` | log, skeleton | violations | NO |
| POWL Structural Validation | `validate_partial_orders()` method | POWL | raises/passes | YES — `validate_partial_orders(model)` + `validate_powl_structure(model_str)` |
| Petri Net Soundness Check | `pm4py.check_soundness()` | PetriNet | soundness bool | YES — `check_soundness(pn_json)` |
| Generalization Metric | `pm4py.generalization()` | log, PetriNet | float [0,1] | YES — `generalization(pn_json, log_json)` |
| Streaming Conformance | `pm4py.streaming.*` | event stream, model | live fitness | YES — `streaming_create/push_trace/snapshot` |

---

## 4. Performance Analysis

| Capability | Key Function | Input | Output | WASM Bridged |
|---|---|---|---|---|
| Case Durations | `pm4py.get_case_durations()` | EventLog | list of durations | YES — `get_case_durations(log_json)` |
| All Case Durations | `pm4py.get_all_case_durations()` | EventLog | flat list ms | YES — `get_all_case_durations(log_json)` |
| Performance Stats | internal | EventLog | stats dict | YES — `get_performance_stats(log_json)` |
| Case Arrival Average | `pm4py.get_case_arrival_average()` | EventLog | float (cases/hr) | YES — `get_case_arrival_average(log_json)` |
| Case Overlap | `pm4py.get_case_overlap()` | EventLog | float [0,1] | YES — `get_case_overlap(log_json)` |
| Minimum Self-distances | `pm4py.get_minimum_self_distances()` | EventLog | dict activity→dist | YES — `get_minimum_self_distances(log_json)` |
| Rework Cases | `pm4py.get_rework_cases_per_activity()` | EventLog | rework count | YES — `get_rework_cases_per_activity(log_json)` |
| Rework Times | `pm4py.get_rework_times()` | EventLog | duration list | YES — `get_rework_times(log_json)` |
| Performance Spectrum | `pm4py.discover_performance_spectrum()` | EventLog, activity | spectrum JSON | YES — `discover_performance_spectrum(log_json, activity)` |
| Social Network Analysis | `pm4py.discover_handover_of_work_network()` | EventLog | SNA graph | NO |
| Roles Discovery | `pm4py.discover_roles()` | EventLog | role dict | NO |

---

## 5. Filtering

| Capability | Key Function | Input | Output | WASM Bridged |
|---|---|---|---|---|
| Filter by time range | `pm4py.filter_time_range()` | log, start, end | filtered log | YES — `filter_time_range(log_json, start_ms, end_ms)` |
| Filter by start activities | `pm4py.filter_start_activities()` | log, activities | filtered log | YES — `filter_start_activities(log_json, activities_json)` |
| Filter by end activities | `pm4py.filter_end_activities()` | log, activities | filtered log | YES — `filter_end_activities(log_json, activities_json)` |
| Filter by case size | `pm4py.filter_case_size()` | log, min, max | filtered log | YES — `filter_case_size(log_json, min, max)` |
| Filter by event attribute | `pm4py.filter_event_attribute_values()` | log, key, values | filtered log | YES — `filter_event_attribute_values(log_json, key, vals_json, positive)` |
| Filter by trace attribute | `pm4py.filter_trace_attribute()` | log, key, values | filtered log | YES — `filter_trace_attribute(log_json, key, vals_json, positive)` |
| Filter directly-follows | `pm4py.filter_directly_follows_relation()` | log, a, b | filtered log | YES — `filter_directly_follows_relation(log_json, a, b)` |
| Filter eventually-follows | `pm4py.filter_eventually_follows_relation()` | log, a, b | filtered log | YES — `filter_eventually_follows_relation(log_json, a, b)` |
| Filter between activities | `pm4py.filter_between()` | log, act1, act2 | filtered log | YES — `filter_between(log_json, act1, act2)` |
| Filter prefixes | `pm4py.filter_prefixes()` | log, prefix | filtered log | YES — `filter_prefixes(log_json, prefix_json)` |
| Filter suffixes | `pm4py.filter_suffixes()` | log, suffix | filtered log | YES — `filter_suffixes(log_json, suffix_json)` |
| Filter trim | `pm4py.filter_trim()` | log, start, end | trimmed log | YES — `filter_trim(log_json, start, end)` |
| Filter by variants (top-k) | `pm4py.filter_variants_top_k()` | log, k | filtered log | YES — `filter_variants_top_k(log_json, k)` |
| Filter by variant coverage | `pm4py.filter_variants_reaching()` | log, min_cov | filtered log | YES — `filter_variants_coverage(log_json, min_coverage)` |
| OCEL filtering | `pm4py.filtering.ocel.*` | OCEL | filtered OCEL | NO |

---

## 6. Variants

| Capability | Key Function | Input | Output | WASM Bridged |
|---|---|---|---|---|
| Get variants | `pm4py.get_variants()` | EventLog | variant dict | YES — `get_variants(log_json)` |
| Get variants as tuples | `pm4py.get_variants_as_tuples()` | EventLog | (seq, count) list | YES — `get_variants_as_tuples(log_json)` |
| Variants with path durations | `pm4py.get_variants_paths_duration()` | EventLog | enriched variants | YES — `get_variants_paths_duration(log_json)` |
| Get prefixes | `pm4py.get_prefixes_from_log()` | EventLog | prefix list | YES — `get_prefixes_from_log(log_json)` |
| Attribute values | `pm4py.get_attribute_values()` | log, key | freq dict | YES — `get_attribute_values(log_json, key)` |
| Start activities | `pm4py.get_start_activities()` | log | freq dict | YES — `get_start_activities(log_json)` |
| End activities | `pm4py.get_end_activities()` | log | freq dict | YES — `get_end_activities(log_json)` |
| Case attributes | `pm4py.get_case_attributes()` | log | attr list | YES — `get_case_attributes(log_json)` |
| Event attributes | `pm4py.get_event_attributes()` | log | attr list | YES — `get_event_attributes(log_json)` |
| Trace attributes | `pm4py.get_trace_attributes()` | log | attr list | YES — `get_trace_attributes(log_json)` |
| Trace attribute values | `pm4py.get_trace_attribute_values()` | log, key | freq list | YES — `get_trace_attribute_values(log_json, key)` |

---

## 7. Visualization

| Capability | Key Function | Input | Output | WASM Bridged |
|---|---|---|---|---|
| Visualize DFG | `pm4py.vis.view_dfg()` | DFG | graphviz display | PARTIAL — `dfg_to_dot_wasm(dfg_json)` (DOT string only) |
| DFG to JSON | internal | DFG | JSON string | YES — `dfg_to_json_wasm(dfg_json)` |
| POWL to string | `model.__repr__()` | POWL | str | YES — `powl_to_string(model)` |
| POWL node info | via arena traversal | PowlModel, idx | JSON | YES — `node_info_json(model, idx)` / `node_to_string(model, idx)` |
| POWL children | via arena traversal | PowlModel, idx | Uint32Array | YES — `get_children(model, idx)` |
| Model diff (structural) | `pm4py.diff_models()` | POWL, POWL | diff JSON | YES — `diff_models(model_a, model_b)` |
| Complexity metrics | `pm4py.analysis.*` | POWL | metrics dict | YES — `measure_complexity(model)` |
| Footprints visualization | `pm4py.vis.*` | footprints | display | NO (only compute exposed) |
| Transition system visualization | `pm4py.vis.*` | TransitionSystem | display | NO |

---

## 8. Organizational Mining

| Capability | Key Function | Input | Output | WASM Bridged |
|---|---|---|---|---|
| Handover of Work Network | `pm4py.discover_handover_of_work_network()` | EventLog | graph | NO |
| Working Together Network | `pm4py.discover_working_together_network()` | EventLog | graph | NO |
| Subcontracting Network | `pm4py.discover_subcontracting_network()` | EventLog | graph | NO |
| Similar Activities | `pm4py.discover_network_analysis()` | EventLog | network | NO |
| Role Discovery | `pm4py.discover_roles()` | EventLog | roles dict | NO |
| Batch detection | `pm4py.discover_batches()` | EventLog | batch patterns | YES — `discover_batches(log_json)` |
| LLM few-shot demos | `pm4py.algo.dspy.powl.*` | domain str | demo list | YES — `get_demos_for_domain(domain)` |
| NL → POWL (DSPy) | `PowlPredictor.forward()` | NL description | POWL str | PARTIAL — `generate_code_from_powl(model_str, target)` (inverse direction) |

---

## 9. Prediction / ML

| Capability | Key Function | Input | Output | WASM Bridged |
|---|---|---|---|---|
| Next activity prediction | `pm4py.ml.*` | EventLog, prefix | activity | NO |
| Remaining time prediction | `pm4py.ml.*` | EventLog, prefix | duration | NO |
| Outcome prediction | `pm4py.ml.*` | EventLog, features | label | NO |
| Decision point mining | `pm4py.discover_decision_tree()` | EventLog, net | decision tree | NO |

---

## 10. Object-Centric Process Mining

| Capability | Key Function | Input | Output | WASM Bridged |
|---|---|---|---|---|
| OC-DFG Discovery | `pm4py.discover_ocdfg()` | OCEL | OC-DFG | NO |
| OC-DFG Conformance | `pm4py.conformance_ocdfg()` | OCEL, OC-DFG | fitness | NO |
| OCPQ Query | `pm4py.ocel.*` | OCEL, query | result | NO |
| OCEL type stats | `pm4py.ocel.*` | OCEL | stats | NO |
| OCEL filtering | `pm4py.filtering.ocel.*` | OCEL | filtered OCEL | NO |
| OCEL 2.0 read/write | `pm4py.read_ocel2_*/write_ocel2_*` | files | OCEL | NO |
| Object interaction graph | `pm4py.ocel.*` | OCEL | graph | NO |
| Object type graph | `pm4py.ocel.*` | OCEL | graph | NO |

---

## Summary

- Total distinct capabilities identified: ~120
- WASM bridged (YES): ~75 (primarily discovery, conformance, filtering, variants, I/O)
- Not bridged (NO): ~45 (organizational mining, prediction/ML, OCEL, DECLARE conformance, log skeleton conformance, visualization rendering)
- PARTIAL: ~5 (POWL discover, visualization DOT, NL→POWL direction)

---

## 11. Rigorous Mathematical Specifications & Robustness

### 11.1 Process Discovery Soundness Guarantees
All block-structured discovery (Inductive Miner variants) in PM4Py and its WebAssembly mirror (`wasm4pm`) must satisfy the classic soundness criteria for a Workflow Net $N = (P, T, F, M_0, M_f)$ mapped from a process tree/POWL structure $\mathcal{T}$:
1. **Option to Complete**: $\forall M \in [N, M_0\rangle, \quad M_f \in [N, M\rangle$
2. **Proper Completion**: $\forall M \in [N, M_0\rangle, \quad (M \ge M_f) \implies (M = M_f)$
3. **No Dead Transitions**: $\forall t \in T, \quad \exists M \in [N, M_0\rangle, \quad M \xrightarrow{t}$

### 11.2 Heuristics Miner Loop Dependency Formulations
To ensure correct noisy log and variant fuzzer handling, the discovery algorithms compute deterministic relations:
- **Heuristic Dependency ($a \neq b$)**:
  $$\text{dep}(a, b) = \frac{|a \to b| - |b \to a|}{|a \to b| + |b \to a| + 1}$$
- **Length-1 Loop ($a = b$)**:
  $$\text{dep}(a, a) = \frac{|a \to a|}{|a \to a| + 1}$$
- **Length-2 Loop ($a \to b \to a$)**:
  $$\text{dep}_2(a, b) = \frac{|a \to b \to a| + |b \to a \to b|}{|a \to b \to a| + |b \to a \to b| + 1}$$

### 11.3 Fuzzer and Noisy Log Hardening
The WASM-mirrored Heuristics Miner and binary parser prevent JIT panic traps and memory leaks on corrupted payloads via:
1. **Boundary Limits**: Maximum unique activities limit ($N_{\text{activities}} \le 1,000$) and max event limits ($E_{\max} = 10,000,000$).
2. **Arithmetic Safety**: Event traversal and index offsets use checked arithmetic (`checked_add`, `checked_mul`) returning `OutOfBounds` or `ERR_QUERY_TIMEOUT` error codes on invalid fuzzed inputs.
3. **Deterministic Tie-Breaking**: For all sorting and dependency evaluations, tie-breaks must be resolved using lexicographical comparisons of SHA-256 hashes of activity name labels.
