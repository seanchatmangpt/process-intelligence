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
    /// Telemetry trace generated for this path
    pub telemetry_trace: crate::otel::OtelTrace,
}

/// MoveKind: classifier for each step in alignment replay
fn serialize_marking(marking: &Marking) -> String {
    let mut parts = Vec::new();
    for (k, v) in &marking.tokens {
        parts.push(format!("\"{}\":{}", k, v));
    }
    format!("{{{}}}", parts.join(","))
}

fn compute_witness_hash(trace_id: &str, activity: &str) -> String {
    let mut hasher = crate::crypto::Blake3::new();
    hasher.update(trace_id.as_bytes());
    hasher.update(activity.as_bytes());
    let hash_bytes = hasher.finalize();
    crate::otel::hex_encode(&hash_bytes)
}

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

        // Trace Chain Cap check
        if event_log.len() > 1000000 {
            return Err(ReplayRefusal::ChainCapExceeded);
        }

        // Timestamp Monotonicity check
        for i in 0..event_log.len() - 1 {
            if event_log[i].timestamp > event_log[i + 1].timestamp {
                return Err(ReplayRefusal::TimestampNonMonotonic);
            }
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

        // We generate a trace ID from the case_id of the first event (or a fallback)
        let trace_id = self.event_log.first()
            .map(|e| format!("trace_{}", e.case_id))
            .unwrap_or_else(|| "trace_replay".to_string());



        // Recursively explore all possible replay paths
        self.explore_paths(
            initial_marking,
            vec![],
            vec![],
            vec![],
            vec![],
            None,
            &trace_id,
            0,
            0,
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

        let best_cost = best_trace_index
            .and_then(|idx| all_paths.get(idx))
            .map(|trace| trace.total_cost as f64)
            .unwrap_or(0.0);

        let fitness = if !self.event_log.is_empty() {
            1.0 - (best_cost / (self.event_log.len() as f64 * 2.0))
        } else {
            0.0
        };

        // Cache the discovered traces in current_trace
        self.current_trace = all_paths.clone();

        Ok(ReplayTraces {
            paths: all_paths,
            best_trace_index,
            fitness,
        })
    }

    /// Recursive path exploration with telemetry emission and loop protection
    #[allow(clippy::too_many_arguments)]
    fn explore_paths(
        &self,
        current_marking: Marking,
        processed_events: Vec<usize>,
        marking_sequence: Vec<Marking>,
        move_costs: Vec<(MoveKind, u32)>,
        spans: Vec<crate::otel::OtelSpan>,
        prior_hash: Option<[u8; 32]>,
        trace_id: &str,
        consecutive_model_moves: usize,
        total_cost: u32,
        all_paths: &mut Vec<ReplayTrace>,
    ) -> Result<(), ReplayRefusal> {
        // Base case: all events processed
        if processed_events.len() == self.event_log.len() {
            let mut final_marking_seq = marking_sequence.clone();
            final_marking_seq.push(current_marking.clone());

            let event_chain_root = if let Some(last_span) = spans.last() {
                last_span.blake3_receipt.clone()
            } else if let Some(p_hash) = prior_hash {
                crate::otel::hex_encode(&p_hash)
            } else {
                "".to_string()
            };

            let telemetry_trace = crate::otel::OtelTrace {
                trace_id: trace_id.to_string(),
                event_chain_root,
                spans: spans.clone(),
            };

            println!(
                "TELEMETRY_REPLAY_PATH: {{ \"trace_id\": \"{}\", \"spans_count\": {}, \"root\": \"{}\" }}",
                trace_id, spans.len(), telemetry_trace.event_chain_root
            );

            all_paths.push(ReplayTrace {
                event_indices: processed_events,
                marking_sequence: final_marking_seq,
                total_cost,
                move_costs,
                telemetry_trace,
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

            // Get enabled transitions
            let enabled = self.net
                .transitions
                .iter()
                .filter(|t| self.net.is_enabled(t, &current_marking))
                .cloned()
                .collect::<Vec<_>>();

            // 1. Try synchronous move: find matching transition
            for transition in &enabled {
                if transition == &event.activity {
                    let new_marking = self.net.fire(transition, &current_marking);
                    let mut new_processed = processed_events.clone();
                    new_processed.push(evt_idx);
                    new_processed.sort();

                    let mut next_marking_seq = marking_sequence.clone();
                    next_marking_seq.push(current_marking.clone());

                    let mut next_moves = move_costs.clone();
                    next_moves.push((MoveKind::Synchronous, 0));

                    // Generate OtelSpan for this step
                    let step_idx = spans.len();
                    let span_id = format!("span_{}", step_idx);
                    let parent_span_id = if step_idx > 0 { Some(format!("span_{}", step_idx - 1)) } else { None };
                    let start_time = event.timestamp as i64;
                    let end_time = start_time + 1000;
                    let instruction_count = 100;

                    let inst_id = trace_id;
                    let act_name = transition;
                    let act_type = Some("task");
                    let lf = "complete";
                    let state_before = serialize_marking(&current_marking);
                    let state_after = serialize_marking(&new_marking);
                    let wit_id = "replay_engine_v30";
                    let wit_hash = compute_witness_hash(trace_id, transition);

                    let computed_hash = crate::otel::hash_span(
                        prior_hash.as_ref(),
                        trace_id,
                        &span_id,
                        parent_span_id.as_deref(),
                        transition,
                        start_time,
                        end_time,
                        instruction_count,
                        inst_id,
                        act_name,
                        act_type,
                        lf,
                        Some(&state_before),
                        Some(&state_after),
                        wit_id,
                        &wit_hash,
                    );
                    let blake3_receipt = crate::otel::hex_encode(&computed_hash);

                    let mut next_spans = spans.clone();
                    next_spans.push(crate::otel::OtelSpan {
                        span_id,
                        parent_span_id,
                        span_name: transition.clone(),
                        start_time_unix_us: start_time,
                        end_time_unix_us: end_time,
                        instruction_count,
                        blake3_receipt,
                        instance_id: inst_id.to_string(),
                        activity_name: act_name.to_string(),
                        activity_type: act_type.map(|s| s.to_string()),
                        lifecycle: lf.to_string(),
                        token_state_before: Some(state_before),
                        token_state_after: Some(state_after),
                        witness_id: wit_id.to_string(),
                        witness_hash: wit_hash,
                    });

                    self.explore_paths(
                        new_marking,
                        new_processed,
                        next_marking_seq,
                        next_moves,
                        next_spans,
                        Some(computed_hash),
                        trace_id,
                        0, // Reset model moves counter on progress
                        total_cost,
                        all_paths,
                    )?;
                }
            }

            // 2. Try log move: skip event (no matching transition)
            {
                let mut new_processed = processed_events.clone();
                new_processed.push(evt_idx);
                new_processed.sort();

                let mut next_marking_seq = marking_sequence.clone();
                next_marking_seq.push(current_marking.clone());

                let mut next_moves = move_costs.clone();
                next_moves.push((MoveKind::LogMove, 1));

                // Generate OtelSpan for this step
                let step_idx = spans.len();
                let span_id = format!("span_{}", step_idx);
                let parent_span_id = if step_idx > 0 { Some(format!("span_{}", step_idx - 1)) } else { None };
                let start_time = event.timestamp as i64;
                let end_time = start_time + 1000;
                let instruction_count = 100;

                let inst_id = trace_id;
                let act_name = format!("Skip:{}", event.activity);
                let act_type = Some("gate");
                let lf = "abort";
                let state_before = serialize_marking(&current_marking);
                let state_after = serialize_marking(&current_marking);
                let wit_id = "replay_engine_v30";
                let wit_hash = compute_witness_hash(trace_id, &act_name);

                let computed_hash = crate::otel::hash_span(
                    prior_hash.as_ref(),
                    trace_id,
                    &span_id,
                    parent_span_id.as_deref(),
                    &act_name,
                    start_time,
                    end_time,
                    instruction_count,
                    inst_id,
                    &act_name,
                    act_type,
                    lf,
                    Some(&state_before),
                    Some(&state_after),
                    wit_id,
                    &wit_hash,
                );
                let blake3_receipt = crate::otel::hex_encode(&computed_hash);

                let mut next_spans = spans.clone();
                next_spans.push(crate::otel::OtelSpan {
                    span_id,
                    parent_span_id,
                    span_name: act_name.clone(),
                    start_time_unix_us: start_time,
                    end_time_unix_us: end_time,
                    instruction_count,
                    blake3_receipt,
                    instance_id: inst_id.to_string(),
                    activity_name: act_name,
                    activity_type: act_type.map(|s| s.to_string()),
                    lifecycle: lf.to_string(),
                    token_state_before: Some(state_before),
                    token_state_after: Some(state_after),
                    witness_id: wit_id.to_string(),
                    witness_hash: wit_hash,
                });

                self.explore_paths(
                    current_marking.clone(),
                    new_processed,
                    next_marking_seq,
                    next_moves,
                    next_spans,
                    Some(computed_hash),
                    trace_id,
                    0, // Reset model moves counter on progress
                    total_cost + 1,
                    all_paths,
                )?;
            }

            // 3. Try model moves: fire any enabled transition (limited exploration to prevent loop)
            if consecutive_model_moves < self.net.transitions.len() {
                for transition in &enabled {
                    if transition != &event.activity {
                        let new_marking = self.net.fire(transition, &current_marking);

                        let mut next_marking_seq = marking_sequence.clone();
                        next_marking_seq.push(current_marking.clone());

                        let mut next_moves = move_costs.clone();
                        next_moves.push((MoveKind::ModelMove, 1));

                        // Generate OtelSpan for this step
                        let step_idx = spans.len();
                        let span_id = format!("span_{}", step_idx);
                        let parent_span_id = if step_idx > 0 { Some(format!("span_{}", step_idx - 1)) } else { None };
                        let start_time = event.timestamp as i64;
                        let end_time = start_time + 1000;
                        let instruction_count = 100;

                        let inst_id = trace_id;
                        let act_name = transition;
                        let act_type = Some("task");
                        let lf = "schedule";
                        let state_before = serialize_marking(&current_marking);
                        let state_after = serialize_marking(&new_marking);
                        let wit_id = "replay_engine_v30";
                        let wit_hash = compute_witness_hash(trace_id, transition);

                        let computed_hash = crate::otel::hash_span(
                            prior_hash.as_ref(),
                            trace_id,
                            &span_id,
                            parent_span_id.as_deref(),
                            transition,
                            start_time,
                            end_time,
                            instruction_count,
                            inst_id,
                            act_name,
                            act_type,
                            lf,
                            Some(&state_before),
                            Some(&state_after),
                            wit_id,
                            &wit_hash,
                        );
                        let blake3_receipt = crate::otel::hex_encode(&computed_hash);

                        let mut next_spans = spans.clone();
                        next_spans.push(crate::otel::OtelSpan {
                            span_id,
                            parent_span_id,
                            span_name: transition.clone(),
                            start_time_unix_us: start_time,
                            end_time_unix_us: end_time,
                            instruction_count,
                            blake3_receipt,
                            instance_id: inst_id.to_string(),
                            activity_name: act_name.to_string(),
                            activity_type: act_type.map(|s| s.to_string()),
                            lifecycle: lf.to_string(),
                            token_state_before: Some(state_before),
                            token_state_after: Some(state_after),
                            witness_id: wit_id.to_string(),
                            witness_hash: wit_hash,
                        });

                        self.explore_paths(
                            new_marking,
                            processed_events.clone(),
                            next_marking_seq,
                            next_moves,
                            next_spans,
                            Some(computed_hash),
                            trace_id,
                            consecutive_model_moves + 1,
                            total_cost + 1,
                            all_paths,
                        )?;
                    }
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

fn hex_decode_32(hex_str: &str) -> Option<[u8; 32]> {
    if hex_str.len() != 64 {
        return None;
    }
    let bytes_slice = hex_str.as_bytes();
    let mut bytes = [0u8; 32];
    for i in 0..32 {
        let high = match bytes_slice[i * 2] {
            b @ b'0'..=b'9' => b - b'0',
            b @ b'a'..=b'f' => b - b'a' + 10,
            b @ b'A'..=b'F' => b - b'A' + 10,
            _ => return None,
        };
        let low = match bytes_slice[i * 2 + 1] {
            b @ b'0'..=b'9' => b - b'0',
            b @ b'a'..=b'f' => b - b'a' + 10,
            b @ b'A'..=b'F' => b - b'A' + 10,
            _ => return None,
        };
        bytes[i] = (high << 4) | low;
    }
    Some(bytes)
}

/// Simulation step trace
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepTrace {
    /// Transitions enabled before this step
    pub enabled_before: Vec<String>,
    /// Activity executed (None if simulation ended)
    pub activity: Option<String>,
    /// Resulting marking after step
    pub resulting_marking: Marking,
    /// Step cost contribution
    pub step_cost: u32,
    /// Telemetry span generated for this step
    pub telemetry_span: Option<crate::otel::OtelSpan>,
    /// BLAKE3 receipt for this step
    pub blake3_receipt: Option<String>,
}

/// Step-by-step simulator for interactive process exploration
pub struct StepSimulator {
    net: PetriNet,
    history: Vec<StepTrace>,
    current_marking: Marking,
    trace_id: String,
}

impl StepSimulator {
    /// Create new step simulator
    pub fn new(net: PetriNet) -> Result<Self, ReplayRefusal> {
        let current_marking = Marking::initial("source".to_string());
        let trace_id = format!("trace_sim_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_micros())
            .unwrap_or(0));

        Ok(StepSimulator {
            net,
            history: vec![],
            current_marking,
            trace_id,
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

        // Generate telemetry span
        let step_idx = self.history.len();
        let span_id = format!("span_{}", step_idx);
        let parent_span_id = if step_idx > 0 { Some(format!("span_{}", step_idx - 1)) } else { None };
        let start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_micros() as i64)
            .unwrap_or(0);
        let end_time = start_time + 1000;
        let instruction_count = 100;

        let prior_hash = if step_idx > 0 {
            if let Some(ref prev_span) = self.history[step_idx - 1].telemetry_span {
                hex_decode_32(&prev_span.blake3_receipt)
            } else {
                None
            }
        } else {
            None
        };

        let inst_id = self.trace_id.clone();
        let act_name = activity.to_string();
        let act_type = Some("task");
        let lf = "complete";
        let state_before = serialize_marking(&self.current_marking);
        let state_after = serialize_marking(&resulting_marking);
        let wit_id = "replay_engine_v30";
        let wit_hash = compute_witness_hash(&self.trace_id, activity);

        let computed_hash = crate::otel::hash_span(
            prior_hash.as_ref(),
            &self.trace_id,
            &span_id,
            parent_span_id.as_deref(),
            activity,
            start_time,
            end_time,
            instruction_count,
            &inst_id,
            &act_name,
            act_type,
            lf,
            Some(&state_before),
            Some(&state_after),
            wit_id,
            &wit_hash,
        );

        let blake3_receipt = crate::otel::hex_encode(&computed_hash);

        let telemetry_span = crate::otel::OtelSpan {
            span_id,
            parent_span_id,
            span_name: activity.to_string(),
            start_time_unix_us: start_time,
            end_time_unix_us: end_time,
            instruction_count,
            blake3_receipt: blake3_receipt.clone(),
            instance_id: inst_id,
            activity_name: act_name,
            activity_type: act_type.map(|s| s.to_string()),
            lifecycle: lf.to_string(),
            token_state_before: Some(state_before),
            token_state_after: Some(state_after),
            witness_id: wit_id.to_string(),
            witness_hash: wit_hash,
        };

        let step = StepTrace {
            enabled_before: enabled,
            activity: Some(activity.to_string()),
            resulting_marking,
            step_cost: 0,
            telemetry_span: Some(telemetry_span),
            blake3_receipt: Some(blake3_receipt),
        };

        self.history.push(step.clone());
        self.current_marking = step.resulting_marking.clone();

        // Print real-time telemetry updates (stdout stream)
        println!(
            "TELEMETRY_UPDATE: {{ \"trace_id\": \"{}\", \"span_id\": \"{}\", \"activity\": \"{}\", \"receipt\": \"{}\" }}",
            self.trace_id, step.telemetry_span.as_ref().unwrap().span_id, activity, step.blake3_receipt.as_ref().unwrap()
        );

        Ok(step)
    }

    /// Reset to initial state
    pub fn reset(&mut self) {
        self.history.clear();
        self.current_marking = Marking::initial("source".to_string());
        self.trace_id = format!("trace_sim_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_micros())
            .unwrap_or(0));
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
    /// Trace chain cap exceeded (max 1,000,000)
    ChainCapExceeded,
    /// Timestamps are not in strict monotonic order
    TimestampNonMonotonic,
}

impl std::fmt::Display for ReplayRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EmptyLog => write!(f, "EmptyLog"),
            Self::NoValidReplay => write!(f, "NoValidReplay"),
            Self::ActivityNotEnabled => write!(f, "ActivityNotEnabled"),
            Self::NoFinalMarking => write!(f, "NoFinalMarking"),
            Self::Deadlock => write!(f, "Deadlock"),
            Self::ChainCapExceeded => write!(f, "ChainCapExceeded"),
            Self::TimestampNonMonotonic => write!(f, "TimestampNonMonotonic"),
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
    1 // Manufacturing era version
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replay_engine_with_telemetry() {
        let mut places = std::collections::BTreeSet::new();
        places.insert("source".to_string());
        places.insert("sink".to_string());

        let mut transitions = std::collections::BTreeSet::new();
        transitions.insert("t_a".to_string());

        // Create a simple Petri Net: source -> t_a -> sink
        let mut net = PetriNet {
            places,
            transitions,
            pre: std::collections::BTreeMap::new(),
            post: std::collections::BTreeMap::new(),
        };

        let mut pre_a = std::collections::BTreeMap::new();
        pre_a.insert("source".to_string(), 1);
        net.pre.insert("t_a".to_string(), pre_a);

        let mut post_a = std::collections::BTreeMap::new();
        post_a.insert("sink".to_string(), 1);
        net.post.insert("t_a".to_string(), post_a);

        let event_log = vec![SimpleEvent {
            activity: "t_a".to_string(),
            timestamp: 1000,
            case_id: "case_1".to_string(),
        }];

        let mut engine = ReplayEngine::new(net, event_log).unwrap();
        let traces = engine.replay().unwrap();

        assert_eq!(traces.paths.len(), 2);
        let best_path = &traces.paths[traces.best_trace_index.unwrap()];
        assert_eq!(best_path.total_cost, 0);
        assert_eq!(best_path.event_indices, vec![0]);

        // Verify telemetry trace
        let telemetry = &best_path.telemetry_trace;
        assert_eq!(telemetry.trace_id, "trace_case_1");
        assert_eq!(telemetry.spans.len(), 1);
        assert_eq!(telemetry.spans[0].span_name, "t_a");
        assert_ne!(telemetry.spans[0].blake3_receipt, "");

        // Verify telemetry trace verification function
        let ok = crate::otel::verify_otel_trace(telemetry).unwrap();
        assert!(ok);
    }

    #[test]
    fn test_step_simulator_with_telemetry() {
        let mut places = std::collections::BTreeSet::new();
        places.insert("source".to_string());
        places.insert("sink".to_string());

        let mut transitions = std::collections::BTreeSet::new();
        transitions.insert("t_a".to_string());

        // Create a simple Petri Net: source -> t_a -> sink
        let mut net = PetriNet {
            places,
            transitions,
            pre: std::collections::BTreeMap::new(),
            post: std::collections::BTreeMap::new(),
        };

        let mut pre_a = std::collections::BTreeMap::new();
        pre_a.insert("source".to_string(), 1);
        net.pre.insert("t_a".to_string(), pre_a);

        let mut post_a = std::collections::BTreeMap::new();
        post_a.insert("sink".to_string(), 1);
        net.post.insert("t_a".to_string(), post_a);

        let mut simulator = StepSimulator::new(net).unwrap();
        assert_eq!(simulator.enabled_activities(), vec!["t_a".to_string()]);

        let step_res = simulator.step("t_a").unwrap();
        assert_eq!(step_res.activity, Some("t_a".to_string()));
        assert!(step_res.telemetry_span.is_some());
        assert!(step_res.blake3_receipt.is_some());

        // Construct OtelTrace and verify
        let spans = simulator.history().iter()
            .map(|s| s.telemetry_span.clone().unwrap())
            .collect::<Vec<_>>();
        let event_chain_root = spans.last().unwrap().blake3_receipt.clone();
        let trace = crate::otel::OtelTrace {
            trace_id: simulator.trace_id.clone(),
            event_chain_root,
            spans,
        };

        let ok = crate::otel::verify_otel_trace(&trace).unwrap();
        assert!(ok);
    }
}

