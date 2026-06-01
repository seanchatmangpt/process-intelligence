# PM4Py Oracle Map — From pm4wasm.d.ts

**Source:** ~/chatmangpt/pm4py/pm4wasm/pkg/pm4wasm.d.ts
**Method:** Every function with a "Mirrors pm4py.*" docstring is listed here as an authoritative oracle entry.
**Purpose:** Enumerate what wasm4pm must provide as Rust types — the WASM bridge defines the surface contract.

Each row: PM4Py Python function | WASM function | Input types | Output type | What wasm4pm should provide

---

## Discovery Functions

| PM4Py Python | WASM Function | WASM Inputs | WASM Output | wasm4pm Surface |
|---|---|---|---|---|
| `pm4py.discover_dfg()` | `discover_dfg(log_json)` | `string` (event log JSON) | `string` (JSON: edges, start_activities, end_activities, activities) | `Dfg` admitted evidence type; `discover_dfg` operator |
| `pm4py.discover_performance_dfg()` | `discover_performance_dfg(log_json)` | `string` | `string` (JSON: edges with avg/min/max_duration_ms) | `PerformanceDfg` with duration-annotated arcs |
| `pm4py.discover_eventually_follows_graph()` | `discover_eventually_follows_graph(log_json)` | `string` | `string` (JSON array: source, target, count) | `EventuallyFollowsGraph` structure |
| `pm4py.discover_petri_net_alpha()` | `discover_petri_net_alpha(log_json)` | `string` | `string` (JSON: net, initial_marking, final_marking) | `AlphaMinerResult` with `PetriNet` + markings |
| `pm4py.discover_petri_net_inductive()` | `discover_petri_net_inductive(log_json)` | `string` | `string` (JSON: net, initial_marking, final_marking) | `InductiveMinerPetriResult` |
| `pm4py.discover_bpmn_inductive()` | `discover_bpmn_inductive(log_json)` | `string` | `string` (BPMN 2.0 XML) | `BpmnXml` newtype around discovered process |
| `pm4py.discover_process_tree_inductive()` | `discover_process_tree_inductive(log_json)` | `string` | `string` (JSON: label, operator, children) | `ProcessTree` with `operator` in {->, X, +, *} |
| `pm4py.discover_heuristics_miner()` | `discover_heuristics_miner(log_json, dependency_threshold)` | `string`, `f64` | `string` (JSON: activities, dependency measures, starts/ends) | `HeuristicsNet` with dependency threshold |
| `pm4py.discover_declare()` | `discover_declare(log_json)` | `string` | `string` (JSON: constraint templates, support, confidence) | `DeclareModel` with constraint types |
| `pm4py.discover_log_skeleton()` | `discover_log_skeleton(log_json)` | `string` | `string` (JSON: equivalence, always_after, always_before, never_together, directly_follows, activ_freq) | `LogSkeleton` with six constraint sets |
| `pm4py.discover_temporal_profile()` | `discover_temporal_profile(log_json)` | `string` | `string` (JSON: DF pairs with mean/stdev duration ms) | `TemporalProfile` |
| `pm4py.discover_batches()` | `discover_batches(log_json)` | `string` | `string` (JSON: batches array with type, activity, instances) | `BatchResult` with pattern types |
| `pm4py.discover_correlation()` | `discover_correlation(log_json, correlation_threshold)` | `string`, `f64` | `string` (JSON: start, end, trace_count, edges) | `CorrelationDfg` for case-less logs |
| `pm4py.discover_performance_spectrum()` | `discover_performance_spectrum(log_json, activity)` | `string`, `string` | `string` (JSON: activity, overall_stats, instance_data) | `PerformanceSpectrum` per activity |

---

## Conformance Functions

| PM4Py Python | WASM Function | WASM Inputs | WASM Output | wasm4pm Surface |
|---|---|---|---|---|
| `pm4py.fitness_alignments()` / `conformance_diagnostics_alignments()` | `align_log(petri_net_json, log_json)` | `string`, `string` | `string` (JSON: total_cost, avg_cost, trace_alignments per-trace) | `AlignmentResult` with move types: sync/log/model |
| `pm4py.fitness_alignments()` (trace) | `align_trace(petri_net_json, trace_json)` | `string`, `string` | `string` (JSON: cost, is_fit, moves) | `TraceAlignment` |
| `pm4py.precision_etconformance()` | `precision_etconformance(petri_net_json, log_json)` | `string`, `string` | `string` (JSON: precision, total_escaping, total_consumed, total_traces) | `EtcPrecision` |
| `pm4py.conformance_diagnostics_token_based_replay()` | `token_replay_fitness(petri_net_json, log_json)` | `string`, `string` | `string` (JSON: percentage, avg_trace_fitness, perfectly_fitting_traces, total_traces, trace_results) | `TokenReplayResult` |
| `pm4py.check_soundness()` | `check_soundness(petri_net_json)` | `string` | `string` (JSON: sound, deadlock_free, bounded, liveness) | `SoundnessCheck` |
| `pm4py.generalization()` | `generalization(petri_net_json, log_json)` | `string`, `string` | `string` (JSON: generalization float) | `GeneralizationMetric` |
| `pm4py.conformance_temporal_profile()` | `check_temporal_conformance(log_json, profile_json, zeta)` | `string`, `string`, `f64` | `string` (JSON: fitness, deviations_count, deviation list) | `TemporalConformanceResult` |
| footprints comparison | `conformance_footprints(log_json, model_str)` | `string`, `string` | `string` (JSON: fitness, precision, recall, f1) | `FootprintsConformance` |
| `pm4py.discover_footprints()` on log | `discover_log_footprints(log_json)` | `string` | `string` (JSON: start/end activities, sequence/parallel pairs) | `LogFootprints` |
| footprints from POWL model | `compute_footprints(model)` | `PowlModel` | `string` (JSON: start_activities, end_activities, activities, skippable, sequence, parallel, activities_always_happening, min_trace_length) | `ModelFootprints` |

---

## I/O Functions

| PM4Py Python | WASM Function | WASM Inputs | WASM Output | wasm4pm Surface |
|---|---|---|---|---|
| `pm4py.read_xes()` | `parse_xes_log(xml)` | `string` (XES XML) | `string` (event log JSON) | `XesAdmission` path through `Admit::admit()` |
| `pm4py.read_csv()` | `parse_csv_log(csv)` | `string` (CSV with headers) | `string` (event log JSON) | `CsvAdmission` path |
| `pm4py.write_xes()` | `write_xes_log(log_json)` | `string` | `string` (XES XML) | `XesExport` with `LossReport` |
| `pm4py.write_csv()` | `write_csv_log(log_json)` | `string` | `string` (CSV) | `CsvExport` |
| `pm4py.read_pnml()` | `from_pnml(xml)` / `from_pnml_string(xml)` | `string` | `string` (PetriNetResult JSON) | `PnmlAdmission` |
| `pm4py.write_pnml()` | `to_pnml(pn_json)` / `to_pnml_json(pn_json)` | `string` | `string` (PNML 2.0 XML) | `PnmlExport` |
| `pm4py.read_bpmn()` | `read_bpmn(bpmn_xml)` | `string` | `string` (POWL model string) | `BpmnAdmission` → `PowlModel` |
| `pm4py.read_dfg()` | `read_dfg(dfg_json)` | `string` | `string` (DFG result JSON) | `DfgAdmission` |
| `pm4py.write_dfg()` | `write_dfg(dfg_json)` | `string` | `string` (canonical JSON) | `DfgExport` |
| process tree → PTML | `to_ptml_json(tree_json)` | `string` | `string` (PTML XML) | `PtmlExport` |
| PTML → process tree | `from_ptml_string(xml)` | `string` | `string` (ProcessTree JSON) | `PtmlAdmission` |
| `pm4py.sort_log()` | `sort_log(log_json)` | `string` | `string` (sorted log JSON) | `sort_log` operator on admitted log |
| `pm4py.project_log()` | `project_log(log_json, attributes_json)` | `string`, `string` | `string` (projected log JSON) | `Project` operator with `LossPolicy` and `LossReport` |

---

## POWL Model Functions

| PM4Py Python | WASM Function | WASM Inputs | WASM Output | wasm4pm Surface |
|---|---|---|---|---|
| `POWL.__repr__()` | `parse_powl(s)` | `string` | `PowlModel` (opaque handle) | Deserialize POWL string into arena-indexed model |
| `str(model)` | `powl_to_string(model)` | `PowlModel` | `string` | Serialize POWL model to canonical string |
| arena traversal | `node_info_json(model, arena_idx)` | `PowlModel`, `u32` | `string` (JSON: type, label or children+edges) | Node inspection by arena index |
| arena traversal | `node_to_string(model, arena_idx)` | `PowlModel`, `u32` | `string` | Per-node string representation |
| `model.root()` access | `model.root()` method | `PowlModel` | `u32` | Root node index in arena |
| `model.len()` | `model.len()` method | `PowlModel` | `u32` | Total node count |
| `model.is_empty()` | `model.is_empty()` method | `PowlModel` | `bool` | Empty model predicate |
| children access | `get_children(model, arena_idx)` | `PowlModel`, `u32` | `Uint32Array` | Child arena indices for SPO or OperatorPOWL |
| SPO ordering | `get_order_of(model, spo_arena_idx)` | `PowlModel`, `u32` | `BinaryRelationJs` | Raw ordering relation as adjacency matrix |
| transitive closure | `transitive_closure(model, spo_arena_idx)` | `PowlModel`, `u32` | `BinaryRelationJs` | Transitive closure of SPO ordering |
| transitive reduction | `transitive_reduction(model, spo_arena_idx)` | `PowlModel`, `u32` | `BinaryRelationJs` | Transitive reduction of SPO ordering |
| validation | `validate_partial_orders(model)` | `PowlModel` | `void` (throws on violation) | Irreflexive + transitive check for all SPO nodes |
| `pm4py.analysis.*` | `measure_complexity(model)` | `PowlModel` | `string` (JSON: cyclomatic, cfc, cognitive, nesting_depth, branching_factor, activity_count, node_count, halstead) | `ComplexityMetrics` |
| structural validation | `validate_powl_structure(model_str)` | `string` | `string` (JSON: verdict, reasoning, violations) | `PowlJudge` soundness check |

---

## Model Conversion Functions

| PM4Py Python | WASM Function | WASM Inputs | WASM Output | wasm4pm Surface |
|---|---|---|---|---|
| `pm4py.convert_to_petri_net(powl)` | `powl_to_petri_net(s)` | `string` (POWL str) | `string` (JSON: net, initial_marking, final_marking) | `PowlToPetriNet` conversion with loss accounting |
| convert to BPMN | `powl_to_bpmn(s)` | `string` (POWL str) | `string` (BPMN 2.0 XML) | `PowlToBpmn` conversion |
| convert to YAWL | `powl_to_yawl(s)` | `string` (POWL str) | `string` (YAWL v6 XML) | `PowlToYawl` conversion |
| `pm4py.convert_to_petri_net(hnet)` | `heuristics_to_petri_net(net_json)` | `string` | `string` | `HeuristicsNetToPetriNet` |
| Petri net reduction | `pm4py.reduce_petri_net()` | `reduce_petri_net(pn_json)` | `string` | `PetriNetReducer` |
| count reducible | `pm4py.count_reducible_elements()` | `count_reducible_elements(pn_json)` | `number` | `ReducibleElementCount` |
| DFG to DOT | `pm4py.vis.*` | `dfg_to_dot_wasm(dfg_json)` | `string` (DOT) | `DfgDotExport` |
| DFG to JSON | internal | `dfg_to_json_wasm(dfg_json)` | `string` | `DfgJsonExport` |

---

## Filtering Functions (full list)

| PM4Py Python | WASM Function | wasm4pm Surface |
|---|---|---|
| `pm4py.filter_time_range()` | `filter_time_range(log_json, start_ms: bigint, end_ms: bigint)` | `TimeRangeFilter` |
| `pm4py.filter_start_activities()` | `filter_start_activities(log_json, activities_json)` | `StartActivityFilter` |
| `pm4py.filter_end_activities()` | `filter_end_activities(log_json, activities_json)` | `EndActivityFilter` |
| `pm4py.filter_case_size()` | `filter_case_size(log_json, min_size, max_size)` | `CaseSizeFilter` |
| `pm4py.filter_event_attribute_values()` | `filter_event_attribute_values(log_json, key, vals_json, positive)` | `EventAttributeFilter` |
| `pm4py.filter_trace_attribute()` | `filter_trace_attribute(log_json, key, vals_json, positive)` | `TraceAttributeFilter` |
| `pm4py.filter_directly_follows_relation()` | `filter_directly_follows_relation(log_json, a, b)` | `DfRelationFilter` |
| `pm4py.filter_eventually_follows_relation()` | `filter_eventually_follows_relation(log_json, a, b)` | `EfRelationFilter` |
| `pm4py.filter_between()` | `filter_between(log_json, act1, act2)` | `BetweenFilter` |
| `pm4py.filter_prefixes()` | `filter_prefixes(log_json, prefix_json)` | `PrefixFilter` |
| `pm4py.filter_suffixes()` | `filter_suffixes(log_json, suffix_json)` | `SuffixFilter` |
| `pm4py.filter_trim()` | `filter_trim(log_json, start, end)` | `TrimFilter` |
| `pm4py.filter_variants_top_k()` | `filter_variants_top_k(log_json, k)` | `TopKVariantFilter` |
| `pm4py.filter_variants_reaching()` | `filter_variants_coverage(log_json, min_coverage)` | `VariantCoverageFilter` |

---

## Statistics / Analytics Functions

| PM4Py Python | WASM Function | wasm4pm Surface |
|---|---|---|
| `pm4py.get_variants()` | `get_variants(log_json)` | `VariantSet` |
| `pm4py.get_variants_as_tuples()` | `get_variants_as_tuples(log_json)` | `VariantTuples` |
| `pm4py.get_variants_paths_duration()` | `get_variants_paths_duration(log_json)` | `VariantPathDurations` |
| `pm4py.get_start_activities()` | `get_start_activities(log_json)` | `StartActivities` |
| `pm4py.get_end_activities()` | `get_end_activities(log_json)` | `EndActivities` |
| `pm4py.get_attribute_values()` | `get_attribute_values(log_json, key)` | `AttributeValueFreqs` |
| `pm4py.get_case_durations()` | `get_case_durations(log_json)` | `CaseDurations` |
| `pm4py.get_all_case_durations()` | `get_all_case_durations(log_json)` | `AllCaseDurations` |
| `pm4py.get_case_arrival_average()` | `get_case_arrival_average(log_json)` | `CaseArrivalRate` |
| `pm4py.get_case_overlap()` | `get_case_overlap(log_json)` | `CaseOverlap` |
| `pm4py.get_minimum_self_distances()` | `get_minimum_self_distances(log_json)` | `MinSelfDistances` |
| `pm4py.get_rework_cases_per_activity()` | `get_rework_cases_per_activity(log_json)` | `ReworkCases` |
| `pm4py.get_rework_times()` | `get_rework_times(log_json)` | `ReworkTimes` |
| `pm4py.get_prefixes_from_log()` | `get_prefixes_from_log(log_json)` | `PrefixFreqs` |
| internal | `get_performance_stats(log_json)` | `PerformanceStats` |
| `pm4py.get_case_attributes()` | `get_case_attributes(log_json)` | `CaseAttributeStats` |
| `pm4py.get_event_attributes()` | `get_event_attributes(log_json)` | `EventAttributeStats` |
| `pm4py.get_trace_attributes()` | `get_trace_attributes(log_json)` | `TraceAttributeStats` |
| `pm4py.get_trace_attribute_values()` | `get_trace_attribute_values(log_json, key)` | `TraceAttributeValues` |

---

## Extended / LLM / Code-Generation Functions

| PM4Py Python | WASM Function | WASM Inputs | WASM Output | wasm4pm Surface |
|---|---|---|---|---|
| DSPy few-shot demos | `get_demos_for_domain(domain)` | `string` (domain name) | `string` (JSON: few-shot example array) | `LlmDemosForDomain` — NOT a Rust type law surface |
| POWL → code generation | `generate_code_from_powl(model_str, target)` | `string`, `string` | `string` (n8n JSON / Temporal Go / Camunda BPMN / YAWL v6 XML) | `CodeGenTarget` enum: n8n, temporal, camunda, yawl |
| model diff | `diff_models(model_a, model_b)` | `string`, `string` | `string` (JSON: added/removed activities, ordering changes, severity) | `ModelDiff` |
| label replacement | `replace_labels(model_str, label_map_json)` | `string`, `string` | `string` (POWL str with new labels) | `LabelMap` transformation |
| simplify frequent transitions | `simplify_frequent_transitions(model)` | `PowlModel` | `PowlModel` | `FrequentTransitionSimplifier` |
| simplify POWL | `simplify_powl(model)` | `PowlModel` | `PowlModel` | `PowlSimplifier` |
| streaming conformance create | `streaming_create(model_str)` | `string` | `number` (handle) | `StreamingConformanceHandle` |
| streaming push trace | `streaming_push_trace(handle, trace_json)` | `number`, `string` | `string` (JSON: fitness + alerts) | `StreamingPushResult` |
| streaming snapshot | `streaming_snapshot(handle)` | `number` | `string` (JSON: fitness, traces_seen, perfect_rate, drift_alerts) | `StreamingSnapshot` |

---

## BinaryRelationJs Class

`BinaryRelationJs` is a Rust-side serializable adjacency matrix for partial orders, exposed to JS.

| Method | Return | Purpose |
|---|---|---|
| `edges_flat()` | `Uint32Array` | All edges as [src0, tgt0, src1, tgt1, ...] |
| `end_nodes()` | `Uint32Array` | Nodes with no outgoing edges |
| `is_edge(i, j)` | `boolean` | Test edge existence |
| `is_irreflexive()` | `boolean` | Partial order irreflexivity check |
| `is_strict_partial_order()` | `boolean` | Combined check |
| `is_transitive()` | `boolean` | Transitivity check |
| `n()` | `number` | Node count |
| `start_nodes()` | `Uint32Array` | Nodes with no incoming edges |

Rust counterpart in wasm4pm-compat: `src/petri.rs` — `WfNetConst<SOUNDNESS>` and `src/law.rs` relation machinery.
