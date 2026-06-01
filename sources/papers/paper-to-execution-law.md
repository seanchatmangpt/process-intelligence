# Process Intelligence Paper to Execution Law Fixtures

This document details the concrete execution fixture obligations, test input models, event logs, and expected output structures required to verify the implementation of each paper.

For mathematical structures, see [paper-canon.md](file:///Users/sac/process-intelligence/sources/papers/paper-canon.md).
For type classification, see [paper-to-type-law.md](file:///Users/sac/process-intelligence/sources/papers/paper-to-type-law.md).
For the ledger registry, see [workflow-ledger.md](file:///Users/sac/process-intelligence/sources/papers/workflow-ledger.md).

---

### Fixtures for van_der_aalst_1998_workflow_nets
- **Input Model**:
  - Petri Net JSON representing a Workflow Net with start place `i` and end place `o`.
  - Edge Case: A model with an unreachable path or multiple source/sink places to test pre-check failures.
- **Input Log**:
  - XES log containing compliant traces (e.g., `<a, b, c>`) and non-compliant traces (e.g., `<a, c>` that skip a mandatory transition, or `<a, b, b, c>` to test soundness/safety violations).
- **Expected Output**:
  - Replay results detailing produced, consumed, missing, and remaining tokens for each trace.
  - Conformance status (e.g., `is_conforming: false` for unsafe or deadlocking paths).

### Fixtures for adriansyah_2014_alignment_conformance
- **Input Model**:
  - Petri Net with silent transitions ($\tau$) and duplicate labels.
- **Input Log**:
  - Traces that deviate from the model (e.g., trace `<a, c, d>` when model allows `<a, b, d>`).
- **Cost Configuration**:
  - Move-on-log cost = 1.0, Move-on-model cost = 1.0, Synchronous move cost = 0.0.
- **Expected Output**:
  - Optimal alignment path (e.g., `[(a, a), (>> , b), (c, >>), (d, d)]`).
  - Total alignment cost and fitness score.

### Fixtures for leemans_2013_inductive_miner
- **Input Model**:
  - Synthesized event log with complex block behaviors (loops, parallel paths, exclusive choices).
- **Expected Output**:
  - Discovered Process Tree represented in string format (e.g., `->(a, x(b, c), d)`).
  - Exported Petri Net that is guaranteed to be a sound WF-net.

### Fixtures for ghahfarokhi_2021_ocel2
- **Input Model**:
  - Object-Centric Petri Net (OCPN) with object types (e.g., `order`, `item`, `delivery`).
- **Input Log**:
  - OCEL 2.0 JSON or XML log featuring events interacting with multiple object types.
- **Expected Output**:
  - Multi-set markings for each place.
  - Trace validation report checking attribute types and referential constraints.

### Fixtures for verbeek_2021_log_trie
- **Input Log**:
  - Event log containing thousands of traces with significant prefix overlap.
- **Expected Output**:
  - Constructed Log Trie structure (nodes, edges, frequencies).
  - Comparative execution time benchmark verifying that trie-based alignment is faster than flat trace-by-trace alignment.

### Fixtures for kuesters_2024_ocpq_tree
- **Input Model**:
  - OCPQ query tree querying relationships between `orders` and `packages` (e.g., "Find all cases where a package is created before the order is paid").
- **Input Log**:
  - OCEL log database.
- **Expected Output**:
  - Matching trace events, object IDs, and timing deviations.

### Fixtures for weske_2019_bpm_principles
- **Input Model**:
  - BPMN model containing AND-gateways, XOR-gateways, and OR-gateways.
- **Expected Output**:
  - Reachability graph of the BPMN model.
  - Diagnostics identifying deadlock conditions (e.g., AND-join paired with XOR-split).

### Fixtures for rosenberg_2020_cloud_lifecycle
- **Input Model**:
  - Cloud lifecycle FSM schema (states: `Provisioned`, `Active`, `Terminated`).
- **Input Log**:
  - Log of API events (e.g., `create`, `terminate`) mapped to resource IDs.
- **Expected Output**:
  - State tracking log showing the current state of each resource.
  - Drift alerts when an invalid API sequence is executed.

### Fixtures for aalst_2016_process_mining_action
- **Input Model**:
  - Event log with varied trace frequencies to test heuristic thresholds.
- **Expected Output**:
  - Matrix of dependency measures between activity pairs.
  - Discovered Heuristic Net graph.

### Fixtures for aalst_2004_workflow_patterns
- **Input Model**:
  - Petri Nets or BPEL workflows implementing advanced patterns (e.g., Multiple Instance, Deferred Choice, Milestone).
- **Expected Output**:
  - Firing logs demonstrating pattern completion without token leaks.

### Fixtures for weidlich_2011_profile_conformance
- **Input Model**:
  - Reference Petri Net model and a modified version with reordered transitions.
- **Expected Output**:
  - Behavioral profiles for both models.
  - Conflict list showing where relations (e.g., strict order vs interleaving) differ.

### Fixtures for de_medeiros_2007_genetic_miner
- **Input Log**:
  - Event log with noise (e.g., sporadic events).
- **Expected Output**:
  - Evolution history (generation, best fitness, average fitness).
  - Final discovered Causal Matrix.

### Fixtures for gunther_2007_fuzzy_miner
- **Input Log**:
  - Spaghetti process event log.
- **Expected Output**:
  - Computed significance and correlation scores.
  - Simplified Fuzzy Causal Graph for various threshold settings.

### Fixtures for song_2008_organizational_mining
- **Input Log**:
  - Event log annotated with originator attributes.
- **Expected Output**:
  - Collaboration network weight matrix.
  - Originator centrality scores.

### Fixtures for dongen_2009_mxml_to_xes
- **Input Log**:
  - An MXML file with audit trail entries.
- **Expected Output**:
  - A valid XES file containing standardized XML tags and extensions.