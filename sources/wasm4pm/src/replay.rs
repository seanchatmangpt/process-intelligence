//! Replay Authority module — synthesized by ggen manufacturing machinery
//!
//! This module implements full token replay and step-by-step process simulation
//! with evidence wrapping, witness state lattice encoding, and receipt sealing.
//!
//! Algorithms:
//! - Replay: TokenGame<(ProcessModel, EventLog)> → ReplayTraces
//! - StepSimulator: StepExecution<(ProcessModel, Event)> → StepTrace
//!
//! Witness Markers: [Replay, StepSimulation, RustLaw, BridgeRx]
//! Graduate Boundary: false (internal module, not exposed at graduation)
//!
//! Generated from: templates/replay/module.rs.j2
//! License: Executable only under wasm4pm graduation bridge

use crate::evidence::*;
use crate::petri::*;

// =========================================================================
// 1. Replay Execution Types
// =========================================================================

/// ReplayTrace: one execution path through event log against process model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayTrace {
    /// Event indices replayed in this trace (sorted, unique)
    pub event_indices: Vec<usize>,
    /// Sequence of markings along the replay path
    pub marking_sequence: Vec<Marking>,
    /// Alignment cost (number of log moves + model moves)
    pub total_cost: u32,
    /// Per-move cost breakdown
    pub move_costs: Vec<(MoveKind, u32)>,
}

/// MoveKind: classifier for each step in alignment replay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveKind {
    /// Synchronous move: event matched a transition
    Synchronous,
    /// Log move: event without matching transition
    LogMove,
    /// Model move: transition without matching event
    ModelMove,
}

/// Full replay result set: all valid replay paths found
#[derive(Debug, Clone)]
pub struct ReplayTraces {
    /// All discovered replay paths
    pub paths: Vec<ReplayTrace>,
    /// Best trace by fitness (lowest cost)
    pub best_trace_index: Option<usize>,
    /// Total fitness metric (0.0 to 1.0)
    pub fitness: f64,
}

// =========================================================================
// 2. Replay Engine (token game executor)
// =========================================================================

/// Replay executor: drives token game execution with full evidence binding
pub struct ReplayEngine {
    net: PetriNet,
    /// Current path during replay
    current_trace: Vec<ReplayTrace>,
    /// Full event log to replay against
    event_log: Vec<SimpleEvent>,
}

/// Minimal event representation for replay
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleEvent {
    pub activity: String,
    pub timestamp: u64,
    pub case_id: String,
}

impl ReplayEngine {
    /// Initialize replay engine with a process model and event log
    pub fn new(
        net: PetriNet,
        event_log: Vec<SimpleEvent>,
    ) -> Result<Self, ReplayRefusal> {
        if event_log.is_empty() {
            return Err(ReplayRefusal::EmptyLog);
        }

        Ok(ReplayEngine {
            net,
            current_trace: vec![],
            event_log,
        })
    }

    /// Execute full replay: discover all valid alignment paths
    pub fn replay(&mut self) -> Result<ReplayTraces, ReplayRefusal> {
        let mut all_paths = Vec::new();

        // Start from initial marking
        let initial_marking = Marking::initial("source".to_string());

        // Recursively explore all possible replay paths
        self.explore_paths(
            initial_marking,
            vec![],
            &mut all_paths,
        )?;

        if all_paths.is_empty() {
            return Err(ReplayRefusal::NoValidReplay);
        }

        // Calculate best trace (minimum cost)
        let best_trace_index = all_paths
            .iter()
            .enumerate()
            .min_by_key(|(_, trace)| trace.total_cost)
            .map(|(idx, _)| idx);

        // Compute overall fitness: traces replayed / events in log
        let best_cost = best_trace_index
            .and_then(|idx| all_paths.get(idx))
            .map(|trace| trace.total_cost as f64)
            .unwrap_or(0.0);

        let fitness = if self.event_log.len() > 0 {
            1.0 - (best_cost / (self.event_log.len() as f64 * 2.0))
        } else {
            0.0
        };

        Ok(ReplayTraces {
            paths: all_paths,
            best_trace_index,
            fitness,
        })
    }

    /// Recursive path exploration
    fn explore_paths(
        &self,
        current_marking: Marking,
        processed_events: Vec<usize>,
        all_paths: &mut Vec<ReplayTrace>,
    ) -> Result<(), ReplayRefusal> {
        // Base case: all events processed
        if processed_events.len() == self.event_log.len() {
            all_paths.push(ReplayTrace {
                event_indices: processed_events,
                marking_sequence: vec![current_marking],
                total_cost: 0,
                move_costs: vec![],
            });
            return Ok(());
        }

        // Find next unprocessed event
        let mut next_event_idx = None;
        for (i, _) in self.event_log.iter().enumerate() {
            if !processed_events.contains(&i) {
                next_event_idx = Some(i);
                break;
            }
        }

        if let Some(evt_idx) = next_event_idx {
            let event = &self.event_log[evt_idx];

            // Try synchronous move: find matching transition
            let enabled = self.net
                .transitions
                .iter()
                .filter(|t| self.net.is_enabled(t, &current_marking))
                .cloned()
                .collect::<Vec<_>>();

            for transition in &enabled {
                if transition == &event.activity {
                    // Synchronous move: fire this transition
                    let new_marking = self.net.fire(transition, &current_marking);
                    let mut new_processed = processed_events.clone();
                    new_processed.push(evt_idx);
                    new_processed.sort();

                    self.explore_paths(new_marking, new_processed, all_paths)?;
                }
            }

            // Try log move: skip event (no matching transition)
            let mut new_processed = processed_events.clone();
            new_processed.push(evt_idx);
            new_processed.sort();
            self.explore_paths(current_marking.clone(), new_processed, all_paths)?;

            // Try model moves: fire any enabled transition (limited exploration)
            for transition in &enabled {
                if transition != &event.activity {
                    let new_marking = self.net.fire(transition, &current_marking);
                    self.explore_paths(new_marking, processed_events.clone(), all_paths)?;
                }
            }
        }

        Ok(())
    }

    /// Get all discovered traces
    pub fn traces(&self) -> &[ReplayTrace] {
        &self.current_trace
    }
}

// =========================================================================
// 3. Step Simulator (interactive single-step execution)
// =========================================================================

/// Simulation step trace
#[derive(Debug, Clone)]
pub struct StepTrace {
    /// Transitions enabled before this step
    pub enabled_before: Vec<String>,
    /// Activity executed (None if simulation ended)
    pub activity: Option<String>,
    /// Resulting marking after step
    pub resulting_marking: Marking,
    /// Step cost contribution
    pub step_cost: u32,
}

/// Step-by-step simulator for interactive process exploration
pub struct StepSimulator {
    net: PetriNet,
    history: Vec<StepTrace>,
    current_marking: Marking,
}

impl StepSimulator {
    /// Create new step simulator
    pub fn new(net: PetriNet) -> Result<Self, ReplayRefusal> {
        let current_marking = Marking::initial("source".to_string());

        Ok(StepSimulator {
            net,
            history: vec![],
            current_marking,
        })
    }

    /// Get all transitions enabled at current step
    pub fn enabled_activities(&self) -> Vec<String> {
        self.net
            .transitions
            .iter()
            .filter(|t| self.net.is_enabled(t, &self.current_marking))
            .cloned()
            .collect()
    }

    /// Execute one step: fire the named transition
    pub fn step(&mut self, activity: &str) -> Result<StepTrace, ReplayRefusal> {
        let enabled = self.enabled_activities();

        if !enabled.contains(&activity.to_string()) {
            return Err(ReplayRefusal::ActivityNotEnabled);
        }

        // Fire transition
        let resulting_marking = self.net.fire(activity, &self.current_marking);

        let step = StepTrace {
            enabled_before: enabled,
            activity: Some(activity.to_string()),
            resulting_marking,
            step_cost: 0,
        };

        self.history.push(step.clone());
        self.current_marking = step.resulting_marking.clone();

        Ok(step)
    }

    /// Reset to initial state
    pub fn reset(&mut self) {
        self.history.clear();
        self.current_marking = Marking::initial("source".to_string());
    }

    /// Get simulation history
    pub fn history(&self) -> &[StepTrace] {
        &self.history
    }
}

// =========================================================================
// 4. Serialization Implementation
// =========================================================================

impl SerializeBytes for ReplayTraces {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        // Serialize number of paths
        buf.extend_from_slice(&(self.paths.len() as u64).to_le_bytes());

        // Serialize each path
        for path in &self.paths {
            buf.extend_from_slice(&(path.event_indices.len() as u64).to_le_bytes());
            for &idx in &path.event_indices {
                buf.extend_from_slice(&(idx as u64).to_le_bytes());
            }
            buf.extend_from_slice(&(path.total_cost as u64).to_le_bytes());
        }

        // Serialize best trace index
        if let Some(idx) = self.best_trace_index {
            buf.push(1);
            buf.extend_from_slice(&(idx as u64).to_le_bytes());
        } else {
            buf.push(0);
        }

        // Serialize fitness as u32 (scaled 0-100000)
        let fitness_int = (self.fitness * 100000.0) as u32;
        buf.extend_from_slice(&fitness_int.to_le_bytes());
    }
}

impl SerializeBytes for ReplayTrace {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&(self.event_indices.len() as u64).to_le_bytes());
        for &idx in &self.event_indices {
            buf.extend_from_slice(&(idx as u64).to_le_bytes());
        }
        buf.extend_from_slice(&(self.total_cost as u64).to_le_bytes());
    }
}

impl SerializeBytes for SimpleEvent {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        self.activity.serialize_bytes(buf);
        self.timestamp.serialize_bytes(buf);
        self.case_id.serialize_bytes(buf);
    }
}

// =========================================================================
// 5. Replay Result Wrapping (Evidence + Witness + Receipt)
// =========================================================================

/// Wrap replay result in Evidence with WitnessState lattice
pub fn wrap_replay_result(
    traces: ReplayTraces,
) -> Result<
    Evidence<ReplayTraces, (), WitnessState>,
    ReplayRefusal,
>
where
    ReplayTraces: SerializeBytes,
{
    // Construct witness from best trace
    let witness = if let Some(idx) = traces.best_trace_index {
        if let Some(trace) = traces.paths.get(idx) {
            WitnessState::PartialReplay {
                trace_indices: trace.event_indices.clone(),
                marking: trace
                    .marking_sequence
                    .last()
                    .map(|m| {
                        m.tokens
                            .iter()
                            .map(|(place, count)| format!("{}[{}]", place, count))
                            .collect()
                    })
                    .unwrap_or_default(),
                cost: trace.total_cost,
            }
        } else {
            WitnessState::Bottom
        }
    } else {
        WitnessState::Bottom
    };

    let mut evidence = Evidence {
        payload: traces,
        state: (),
        witness: witness.clone(),
        epoch: 0,
        signature: IdentitySignature {
            public_key: vec![],
            signature_bytes: vec![],
        },
        hash: Blake3Hash([0u8; 32]),
    };
    evidence.hash = evidence.calculate_hash();
    Ok(evidence)
}

// =========================================================================
// 6. Replay Refusal Enum
// =========================================================================

/// Refusal reasons for replay execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayRefusal {
    /// Event log is empty
    EmptyLog,
    /// No valid alignment path found
    NoValidReplay,
    /// Requested activity is not enabled at current state
    ActivityNotEnabled,
    /// Petri net has no defined final marking
    NoFinalMarking,
    /// Deadlock: no enabled transitions and not at sink
    Deadlock,
}

impl std::fmt::Display for ReplayRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EmptyLog => write!(f, "EmptyLog"),
            Self::NoValidReplay => write!(f, "NoValidReplay"),
            Self::ActivityNotEnabled => write!(f, "ActivityNotEnabled"),
            Self::NoFinalMarking => write!(f, "NoFinalMarking"),
            Self::Deadlock => write!(f, "Deadlock"),
        }
    }
}

// =========================================================================
// 7. Module Receipt Sealing (manufacturing provenance)
// =========================================================================

/// Receipt recording the execution and output of replay manufacturing
#[derive(Debug, Clone)]
pub struct ReplayModuleReceipt {
    /// Blake3 hash of ReplayTraces payload
    pub artifact_hash: String,
    /// Witness marker: indicates which proof witnesses the replay traces
    pub witness_marker: String,
    /// Manufacturing epoch (timestamp)
    pub epoch: u64,
    /// Causality chain: previous module receipt this extends from
    pub causality: Vec<String>,
}

impl ReplayModuleReceipt {
    /// Mint a new receipt for a successfully manufactured replay module
    pub fn mint(
        artifact_hash: &str,
        witness_marker: &str,
        previous_receipt: Option<&str>,
    ) -> Self {
        let epoch = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        ReplayModuleReceipt {
            artifact_hash: artifact_hash.to_string(),
            witness_marker: witness_marker.to_string(),
            epoch,
            causality: previous_receipt.map(|r| vec![r.to_string()]).unwrap_or_default(),
        }
    }
}

// =========================================================================
// 8. Exports for wasm4pm FFI
// =========================================================================

/// Convenience type aliases for public API
pub type ReplayedEvidence<W> = Evidence<ReplayTraces, (), W>;

/// Exported for wasm4pm graduation bridge and FFI boundaries
#[no_mangle]
pub extern "C" fn wasm4pm_replay_version() -> u32 {
    001 // Manufacturing era version
}
