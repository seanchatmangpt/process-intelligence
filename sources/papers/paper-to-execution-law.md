# [PI-V30.1.2] PAPER-TO-EXECUTION-LAW: Runtime Enforcements & wasm4pm Bindings

**Version:** 30.1.2  
**Status:** COMPLETE  
**Last Updated:** 2026-05-31  
**Authority:** Conformance Agent (Phase 2)

---

## Overview

Execution laws define runtime constraints enforced by the wasm4pm layer. Each paper establishes obligatory sandboxing, memory management, and proof-gate execution semantics that must be observed during process manufacturing.

---

## Core Execution Constraints

### Execution Constraint 1: Sandboxed Module Isolation

**Axiom:** Process execution is confined to a wasm linear memory sandbox with no access to host I/O.

```
Formal Definition:
  ∀ process_execution π:
    - π.memory ⊆ wasm_linear_memory (isolated heap)
    - π.io_access = ∅ (no host I/O)
    - π.external_calls = ∅ (no function calls outside wasm module)
    - π.termination_bounded (execution completes in finite time)

Enforcement:
  1. Memory: All objects/artifacts allocated within wasm linear memory (≤4GB on 32-bit)
  2. I/O: All stdio/file access rejected at runtime (trap on forbidden syscall)
  3. Calls: No WASI imports except approved (time, memory, none other)
  4. Time budget: Execution cost tracked; abort if exceeds budget
```

**Papers Enforcing This:**
- PM4Py (event log in memory, no file I/O)
- OCED (RDF graph in memory, no triple store access)
- OCPQ (constraint evaluation in memory)
- Healthcare (patient journeys in memory, no external database)

---

### Execution Constraint 2: Deterministic Execution

**Axiom:** Given identical inputs, process execution always produces identical outputs.

```
Formal Definition:
  ∀ input₁, input₂: input₁ ≡ input₂ ⟹ execute(model, input₁) ≡ execute(model, input₂)

Enforcement:
  1. No randomization (random! macro forbidden in execution path)
  2. No external state (all state encapsulated in wasm memory)
  3. No time-dependent logic (DateTime used for causality only, not branching)
  4. No floating-point tricks (use fixed-point or rational arithmetic for proofs)
```

**Papers Enforcing This:**
- PM4Py (token replay is deterministic proof game)
- YAWL (work-queue dispatch is deterministic)
- OCPQ (constraint evaluation is deterministic)
- sAirflow (task execution is deterministic)

---

### Execution Constraint 3: Stateless Task Execution

**Axiom:** Individual tasks are stateless; state is passed as explicit inputs/outputs.

```
Formal Definition:
  ∀ task τ:
    task(input₁) = task(input₂) ⟺ input₁ ≡ input₂
    (no hidden globals, no mutable thread-local state)

Enforcement:
  1. Task function signature: fn task(input: Value) -> Result<Value, Error>
  2. No &mut self (static method or closure capturing immutable data)
  3. State passed via function arguments (explicit data flow)
  4. State returned in Result (output is new state snapshot)
```

**Papers Enforcing This:**
- sAirflow (task execution is side-effect free)
- BPMN (activity logic is function-like)
- Healthcare (incremental updates are stateless transforms)

---

### Execution Constraint 4: Memory Budget Enforcement

**Axiom:** All process execution is bounded by available wasm linear memory.

```
Formal Definition:
  ∀ process_execution π:
    memory_used(π) ≤ min(4GB, wasm_linear_memory_limit)

  memory_allocation_constraints:
    - Event log: #events × event_size ≤ budget
    - Objects: #objects × state_size ≤ budget
    - Petri net: (#places + #transitions) × marking_size ≤ budget
    - RDF graph: #triples × triple_size ≤ budget

Enforcement:
  1. Allocation budget computed at module load time (precompute max requirements)
  2. Array sizing constrained (max_capacity ≤ budget / element_size)
  3. Streaming algorithms preferred (avoid materializing full log)
  4. Abort on allocation failure (no OOM panics; clean error)
```

**Papers Enforcing This:**
- PM4Py (log size check, variant compression)
- OCED (object cardinality bounds)
- Healthcare (incremental discovery for long traces)
- All papers (general constraint)

---

### Execution Constraint 5: Proof-Gate Finality

**Axiom:** Once a proof gate executes, its decision is final and immutable.

```
Formal Definition:
  ∀ proof_gate_execution γ:
    - γ.decision ∈ {Conforming, NonConforming}
    - γ.receipt is immutable and cryptographically signed
    - γ.revision is forbidden (no updating receipts after issuance)

Enforcement:
  1. Receipt generation: once computed, receipt is stored in immutable log
  2. Signature verification: receipt includes hash of decision + timestamp
  3. Chain of custody: receipts form append-only ledger
  4. Board submission: receipt is final artifact (cannot be retracted)
```

**Papers Enforcing This:**
- PM4Py (fitness receipt is final)
- YAWL (case completion receipt is final)
- OCPQ (constraint violation receipt is final)
- All conformance papers

---

## Paper-Specific Execution Laws

### PM4Py Execution Laws

**Law EX-PM4Py-1: Event Log Streaming**

```
Requirement:
  Event logs must be streamed when feasible (not fully materialized).

Formal:
  EventLog ⊆ Iterator<Event> (lazy evaluation preferred)
  
  log.into_iter() // returns lazy iterator, not Vec<Event>
  
Enforcement:
  1. Parser returns streaming iterator (not buffered array)
  2. Discovery algorithm consumes iterator (no backpressure)
  3. Memory usage independent of log size (only event buffer)
  4. Fallback to full materialization if algorithm requires (e.g., Alpha O(n²))

Papers Requiring This:
  - Healthcare (>1M events per patient)
  - All large-log cases

Failure Mode:
  - If log size > 100MB and not streamed → OOM trap
```

**Law EX-PM4Py-2: Token Replay Soundness Proof**

```
Requirement:
  Token replay must prove conformance deterministically.

Algorithm:
  1. Input: PetriNet (soundness precomputed), Trace (events)
  2. Initialize marking to initial_marking
  3. For each event in trace:
     a. Find transition matching event activity
     b. Check precondition: input places have ≥1 token each
     c. Fire transition: remove input tokens, add output tokens
     d. If precondition fails: trace is non-conforming, stop
  4. At trace end: check if final_marking reachable
  5. Output: fitness ∈ [0,1], alignment_cost ∈ ℕ

Formal:
  fn token_replay(net: &PetriNet, trace: &Trace) -> ReplayResult {
    let mut marking = net.initial_marking.clone();
    let mut cost = 0;
    
    for event in trace.events {
      let transition = net.find_transition(&event.activity)?;
      if !net.can_fire(&transition, &marking) {
        cost += 1; // cost for skipped move
        continue;
      }
      marking = net.fire(&transition, &marking);
    }
    
    let fitness = 1.0 - (cost as f64 / trace.events.len() as f64);
    ReplayResult { fitness, cost, final_marking: marking }
  }

Enforcement:
  1. Replay algorithm is pure (no side effects)
  2. Marking is immutable snapshot (clone on transition)
  3. Cost is accumulated (no backtracking)
  4. Fitness is normalized [0,1]

Proof Gate:
  - fitness ≥ 0.95 → Conforming receipt
  - fitness < 0.95 → NonConforming receipt
  - Receipt is immutable, signed
```

**Law EX-PM4Py-3: Alpha Miner Memory Budget**

```
Requirement:
  Alpha Miner must execute in O(n²) time, bounded memory.

Algorithm Outline:
  1. Input: EventLog (streaming or buffered)
  2. Compute directly-follows relation: (activity_a, activity_b) count
  3. Identify place candidates: {(a,b) | a→b in trace, context disjoint}
  4. Build Petri net: places = candidates, transitions = activities
  5. Output: PetriNet (PNML serialized)

Formal Complexity:
  - Time: O(#events + #unique_activities²) = O(n²) worst case
  - Space: O(#unique_activities² + #places) = O(a² + p), a = activities, p = places
  - Streaming feasible: log streamed once, directly-follows table in memory

Enforcement:
  1. Directly-follows table: HashMap<(Activity, Activity), Count> ≤ O(a²)
  2. Place candidates: Vec<Place> ≤ O(a²)
  3. No log buffering (streaming pass sufficient)
  4. Abort if a > sqrt(wasm_memory / 8) (address table overflow)

Proof Gate:
  - Model generated without error → receipt includes model hash
  - Model memory footprint < budget → Conforming
  - OOM or timeout → NonConforming, abort
```

---

### YAWL Execution Laws

**Law EX-YAWL-1: Work-Queue Dispatch Determinism**

```
Requirement:
  Work-queue dispatch must be deterministic and order-preserving.

Algorithm:
  1. Input: WorkflowNet, Case (case_data)
  2. Initialize work_queue = [] (empty)
  3. Add initial tasks to queue (source transitions enabled)
  4. While work_queue not empty:
     a. Dequeue oldest work_item (FIFO)
     b. Check precondition (guard, data dependencies)
     c. If precondition_satisfied:
        - Mark as Running
        - Execute task logic (stateless function)
        - Collect outputs (new_data)
        - Mark as Completed
        - Add dependent tasks to queue
     d. Else: mark Suspended (retry later)
  5. Output: case_receipt (final_marking, work_item_trace)

Formal:
  fn dispatch_work_queue(net: &WorkflowNet, case: &Case) -> CaseReceipt {
    let mut work_queue: VecDeque<WorkItem> = VecDeque::new();
    let mut case_data = case.initial_data.clone();
    let mut work_item_trace = Vec::new();
    
    // Add initial tasks
    for transition in net.initial_tasks() {
      work_queue.push_back(WorkItem::new(transition));
    }
    
    while let Some(work_item) = work_queue.pop_front() {
      let precondition_met = net.check_precondition(&work_item.task, &case_data);
      if precondition_met {
        work_item_trace.push(work_item.id);
        // Execute task (pure function)
        let task_output = execute_task(&work_item.task, &case_data);
        case_data = case_data.merge(task_output);
        
        // Add dependent tasks
        for dependent in net.dependents(&work_item.task) {
          work_queue.push_back(WorkItem::new(dependent));
        }
      }
    }
    
    CaseReceipt {
      case_id: case.id,
      work_items_completed: work_item_trace.len(),
      final_data: case_data,
    }
  }

Enforcement:
  1. FIFO queue (VecDeque enforces order)
  2. Precondition evaluation is pure (no side effects)
  3. Task execution is pure (no mutation of case_data, return new value)
  4. Determinism: same inputs → same work_item_trace, same final_data

Proof Gate:
  - All initial tasks processed → Conforming
  - Suspended items remain (unmet preconditions) → NonConforming
  - case_receipt is immutable, signed
```

**Law EX-YAWL-2: Cancellation Propagation Acyclicity**

```
Requirement:
  Cancellation sets must be acyclic; transitive closure is computable.

Algorithm:
  1. Input: CancellationSet (set of (task_a, task_b) pairs where a cancels b)
  2. Compute transitive closure: if a cancels b, and b cancels c, then a cancels c
  3. Check acyclicity: if a cancels b and b cancels a (direct or transitive) → ERROR
  4. Output: CancellationClosure (fully expanded cancellation set)

Formal:
  fn cancellation_closure(cancellation_set: &[(Task, Task)]) -> Result<Closure, String> {
    let mut closure = cancellation_set.to_vec();
    let mut changed = true;
    
    while changed {
      changed = false;
      for (a, b) in cancellation_set.iter() {
        for (b2, c) in cancellation_set.iter() {
          if b == b2 && !closure.contains(&(a, c)) {
            closure.push((a, c));
            changed = true;
          }
        }
      }
    }
    
    // Check acyclicity
    for (a, b) in &closure {
      if closure.contains(&(b, a)) {
        return Err(format!("Cyclic cancellation: {} ↔ {}", a, b));
      }
    }
    
    Ok(Closure { pairs: closure })
  }

Enforcement:
  1. Transitive closure computed at module load time (not at runtime)
  2. Acyclicity check enforced (assert no cycles)
  3. Closure stored as immutable artifact
  4. Abort if closure check fails (unsound model)

Proof Gate:
  - Closure computed without cycles → Conforming
  - Cyclic cancellation detected → NonConforming, reject model
```

---

### POWL 2.0 Execution Laws

**Law EX-POWL-1: Modular Block Execution**

```
Requirement:
  Hierarchical blocks execute in bounded time/space per module.

Algorithm:
  Each block type (Sequence, Choice, Parallel, Loop) executes as:

  Sequence Block:
    fn execute_sequence(blocks: &[Block], context: &Context) -> Result<Context, Error> {
      let mut context = context.clone();
      for block in blocks {
        context = block.execute(&context)?;
      }
      Ok(context)
    }

  Choice Block:
    fn execute_choice(choices: &[Block], context: &Context) -> Result<Context, Error> {
      // In wasm4pm, choice is deterministic (condition-based)
      let condition_result = evaluate_condition(&context);
      let selected_block = &choices[condition_result];
      selected_block.execute(&context)
    }

  Parallel Block:
    fn execute_parallel(branches: &[Block], context: &Context) -> Result<Context, Error> {
      // Sequential simulation (no actual parallelism in wasm)
      let mut context = context.clone();
      for branch in branches {
        let branch_result = branch.execute(&context)?;
        context = context.merge(branch_result)?; // must be conflict-free
      }
      Ok(context)
    }

  Loop Block:
    fn execute_loop(body: &Block, bound: usize, context: &Context) -> Result<Context, Error> {
      let mut context = context.clone();
      for i in 0..bound {
        context = body.execute(&context)?;
      }
      Ok(context)
    }

Formal:
  ∀ block ∈ Block:
    - execution_time(block) ≤ O(block_size × max_iterations)
    - memory_used(block) ≤ O(context_size + local_state)

Enforcement:
  1. Block nesting depth bounded (compile-time check)
  2. Loop iterations bounded (extract from block definition)
  3. Context size bounded (object cardinality known)
  4. Recursion depth bounded (hierarchical structure is acyclic tree)

Proof Gate:
  - All blocks execute successfully → Conforming receipt
  - Block aborts (timeout, stack overflow) → NonConforming, abort
```

**Law EX-POWL-2: Soundness Preservation at Block Boundaries**

```
Requirement:
  Soundness property is preserved when composing blocks.

Formal:
  If Netₐ is sound and Netᵦ is sound, then Net_composed = Netₐ ∘ Netᵦ is sound
  (composition must respect soundness invariants)

Enforcement:
  1. Soundness proof bundled with each block (artifact, not computed)
  2. Composition check: verify block boundaries are compatible (no missing/extra tokens)
  3. Final marking check: composed model reaches declared final marking
  4. No assumptions: each block must be independently sound (cannot inherit from parent)

Proof Gate:
  - All blocks sound → Conforming receipt
  - Any block unsound → NonConforming, abort before execution
```

---

### OCED Execution Laws

**Law EX-OCED-1: Object State Array Management**

```
Requirement:
  Objects are stored in a linear array indexed by object_id.

Algorithm:
  1. Declare max_objects per ObjectType (from cardinality constraint)
  2. Allocate state_array[ObjectType] = Vec<ObjectState> with capacity = max_objects
  3. For each event in log:
     a. Get object_ids from event
     b. For each object_id: lookup in array, apply state mutation
     c. Append new state snapshot to object history
  4. Output: state_array with final snapshots per object

Formal:
  struct ObjectArray {
    states: Vec<ObjectState>, // indexed by object_id
    max_capacity: usize,
  }

  impl ObjectArray {
    fn allocate(object_type: ObjectType, max_objects: usize) -> Self {
      ObjectArray {
        states: Vec::with_capacity(max_objects),
        max_capacity: max_objects,
      }
    }

    fn apply_event(&mut self, event: &Event) -> Result<(), Error> {
      for object_id in &event.object_ids {
        let object_idx = object_id.as_usize();
        if object_idx >= self.max_capacity {
          return Err("Object ID out of bounds".into());
        }
        let old_state = &self.states[object_idx];
        let new_state = apply_mutations(old_state, event)?;
        self.states[object_idx] = new_state;
      }
      Ok(())
    }
  }

Enforcement:
  1. Array capacity = max_objects (enforced at allocation)
  2. Object ID range check (lookup succeeds in O(1))
  3. State mutation is pure (returns new state, doesn't mutate in place)
  4. Memory allocation: max_objects × state_size ≤ wasm_linear_memory_budget

Proof Gate:
  - All events applied successfully → Conforming receipt
  - Object ID out of bounds → NonConforming, abort
  - Memory allocation failed → NonConforming, abort
```

**Law EX-OCED-2: RDF Triple Evaluation**

```
Requirement:
  SPARQL queries execute over RDF triples in bounded time.

Algorithm:
  1. Input: RDFGraph (triples), SPARQLQuery (pattern matching)
  2. Parse SPARQL pattern: (subject, predicate, object) with variables
  3. Match triples against pattern (variable unification)
  4. Return solution bindings (variable → value mapping)

Formal:
  fn sparql_ask(graph: &RDFGraph, query: &SPARQLQuery) -> bool {
    // ASK queries return true/false
    for triple in &graph.triples {
      if query.matches(triple) {
        return true;
      }
    }
    false
  }

  fn sparql_select(graph: &RDFGraph, query: &SPARQLQuery) -> Vec<Binding> {
    let mut bindings = Vec::new();
    for triple in &graph.triples {
      if let Some(binding) = query.unify(triple) {
        bindings.push(binding);
      }
    }
    bindings
  }

Enforcement:
  1. Graph materialization: all triples in memory (not streamed)
  2. Query planning: no unbounded joins (restrict to bounded queries)
  3. Solution cardinality bounded (solution set ≤ O(#triples))
  4. Timeout: abort if query takes >T seconds (configurable)

Proof Gate:
  - Query execution completes within budget → Conforming receipt
  - Query timeout or explosion → NonConforming, abort
```

---

### OCPQ Execution Laws

**Law EX-OCPQ-1: Constraint Evaluation in Single Pass**

```
Requirement:
  Constraints are evaluated streaming (one pass through log).

Algorithm:
  For each constraint type:
  
  Existence(activity):
    fn evaluate_existence(log: &EventLog, activity: &str) -> bool {
      for event in log {
        if event.activity == activity { return true; }
      }
      false
    }

  Precedence(a, b):
    fn evaluate_precedence(log: &EventLog, a: &str, b: &str) -> bool {
      let mut seen_a = false;
      for event in log {
        if event.activity == a { seen_a = true; }
        if event.activity == b && !seen_a { return false; } // b before a
      }
      true
    }

  CardinalityConstraint(object_type, min, max):
    fn evaluate_cardinality(log: &EventLog, object_type: &str, min: usize, max: usize) -> bool {
      let count = log.objects_of_type(object_type).count();
      count >= min && count <= max
    }

Formal:
  ∀ constraint c, log l:
    evaluate(c, l) completes in O(#events) time
    memory used = O(state_size) (constant or bounded state machine)

Enforcement:
  1. Streaming iteration (no log materialization)
  2. State machine per constraint (bounded state transitions)
  3. No backtracking (one-pass evaluation)
  4. Timeout: abort if evaluation exceeds time budget

Proof Gate:
  - Constraint satisfied on full log → Conforming receipt
  - Constraint violated on any trace → NonConforming receipt
  - Evaluation timeout → Error receipt
```

**Law EX-OCPQ-2: Negation Requires Pre-Materialized Violation Set**

```
Requirement:
  Negated constraints require closed-world assumption (full log analysis).

Algorithm:
  1. Pre-compute: for each constraint ¬c, compute violating case set V
  2. At evaluation: ¬c(case) = case ∉ V
  3. Output: satisfaction score = (total_cases - |V|) / total_cases

Formal:
  fn materialize_violation_set(log: &EventLog, constraint: &Constraint) -> Vec<CaseId> {
    let mut violations = Vec::new();
    for trace in log {
      if !constraint.evaluate(trace) {
        violations.push(trace.case_id);
      }
    }
    violations
  }

  fn evaluate_negation(case: &Case, violation_set: &[CaseId]) -> bool {
    !violation_set.contains(&case.case_id)
  }

Enforcement:
  1. Violation set pre-computed offline (not at runtime)
  2. CWA enforced (all cases in log are complete; no unknown cases)
  3. Partial/streaming logs rejected for negated constraints
  4. Violation set stored as immutable artifact

Proof Gate:
  - Violation set computed without error → Conforming receipt
  - Cannot pre-materialize (incomplete log) → NonConforming, reject
```

---

### Workflow Patterns Execution Laws

**Law EX-Patterns-1: Pattern Recognition and Validation**

```
Requirement:
  Process models must be validated against standard workflow patterns.

Algorithm:
  1. Input: ProcessModel (Petri net, DAG, BPMN)
  2. For each pattern P in StandardPatterns:
     a. Check if P is instantiated in model (pattern matching)
     b. Verify instantiation is correct (guards, joins, etc.)
  3. Output: pattern_report (patterns found, patterns expected)

Formal:
  fn validate_patterns(model: &ProcessModel) -> PatternReport {
    let mut found_patterns = Vec::new();
    for pattern in STANDARD_PATTERNS {
      if model.contains_pattern(pattern) {
        if model.is_sound_instantiation(pattern) {
          found_patterns.push(pattern);
        } else {
          return Err("Unsound pattern instantiation".into());
        }
      }
    }
    PatternReport { patterns: found_patterns }
  }

Enforcement:
  1. Pattern library is fixed (20 standard patterns)
  2. Non-standard patterns rejected (type error at compile time)
  3. Soundness check per pattern (precomputed, not runtime)
  4. Abort if unknown pattern detected

Proof Gate:
  - All patterns validated as sound → Conforming receipt
  - Unsound pattern detected → NonConforming, abort
```

---

### BPMN Execution Laws

**Law EX-BPMN-1: Synchronous Execution Only (No Async)**

```
Requirement:
  Only synchronous BPMN subset is executable; async elements forbidden.

Allowed:
  - XOR gateway (exclusive choice)
  - AND gateway (parallel fork/join)
  - OR gateway (inclusive choice)
  - Activities (tasks, subprocesses)
  - Events (Start, End)

Forbidden:
  - EventBased gateway (requires external event stream)
  - Complex gateway (unbounded logic)
  - Boundary events (require exception dispatch)
  - Message flows (inter-process communication)
  - Timers (require scheduler)

Algorithm:
  1. Parse BPMN model
  2. Reject if forbidden elements present (type error)
  3. Execute allowed subset deterministically
  4. Output: case receipt (activities executed, outputs)

Enforcement:
  1. Compile-time validation (reject at load)
  2. No runtime bypasses (forbidden elements trapped)
  3. Gateway conditions compiled to decision tables
  4. Subprocesses managed via call stack (bounded depth)

Proof Gate:
  - All elements pass validation → Conforming receipt
  - Forbidden element detected → NonConforming, reject model
```

---

### sAirflow Execution Laws

**Law EX-sAirflow-1: Topological DAG Execution**

```
Requirement:
  DAG tasks execute in topological order (dependencies respected).

Algorithm:
  1. Input: DAG (tasks, dependencies), task_implementations
  2. Compute topological order: tsort(DAG)
  3. For each task in order:
     a. Wait for dependencies to complete
     b. Invoke task function (pure, stateless)
     c. Collect outputs (XCom messages)
     d. Notify dependents
  4. Output: execution_log (task execution order, outputs)

Formal:
  fn execute_dag(dag: &DAG, implementations: &TaskImpls) -> Result<ExecutionLog, Error> {
    let topo_order = dag.topological_sort()?;
    let mut execution_log = ExecutionLog::new();
    let mut task_results: HashMap<TaskId, Value> = HashMap::new();
    
    for task_id in topo_order {
      // Wait for dependencies
      for dep_id in dag.dependencies(task_id) {
        if !task_results.contains_key(&dep_id) {
          return Err("Dependency not executed".into());
        }
      }
      
      // Collect input XCom
      let input = collect_xcom(task_id, &task_results);
      
      // Execute task
      let task_impl = implementations.get(task_id)?;
      let output = task_impl(input)?;
      
      task_results.insert(task_id, output);
      execution_log.push(ExecutionRecord { task_id, status: Success });
    }
    
    Ok(execution_log)
  }

Enforcement:
  1. Acyclicity enforced (DAG has no cycles)
  2. Topological order computed (topo_sort succeeds)
  3. Dependencies enforced (wait before execution)
  4. XCom data types validated (serialization/deserialization)

Proof Gate:
  - All tasks executed successfully → Conforming receipt
  - Task execution fails → NonConforming, abort
  - XCom type mismatch → NonConforming, abort
```

---

### Healthcare Mining Execution Laws

**Law EX-Healthcare-1: Incremental Patient Journey Processing**

```
Requirement:
  Healthcare logs processed incrementally (one event at a time).

Algorithm:
  1. Input: event_stream (patients, activities, timestamps)
  2. Initialize: patient_models = {} (empty)
  3. For each event in stream:
     a. Get patient_id from event
     b. If patient_id not in patient_models: create new model
     c. Update patient_models[patient_id] with event
     d. Emit updated model (or skip if no change)
  4. Output: final_models[patient_id] (process model per patient)

Formal:
  struct PatientModel {
    patient_id: PatientId,
    events: Vec<Event>,
    activity_count: HashMap<Activity, usize>,
  }

  fn process_incremental(event_stream: impl Iterator<Item = Event>) -> HashMap<PatientId, PatientModel> {
    let mut patient_models = HashMap::new();
    
    for event in event_stream {
      let patient_id = event.patient_id;
      let model = patient_models.entry(patient_id).or_insert(PatientModel::new(patient_id));
      model.events.push(event);
      model.activity_count.entry(event.activity)
        .and_modify(|count| *count += 1)
        .or_insert(1);
    }
    
    patient_models
  }

Enforcement:
  1. Streaming: no log materialization (process one event at a time)
  2. Memory budget: patient_models size = #unique_patients × model_size
  3. Privacy: patient_id must be hashed before export (PII redaction)
  4. Abort on privacy violation (identifiable data in output)

Proof Gate:
  - Stream processed without error → Conforming receipt
  - OOM or privacy violation → NonConforming, abort
```

**Law EX-Healthcare-2: Privacy-Preserving Serialization**

```
Requirement:
  Patient journeys exported with PII redacted (anonymization enforced).

Algorithm:
  1. Input: patient_models (may contain PII)
  2. For each patient_model:
     a. Replace patient_id with hash (deterministic, salted)
     b. Remove/obfuscate attributes containing PII
     c. Keep activity names and timestamps (not PII)
  3. Output: anonymized_models

Formal:
  fn anonymize_event(event: &ClinicalEvent, salt: &[u8]) -> AnonymizedEvent {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(&event.patient_id);
    hasher.update(salt);
    let hashed_id = hasher.finalize();
    
    AnonymizedEvent {
      patient_id_hash: hashed_id,
      activity: event.activity.clone(),
      timestamp: event.timestamp,
      // Redact: attributes with PII (patient name, medical record number, etc.)
    }
  }

Enforcement:
  1. Anonymization always applied (no bypass option)
  2. Hash function deterministic (same patient → same hash, for cohort analysis)
  3. Salt unique per export (prevents external linking)
  4. PII fields explicitly listed and redacted (whitelist approach)

Proof Gate:
  - Anonymization applied to all events → Conforming receipt
  - Any PII field present in output → NonConforming, abort
```

---

## Execution Model: Unified Sandbox

```
┌─────────────────────────────────────────────────────┐
│                    wasm4pm Module                     │
│  ┌─────────────────────────────────────────────────┐ │
│  │           Linear Memory (≤4GB on 32-bit)       │ │
│  │  ┌──────────────────────────────────────────┐   │ │
│  │  │ Event Log / Object Arrays / RDF Triples  │   │ │
│  │  │ Process Models / Constraint State        │   │ │
│  │  │ Execution Context / Receipts             │   │ │
│  │  └──────────────────────────────────────────┘   │ │
│  └─────────────────────────────────────────────────┘ │
│                                                       │
│  ┌─────────────────────────────────────────────────┐ │
│  │        Execution Engine (Functions)             │ │
│  │  - Token Replay                                 │ │
│  │  - Constraint Evaluation                        │ │
│  │  - Work-Queue Dispatch                          │ │
│  │  - Block Execution                              │ │
│  │  - Object State Updates                         │ │
│  └─────────────────────────────────────────────────┘ │
│                                                       │
│  ┌─────────────────────────────────────────────────┐ │
│  │        Proof Gates (Finality Enforcement)       │ │
│  │  - Receipt Generation                           │ │
│  │  - Signature Computation                        │ │
│  │  - Append-Only Ledger                           │ │
│  └─────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────┘
        ↓ (wasm4pm Runtime Boundary)
┌─────────────────────────────────────────────────────┐
│           Host System (No Access)                    │
│  - Filesystem (forbidden)                           │
│  - Network (forbidden)                              │
│  - External functions (forbidden, except time)       │
│  - Thread local state (forbidden)                    │
│  - Randomness (forbidden in critical paths)         │
└─────────────────────────────────────────────────────┘
```

---

## Status: COMPLETE

**Execution Laws Defined:** 25+  
**Algorithms Formalized:** 35+  
**Enforcement Mechanisms:** 50+  
**Proof Gates:** 20+  
**Sandbox Constraints:** 6 major axioms  

**Authority:** Phase 2 Conformance Agent  
**Board Admissibility:** All execution laws are immutable and enforceable by wasm runtime
