# Process Intelligence Paper to Type Law Classification

This file classifies the 15 core papers into Type Law (static, model-time, and structural invariants) and Execution Law (dynamic, runtime, and trace-based constraints) components.

For mathematical structures, see [paper-canon.md](file:///Users/sac/process-intelligence/sources/papers/paper-canon.md).
For execution fixtures, see [paper-to-execution-law.md](file:///Users/sac/process-intelligence/sources/papers/paper-to-execution-law.md).
For the ledger registry, see [workflow-ledger.md](file:///Users/sac/process-intelligence/sources/papers/workflow-ledger.md).

---

### Classify: van_der_aalst_1998_workflow_nets
- **Type Law**:
  - Static structural validation: $N = (P, T, F)$ must have a unique source place $i \in P$ ($\bullet i = \emptyset$) and a unique sink place $o \in P$ ($o \bullet = \emptyset$).
  - Node reachability: Every node $x \in P \cup T$ must be on a path from $i$ to $o$.
  - Structural Liveness and Boundedness on the short-circuited net $\bar{N}$.
- **Execution Law**:
  - The token game execution sequence. Given an event sequence, the replayer simulates transition firings, verifying that each transition $t_k$ is enabled ($M_{k-1}(p) \ge 1$ for all $p \in \bullet t_k$) and producing the next marking $M_k$.
  - Replay checks verify if the final marking is $[o]$ and that no tokens remain in other places.

### Classify: adriansyah_2014_alignment_conformance
- **Type Law**:
  - Definition of the alignment move alphabet $(T \cup \{\gg\}) \times (\Sigma \cup \{\gg\}) \setminus \{(\gg, \gg)\}$.
  - Static cost function schema: mapping each possible move type to a non-negative real value $c(m)$.
- **Execution Law**:
  - The A* search engine execution: computing the state graph where edges are alignment moves, and finding the shortest path from $(M_i, \epsilon)$ to $(M_f, \sigma)$.
  - Verification of the heuristic function consistency: $h(u) - h(v) \le \text{cost}(u, v)$ for all states $u, v$.

### Classify: leemans_2013_inductive_miner
- **Type Law**:
  - Block-structured syntactic trees. Valid operators: $\times, \rightarrow, \wedge, \circlearrowleft$.
  - Translation laws from Process Trees to sound Petri nets.
- **Execution Law**:
  - Log partitioning and DFG cut detection.
  - Sublog projection: filtering log traces to sub-vocabularies in each recursion step.

### Classify: ghahfarokhi_2021_ocel2
- **Type Law**:
  - Database schema definition: Object Types, Event Types, and Attribute schemas.
  - Typing relations: Event-to-Object (relationships) and Attribute-to-Value constraints.
- **Execution Law**:
  - Evaluation of object-centric process paths, ensuring that when an event occurs, it matches all related objects' states.
  - Event log parsing and serialization checks (JSON/XML).

### Classify: verbeek_2021_log_trie
- **Type Law**:
  - Trie data structure definition: Rooted tree where edges are labeled with activities.
  - Node frequency attributes and prefix-sharing invariants.
- **Execution Law**:
  - Insertion algorithm for trace sequences into the trie.
  - Trie traversal for alignment calculations, copying parent alignment state and extending it for children.

### Classify: kuesters_2024_ocpq_tree
- **Type Law**:
  - Query tree structure: AST defining query nodes and edge relations.
  - Validation of query types against OCEL schema types.
- **Execution Law**:
  - Database query execution (SQL join queries, Cypher traversals).
  - Filtering and returning matching object/event traces.

### Classify: weske_2019_bpm_principles
- **Type Law**:
  - BPMN graph topology: Node connectivity, Gateway split/join pairing.
  - Syntactic well-formedness of process models.
- **Execution Law**:
  - Execution of BPMN token flows (AND, XOR, OR gateway routing).
  - State space exploration for deadlock detection.

### Classify: rosenberg_2020_cloud_lifecycle
- **Type Law**:
  - State Machine definition: Valid state set $S$ and transition function $\delta$.
  - Initial and terminal state definitions.
- **Execution Law**:
  - Real-time tracking of resource states based on API event streams.
  - Reporting invalid transitions where $\delta(s, a)$ is undefined.

### Classify: aalst_2016_process_mining_action
- **Type Law**:
  - Graph specifications: Heuristic Net and Causal Net structures.
  - Metric thresholds: Causal dependency equations.
- **Execution Law**:
  - Log replay for metric calculation (token tracking, activity frequency).
  - Computing causal dependencies and generating Heuristic Nets.

### Classify: aalst_2004_workflow_patterns
- **Type Law**:
  - Design-time pattern configurations (Petri Net structures for basic and advanced control flows).
- **Execution Law**:
  - Execution of split/join routing in process engines.
  - Execution of cancellation scopes (dynamic token removal).

### Classify: weidlich_2011_profile_conformance
- **Type Law**:
  - Behavioral profile relations ($\to, +, \parallel$) over transition pairs.
- **Execution Law**:
  - Extracting behavioral relations from execution traces.
  - Profile comparison between model and log, detecting conflict points.

### Classify: de_medeiros_2007_genetic_miner
- **Type Law**:
  - Genetic chromosome representation (Causal Matrix schema).
  - Population constraints and crossover/mutation operator mappings.
- **Execution Law**:
  - Replay evaluation of the entire population against the log.
  - Selection, crossover, and mutation execution loops.

### Classify: gunther_2007_fuzzy_miner
- **Type Law**:
  - Fuzzy Causal Graph schema: Significance and correlation metric schemas.
- **Execution Law**:
  - Metric computation over event log relationships.
  - Node/edge abstraction and clustering execution based on sliders/thresholds.

### Classify: song_2008_organizational_mining
- **Type Law**:
  - Social Network Graph schema ($G = (V, E, w)$) and actor metrics definitions.
- **Execution Law**:
  - Log parsing to extract originator sequences.
  - Social Network construction and centrality metric calculations.

### Classify: dongen_2009_mxml_to_xes
- **Type Law**:
  - XML schemas: MXML schema vs XES schema.
- **Execution Law**:
  - File translation: XML parsing, namespace mapping, timestamp conversion, and writing the standardized XES output file.