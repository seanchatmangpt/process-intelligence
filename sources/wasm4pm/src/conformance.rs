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
                other_f.cmp(&self_f) // Min-heap behavior
            }
        }

        impl PartialOrd for AStarState {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        let initial_marking = Marking::initial(source_place);
        let mut heap = std::collections::BinaryHeap::new();
        
        heap.push(AStarState {
            cost: 0,
            heuristic: trace.len(),
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
                    heap.push(AStarState {
                        cost: state.cost + 1,
                        heuristic: trace.len() - state.trace_index,
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
                    heap.push(AStarState {
                        cost: state.cost,
                        heuristic: trace.len() - (state.trace_index + 1),
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
                heap.push(AStarState {
                    cost: state.cost + 1,
                    heuristic: trace.len() - (state.trace_index + 1),
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

    /// Align entire event log and compute aggregate conformance verdicts
    pub fn align_log(
        &self,
        cases: &[(String, Vec<String>)],
    ) -> Result<Evidence<ConformanceVerdicts, Admitted, AlignmentWitness>, ConformanceRefusal> {
        if cases.is_empty() {
            return Err(ConformanceRefusal::EmptyLog);
        }

        let mut verdicts = ConformanceVerdicts::new();

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

            let fitness = alignment.fitness(trace.len(), 0); // model_distance to be computed
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
}
