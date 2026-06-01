//! Conformance Authority module — synthesized by ggen manufacturing machinery
//!
//! **Module Spec:**
//! ```yaml
//! name: conformance
//! algorithms:
//!   - TokenReplay: { input: (ProcessModel, EventLog), output: ConformanceVerdicts }
//!   - Alignment: { input: (ProcessModel, EventLog), output: ConformanceVerdicts }
//! witness_markers: [TokenReplay, Alignment, RustLaw, BridgeRx]
//! graduate_boundary: false
//! ```
//!
//! **Authority References:**
//! - van der Aalst (1999): "Event log analysis using conformance checking"
//! - Adriansyah et al. (2011): "Conformance Checking using Alignments"
//! - Adriansyah (2014): "Alignment-Based Process Conformance Checking"
//!
//! **Witness Markers:** TokenReplay, Alignment, RustLaw, BridgeRx
//! **Generated from:** conformance module specification v30.1.2
//! **License:** Executable only under wasm4pm graduation bridge
//! **Graduation Status:** NOT GRADUATED — conformance_boundary=false

use crate::evidence::*;
use crate::petri::*;

// =========================================================================
// CONFORMANCE VERDICTS: Core Verdict Types
// =========================================================================

/// Conformance verdict for a single case/trace
#[derive(Debug, Clone, PartialEq)]
pub enum ConformanceVerdict {
    /// Case conforms with no deviations (perfect fitness)
    FullyConforming,
    /// Case conforms with bounded deviations (fitness in (0,1))
    PartiallyConforming {
        fitness: f64,
        deviations: usize,
    },
    /// Case does not conform (fitness = 0 or irreparable deviations)
    NonConforming {
        reason: ConformanceRefusal,
    },
}

impl ConformanceVerdict {
    /// Extract fitness metric as Between01
    pub fn fitness_score(&self) -> f64 {
        match self {
            Self::FullyConforming => 1.0,
            Self::PartiallyConforming { fitness, .. } => *fitness,
            Self::NonConforming { .. } => 0.0,
        }
    }

    /// Check if verdict indicates conformance admission
    pub fn admits(&self) -> bool {
        match self {
            Self::FullyConforming => true,
            Self::PartiallyConforming { fitness, .. } => *fitness >= 0.8, // Default threshold
            Self::NonConforming { .. } => false,
        }
    }
}

/// Aggregated conformance results across all cases
#[derive(Debug, Clone)]
pub struct ConformanceVerdicts {
    /// Case-by-case verdicts
    pub case_verdicts: Vec<(String, ConformanceVerdict)>,
    /// Overall fitness metric: average across all cases
    pub aggregate_fitness: f64,
    /// Overall precision metric: agreement rate between log and model
    pub aggregate_precision: f64,
    /// Number of cases admitted (fitness >= threshold)
    pub admitted_cases: usize,
    /// Total number of cases analyzed
    pub total_cases: usize,
}

impl Default for ConformanceVerdicts {
    fn default() -> Self {
        Self::new()
    }
}

impl ConformanceVerdicts {
    /// Create empty conformance verdicts
    pub fn new() -> Self {
        ConformanceVerdicts {
            case_verdicts: vec![],
            aggregate_fitness: 0.0,
            aggregate_precision: 0.0,
            admitted_cases: 0,
            total_cases: 0,
        }
    }

    /// Add a case verdict and update aggregates
    pub fn add_case(&mut self, case_id: String, verdict: ConformanceVerdict) {
        let fitness = verdict.fitness_score();
        let is_admitted = verdict.admits();

        self.case_verdicts.push((case_id, verdict));
        self.total_cases += 1;

        if is_admitted {
            self.admitted_cases += 1;
        }

        // Recompute aggregate fitness (incremental average)
        let prev_sum = self.aggregate_fitness * ((self.total_cases - 1) as f64);
        self.aggregate_fitness = (prev_sum + fitness) / (self.total_cases as f64);
    }

    /// Check if all cases are admitted
    pub fn all_admitted(&self) -> bool {
        self.admitted_cases == self.total_cases && self.total_cases > 0
    }

    /// Admission rate as fraction
    pub fn admission_rate(&self) -> f64 {
        if self.total_cases == 0 {
            0.0
        } else {
            (self.admitted_cases as f64) / (self.total_cases as f64)
        }
    }
}

impl SerializeBytes for ConformanceVerdicts {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        // Serialize fitness and precision as u64 percentages (0-1 scaled to 0-10000)
        let fitness_u64 = (self.aggregate_fitness * 10000.0) as u64;
        let precision_u64 = (self.aggregate_precision * 10000.0) as u64;

        fitness_u64.serialize_bytes(buf);
        precision_u64.serialize_bytes(buf);
        (self.admitted_cases as u64).serialize_bytes(buf);
        (self.total_cases as u64).serialize_bytes(buf);
    }
}

// =========================================================================
// CONFORMANCE REFUSAL REASONS
// =========================================================================

/// Reasons conformance checking may be refused
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConformanceRefusal {
    /// Event log is empty
    EmptyLog,
    /// Petri Net has no transitions
    EmptyModel,
    /// Net is not sound (no lawful firing sequences exist)
    UnsoundNet,
    /// Activity in case is not a valid transition
    UnknownActivity,
    /// Token replay exhausted after first few events
    EarlyTermination,
    /// State space explosion (alignment search space too large)
    StateSpaceExceeded,
    /// Case sequence is malformed (cycles, nulls, etc.)
    MalformedCase,
    /// Not implemented
    NotImplementedYet,
}

impl std::fmt::Display for ConformanceRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EmptyLog => write!(f, "EmptyLog"),
            Self::EmptyModel => write!(f, "EmptyModel"),
            Self::UnsoundNet => write!(f, "UnsoundNet"),
            Self::UnknownActivity => write!(f, "UnknownActivity"),
            Self::EarlyTermination => write!(f, "EarlyTermination"),
            Self::StateSpaceExceeded => write!(f, "StateSpaceExceeded"),
            Self::MalformedCase => write!(f, "MalformedCase"),
            Self::NotImplementedYet => write!(f, "NotImplementedYet"),
        }
    }
}

impl SerializeBytes for ConformanceRefusal {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        let code = match self {
            Self::EmptyLog => 1u32,
            Self::EmptyModel => 2u32,
            Self::UnsoundNet => 3u32,
            Self::UnknownActivity => 4u32,
            Self::EarlyTermination => 5u32,
            Self::StateSpaceExceeded => 6u32,
            Self::MalformedCase => 7u32,
            Self::NotImplementedYet => 255u32,
        };
        code.serialize_bytes(buf);
    }
}

// =========================================================================
// TOKEN REPLAY ENGINE
// =========================================================================

/// Token replay conformance result
#[derive(Debug, Clone)]
pub struct TokenReplayResult {
    /// Number of tokens produced (ideal = number of cases)
    pub tokens_produced: usize,
    /// Number of tokens missing during replay
    pub tokens_missing: usize,
    /// Number of tokens left over after replay
    pub tokens_remaining: usize,
    /// Fitness metric: 1.0 - (missing + remaining) / (2 * cases)
    pub fitness: f64,
}

impl SerializeBytes for TokenReplayResult {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        (self.tokens_produced as u64).serialize_bytes(buf);
        (self.tokens_missing as u64).serialize_bytes(buf);
        (self.tokens_remaining as u64).serialize_bytes(buf);
        ((self.fitness * 10000.0) as u64).serialize_bytes(buf);
    }
}

/// Token replay engine for a Petri Net
#[allow(dead_code)]
pub struct TokenReplayEngine {
    net: PetriNet,
}

impl TokenReplayEngine {
    /// Create a new token replay engine for a Petri Net
    pub fn new(net: PetriNet) -> Self {
        TokenReplayEngine { net }
    }

    /// Replay a case (sequence of activities) through the net
    /// Returns Evidence<TokenReplayResult, Admitted, TokenReplay>
    pub fn replay_case(
        &self,
        activities: &[String],
    ) -> Result<Evidence<TokenReplayResult, Admitted, TokenReplay>, ConformanceRefusal> {
        // Validation
        if activities.is_empty() {
            return Err(ConformanceRefusal::EmptyLog);
        }

        // Real token replay algorithm
        let source_place = self.net.places.iter().find(|p| p.to_lowercase() == "source" || p.to_lowercase() == "i")
            .ok_or(ConformanceRefusal::UnsoundNet)?.clone();

        let mut marking = Marking::initial(source_place);

        let mut produced = 1u32;
        let mut consumed = 0u32;
        let mut missing = 0u32;
        let mut remaining = 0u32;

        for act in activities {
            if !self.net.transitions.contains(act) {
                return Err(ConformanceRefusal::UnknownActivity);
            }

            // Check and handle enabled transitions
            if let Some(inputs) = self.net.pre.get(act) {
                for (place, &weight) in inputs {
                    let cur = marking.get_tokens(place);
                    if cur < weight {
                        let diff = weight - cur;
                        missing += diff;
                        produced += diff;
                        marking.tokens.insert(place.clone(), weight);
                    }
                }
            }

            // Consume inputs & Produce outputs
            marking = self.net.fire(act, &marking);

            if let Some(inputs) = self.net.pre.get(act) {
                for &weight in inputs.values() {
                    consumed += weight;
                }
            }
            if let Some(outputs) = self.net.post.get(act) {
                for &weight in outputs.values() {
                    produced += weight;
                }
            }
        }

        // Handle final marking and sink place
        if let Some(sink) = self.net.places.iter().find(|p| p.to_lowercase() == "sink" || p.to_lowercase() == "o") {
            let final_sink_tokens = marking.get_tokens(sink);
            if final_sink_tokens < 1 {
                let diff = 1 - final_sink_tokens;
                missing += diff;
                consumed += diff;
            } else {
                consumed += 1;
            }
            // All other remaining tokens
            for (place, &tokens) in &marking.tokens {
                if place != sink {
                    remaining += tokens;
                }
            }
        }

        let f_numerator = if consumed == 0 { 0.0 } else { missing as f64 / consumed as f64 };
        let f_denominator = if produced == 0 { 0.0 } else { remaining as f64 / produced as f64 };
        let fitness = 0.5 * (1.0 - f_numerator) + 0.5 * (1.0 - f_denominator);

        let result = TokenReplayResult {
            tokens_produced: produced as usize,
            tokens_missing: missing as usize,
            tokens_remaining: remaining as usize,
            fitness,
        };

        // Create Evidence with witness marker
        let evidence = Evidence {
            payload: result,
            state: Admitted::Yes,
            witness: TokenReplay,
            epoch: 0,
            signature: IdentitySignature {
                public_key: vec![],
                signature_bytes: vec![],
            },
            hash: Blake3Hash([0u8; 32]),
        };

        Ok(evidence)
    }

    /// Replay entire event log and compute aggregate fitness
    pub fn replay_log(
        &self,
        cases: &[(String, Vec<String>)],
    ) -> Result<Evidence<ConformanceVerdicts, Admitted, TokenReplay>, ConformanceRefusal> {
        if cases.is_empty() {
            return Err(ConformanceRefusal::EmptyLog);
        }

        let mut verdicts = ConformanceVerdicts::new();

        for (case_id, activities) in cases {
            let result = match self.replay_case(activities) {
                Ok(evidence) => evidence.payload,
                Err(e) => {
                    verdicts.add_case(
                        case_id.clone(),
                        ConformanceVerdict::NonConforming { reason: e },
                    );
                    continue;
                }
            };

            let verdict = if result.fitness >= 1.0 {
                ConformanceVerdict::FullyConforming
            } else if result.fitness > 0.0 {
                ConformanceVerdict::PartiallyConforming {
                    fitness: result.fitness,
                    deviations: result.tokens_missing + result.tokens_remaining,
                }
            } else {
                ConformanceVerdict::NonConforming {
                    reason: ConformanceRefusal::EarlyTermination,
                }
            };

            verdicts.add_case(case_id.clone(), verdict);
        }

        // Create Evidence with witness marker
        let evidence = Evidence {
            payload: verdicts,
            state: Admitted::Yes,
            witness: TokenReplay,
            epoch: 0,
            signature: IdentitySignature {
                public_key: vec![],
                signature_bytes: vec![],
            },
            hash: Blake3Hash([0u8; 32]),
        };

        Ok(evidence)
    }
}

/// Result of replaying a single case
#[derive(Debug, Clone)]
pub struct CaseReplay {
    pub case_id: String,
    pub missing: usize,
    pub remaining: usize,
}

// =========================================================================
// ALIGNMENT ENGINE
// =========================================================================

/// Alignment between a trace and a process model
#[derive(Debug, Clone)]
pub struct Alignment {
    /// Case identifier
    pub case_id: String,
    /// Sequence of (log_activity, model_activity) pairs
    /// None in log position = model-only move
    /// None in model position = log-only move (deviation)
    pub moves: Vec<(Option<String>, Option<String>)>,
    /// Alignment cost: number of deviating moves
    pub cost: usize,
}

impl Alignment {
    /// Compute fitness from alignment cost
    pub fn fitness(&self, trace_length: usize, model_distance: usize) -> f64 {
        let denominator = (trace_length + model_distance) as f64;
        if denominator == 0.0 {
            1.0
        } else {
            1.0 - (self.cost as f64) / denominator
        }
    }
}

impl SerializeBytes for Alignment {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        self.case_id.serialize_bytes(buf);
        (self.moves.len() as u64).serialize_bytes(buf);
        for (log_act, model_act) in &self.moves {
            match log_act {
                Some(a) => {
                    (1u8).serialize_bytes(buf);
                    a.serialize_bytes(buf);
                }
                None => (0u8).serialize_bytes(buf),
            }
            match model_act {
                Some(t) => {
                    (1u8).serialize_bytes(buf);
                    t.serialize_bytes(buf);
                }
                None => (0u8).serialize_bytes(buf),
            }
        }
        (self.cost as u64).serialize_bytes(buf);
    }
}

/// Alignment engine for conformance checking
#[allow(dead_code)]
pub struct AlignmentEngine {
    net: PetriNet,
}

impl AlignmentEngine {
    /// Create a new alignment engine
    pub fn new(net: PetriNet) -> Self {
        AlignmentEngine { net }
    }

    /// Compute the shortest path distance in the Petri Net from source to sink place.
    pub fn compute_model_distance(&self, source_place: &str, sink_place: &str) -> usize {
        let initial_marking = Marking::initial(source_place.to_string());
        let target_marking = Marking::initial(sink_place.to_string());

        let mut queue = std::collections::VecDeque::new();
        let mut visited = std::collections::HashSet::new();
        queue.push_back((initial_marking.clone(), 0));
        visited.insert(initial_marking);

        while let Some((marking, dist)) = queue.pop_front() {
            if marking == target_marking {
                return dist;
            }

            for t in &self.net.transitions {
                if self.net.is_enabled(t, &marking) {
                    let next_marking = self.net.fire(t, &marking);
                    if !visited.contains(&next_marking) {
                        visited.insert(next_marking.clone());
                        queue.push_back((next_marking, dist + 1));
                    }
                }
            }
        }

        0
    }

    /// Compute optimal alignment between a trace and the net
    /// Returns Evidence<Alignment, Admitted, AlignmentWitness>
    pub fn align_trace(
        &self,
        case_id: &str,
        trace: &[String],
    ) -> Result<Evidence<Alignment, Admitted, AlignmentWitness>, ConformanceRefusal> {
        if trace.is_empty() {
            return Err(ConformanceRefusal::EmptyLog);
        }

        let source_place = self.net.places.iter().find(|p| p.to_lowercase() == "source" || p.to_lowercase() == "i")
            .ok_or(ConformanceRefusal::UnsoundNet)?.clone();
        
        let sink_place = self.net.places.iter().find(|p| p.to_lowercase() == "sink" || p.to_lowercase() == "o")
            .ok_or(ConformanceRefusal::UnsoundNet)?.clone();

        // A* search for lowest-cost alignment
        #[derive(Clone, Eq, PartialEq)]
        struct AStarState {
            cost: usize,
            heuristic: usize,
            trace_index: usize,
            marking: Marking,
            moves: Vec<(Option<String>, Option<String>)>,
        }

        impl Ord for AStarState {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                let self_f = self.cost + self.heuristic;
                let other_f = other.cost + other.heuristic;
                match other_f.cmp(&self_f) {
                    std::cmp::Ordering::Equal => self.trace_index.cmp(&other.trace_index),
                    ord => ord,
                }
            }
        }

        impl PartialOrd for AStarState {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        let initial_marking = Marking::initial(source_place);
        let mut heap = std::collections::BinaryHeap::new();
        let initial_h = self.calculate_heuristic(&initial_marking, 0, trace.len(), &sink_place);
        
        heap.push(AStarState {
            cost: 0,
            heuristic: initial_h,
            trace_index: 0,
            marking: initial_marking,
            moves: Vec::new(),
        });

        let mut visited = std::collections::HashSet::new();
        let mut best_alignment = None;
        let mut iterations = 0;

        while let Some(state) = heap.pop() {
            iterations += 1;
            if iterations > 5000 {
                return Err(ConformanceRefusal::StateSpaceExceeded);
            }

            let state_key = (state.marking.clone(), state.trace_index);
            if visited.contains(&state_key) {
                continue;
            }
            visited.insert(state_key);

            // Goal test: all trace events replayed and only token is in sink place
            if state.trace_index == trace.len() 
                && state.marking.tokens.len() == 1 
                && state.marking.get_tokens(&sink_place) == 1 
            {
                best_alignment = Some(Alignment {
                    case_id: case_id.to_string(),
                    moves: state.moves,
                    cost: state.cost,
                });
                break;
            }

            // Generative transition rules:
            // 1. Model-only moves (for any transition enabled in net)
            for t in &self.net.transitions {
                if self.net.is_enabled(t, &state.marking) {
                    let next_marking = self.net.fire(t, &state.marking);
                    let mut next_moves = state.moves.clone();
                    next_moves.push((None, Some(t.clone())));
                    let h = self.calculate_heuristic(&next_marking, state.trace_index, trace.len(), &sink_place);
                    heap.push(AStarState {
                        cost: state.cost + 1,
                        heuristic: h,
                        trace_index: state.trace_index,
                        marking: next_marking,
                        moves: next_moves,
                    });
                }
            }

            // 2. Synchronous moves (next trace event matches enabled transition)
            if state.trace_index < trace.len() {
                let next_event = &trace[state.trace_index];
                if self.net.transitions.contains(next_event) && self.net.is_enabled(next_event, &state.marking) {
                    let next_marking = self.net.fire(next_event, &state.marking);
                    let mut next_moves = state.moves.clone();
                    next_moves.push((Some(next_event.clone()), Some(next_event.clone())));
                    let h = self.calculate_heuristic(&next_marking, state.trace_index + 1, trace.len(), &sink_place);
                    heap.push(AStarState {
                        cost: state.cost,
                        heuristic: h,
                        trace_index: state.trace_index + 1,
                        marking: next_marking,
                        moves: next_moves,
                    });
                }
            }

            // 3. Log-only moves (deviation: skip next trace event)
            if state.trace_index < trace.len() {
                let next_event = &trace[state.trace_index];
                let mut next_moves = state.moves.clone();
                next_moves.push((Some(next_event.clone()), None));
                let h = self.calculate_heuristic(&state.marking, state.trace_index + 1, trace.len(), &sink_place);
                heap.push(AStarState {
                    cost: state.cost + 1,
                    heuristic: h,
                    trace_index: state.trace_index + 1,
                    marking: state.marking.clone(),
                    moves: next_moves,
                });
            }
        }

        let alignment = best_alignment.ok_or(ConformanceRefusal::EarlyTermination)?;

        // Create Evidence with witness marker
        let evidence = Evidence {
            payload: alignment,
            state: Admitted::Yes,
            witness: AlignmentWitness,
            epoch: 0,
            signature: IdentitySignature {
                public_key: vec![],
                signature_bytes: vec![],
            },
            hash: Blake3Hash([0u8; 32]),
        };

        Ok(evidence)
    }

    /// Finds the shortest transition distance from a place to the sink place.
    pub fn get_place_distance(&self, place: &str, sink_place: &str) -> usize {
        if place == sink_place {
            return 0;
        }

        let mut queue = std::collections::VecDeque::new();
        let mut visited = std::collections::HashSet::new();
        queue.push_back((place.to_string(), 0));
        visited.insert(place.to_string());

        while let Some((curr_place, dist)) = queue.pop_front() {
            if curr_place == sink_place {
                return dist;
            }

            for t in &self.net.transitions {
                if let Some(inputs) = self.net.pre.get(t) {
                    if inputs.contains_key(&curr_place) {
                        if let Some(outputs) = self.net.post.get(t) {
                            for out in outputs.keys() {
                                if !visited.contains(out) {
                                    visited.insert(out.clone());
                                    queue.push_back((out.clone(), dist + 1));
                                }
                            }
                        }
                    }
                }
            }
        }

        10 // Default penalty if sink is unreachable
    }

    fn calculate_heuristic(&self, marking: &Marking, trace_index: usize, trace_len: usize, sink_place: &str) -> usize {
        let log_remaining = trace_len - trace_index;
        let mut model_remaining = 0;
        for (place, &tokens) in &marking.tokens {
            if tokens > 0 {
                let dist = self.get_place_distance(place, sink_place);
                if dist > model_remaining {
                    model_remaining = dist;
                }
            }
        }
        log_remaining.abs_diff(model_remaining)
    }

    /// Align entire event log and compute aggregate conformance verdicts
    pub fn align_log(
        &self,
        cases: &[(String, Vec<String>)],
    ) -> Result<Evidence<ConformanceVerdicts, Admitted, AlignmentWitness>, ConformanceRefusal> {
        if cases.is_empty() {
            return Err(ConformanceRefusal::EmptyLog);
        }

        let mut verdicts = ConformanceVerdicts::new();

        let source_place = self.net.places.iter().find(|p| p.to_lowercase() == "source" || p.to_lowercase() == "i")
            .ok_or(ConformanceRefusal::UnsoundNet)?.clone();
        
        let sink_place = self.net.places.iter().find(|p| p.to_lowercase() == "sink" || p.to_lowercase() == "o")
            .ok_or(ConformanceRefusal::UnsoundNet)?.clone();

        let model_distance = self.compute_model_distance(&source_place, &sink_place);

        for (case_id, trace) in cases {
            let alignment = match self.align_trace(case_id, trace) {
                Ok(evidence) => evidence.payload,
                Err(e) => {
                    verdicts.add_case(
                        case_id.clone(),
                        ConformanceVerdict::NonConforming { reason: e },
                    );
                    continue;
                }
            };

            let fitness = alignment.fitness(trace.len(), model_distance);
            let verdict = if fitness >= 1.0 {
                ConformanceVerdict::FullyConforming
            } else if fitness > 0.0 {
                ConformanceVerdict::PartiallyConforming {
                    fitness,
                    deviations: alignment.cost,
                }
            } else {
                ConformanceVerdict::NonConforming {
                    reason: ConformanceRefusal::EarlyTermination,
                }
            };

            verdicts.add_case(case_id.clone(), verdict);
        }

        // Create Evidence with witness marker
        let evidence = Evidence {
            payload: verdicts,
            state: Admitted::Yes,
            witness: AlignmentWitness,
            epoch: 0,
            signature: IdentitySignature {
                public_key: vec![],
                signature_bytes: vec![],
            },
            hash: Blake3Hash([0u8; 32]),
        };

        Ok(evidence)
    }
}

// =========================================================================
// WITNESS MARKERS
// =========================================================================

/// Witness marker: Token Replay conformance method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenReplay;

impl SerializeBytes for TokenReplay {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        1u32.serialize_bytes(buf);
    }
}

impl Lattice for TokenReplay {
    fn bottom() -> Self {
        TokenReplay
    }
    fn top() -> Self {
        TokenReplay
    }
    fn is_top(&self) -> bool {
        false
    }
    fn is_bottom(&self) -> bool {
        true
    }
    fn join(&self, _other: &Self) -> Self {
        TokenReplay
    }
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}

/// Witness marker: Alignment conformance method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlignmentWitness;

impl SerializeBytes for AlignmentWitness {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        2u32.serialize_bytes(buf);
    }
}

impl Lattice for AlignmentWitness {
    fn bottom() -> Self {
        AlignmentWitness
    }
    fn top() -> Self {
        AlignmentWitness
    }
    fn is_top(&self) -> bool {
        false
    }
    fn is_bottom(&self) -> bool {
        true
    }
    fn join(&self, _other: &Self) -> Self {
        AlignmentWitness
    }
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}

// =========================================================================
// RATIONAL METRICS & COMPILE-TIME/RUNTIME VALIDATION
// =========================================================================

/// Quality metric kind values
pub const METRIC_KIND_FITNESS: u8 = 0;
pub const METRIC_KIND_PRECISION: u8 = 1;
pub const METRIC_KIND_F1: u8 = 2;
pub const METRIC_KIND_GENERALIZATION: u8 = 3;
pub const METRIC_KIND_SIMPLICITY: u8 = 4;

/// A rational metric provably in [0,1] at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Between01<const NUM: u64, const DEN: u64>;

impl<const NUM: u64, const DEN: u64> Default for Between01<NUM, DEN> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const NUM: u64, const DEN: u64> Between01<NUM, DEN> {
    pub const VALID: () = {
        assert!(DEN > 0, "Denominator must be greater than 0");
        assert!(NUM <= DEN, "Between01 metric must be in [0, 1] (NUM <= DEN)");
    };

    /// Create a new compile-time validated Between01 metric.
    /// This will trigger a compile-time check when monomorphized.
    #[allow(path_statements)]
    pub fn new() -> Self {
        Self::VALID;
        Between01
    }

    /// Convert to f64
    pub fn value(&self) -> f64 {
        NUM as f64 / DEN as f64
    }
}

/// Generic Metric type parameterised by MetricKind and rational bounds
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Metric<const KIND: u8, const NUM: u64, const DEN: u64>;

impl<const KIND: u8, const NUM: u64, const DEN: u64> Default for Metric<KIND, NUM, DEN> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const KIND: u8, const NUM: u64, const DEN: u64> Metric<KIND, NUM, DEN> {
    pub const VALID: () = {
        assert!(DEN > 0, "Denominator must be greater than 0");
        assert!(NUM <= DEN, "Metric must be in [0, 1] (NUM <= DEN)");
    };

    /// Create a new compile-time validated Metric.
    /// This will trigger a compile-time check when monomorphized.
    #[allow(path_statements)]
    pub fn new() -> Self {
        Self::VALID;
        Metric
    }

    pub fn value(&self) -> f64 {
        NUM as f64 / DEN as f64
    }
}

pub type FitnessConst<const NUM: u64, const DEN: u64> = Metric<METRIC_KIND_FITNESS, NUM, DEN>;
pub type PrecisionConst<const NUM: u64, const DEN: u64> = Metric<METRIC_KIND_PRECISION, NUM, DEN>;
pub type F1Const<const NUM: u64, const DEN: u64> = Metric<METRIC_KIND_F1, NUM, DEN>;

// =========================================================================
// RUNTIME MATHEMATICAL VALIDATION
// =========================================================================

/// Errors returned during fractional metric validation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricError {
    NaNValue,
    InfiniteValue,
    OutOfBounds,
    DivisionByZero,
}

impl std::fmt::Display for MetricError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NaNValue => write!(f, "Metric value cannot be NaN"),
            Self::InfiniteValue => write!(f, "Metric value cannot be Infinite"),
            Self::OutOfBounds => write!(f, "Metric value must be in [0.0, 1.0]"),
            Self::DivisionByZero => write!(f, "Denominator cannot be zero"),
        }
    }
}

/// Runtime-validated fractional metric bounded in [0.0, 1.0]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RuntimeBetween01 {
    value: f64,
}

impl RuntimeBetween01 {
    /// Create a new RuntimeBetween01 with mathematical validation
    pub fn new(value: f64) -> Result<Self, MetricError> {
        if value.is_nan() {
            return Err(MetricError::NaNValue);
        }
        if value.is_infinite() {
            return Err(MetricError::InfiniteValue);
        }
        if !(0.0..=1.0).contains(&value) {
            return Err(MetricError::OutOfBounds);
        }
        Ok(RuntimeBetween01 { value })
    }

    /// Create from a ratio (numerator / denominator) with mathematical validation
    pub fn from_ratio(num: usize, den: usize) -> Result<Self, MetricError> {
        if den == 0 {
            return Err(MetricError::DivisionByZero);
        }
        let val = num as f64 / den as f64;
        Self::new(val)
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

/// Runtime-validated typed fitness metric
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FitnessMetric(pub RuntimeBetween01);

impl FitnessMetric {
    pub fn new(value: f64) -> Result<Self, MetricError> {
        Ok(FitnessMetric(RuntimeBetween01::new(value)?))
    }

    pub fn from_ratio(num: usize, den: usize) -> Result<Self, MetricError> {
        Ok(FitnessMetric(RuntimeBetween01::from_ratio(num, den)?))
    }

    pub fn value(&self) -> f64 {
        self.0.value()
    }
}

/// Runtime-validated typed precision metric
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PrecisionMetric(pub RuntimeBetween01);

impl PrecisionMetric {
    pub fn new(value: f64) -> Result<Self, MetricError> {
        Ok(PrecisionMetric(RuntimeBetween01::new(value)?))
    }

    pub fn from_ratio(num: usize, den: usize) -> Result<Self, MetricError> {
        Ok(PrecisionMetric(RuntimeBetween01::from_ratio(num, den)?))
    }

    pub fn value(&self) -> f64 {
        self.0.value()
    }
}

// =========================================================================
// ADMISSION STATE
// =========================================================================

/// Admission state: Yes or No (conformance verdict admitted or rejected)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Admitted {
    Yes,
    No,
}

impl SerializeBytes for Admitted {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        match self {
            Self::Yes => 1u8.serialize_bytes(buf),
            Self::No => 0u8.serialize_bytes(buf),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conformance_verdict_fitness() {
        let v1 = ConformanceVerdict::FullyConforming;
        assert_eq!(v1.fitness_score(), 1.0);
        assert!(v1.admits());

        let v2 = ConformanceVerdict::PartiallyConforming {
            fitness: 0.9,
            deviations: 1,
        };
        assert_eq!(v2.fitness_score(), 0.9);
        assert!(v2.admits());

        let v3 = ConformanceVerdict::PartiallyConforming {
            fitness: 0.7,
            deviations: 3,
        };
        assert_eq!(v3.fitness_score(), 0.7);
        assert!(!v3.admits());

        let v4 = ConformanceVerdict::NonConforming {
            reason: ConformanceRefusal::UnknownActivity,
        };
        assert_eq!(v4.fitness_score(), 0.0);
        assert!(!v4.admits());
    }

    #[test]
    fn test_conformance_verdicts_aggregation() {
        let mut verdicts = ConformanceVerdicts::new();

        verdicts.add_case(
            "case1".to_string(),
            ConformanceVerdict::FullyConforming,
        );
        verdicts.add_case(
            "case2".to_string(),
            ConformanceVerdict::PartiallyConforming {
                fitness: 0.8,
                deviations: 1,
            },
        );

        assert_eq!(verdicts.total_cases, 2);
        assert_eq!(verdicts.admitted_cases, 2);
        assert_eq!(verdicts.admission_rate(), 1.0);
        assert!(verdicts.all_admitted());
    }

    #[test]
    fn test_alignment_fitness() {
        let alignment = Alignment {
            case_id: "test".to_string(),
            moves: vec![],
            cost: 1,
        };
        let fitness = alignment.fitness(10, 5);
        assert!(fitness > 0.0 && fitness < 1.0);
    }

    #[test]
    fn test_astar_alignment_correctness_and_performance() {
        let mut places = std::collections::BTreeSet::new();
        for p in &["source", "p1", "p2", "sink"] {
            places.insert(p.to_string());
        }

        let mut transitions = std::collections::BTreeSet::new();
        for t in &["Register", "Approve", "Ship"] {
            transitions.insert(t.to_string());
        }

        let mut pre = std::collections::BTreeMap::new();
        let mut post = std::collections::BTreeMap::new();

        // Register
        let mut t_reg_pre = std::collections::BTreeMap::new();
        t_reg_pre.insert("source".to_string(), 1);
        pre.insert("Register".to_string(), t_reg_pre);

        let mut t_reg_post = std::collections::BTreeMap::new();
        t_reg_post.insert("p1".to_string(), 1);
        post.insert("Register".to_string(), t_reg_post);

        // Approve
        let mut t_app_pre = std::collections::BTreeMap::new();
        t_app_pre.insert("p1".to_string(), 1);
        pre.insert("Approve".to_string(), t_app_pre);

        let mut t_app_post = std::collections::BTreeMap::new();
        t_app_post.insert("p2".to_string(), 1);
        post.insert("Approve".to_string(), t_app_post);

        // Ship
        let mut t_shp_pre = std::collections::BTreeMap::new();
        t_shp_pre.insert("p2".to_string(), 1);
        pre.insert("Ship".to_string(), t_shp_pre);

        let mut t_shp_post = std::collections::BTreeMap::new();
        t_shp_post.insert("sink".to_string(), 1);
        post.insert("Ship".to_string(), t_shp_post);

        let net = PetriNet::new(places, transitions, pre, post);
        let engine = AlignmentEngine::new(net);

        // Check model distance
        let dist = engine.compute_model_distance("source", "sink");
        assert_eq!(dist, 3);

        // 1. Fully conforming trace: ["Register", "Approve", "Ship"]
        let trace1 = vec!["Register".to_string(), "Approve".to_string(), "Ship".to_string()];
        let start = std::time::Instant::now();
        let alignment1 = engine.align_trace("case1", &trace1).unwrap().payload;
        let duration1 = start.elapsed();
        println!("Conforming alignment duration: {:?}", duration1);
        assert!(duration1.as_millis() <= 10);
        assert_eq!(alignment1.cost, 0);
        assert_eq!(alignment1.moves.len(), 3);
        for m in &alignment1.moves {
            assert!(m.0.is_some());
            assert!(m.1.is_some());
            assert_eq!(m.0, m.1);
        }

        // 2. Partially conforming trace (Move on Model): ["Approve", "Ship"] (missing "Register")
        let trace2 = vec!["Approve".to_string(), "Ship".to_string()];
        let start = std::time::Instant::now();
        let alignment2 = engine.align_trace("case2", &trace2).unwrap().payload;
        let duration2 = start.elapsed();
        println!("Missing Register alignment duration: {:?}", duration2);
        assert!(duration2.as_millis() <= 10);
        assert_eq!(alignment2.cost, 1);
        assert_eq!(alignment2.moves.len(), 3);
        assert_eq!(alignment2.moves[0], (None, Some("Register".to_string())));
        assert_eq!(alignment2.moves[1], (Some("Approve".to_string()), Some("Approve".to_string())));
        assert_eq!(alignment2.moves[2], (Some("Ship".to_string()), Some("Ship".to_string())));

        // 3. Partially conforming trace (Move on Log): ["Register", "Audit", "Approve", "Ship"] (extra "Audit")
        let trace3 = vec!["Register".to_string(), "Audit".to_string(), "Approve".to_string(), "Ship".to_string()];
        let start = std::time::Instant::now();
        let alignment3 = engine.align_trace("case3", &trace3).unwrap().payload;
        let duration3 = start.elapsed();
        println!("Extra Audit alignment duration: {:?}", duration3);
        assert!(duration3.as_millis() <= 10);
        assert_eq!(alignment3.cost, 1);
        assert_eq!(alignment3.moves.len(), 4);
        assert_eq!(alignment3.moves[0], (Some("Register".to_string()), Some("Register".to_string())));
        assert_eq!(alignment3.moves[1], (Some("Audit".to_string()), None));
        assert_eq!(alignment3.moves[2], (Some("Approve".to_string()), Some("Approve".to_string())));
        assert_eq!(alignment3.moves[3], (Some("Ship".to_string()), Some("Ship".to_string())));
    }

    #[test]
    fn test_compile_time_metrics() {
        // Compile-time checks: these must compile and execute successfully
        let m1 = Between01::<3, 4>::new();
        assert_eq!(m1.value(), 0.75);

        let m2 = Between01::<1, 1>::new();
        assert_eq!(m2.value(), 1.0);

        let m3 = Between01::<0, 5>::new();
        assert_eq!(m3.value(), 0.0);

        let f = FitnessConst::<8, 10>::new();
        assert_eq!(f.value(), 0.8);

        let p = PrecisionConst::<9, 10>::new();
        assert_eq!(p.value(), 0.9);

        let f1 = F1Const::<5, 10>::new();
        assert_eq!(f1.value(), 0.5);
    }

    #[test]
    fn test_runtime_metric_validation() {
        // Valid values
        assert!(RuntimeBetween01::new(0.0).is_ok());
        assert!(RuntimeBetween01::new(0.5).is_ok());
        assert!(RuntimeBetween01::new(1.0).is_ok());
        assert_eq!(RuntimeBetween01::new(0.75).unwrap().value(), 0.75);

        // Out of bounds
        assert_eq!(RuntimeBetween01::new(-0.1).unwrap_err(), MetricError::OutOfBounds);
        assert_eq!(RuntimeBetween01::new(1.01).unwrap_err(), MetricError::OutOfBounds);

        // NaN & Infinite
        assert_eq!(RuntimeBetween01::new(f64::NAN).unwrap_err(), MetricError::NaNValue);
        assert_eq!(RuntimeBetween01::new(f64::INFINITY).unwrap_err(), MetricError::InfiniteValue);
        assert_eq!(RuntimeBetween01::new(f64::NEG_INFINITY).unwrap_err(), MetricError::InfiniteValue);

        // Ratio creation
        assert!(RuntimeBetween01::from_ratio(3, 4).is_ok());
        assert_eq!(RuntimeBetween01::from_ratio(5, 4).unwrap_err(), MetricError::OutOfBounds);
        assert_eq!(RuntimeBetween01::from_ratio(3, 0).unwrap_err(), MetricError::DivisionByZero);

        // Typed wrappers
        assert!(FitnessMetric::new(0.85).is_ok());
        assert_eq!(FitnessMetric::new(1.5).unwrap_err(), MetricError::OutOfBounds);
        assert_eq!(FitnessMetric::from_ratio(4, 5).unwrap().value(), 0.8);

        assert!(PrecisionMetric::new(0.95).is_ok());
        assert_eq!(PrecisionMetric::new(-0.5).unwrap_err(), MetricError::OutOfBounds);
        assert_eq!(PrecisionMetric::from_ratio(9, 10).unwrap().value(), 0.9);
    }
}
