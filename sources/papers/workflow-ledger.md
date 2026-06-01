# Process Intelligence Workflow Ledger

This ledger registers the 15 core workflow papers, mapping them to their structural invariants, concurrency constraints, and verification status in the Process Intelligence Foundry.

For mathematical structures, see [paper-canon.md](file:///Users/sac/process-intelligence/sources/papers/paper-canon.md).
For type classification, see [paper-to-type-law.md](file:///Users/sac/process-intelligence/sources/papers/paper-to-type-law.md).
For execution fixtures, see [paper-to-execution-law.md](file:///Users/sac/process-intelligence/sources/papers/paper-to-execution-law.md).

---

* **Paper Ledger Entry 1**: `van_der_aalst_1998_workflow_nets`
  - *Structural Invariants*: Source-sink isolation, path connectivity.
  - *Concurrency Constraints*: Soundness of the short-circuited net $\bar{N}$ requires boundedness (finite markings) and liveness (no dead transitions).
  - *Verification Status*: Checked by static analysis of net structure and reachability analysis of the token game.

* **Paper Ledger Entry 2**: `adriansyah_2014_alignment_conformance`
  - *Structural Invariants*: Alignment move constraints.
  - *Concurrency Constraints*: Optimal path cost search bounds on model reachability graph.
  - *Verification Status*: Checked via A* state space traversal cost verification.

* **Paper Ledger Entry 3**: `leemans_2013_inductive_miner`
  - *Structural Invariants*: Block structure Process Tree syntax.
  - *Concurrency Constraints*: Mutually exclusive cut partitions on the directly-follows graph.
  - *Verification Status*: Verified by recursive log partitioning.

* **Paper Ledger Entry 4**: `ghahfarokhi_2021_ocel2`
  - *Structural Invariants*: Object-centric schema constraints, typed relationships.
  - *Concurrency Constraints*: Multi-entity token synchronization across object types.
  - *Verification Status*: Validated against OCEL 2.0 schema definition files.

* **Paper Ledger Entry 5**: `verbeek_2021_log_trie`
  - *Structural Invariants*: Prefix tree topology.
  - *Concurrency Constraints*: State sharing along common trace prefixes.
  - *Verification Status*: Verified by trie construction and search space reduction benchmarks.

* **Paper Ledger Entry 6**: `kuesters_2024_ocpq_tree`
  - *Structural Invariants*: Query AST structures.
  - *Concurrency Constraints*: Object-centric pattern matching rules across temporal sequences.
  - *Verification Status*: Verified by query compilation correctness tests.

* **Paper Ledger Entry 7**: `weske_2019_bpm_principles`
  - *Structural Invariants*: BPMN graph well-formedness.
  - *Concurrency Constraints*: Gateway execution semantics (AND-join, XOR-split, OR-routing).
  - *Verification Status*: Checked by reachability state space analysis.

* **Paper Ledger Entry 8**: `rosenberg_2020_cloud_lifecycle`
  - *Structural Invariants*: FSM transition mappings.
  - *Concurrency Constraints*: Deterministic state progression for cloud resources.
  - *Verification Status*: Verified by event stream simulation.

* **Paper Ledger Entry 9**: `aalst_2016_process_mining_action`
  - *Structural Invariants*: Heuristic Net causal mappings.
  - *Concurrency Constraints*: Dependency thresholds on activity direct-follows frequencies.
  - *Verification Status*: Verified by Heuristic Miner log discovery.

* **Paper Ledger Entry 10**: `aalst_2004_workflow_patterns`
  - *Structural Invariants*: Control-flow pattern topologies.
  - *Concurrency Constraints*: Synchronization, choice, loop, and cancellation semantics.
  - *Verification Status*: Verified by pattern composition soundness proofs.

* **Paper Ledger Entry 11**: `weidlich_2011_profile_conformance`
  - *Structural Invariants*: Behavioral Profile relations ($\to, +, \parallel$).
  - *Concurrency Constraints*: Mutually exclusive behavioral constraints on transition pairs.
  - *Verification Status*: Verified by profile matrix extraction and comparison.

* **Paper Ledger Entry 12**: `de_medeiros_2007_genetic_miner`
  - *Structural Invariants*: Causal Matrix schemas.
  - *Concurrency Constraints*: Population crossover constraints and fitness optimization.
  - *Verification Status*: Checked via genetic generation metrics.

* **Paper Ledger Entry 13**: `gunther_2007_fuzzy_miner`
  - *Structural Invariants*: Fuzzy Causal Graph.
  - *Concurrency Constraints*: Significance/correlation thresholds for abstraction.
  - *Verification Status*: Verified by simplified graph rendering and edge reduction.

* **Paper Ledger Entry 14**: `song_2008_organizational_mining`
  - *Structural Invariants*: Originator-collaboration social graph.
  - *Concurrency Constraints*: Originator handover and closeness centralities.
  - *Verification Status*: Verified by social network weight matrix verification.

* **Paper Ledger Entry 15**: `dongen_2009_mxml_to_xes`
  - *Structural Invariants*: MXML and XES XML tree schemas.
  - *Concurrency Constraints*: Normalized temporal ordering.
  - *Verification Status*: Checked via XML schema validator.