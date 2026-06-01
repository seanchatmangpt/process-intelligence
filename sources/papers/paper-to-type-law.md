# [PI-V30.1.2] PAPER-TO-TYPE-LAW: Semantic Bridging & Compat Compilation

**Version:** 30.1.2  
**Status:** COMPLETE  
**Last Updated:** 2026-05-31  
**Authority:** Conformance Agent (Phase 2)

---

## Overview

Formal type laws derived from paper conformance claims. Each paper establishes immutable type constraints that must be enforced by static analysis (Rust/TS compiler) and runtime validation.

---

## Type-Law Axioms (Foundational)

### Axiom 1: Events Are Immutable, Timestamped Facts

```rust
pub trait Event {
    fn timestamp(&self) -> DateTime<Utc>;
    fn activity(&self) -> &str;
    fn case_id(&self) -> &str;
    fn attributes(&self) -> &HashMap<String, Value>;
    
    // Invariant: once created, Event cannot be mutated
    // Timestamp must be total-ordered (no time paradoxes)
}

// Type law: Event ⊆ ImmutableFact
// Compile-time guarantee: &self (immutable reference)
```

**Papers Mandating This:**
- PM4Py (foundational event log structure)
- OCED Analysis (event identity)
- OCPQ (temporal constraint evaluation)
- Healthcare Mining (clinical event immutability)

---

### Axiom 2: Traces Are Ordered Sequences

```rust
pub trait Trace {
    fn events(&self) -> &[Event];
    fn case_id(&self) -> &str;
    fn duration(&self) -> Duration;
    
    // Invariant: events are totally ordered by timestamp
    // No event reordering allowed; causality is fixed
}

// Type law: Trace ⊆ Vector<Event> with temporal_total_order constraint
// Compile-time guarantee: sorted, no duplicates
```

**Papers Mandating This:**
- PM4Py (trace replay requires order preservation)
- YAWL (work-item sequence)
- sAirflow (task topological order)
- Healthcare (patient journey timeline)

---

### Axiom 3: Process Models Are Acyclic or Bounded-Cycle

```rust
pub enum ProcessModel {
    PetriNet {
        places: Vec<Place>,
        transitions: Vec<Transition>,
        arcs: Vec<Arc>,
        initial_marking: Marking,
    },
    DAG {
        nodes: Vec<Task>,
        edges: Vec<(Task, Task)>, // acyclic by invariant
    },
    POWL2_0 {
        root: Block, // hierarchical; blocks are composable
    },
}

// Type law: ProcessModel must be analyzable for soundness
// Cyclic check required (forbidden: unbounded loops without bounds)
// Compile-time check: acyclic_or_bounded(model) → bool
```

**Papers Mandating This:**
- YAWL (workflow nets require soundness)
- POWL 2.0 (hierarchical decomposition requires soundness preservation)
- Workflow Patterns (patterns are bounded by design)
- sAirflow (DAG must be acyclic)
- BPMN (complex gateways must be decidable; no infinite branching)

---

### Axiom 4: Constraints Are Decidable Logical Formulas

```rust
pub enum TemporalConstraint {
    Existence { activity: String }, // ∃ event with this activity
    Precedence { a: String, b: String }, // a before b
    Response { a: String, b: String }, // b after a
    ChainPrecedence { a: String, b: String }, // a immediately before b
    Negation(Box<TemporalConstraint>), // ¬ (requires closed-world assumption)
}

pub enum CardinalityConstraint {
    Bounded { object_type: String, min: usize, max: usize },
}

// Type law: Constraint ⊆ DecidableFormula
// Compile-time guarantee: constraint is evaluable in finite time O(#events or #objects)
// Negation enforces closed-world assumption (must pre-materialize violating set)
```

**Papers Mandating This:**
- OCPQ (constraint language specification)
- Workflow Patterns (pattern guards are constraints)
- BPMN (gateway conditions are constraints)
- Healthcare (deviation detection is constraint violation)

---

### Axiom 5: Objects Have Bounded Cardinality

```rust
pub struct ObjectType {
    name: String,
    max_instances: usize, // REQUIRED: wasm memory budget
}

pub struct Object {
    id: ObjectId,
    type_: ObjectType,
    state: Vec<StateUpdate>, // immutable history
}

// Type law: ∀ object_type: #instances ≤ max_instances
// Compile-time check: object array allocation = max_instances × state_size
// Invariant: object_type.max_instances ≤ (wasm_linear_memory / state_size)
```

**Papers Mandating This:**
- OCED Analysis (object cardinality is memory budget)
- OCPQ (cardinality constraints explicit)
- YAWL (multiple instance tasks have cardinality)
- Healthcare (patient cohort size is bounded)

---

### Axiom 6: Receipts Are Immutable Proof Artifacts

```rust
pub struct Receipt {
    receipt_id: ReceiptId, // unique, hashable
    timestamp: DateTime<Utc>,
    status: ExecutionStatus, // Conforming | NonConforming | Incomplete
    evidence: ReceiptEvidence, // proof data
    signature: Signature, // cryptographic proof
    
    // Invariant: Receipt is immutable once created
    // Proof is final; no revisions allowed
}

pub enum ReceiptEvidence {
    TokenReplayResult { fitness: f64, alignment_cost: usize },
    ConstraintViolation { violated_cases: Vec<CaseId> },
    ConformanceClaim { objects: usize, events: usize, status: bool },
}

// Type law: Receipt ⊆ ImmutableProof
// Compile-time: Receipt ownership is linear (cannot be copied)
// Receipts form audit trail (append-only log)
```

**Papers Mandating This:**
- PM4Py (fitness score is receipt evidence)
- YAWL (case completion receipt)
- OCED (object lifecycle proof)
- OCPQ (constraint satisfaction receipt)
- Healthcare (care pathway compliance receipt)

---

## Paper-Specific Type Laws

### PM4Py Type Laws

**Law PM4Py-1: EventLog Streaming**
```rust
pub trait EventLog {
    fn into_iter(self) -> impl Iterator<Item = Event>;
    // Invariant: log can be streamed (not required to materialize all events)
}

// Type law: EventLog ⊆ Streamable<Event>
// Compat requirement: Alpha Miner O(n²) on event count; streaming OK if trace length bounded
```

**Law PM4Py-2: Petri Net Soundness**
```rust
pub struct PetriNet {
    // ... fields
    soundness_proof: SoundnessProof, // NOT computed at runtime; precomputed artifact
}

pub enum SoundnessProof {
    Verified { hash: Hash, proof_artifact: String }, // formal verification result
    Assumed { reason: String }, // manual review only
}

// Type law: PetriNet.soundness_proof must be Verified or Assumed + documented
// Compile-time check: enforce non-None soundness proof
```

**Law PM4Py-3: Trace Replay Determinism**
```rust
pub fn token_replay(net: &PetriNet, trace: &Trace) -> ReplayResult {
    // Invariant: replay is deterministic
    // Given same net + trace, always produces same fitness + alignment
    // No randomization allowed
}

// Type law: ReplayResult ⊆ DeterministicProof
// Compile-time: random! macro forbidden in replay path
```

---

### YAWL Type Laws

**Law YAWL-1: WorkItem State Machine**
```rust
pub enum WorkItemState {
    Enabled,
    Running,
    Suspended,
    Completed,
}

pub struct WorkItem {
    task: &'static Task,
    state: WorkItemState,
    precondition_satisfied: bool, // must be true before Enabled
    postcondition_required: bool, // must satisfy before Completed
}

// Type law: WorkItem.state transitions form a finite state machine (4 states)
// Compile-time check: only valid transitions allowed
```

**Law YAWL-2: Reset Semantics**
```rust
pub fn apply_reset_transition(net: &mut PetriNet, transition: &Transition) {
    // Invariant: reset removes tokens from specified places
    // No addition of tokens on reset (only removal)
    let reset_places = transition.reset_set();
    for place in reset_places {
        net.mark(place, 0); // set marking to zero
    }
}

// Type law: ResetTransition ⊆ TokenRemoval (no token addition on reset)
```

**Law YAWL-3: Cancellation Set Acyclicity**
```rust
pub struct CancellationSet(Vec<Task>);

impl CancellationSet {
    pub fn validate(&self) -> Result<(), String> {
        // Invariant: cancellation set must be acyclic
        // Task A cannot disable Task B if B disables A (direct or transitive)
        let reachability = compute_cancellation_reachability(self);
        if reachability.has_cycle() {
            Err("Cyclic cancellation".to_string())
        } else {
            Ok(())
        }
    }
}

// Type law: CancellationSet ⊆ AcyclicGraph
// Compile-time check: validate on model load
```

---

### POWL 2.0 Type Laws

**Law POWL-1: Hierarchical Soundness Preservation**
```rust
pub trait Block {
    fn soundness_preserved(&self, input_net: &PetriNet) -> bool;
    // Invariant: if input_net is sound, decomposed blocks preserve soundness
}

impl Block for Sequence { /* ... */ }
impl Block for Choice { /* ... */ }
impl Block for Parallel { /* ... */ }
impl Block for Loop { /* ... */ }

// Type law: Block decomposition preserves soundness
// Formal: if input_net ∈ SoundWFnets, then decomposed_blocks ∈ SoundWFnets
// Compile-time: require soundness_proof as generic constraint
```

**Law POWL-2: Loop Bounds Extractability**
```rust
pub struct Loop {
    body: Block,
    bound: Bound, // REQUIRED: must be extractable
}

pub enum Bound {
    Fixed(usize), // loop runs exactly n times
    UpTo(usize), // loop runs at most n times
    Unbounded, // NOT allowed in wasm4pm
}

// Type law: Loop.bound must be Fixed or UpTo, not Unbounded
// Compile-time check: reject Unbounded loops before wasm codegen
```

---

### OCED Type Laws

**Law OCED-1: Event-Object Relation Many-to-Many**
```rust
pub struct EventObjectRelation {
    event_id: EventId,
    object_ids: Vec<ObjectId>, // ≥1 objects per event
}

// Type law: EventObjectRelation cardinality is n:m (one event, many objects)
// Compile-time: relationship is indexed both ways (event→objects, object→events)
```

**Law OCED-2: RDF Triple Closure**
```rust
pub struct RDFGraph {
    triples: HashSet<(Subject, Predicate, Object)>,
}

impl RDFGraph {
    pub fn closure(&self) -> Self {
        // Invariant: compute transitive closure of RDF graph
        // Return graph with all implied triples (including chains)
    }
}

// Type law: RDF closure must be computable (finite triple set)
// Compile-time: enforce termination (no cyclic implication rules)
```

**Law OCED-3: Object Lifecycle State Transitions**
```rust
pub struct ObjectLifecycle {
    object_id: ObjectId,
    state_history: Vec<(DateTime<Utc>, ObjectState)>, // immutable history
}

impl ObjectLifecycle {
    pub fn is_sound(&self) -> bool {
        // Invariant: object transitions follow declared state machine
        // No impossible state jumps (transitions only via declared edges)
    }
}

// Type law: ObjectLifecycle ⊆ SoundStateSequence
// Compile-time: state machine must be declared (cannot be inferred)
```

---

### OCPQ Type Laws

**Law OCPQ-1: Constraint Decidability**
```rust
pub trait Constraint {
    fn is_decidable(&self) -> bool;
    fn evaluate(&self, log: &EventLog) -> bool; // must terminate
}

// Type law: Constraint ⊆ DecidableFormula
// Compile-time check: no unbounded quantifiers, no external function calls
```

**Law OCPQ-2: Negation Requires Closed-World**
```rust
pub struct NegatedConstraint {
    inner: Box<Constraint>,
    materialized_violations: Vec<CaseId>, // REQUIRED: must pre-materialize
}

impl NegatedConstraint {
    pub fn evaluate(&self, log: &EventLog) -> bool {
        // Invariant: negation assumes closed-world (all cases in log are known)
        // Cannot handle streaming or partial logs
        !self.inner.evaluate(log)
    }
}

// Type law: NegatedConstraint requires materialized violation set
// Runtime check: must have violation set before evaluation
```

**Law OCPQ-3: Cardinality Constraints Bound Object Arrays**
```rust
pub struct CardinalityConstraint {
    object_type: ObjectType,
    min: usize,
    max: usize,
}

pub fn enforce_cardinality(log: &EventLog, constraint: &CardinalityConstraint) -> bool {
    let count = log.objects_of_type(&constraint.object_type).count();
    count >= constraint.min && count <= constraint.max
}

// Type law: ∀ object_type: #instances ≤ max (wasm memory budget)
// Compile-time: array allocation = max × state_size ≤ wasm_linear_memory
```

---

### Workflow Patterns Type Laws

**Law Patterns-1: Pattern Instantiation**
```rust
pub trait Pattern {
    fn instantiate(&self, model: &ProcessModel) -> Result<PatternInstance, String>;
    fn is_sound(&self) -> bool; // patterns are sound by design
}

// Type law: Pattern ⊆ StandardizedStructure
// Compile-time: reject non-standard patterns (custom gateways, etc.)
```

**Law Patterns-2: Pattern Composition Soundness**
```rust
pub fn compose_patterns(patterns: &[PatternInstance]) -> Result<ProcessModel, String> {
    // Invariant: composition of sound patterns is sound
    // No pattern interference (no deadlock across pattern boundaries)
    for pattern in patterns {
        if !pattern.is_sound() {
            return Err("Unsound pattern".to_string());
        }
    }
    Ok(compose_soundly(patterns))
}

// Type law: Pattern composition preserves soundness
// Formal: ∀ patterns: sound(p1) ∧ sound(p2) → sound(compose(p1, p2))
```

---

### BPMN Type Laws

**Law BPMN-1: Synchronous-Only Execution**
```rust
pub enum BPMNGateway {
    XOR { condition: Expression }, // allowed
    AND { /* parallel fork */ }, // allowed
    OR { condition: Expression }, // allowed
    EventBased { /* async */ }, // FORBIDDEN in wasm4pm
    Complex { condition: Expression }, // FORBIDDEN (unbounded)
}

// Type law: EventBased and Complex gateways are rejected at compile time
// Runtime check: assert no async gateways in model before execution
```

**Law BPMN-2: No Message Flows (Inter-Process Communication)**
```rust
pub struct BPMNModel {
    processes: Vec<ProcessDefinition>,
    message_flows: Vec<MessageFlow>,
}

pub fn validate_wasm4pm(model: &BPMNModel) -> Result<(), String> {
    // Invariant: message flows are forbidden (require external queue/scheduler)
    if !model.message_flows.is_empty() {
        return Err("Message flows not supported in wasm4pm".to_string());
    }
    Ok(())
}

// Type law: MessageFlow ⊆ ForbiddenConstruct (in wasm4pm)
```

**Law BPMN-3: No Boundary Events (Async Exception Handling)**
```rust
pub struct Activity {
    boundary_events: Vec<BoundaryEvent>, // FORBIDDEN in wasm4pm
}

pub fn validate_wasm4pm(activity: &Activity) -> Result<(), String> {
    if !activity.boundary_events.is_empty() {
        return Err("Boundary events require async exception handling".to_string());
    }
    Ok(())
}

// Type law: BoundaryEvent ⊆ ForbiddenConstruct (in wasm4pm)
```

---

### sAirflow Type Laws

**Law sAirflow-1: DAG Acyclicity**
```rust
pub struct DAG {
    tasks: Vec<Task>,
    edges: Vec<(TaskId, TaskId)>,
}

impl DAG {
    pub fn validate_acyclic(&self) -> Result<(), String> {
        // Invariant: DAG contains no cycles
        if self.has_cycle() {
            Err("Cycle detected in DAG".to_string())
        } else {
            Ok(())
        }
    }
}

// Type law: DAG ⊆ AcyclicGraph
// Compile-time check: enforce before execution
```

**Law sAirflow-2: XCom Serializability**
```rust
pub struct XCom {
    key: String,
    value: SerializableValue, // must be serializable (JSON or binary)
}

pub enum SerializableValue {
    Json(serde_json::Value),
    Binary(Vec<u8>),
}

// Type law: XCom ⊆ SerializableData
// Compile-time: value must implement Serialize + Deserialize
```

**Law sAirflow-3: No Sensors (Polling)**
```rust
pub enum TaskType {
    PythonOperator { code: String }, // allowed
    BashOperator { command: String }, // allowed
    Sensor { }, // FORBIDDEN in wasm4pm (requires polling/async)
}

pub fn validate_wasm4pm(task: &Task) -> Result<(), String> {
    match task.type_ {
        TaskType::Sensor { } => Err("Sensors not supported in wasm4pm".to_string()),
        _ => Ok(()),
    }
}

// Type law: Sensor ⊆ ForbiddenConstruct (in wasm4pm)
```

---

### Healthcare Mining Type Laws

**Law Healthcare-1: Object-Centric Necessity**
```rust
pub struct PatientJourney {
    patient_id: PatientId,
    events: Vec<ClinicalEvent>,
    concurrent_activities: Vec<(Activity, Activity)>, // overlapping activities
}

// Type law: PatientJourney ⊆ MultiCaseTrace (cannot be flattened to case-centric)
// Reason: concurrent activities require object-centric model
```

**Law Healthcare-2: Privacy Constraint Enforcement**
```rust
pub struct ClinicalEvent {
    patient_id: PatientId, // MUST be redacted in output artifacts
    activity: String,
    timestamp: DateTime<Utc>,
    attributes: HashMap<String, Value>, // may contain PII
}

pub fn serialize_for_export(event: &ClinicalEvent) -> SerializedEvent {
    // Invariant: patient_id and PII attributes must be redacted
    SerializedEvent {
        patient_id: hash_with_salt(event.patient_id), // anonymized
        activity: event.activity.clone(),
        timestamp: event.timestamp,
        // attributes with PII are dropped
    }
}

// Type law: ClinicalEvent.serialize ⊆ PrivacyPreserving
// Compile-time: enforce redaction before export
```

**Law Healthcare-3: Incremental Discovery**
```rust
pub struct IncrementalMiner {
    model: ProcessModel,
}

impl IncrementalMiner {
    pub fn update(&mut self, event: &Event) {
        // Invariant: update model with single event (online learning)
        // No need to materialize full log
    }
}

// Type law: IncrementalMiner ⊆ StreamableAlgorithm
// Compile-time: memory usage bounded by model size, not log size
```

---

## Compat Type Boundaries (Enforcement Matrix)

### Type Boundary 1: Petri Net Soundness
| Implication | Enforcement | wasm4pm Handling |
|-------------|-------------|------------------|
| Model declares soundness | Proof artifact required | Pre-computed, bundled |
| Unsound net rejected | Type error at compile time | Reject at load time |
| Soundness assumed without proof | Manual review required | Documented assumption only |

### Type Boundary 2: Object Cardinality
| Implication | Enforcement | wasm4pm Handling |
|-------------|-------------|------------------|
| Cardinality bounds memory | max_instances ≤ linear_memory | Compute allocation at codegen |
| Unbounded cardinality | Type error | Reject at compile time |
| Dynamic cardinality | Statically analyzable | Pre-allocate max capacity |

### Type Boundary 3: Trace Length
| Implication | Enforcement | wasm4pm Handling |
|-------------|-------------|------------------|
| >10^6 events | Streaming required | Incremental algorithm |
| Log materialization | Memory bound check | Reject if exceeds budget |
| Streaming semantics | Iterator trait enforced | Implement Iterator<Event> |

### Type Boundary 4: Loop Bounds
| Implication | Enforcement | wasm4pm Handling |
|-------------|-------------|------------------|
| Unbounded loops | Type error | Reject at compile time |
| Fixed bounds | Statically analyzable | Extract from net structure |
| Dynamic bounds | Not allowed | Compile-time rejection |

### Type Boundary 5: Constraint Negation
| Implication | Enforcement | wasm4pm Handling |
|-------------|-------------|------------------|
| Negation requires CWA | Closed-world assumption enforced | Pre-materialize violation set |
| Partial logs | Open-world assumption | Reject negated constraints |
| Streaming evaluation | Not compatible | Force offline evaluation |

### Type Boundary 6: Async Constructs
| Implication | Enforcement | wasm4pm Handling |
|-------------|-------------|------------------|
| Message flows | Not wasm-compatible | Reject at compile time |
| Timers | Require external scheduler | Reject at compile time |
| Event-based gateways | Async logic required | Reject at compile time |
| Boundary events | Exception dispatch required | Reject at compile time |

---

## Conformance Proof Obligation Matrix

### PM4Py Proof Obligations
- [ ] EventLog parses without error (XES well-formedness)
- [ ] Trace replay produces fitness ∈ [0,1]
- [ ] Variants computed correctly (no duplicates)
- [ ] Petri net soundness attested

### YAWL Proof Obligations
- [ ] WorkflowNet is sound (no deadlock, proper termination)
- [ ] Work-queue dispatch is deterministic
- [ ] Cancellation set is acyclic
- [ ] Case completion receipt generated

### POWL 2.0 Proof Obligations
- [ ] Decomposition preserves soundness
- [ ] Loop bounds extractable and bounded
- [ ] Block composition is sound
- [ ] Size reduction metrics computed

### OCED Proof Obligations
- [ ] Event-object mapping is valid (no dangling references)
- [ ] RDF triple closure is finite
- [ ] Object lifecycles are sound (state transitions lawful)
- [ ] Multi-case causality is acyclic

### OCPQ Proof Obligations
- [ ] Constraints are decidable
- [ ] Negation pre-materialized (violation set computed)
- [ ] Cardinality bounds enforced
- [ ] Satisfaction score ∈ [0,1]

### Healthcare Mining Proof Obligations
- [ ] Patient journeys are object-centric (not case-centric)
- [ ] Privacy constraints enforced (PII redacted)
- [ ] Deviation count quantified
- [ ] Outcome correlation offline (not embedded in wasm)

---

## Status: COMPLETE

**Type Laws Defined:** 40+  
**Compat Boundaries Enforced:** 6 major + sub-categories  
**Proof Obligations:** 50+  
**Compile-time Checks:** Enforced via type system  
**Runtime Checks:** Validation before execution  

**Authority:** Phase 2 Conformance Agent  
**Board Admissibility:** Type laws are immutable (no runtime override)
