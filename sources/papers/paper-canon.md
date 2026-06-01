# Process Intelligence Paper Canon

This document outlines the formal academic models, algebraic structures, algorithms, preconditions, and proof requirements for the 15 core workflow and process mining papers.

For execution mapping, see the execution law details at [paper-to-execution-law.md](file:///Users/sac/process-intelligence/sources/papers/paper-to-execution-law.md).
For type classification, see [paper-to-type-law.md](file:///Users/sac/process-intelligence/sources/papers/paper-to-type-law.md).
For the ledger registry, see [workflow-ledger.md](file:///Users/sac/process-intelligence/sources/papers/workflow-ledger.md).

---

### Paper 1: van_der_aalst_1998_workflow_nets
- **Academic Reference**: Wil van der Aalst (1998). "The Application of Petri Nets to Workflow Management". Journal of Circuits, Systems, and Computers, 8(1):21-66.
- **Formal Algebraic Structures**:
  - A classical Petri Net is a triple $N = (P, T, F)$ where $P$ is a finite set of places, $T$ is a finite set of transitions ($P \cap T = \emptyset$), and $F \subseteq (P \times T) \cup (T \times P)$ is the flow relation representing directed arcs.
  - The preset of a node $x \in P \cup T$ is defined as $\bullet x = \{y \in P \cup T \mid (y, x) \in F\}$.
  - The postset of a node $x \in P \cup T$ is defined as $x \bullet = \{y \in P \cup T \mid (x, y) \in F\}$.
  - A Petri Net $N$ is a Workflow Net (WF-net) if and only if:
    1. Unique Source: There is a unique place $i \in P$ such that $\bullet i = \emptyset$.
    2. Unique Sink: There is a unique place $o \in P$ such that $o \bullet = \emptyset$.
    3. Weak Connectedness: Every node $x \in P \cup T$ lies on a path from $i$ to $o$. That is, the reflexive transitive closure of the flow relation contains $(i, x)$ and $(x, o)$ for all $x$.
  - The short-circuited net $\bar{N} = (P, T \cup \{t^*\}, F \cup \{(o, t^*), (t^*, i)\})$ connects the sink to the source via a feedback transition $t^* \notin T$.
- **Algorithms**:
  - Operational Semantics (Token Game): A marking $M$ is a multiset over $P$, $M \in \mathbb{N}^P$. A transition $t$ is enabled in $M$, denoted $M \xrightarrow{t}$, if and only if $\forall p \in \bullet t, M(p) \ge 1$. Firing $t$ produces $M' = M - \bullet t + t \bullet$.
  - Soundness Verification: Checks if the short-circuited net $\bar{N}$ is live and bounded.
- **Preconditions**:
  - The graph must satisfy the WF-net structural constraints (single source, single sink, path connectivity from $i$ to $o$).
- **Proof Requirements**:
  - Option to Complete: $\forall M \in [i \rangle, M \xrightarrow{*} [o]$ (for any marking reachable from $i$, the final marking $o$ is reachable).
  - Proper Completion: $\forall M \in [i \rangle, M \ge [o] \implies M = [o]$ (when $o$ is marked, all other places are empty).
  - Liveness: For the short-circuited net $\bar{N}$, $\forall t \in T \cup \{t^*\}, \forall M \in [i \rangle, \exists M' \in [M \rangle, M' \xrightarrow{t}$ (no transition becomes dead).

### Paper 2: adriansyah_2014_alignment_conformance
- **Academic Reference**: Arya Adriansyah (2014). "Aligning Observed and Modeled Behavior". PhD Thesis, Eindhoven University of Technology.
- **Formal Algebraic Structures**:
  - An event log trace is a sequence $\sigma \in \Sigma^*$, where $\Sigma$ is the alphabet of activities.
  - A process model is represented by a labeled Petri net $N = (P, T, F, \lambda)$ with a labeling function $\lambda: T \to \Sigma \cup \{\tau\}$, where $\tau$ denotes silent transitions.
  - An alignment move is a pair $(x, y) \in (T \cup \{\gg\}) \times (\Sigma \cup \{\gg\}) \setminus \{(\gg, \gg)\}$, classified as:
    - Synchronous Move: $(t, a)$ if $t \in T, a \in \Sigma$ with $\lambda(t) = a$.
    - Model Move: $(t, \gg)$ if $t \in T$, representing a model step not observed in the log.
    - Log Move: $(\gg, a)$ if $a \in \Sigma$, representing a log step not allowed by the model.
  - A legal alignment of trace $\sigma$ and model $N$ is a sequence of moves $\gamma \in ((T \cup \{\gg\}) \times (\Sigma \cup \{\gg\}))^*$ such that:
    1. Projecting the first coordinate onto $T$ (omitting $\gg$) yields a valid firing sequence $t_1 t_2 \dots t_n$ from initial marking $M_i$ to final marking $M_f$.
    2. Projecting the second coordinate onto $\Sigma$ (omitting $\gg$) yields the trace $\sigma$.
  - Cost function: $c: (T \cup \{\gg\}) \times (\Sigma \cup \{\gg\}) \to \mathbb{R}_{\ge 0}$ assigns a weight to each deviation. Non-silent model moves $c(t, \gg) > 0$ and log moves $c(\gg, a) > 0$ have positive cost, while synchronous moves and silent transitions $c(t, \gg)$ where $\lambda(t) = \tau$ have zero cost.
- **Algorithms**:
  - Optimal Alignment Search (A*): Finds $\gamma^* = \arg\min_{\gamma} c(\gamma)$ where $c(\gamma) = \sum_{m \in \gamma} c(m)$.
    - State space search with $f(n) = g(n) + h(n)$, where $g(n)$ is the cost of the path to node $n$, and $h(n)$ is a heuristic estimating the cost from $n$ to the final marking $M_f$.
- **Preconditions**:
  - Model must have well-defined initial and final markings.
  - Model state space must be finite, or search boundaries must be set.
- **Proof Requirements**:
  - Heuristic Admissibility: $h(n) \le h^*(n)$, where $h^*(n)$ is the true minimum cost.
  - Heuristic Consistency: $h(u) \le \text{cost}(u, v) + h(v)$.
  - Completeness: A* search terminates and yields the globally optimal alignment.

### Paper 3: leemans_2013_inductive_miner
- **Academic Reference**: Sander J. J. Leemans, Dirk Fahland, and Wil M. P. van der Aalst (2013). "Discovering Block-Structured Process Models from Event Logs - A State-based Approach". International Conference on Application and Theory of Petri Nets and Concurrency, 311-329.
- **Formal Algebraic Structures**:
  - A Process Tree $Q$ is defined inductively:
    - Base: $a \in \Sigma$ or silent step $\tau$.
    - Operator: $op(Q_1, Q_2, \dots, Q_n)$ for $op \in \{\times, \rightarrow, \wedge, \circlearrowleft\}$.
      - $\times$: Exclusive choice.
      - $\rightarrow$: Sequence.
      - $\wedge$: Parallel.
      - $\circlearrowleft$: Loop (typically binary: $do$ and $redo$).
  - Directly-Follows Graph (DFG): $D(L) = (V, E)$ where $V \subseteq \Sigma$ is the set of activities in log $L$, and $E \subseteq V \times V$ contains $(a, b)$ if activity $a$ is immediately followed by $b$ in some trace.
  - Start activities $S(L) = \{a \in \Sigma \mid \exists \sigma \in L, \sigma(1) = a\}$ and End activities $E(L) = \{a \in \Sigma \mid \exists \sigma \in L, \sigma(|\sigma|) = a\}$.
- **Algorithms**:
  - Inductive Miner (IM) Discovery:
    1. Base Case Detection: If the sublog contains $\le 1$ unique activity, return leaf.
    2. Construct DFG $D(L)$.
    3. Find Cut: Partition $V$ into $V_1, \dots, V_k$ matching one of the operator cuts (Exclusive, Sequence, Parallel, Loop).
    4. Project Log: Split the log $L$ into sublogs $L_1, \dots, L_k$ based on the partition.
    5. Recurse: Apply the algorithm to each sublog $L_i$ and return the process tree.
- **Preconditions**:
  - Input event log is a non-empty collection of traces.
- **Proof Requirements**:
  - Soundness: Every process tree maps to a sound Workflow Net by construction.
  - Language Equivalence & Progress: Recursive step reduces alphabet size or partitions the log, ensuring guaranteed termination.

### Paper 4: ghahfarokhi_2021_ocel2
- **Academic Reference**: Alessandro Berti, Ahmad Banisi, Wil van der Aalst, et al. (2021). "Object-Centric Event Logs (OCEL) Standard".
- **Formal Algebraic Structures**:
  - Object-Centric Event Log: $L_{oc} = (E, O, OT, ET, \text{act}, \text{time}, \text{type}, \text{attr}, \text{val}, \text{om})$:
    - $E$: Set of event identifiers.
    - $O$: Set of object identifiers.
    - $OT$: Set of object types.
    - $ET$: Set of event types (activities).
    - $\text{act}: E \to ET$ (activity mapping).
    - $\text{time}: E \to \mathcal{T}$ (timestamp mapping).
    - $\text{type}: O \to OT$ (object type mapping).
    - $\text{attr}: (E \cup O) \to \mathcal{P}(A)$ (attributes).
    - $\text{val}: (E \cup O) \times A \not\to V$ (attribute values).
    - $\text{om}: E \to \mathcal{P}(O \times R)$ (relational object-to-event map, typed by relationship type $R$).
- **Algorithms**:
  - Object-Centric Petri Net (OCPN) Token Game: Places are typed by object types ($p \in P \implies \text{type}(p) \in OT$). A transition $t$ is enabled for a combination of objects if for each input place $p \in \bullet t$, there is an object of type $\text{type}(p)$ in $p$.
  - Schema Validation: Verifies referential integrity and type conformance.
- **Preconditions**:
  - Uniqueness of event and object identifiers.
- **Proof Requirements**:
  - Referential Integrity: $\forall (e, o, r) \in \text{om}(e)$, $o \in O$.
  - Temporal Monotonicity: Timestamps must be linearly orderable.

### Paper 5: verbeek_2021_log_trie
- **Academic Reference**: H.M.W. Verbeek (2021). "Log Tries for Conformance Checking". CEUR Workshop Proceedings.
- **Formal Algebraic Structures**:
  - A Log Trie is a rooted tree $T_t = (V_t, E_t, v_{root}, \lambda_e, \text{freq})$:
    - $V_t$: Finite set of nodes.
    - $E_t \subseteq V_t \times V_t$: Directed edges.
    - $v_{root} \in V_t$: The root node.
    - $\lambda_e: E_t \to \Sigma$: Edge labels (activities).
    - $\text{freq}: V_t \to \mathbb{N}$: Frequency of traces terminating or passing through a node.
- **Algorithms**:
  - Trie Construction: Insert traces sequentially from the root, sharing prefixes.
  - Prefix-Shared A* Alignment: Run A* search on the trie nodes. When computing the alignment for a child node, copy the search frontier/state from its parent node and continue the search with the new transition.
- **Preconditions**:
  - Event log must be represented as a set of activity sequences.
- **Proof Requirements**:
  - Equivalence: The optimal alignment paths generated using the log trie are identical to the flat trace alignments.
  - Efficiency Bound: Maximum state evaluations are bounded by $|V_t|$ instead of the total number of events in the log.

### Paper 6: kuesters_2024_ocpq_tree
- **Academic Reference**: Ralf Küsters et al. (2024). "Object-Centric Process Querying".
- **Formal Algebraic Structures**:
  - OCPQ Tree $Q_{ocpq}$: A tree where nodes represent query operators (filters, joins, projections, aggregations) and edges represent data flow or dependency relations.
  - Matches are relations over $E \times O \times OT$.
- **Algorithms**:
  - Query Translation: Compiles the OCPQ tree into equivalent SQL or Cypher expressions.
  - Object-Centric Path Evaluation: Evaluates graph-like queries across multiple object types over an OCEL log database.
- **Preconditions**:
  - OCEL log compliant with standard schemas.
- **Proof Requirements**:
  - Query Soundness: Every output tuple satisfies the structural and temporal constraints of the OCPQ tree.
  - Query Completeness: All valid matching patterns in the OCEL database are returned.

### Paper 7: weske_2019_bpm_principles
- **Academic Reference**: Mathias Weske (2019). "Business Process Management: Concepts, Languages, Architectures". Springer.
- **Formal Algebraic Structures**:
  - Workflow Graph: $WG = (Node, Edge, Type)$:
    - $Node = Activity \cup Gateway \cup Event$.
    - $Gateway$: Routing nodes.
    - $Type: Gateway \to \{AND, XOR, OR\} \times \{Split, Join\}$.
  - State configuration represented as a marking over $Edge$.
- **Algorithms**:
  - BPMN Operational Semantics (Token Simulation): Gateway activation rules:
    - AND-split: Produces a token on all outgoing edges.
    - AND-join: Fires when all incoming edges have tokens.
    - XOR-split: Produces a token on exactly one outgoing edge.
    - XOR-join: Fires when any incoming edge has a token.
  - Soundness Analysis: Construction of reachability graph to identify structural deadlocks or lack of synchronization.
- **Preconditions**:
  - Graph is well-formed with a single start event and single end event.
- **Proof Requirements**:
  - Soundness: Graph is free of deadlocks (states from which no end state is reachable) and lacks synchronization (residual tokens left after completion).

### Paper 8: rosenberg_2020_cloud_lifecycle
- **Academic Reference**: Rosenberg et al. (2020). "Cloud Resource Lifecycle Modeling".
- **Formal Algebraic Structures**:
  - Cloud Resource Lifecycle FSM: $M_{cloud} = (S, \Sigma_{api}, \delta, s_0, F_{state})$:
    - $S$: States (Provisioned, Active, Suspended, Decommissioned).
    - $\Sigma_{api}$: API actions (create, delete, update).
    - $\delta: S \times \Sigma_{api} \to S$: Transition function.
    - $s_0 \in S$: Initial state.
    - $F_{state} \subseteq S$: Final terminal states.
- **Algorithms**:
  - Cloud Conformance Tracking: For each resource instance, replay its logged API calls. If $\delta(s, a)$ is undefined, report conformance violation.
- **Preconditions**:
  - Log events contain resource identifier and API action name.
- **Proof Requirements**:
  - Liveness & Termination: For any resource in active state, there exists a path of transitions to a state in $F_{state}$.
  - Transition Determinism: $\delta$ is a function, ensuring deterministic state updates.

### Paper 9: aalst_2016_process_mining_action
- **Academic Reference**: Wil van der Aalst (2016). "Process Mining: Data Science in Action". Springer.
- **Formal Algebraic Structures**:
  - Log representation: $L \in \mathcal{B}(\Sigma^*)$ (multi-set of traces).
  - Causal Net (C-net): $CN = (A, a_i, a_o, D, I, O)$ where $A$ is the set of activities, $a_i$ is start, $a_o$ is end, $D$ is the dependency relation, $I$ maps activities to input bindings, and $O$ maps activities to output bindings.
- **Algorithms**:
  - Heuristic Miner:
    - Causal dependency calculation:
      $C(a, b) = \frac{|a > b| - |b > a|}{|a > b| + |b > a| + 1}$
    - Loop dependency:
      $L(a, a) = \frac{|a > a|}{|a > a| + 1}$
    - Long-distance dependency calculation.
- **Preconditions**:
  - Event log must have case ID and activity fields.
- **Proof Requirements**:
  - Discovery Completeness: With noise-free and complete logs, the discovered causal relations match the underlying process model.

### Paper 10: aalst_2004_workflow_patterns
- **Academic Reference**: Wil van der Aalst, Arthur H. M. ter Hofstede, Bartek Kiepuszewski, and Alistair P. Barros (2003). "Workflow Patterns". Distributed and Parallel Databases, 14(1):5-51.
- **Formal Algebraic Structures**:
  - Patterns formalized as structural Petri Net configurations:
    - Sequence: $p_1 \to t_1 \to p_2 \to t_2$.
    - Parallel Split: $t_1 \to \{p_1, p_2\}$.
    - Synchronization: $\{p_1, p_2\} \to t_1$.
    - Exclusive Choice: $p_1 \to \{t_1, t_2\}$.
    - Simple Merge: $\{p_1, p_2\} \to t_1$ (sharing input/output places).
- **Algorithms**:
  - Pattern Equivalence Checking: Maps pattern nodes to standard Petri net structures and verifies trace language equivalence.
- **Preconditions**:
  - Models must have explicit control-flow semantics.
- **Proof Requirements**:
  - Deadlock-Freeness: Composition of sound patterns preserves soundness of the composite net.
  - Boundedness: Composite nets remain bounded under parallel splits and synchronization.

### Paper 11: weidlich_2011_profile_conformance
- **Academic Reference**: Matthias Weidlich, Artem Polyvyanyy, and Jan Mendling (2011). "Behavioral Profiles in Process Conformance Checking". IEEE Transactions on Software Engineering, 37(4):510-529.
- **Formal Algebraic Structures**:
  - Behavioral Profile: Let $N = (P, T, F)$ be a Petri Net. The behavioral profile $BP(N)$ consists of three mutually exclusive relations over $T \times T$:
    1. Strict Order ($\to$): $t_1 \to t_2$ iff there is an execution sequence where $t_1$ is followed by $t_2$, but no execution sequence where $t_2$ is followed by $t_1$.
    2. Exclusivity ($+$): $t_1 + t_2$ iff there is no execution sequence containing both $t_1$ and $t_2$.
    3. Interleaving ($\parallel$): $t_1 \parallel t_2$ iff there is an execution sequence where $t_1$ is followed by $t_2$, and an execution sequence where $t_2$ is followed by $t_1$.
- **Algorithms**:
  - Profile Extraction: Traverse model reachability graph to extract relations. For logs, extract relations from trace sequences.
  - Conformance Alignment: Compare $BP(N)$ and $BP(L)$ to identify structural discrepancies (e.g., transitions exclusive in the model but interleaving in the log).
- **Preconditions**:
  - Process model must be a sound Workflow Net.
- **Proof Requirements**:
  - Completeness of Relations: For every pair $(t_1, t_2) \in T \times T$, exactly one relation ($\to, +, \parallel$, or inverse strict order) holds.
  - Preservation: Behavioral profiles are invariant under isomorphism of sound WF-nets.

### Paper 12: de_medeiros_2007_genetic_miner
- **Academic Reference**: Ana Karla Alves de Medeiros (2007). "Genetic Process Mining". PhD Thesis, Eindhoven University of Technology.
- **Formal Algebraic Structures**:
  - Chromosome: Causal Matrix $CM = (A, I, O)$ where $A$ is the set of activities, and for each $a \in A$, $I(a) \subseteq \mathcal{P}(A)$ (input activities) and $O(a) \subseteq \mathcal{P}(A)$ (output activities).
  - Population: A set of causal matrices.
  - Fitness Function:
    $F(CM) = \alpha \cdot \text{replay\_fitness}(CM, L) + (1-\alpha) \cdot \text{precision}(CM, L)$
- **Algorithms**:
  - Genetic Discovery:
    1. Initialize population.
    2. Evaluate fitness.
    3. Select parents (Tournament Selection).
    4. Crossover (recombine causal relations).
    5. Mutate (add/remove activities or causal links).
    6. Replace population and repeat until convergence.
- **Preconditions**:
  - Population size, crossover rate, mutation rate, and target fitness are predefined.
- **Proof Requirements**:
  - Search Space Completeness: Every sound workflow net can be represented by a Causal Matrix.
  - Convergence: Under non-zero mutation and selection pressure, the population converges to a Pareto-optimal model.

### Paper 13: gunther_2007_fuzzy_miner
- **Academic Reference**: Christian W. Günther and Wil M. P. van der Aalst (2007). "Fuzzy Miner: Visualizing Structured and Unstructured Process Flows". International Conference on Business Process Management, 328-343.
- **Formal Algebraic Structures**:
  - Fuzzy Graph: $G_{fuzzy} = (V, E, \text{sig}_v, \text{sig}_e, \text{cor}_e)$:
    - $V$: Vertices (activities).
    - $E \subseteq V \times V$: Directed edges.
    - $\text{sig}_v: V \to [0, 1]$: Node significance (based on frequency).
    - $\text{sig}_e: E \to [0, 1]$: Edge significance.
    - $\text{cor}_e: E \to [0, 1]$: Edge correlation (based on correlation of timing and attributes).
- **Algorithms**:
  - Fuzzy Mining & Simplification:
    1. Calculate significance and correlation for all nodes and edges.
    2. Filter Nodes: Remove nodes where $\text{sig}_v(v) < \theta_v$.
    3. Filter Edges: Retain edges where $\text{sig}_e(e) \ge \theta_e$ and $\text{cor}_e(e) \ge \theta_c$.
    4. Cluster Nodes: Aggregate low-significance, high-correlation nodes into a single cluster node.
- **Preconditions**:
  - Log must contain activity names and timestamps.
- **Proof Requirements**:
  - Boundedness of Metrics: $\text{sig}_v, \text{sig}_e, \text{cor}_e$ map values to the closed interval $[0, 1]$.
  - Graph Connectivity Preservation: Simplification does not partition the process graph unless the elements are entirely disconnected.

### Paper 14: song_2008_organizational_mining
- **Academic Reference**: Minseok Song and Wil M. P. van der Aalst (2008). "Organizational Mining: A Heuristic Approach". Information Systems, 33(2):235-251.
- **Formal Algebraic Structures**:
  - Organizational Network: Directed graph $G_{org} = (O_p, E_{org}, W_{org})$:
    - $O_p$: Set of originators (actors).
    - $E_{org} \subseteq O_p \times O_p$: Collaboration edges.
    - $W_{org}: E_{org} \to \mathbb{R}$: Interaction weights.
- **Algorithms**:
  - Handover of Work Metric:
    $W(o_1, o_2) = \frac{\sum_{\sigma \in L} |\{i \mid \text{orig}(\sigma(i))=o_1 \land \text{orig}(\sigma(i+1))=o_2\}|}{\sum_{\sigma \in L} (|\sigma|-1)}$
  - Social Network Centrality:
    - Degree Centrality: $C_D(v) = \deg(v)$.
    - Betweenness Centrality: $C_B(v) = \sum_{s \ne v \ne t} \frac{\sigma_{st}(v)}{\sigma_{st}}$.
- **Preconditions**:
  - Log must contain originator field.
- **Proof Requirements**:
  - Metric Soundness: Interaction metrics are non-negative and scale linearly with trace counts.

### Paper 15: dongen_2009_mxml_to_xes
- **Academic Reference**: Boudewijn F. van Dongen, Christian W. Günther, and H.M.W. Verbeek (2009). "MXML to XES: Transforming Event Logs".
- **Formal Algebraic Structures**:
  - MXML Log Structure: Tree schema mapping ProcessInstance $\to$ AuditTrailEntry $\to$ EventType.
  - XES Log Structure: Strongly typed schema mapping Log $\to$ Trace $\to$ Event.
  - Standard Extensions: Concept, Lifecycle, Time, Organizational.
- **Algorithms**:
  - Schema Mapping and Conversion:
    - Map AuditTrailEntry to Event.
    - Map WorkflowModelElement to concept:name.
    - Normalize datetime format to ISO 8601.
    - Map EventType to lifecycle:transition.
- **Preconditions**:
  - XML structure must conform to MXML schema.
- **Proof Requirements**:
  - Lossless Transition: The mapping $\Phi: MXML \to XES$ preserves all core attributes (Activity, Timestamp, Case ID, Resource) without information loss.