use wasm4pm::evidence::{Evidence, Lattice, WitnessState, IdentitySignature, Blake3Hash, ConstraintValue, SerializeBytes};
use wasm4pm::petri::{PetriNet, Marking};
use wasm4pm::sandbox::{GasMeter, RecursionGuard, execute_oblivion_protocol};
use wasm4pm::safety::FfiSafetyChecker;
use wasm4pm::ffi;
use wasm4pm::allocator;
use wasm4pm::otel::{OtelTrace, verify_otel_trace, hash_span};
use std::sync::Mutex;
use std::collections::{BTreeMap, BTreeSet};

static TEST_MUTEX: Mutex<()> = Mutex::new(());

// =========================================================================
// Typestates generic over Parsed, ValidatedSound, and Replayed
// =========================================================================
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parsed;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidatedSound;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Replayed;

impl SerializeBytes for Parsed {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(b"Parsed");
    }
}

impl SerializeBytes for ValidatedSound {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(b"ValidatedSound");
    }
}

impl SerializeBytes for Replayed {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(b"Replayed");
    }
}

// =========================================================================
// Helper Functions for Lattice and Decoding
// =========================================================================
fn meet_constraint_value(a: &ConstraintValue, b: &ConstraintValue) -> ConstraintValue {
    match (a, b) {
        (ConstraintValue::Top, any) | (any, ConstraintValue::Top) => any.clone(),
        (ConstraintValue::Bottom, _) | (_, ConstraintValue::Bottom) => ConstraintValue::Bottom,
        (ConstraintValue::PossiblySatisfied, _) | (_, ConstraintValue::PossiblySatisfied) => ConstraintValue::PossiblySatisfied,
        (ConstraintValue::Satisfied, ConstraintValue::Satisfied) => ConstraintValue::Satisfied,
        (ConstraintValue::Violated, ConstraintValue::Violated) => ConstraintValue::Violated,
        (ConstraintValue::Satisfied, ConstraintValue::Violated) |
        (ConstraintValue::Violated, ConstraintValue::Satisfied) => ConstraintValue::PossiblySatisfied,
    }
}

fn meet_witness_state(a: &WitnessState, b: &WitnessState) -> WitnessState {
    match (a, b) {
        (WitnessState::Top, any) | (any, WitnessState::Top) => any.clone(),
        (WitnessState::Bottom, _) | (_, WitnessState::Bottom) => WitnessState::Bottom,
        (WitnessState::PartialReplay { trace_indices: t1, marking: m1, cost: c1 },
         WitnessState::PartialReplay { trace_indices: t2, marking: m2, cost: c2 }) => {
            if t1 != t2 {
                WitnessState::Bottom
            } else {
                let mut intersected_marking = Vec::new();
                for place in m1 {
                    if m2.contains(place) {
                        intersected_marking.push(place.clone());
                    }
                }
                WitnessState::PartialReplay {
                    trace_indices: t1.clone(),
                    marking: intersected_marking,
                    cost: std::cmp::min(*c1, *c2),
                }
            }
        }
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

// Helper to construct a valid zero-copy OCEL 2.0 binary log buffer
fn build_valid_ocel_buffer() -> Vec<u8> {
    let mut buf = vec![0u8; 256];

    // Magic "OCEL"
    buf[0..4].copy_from_slice(&0x4F43454Cu32.to_be_bytes());
    // Version 2
    buf[4..8].copy_from_slice(&2u32.to_le_bytes());

    // Section Offsets
    let events_offset = 140u32;
    let events_count = 2u32;
    let objects_offset = 188u32;
    let objects_count = 1u32;
    let e2o_offset = 200u32;
    let o2o_offset = 224u32;
    let string_table_offset = 40u32;
    let string_table_size = 100u32;

    buf[8..12].copy_from_slice(&events_offset.to_le_bytes());
    buf[12..16].copy_from_slice(&events_count.to_le_bytes());
    buf[16..20].copy_from_slice(&objects_offset.to_le_bytes());
    buf[20..24].copy_from_slice(&objects_count.to_le_bytes());
    buf[24..28].copy_from_slice(&e2o_offset.to_le_bytes());
    buf[28..32].copy_from_slice(&o2o_offset.to_le_bytes());
    buf[32..36].copy_from_slice(&string_table_offset.to_le_bytes());
    buf[36..40].copy_from_slice(&string_table_size.to_le_bytes());

    // Write String Table at offset 40
    // Offset 0 in table: "e1"
    buf[40..44].copy_from_slice(&2u32.to_le_bytes());
    buf[44..46].copy_from_slice(b"e1");
    
    // Offset 8: "e2"
    buf[48..52].copy_from_slice(&2u32.to_le_bytes());
    buf[52..54].copy_from_slice(b"e2");

    // Offset 16: "create_order"
    buf[56..60].copy_from_slice(&12u32.to_le_bytes());
    buf[60..72].copy_from_slice(b"create_order");

    // Offset 32: "approve_order"
    buf[72..76].copy_from_slice(&13u32.to_le_bytes());
    buf[76..89].copy_from_slice(b"approve_order");

    // Offset 52: "order_1"
    buf[92..96].copy_from_slice(&7u32.to_le_bytes());
    buf[96..103].copy_from_slice(b"order_1");

    // Offset 64: "Order"
    buf[104..108].copy_from_slice(&5u32.to_le_bytes());
    buf[108..113].copy_from_slice(b"Order");

    // Write Events at 140
    // Event 0: id_offset=0, act_offset=16, ts=1000 (i64), type_offset=64, attr_count=0
    buf[140..144].copy_from_slice(&0u32.to_le_bytes());
    buf[144..148].copy_from_slice(&16u32.to_le_bytes());
    buf[148..156].copy_from_slice(&1000i64.to_le_bytes());
    buf[156..160].copy_from_slice(&64u32.to_le_bytes());
    buf[160..162].copy_from_slice(&0u16.to_le_bytes());

    // Event 1: id_offset=8, act_offset=32, ts=5000 (i64), type_offset=64, attr_count=0
    buf[164..168].copy_from_slice(&8u32.to_le_bytes());
    buf[168..172].copy_from_slice(&32u32.to_le_bytes());
    buf[172..180].copy_from_slice(&5000i64.to_le_bytes());
    buf[180..184].copy_from_slice(&64u32.to_le_bytes());
    buf[184..186].copy_from_slice(&0u16.to_le_bytes());

    // Write Objects at 188
    // Object 0: id_offset=52, type_offset=64, attr_count=0
    buf[188..192].copy_from_slice(&52u32.to_le_bytes());
    buf[192..196].copy_from_slice(&64u32.to_le_bytes());
    buf[196..198].copy_from_slice(&0u16.to_le_bytes());

    // Write E2O at 200
    // Entry 0: offset=16, count=1.
    // Entry 1: offset=20, count=1.
    buf[200..204].copy_from_slice(&16u32.to_le_bytes());
    buf[204..208].copy_from_slice(&1u32.to_le_bytes());
    buf[208..212].copy_from_slice(&20u32.to_le_bytes());
    buf[212..216].copy_from_slice(&1u32.to_le_bytes());
    // Element array starts at E2O offset + 16 = 216
    buf[216..220].copy_from_slice(&0u32.to_le_bytes()); // Event 0 -> Object 0
    buf[220..224].copy_from_slice(&0u32.to_le_bytes()); // Event 1 -> Object 0

    // Write O2O at 224
    // Entry 0: offset=8, count=0
    buf[224..228].copy_from_slice(&8u32.to_le_bytes());
    buf[228..232].copy_from_slice(&0u32.to_le_bytes());

    buf
}

// =========================================================================
// REQUIRED E2E TESTS
// =========================================================================

#[test]
fn test_e2e_typelaw_monotonicity() {
    // 1. Verify join properties on ConstraintValue
    let bot = ConstraintValue::bottom();
    let _top = ConstraintValue::top();
    let ps = ConstraintValue::PossiblySatisfied;
    let sat = ConstraintValue::Satisfied;
    let viol = ConstraintValue::Violated;

    // Idempotency: a join a = a
    assert_eq!(bot.join(&bot), bot);
    assert_eq!(ps.join(&ps), ps);
    assert_eq!(sat.join(&sat), sat);

    // Commutativity: a join b = b join a
    assert_eq!(sat.join(&viol), viol.join(&sat));
    assert_eq!(ps.join(&sat), sat.join(&ps));

    // Associativity: (a join b) join c = a join (b join c)
    assert_eq!((sat.join(&viol)).join(&ps), sat.join(&viol.join(&ps)));

    // Custom meet and Absorption laws:
    // a join (a meet b) = a
    // a meet (a join b) = a
    let meet_sat_viol = meet_constraint_value(&sat, &viol);
    assert_eq!(sat.join(&meet_sat_viol), sat);

    let join_sat_viol = sat.join(&viol);
    assert_eq!(meet_constraint_value(&sat, &join_sat_viol), sat);

    // 2. Verify join properties on WitnessState
    let _ws_bot = WitnessState::bottom();
    let _ws_top = WitnessState::top();
    let ws1 = WitnessState::PartialReplay {
        trace_indices: vec![1, 2],
        marking: vec!["p1".to_string()],
        cost: 10,
    };
    let ws2 = WitnessState::PartialReplay {
        trace_indices: vec![3],
        marking: vec!["p2".to_string()],
        cost: 5,
    };

    // Idempotency
    assert_eq!(ws1.join(&ws1), ws1);
    // Commutativity
    assert_eq!(ws1.join(&ws2), ws2.join(&ws1));
    // Absorption: ws1 join (ws1 meet ws2) = ws1
    let meet_ws = meet_witness_state(&ws1, &ws2);
    assert_eq!(ws1.join(&meet_ws), ws1);
}

#[test]
fn test_e2e_typelaw_evidence_lifecycle() {
    let payload = "process_log_segment".to_string();
    let sig = IdentitySignature {
        public_key: vec![0; 32],
        signature_bytes: vec![0; 64],
    };

    // 1. Initial State: Parsed
    let ev_parsed = Evidence {
        payload: payload.clone(),
        state: Parsed,
        witness: WitnessState::Bottom,
        epoch: 1,
        signature: sig.clone(),
        hash: Blake3Hash([0; 32]),
    };
    
    // 2. Transition: Parsed -> ValidatedSound
    let ev_validated = Evidence {
        payload: ev_parsed.payload.clone(),
        state: ValidatedSound,
        witness: WitnessState::PartialReplay {
            trace_indices: vec![1],
            marking: vec!["p1".to_string()],
            cost: 0,
        },
        epoch: 2,
        signature: sig.clone(),
        hash: Blake3Hash([0; 32]),
    };

    // 3. Transition: ValidatedSound -> Replayed (monotonic progression: same trace_indices, larger marking subset, larger cost)
    let ev_replayed = Evidence {
        payload: ev_validated.payload.clone(),
        state: Replayed,
        witness: WitnessState::PartialReplay {
            trace_indices: vec![1],
            marking: vec!["p1".to_string(), "p2".to_string()],
            cost: 10,
        },
        epoch: 3,
        signature: sig.clone(),
        hash: Blake3Hash([0; 32]),
    };

    // Verify typestate sequence
    assert_eq!(ev_parsed.epoch, 1);
    assert_eq!(ev_validated.epoch, 2);
    assert_eq!(ev_replayed.epoch, 3);

    // Verify that the witness transitioned monotonically
    let join_1 = ev_parsed.witness.join(&ev_validated.witness);
    assert_eq!(join_1, ev_validated.witness);

    let join_2 = ev_validated.witness.join(&ev_replayed.witness);
    assert_eq!(join_2, ev_replayed.witness);
}

#[test]
fn test_e2e_petri_net_token_firing() {
    let places: BTreeSet<String> = vec!["p1", "p2", "p3"].into_iter().map(String::from).collect();
    let transitions: BTreeSet<String> = vec!["t1", "t2"].into_iter().map(String::from).collect();

    let mut pre = BTreeMap::new();
    let mut t1_pre = BTreeMap::new();
    t1_pre.insert("p1".to_string(), 1);
    pre.insert("t1".to_string(), t1_pre);

    let mut t2_pre = BTreeMap::new();
    t2_pre.insert("p2".to_string(), 1);
    pre.insert("t2".to_string(), t2_pre);

    let mut post = BTreeMap::new();
    let mut t1_post = BTreeMap::new();
    t1_post.insert("p2".to_string(), 1);
    post.insert("t1".to_string(), t1_post);

    let mut t2_post = BTreeMap::new();
    t2_post.insert("p3".to_string(), 1);
    post.insert("t2".to_string(), t2_post);

    let net = PetriNet::new(places, transitions, pre, post);

    // Initial marking with 1 token in p1
    let m0 = Marking::initial("p1".to_string());
    assert!(net.is_enabled("t1", &m0));
    assert!(!net.is_enabled("t2", &m0));

    // Fire t1
    let m1 = net.fire("t1", &m0);
    assert_eq!(m1.get_tokens("p1"), 0);
    assert_eq!(m1.get_tokens("p2"), 1);
    assert_eq!(m1.get_tokens("p3"), 0);

    // Now t2 is enabled
    assert!(!net.is_enabled("t1", &m1));
    assert!(net.is_enabled("t2", &m1));

    // Fire t2
    let m2 = net.fire("t2", &m1);
    assert_eq!(m2.get_tokens("p2"), 0);
    assert_eq!(m2.get_tokens("p3"), 1);
}

#[test]
fn test_e2e_petri_net_soundness() {
    let make_arc = |place: &str, weight: u32| {
        let mut map = BTreeMap::new();
        map.insert(place.to_string(), weight);
        map
    };

    // 1. Sound WF-net topology
    let places: BTreeSet<String> = vec!["source", "p1", "sink"].into_iter().map(String::from).collect();
    let transitions: BTreeSet<String> = vec!["t1", "t2"].into_iter().map(String::from).collect();

    let mut pre = BTreeMap::new();
    pre.insert("t1".to_string(), make_arc("source", 1));
    pre.insert("t2".to_string(), make_arc("p1", 1));

    let mut post = BTreeMap::new();
    post.insert("t1".to_string(), make_arc("p1", 1));
    post.insert("t2".to_string(), make_arc("sink", 1));

    let net_sound = PetriNet::new(places, transitions, pre, post);
    let result_sound = net_sound.analyze_soundness();

    assert!(result_sound.is_wf_net);
    assert_eq!(result_sound.source_place, Some("source".to_string()));
    assert_eq!(result_sound.sink_place, Some("sink".to_string()));
    assert!(result_sound.is_1_bounded);
    assert!(!result_sound.has_deadlock);
    assert!(result_sound.proper_completion);
    assert!(result_sound.option_to_complete);

    // 2. Unsound WF-net (Deadlock Topology)
    let places_dl: BTreeSet<String> = vec!["source", "p1", "p2", "sink"].into_iter().map(String::from).collect();
    let transitions_dl: BTreeSet<String> = vec!["t1", "t2", "t3"].into_iter().map(String::from).collect();

    let mut pre_dl = BTreeMap::new();
    pre_dl.insert("t1".to_string(), make_arc("source", 1));
    pre_dl.insert("t2".to_string(), make_arc("source", 1));
    let mut t3_inputs = BTreeMap::new();
    t3_inputs.insert("p1".to_string(), 1);
    t3_inputs.insert("p2".to_string(), 1);
    pre_dl.insert("t3".to_string(), t3_inputs);

    let mut post_dl = BTreeMap::new();
    post_dl.insert("t1".to_string(), make_arc("p1", 1));
    post_dl.insert("t2".to_string(), make_arc("p2", 1));
    post_dl.insert("t3".to_string(), make_arc("sink", 1));

    let net_deadlock = PetriNet::new(places_dl, transitions_dl, pre_dl, post_dl);
    let result_deadlock = net_deadlock.analyze_soundness();

    assert!(result_deadlock.is_wf_net);
    assert!(result_deadlock.has_deadlock);
    assert!(!result_deadlock.option_to_complete);
}

#[test]
fn test_e2e_sandbox_gas_meter() {
    // Basic gas consumption
    let mut meter = GasMeter::new(1000);
    assert_eq!(meter.consumed(), 0);
    assert!(meter.consume(500).is_ok());
    assert_eq!(meter.consumed(), 500);
    assert!(meter.consume(500).is_ok());
    assert_eq!(meter.consumed(), 1000);
    assert!(meter.consume(1).is_err());

    // Balance/Clamping check
    let mut meter_large = GasMeter::new(20_000_000); // requested above 10,000,000 ceiling
    assert!(meter_large.consume(10_000_000).is_ok());
    assert!(meter_large.consume(1).is_err()); // clamped at 10,000,000
}

#[test]
fn test_e2e_sandbox_recursion_guard() {
    let mut guard = RecursionGuard::new(3);
    assert!(guard.enter().is_ok());
    assert!(guard.enter().is_ok());
    assert!(guard.enter().is_ok());
    assert!(guard.enter().is_err()); // exceeds max_depth of 3
    guard.exit();
    assert!(guard.enter().is_ok());

    // Clamping check (capped at 100)
    let mut guard_large = RecursionGuard::new(150);
    for _ in 0..100 {
        assert!(guard_large.enter().is_ok());
    }
    assert!(guard_large.enter().is_err()); // fails at 101st
}

#[test]
fn test_e2e_sandbox_oblivion_protocol() {
    let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    
    // Allocate global arena
    allocator::init_global_arena(1024).unwrap();
    
    // Allocate permanent memory block and write pattern
    let layout = std::alloc::Layout::from_size_align(100, 8).unwrap();
    let ptr = allocator::alloc_permanent(layout).unwrap();
    unsafe {
        std::ptr::write_bytes(ptr, 0xAA, 100);
    }
    
    let slice = unsafe { std::slice::from_raw_parts(ptr, 100) };
    assert_eq!(slice[0], 0xAA);
    
    // Execute oblivion protocol
    let seed = [0u8; 32];
    execute_oblivion_protocol(&seed);
    
    // Verify memory was scrubbed and does not contain 0xAA anymore
    assert_ne!(slice[0], 0xAA);
}

#[test]
fn test_e2e_safety_checker() {
    let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    allocator::init_global_arena(10 * 1024 * 1024).unwrap();

    let bounds = allocator::get_arena_boundaries().unwrap();
    let base_ptr = bounds.base_addr as *const u8;
    let _trans_start_ptr = (bounds.base_addr + bounds.transient_start) as *const u8;

    // Alignment checking
    assert!(FfiSafetyChecker::check_alignment(base_ptr, 8));
    let unaligned_ptr = (bounds.base_addr + 1) as *const u8;
    assert!(!FfiSafetyChecker::check_alignment(unaligned_ptr, 8));

    // Overflow checking
    assert!(FfiSafetyChecker::check_overflow(base_ptr, 1000));
    let overflow_ptr = usize::MAX - 10;
    assert!(!FfiSafetyChecker::check_overflow(overflow_ptr as *const u8, 20));

    // Partition boundaries checking
    assert!(FfiSafetyChecker::check_partition_boundaries(base_ptr, 100));
    let crossing_ptr = (bounds.base_addr + bounds.transient_start - 50) as *const u8;
    assert!(!FfiSafetyChecker::check_partition_boundaries(crossing_ptr, 100)); // crosses permanent -> transient boundary

    // Disjointness checking
    let ptr1 = base_ptr;
    let ptr2 = (base_ptr as usize + 200) as *const u8;
    let ptr3 = (base_ptr as usize + 50) as *const u8;

    assert!(FfiSafetyChecker::check_disjoint(ptr1, 100, ptr2, 100));
    assert!(!FfiSafetyChecker::check_disjoint(ptr1, 100, ptr3, 100)); // overlap
}

#[test]
fn test_e2e_ffi_lifecycle() {
    let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    // 1. Initialize arena
    let init_code = ffi::wasm_init(10 * 1024 * 1024);
    assert_eq!(init_code, 0);

    let ocel_buf = build_valid_ocel_buffer();
    let log_len = ocel_buf.len() as u32;

    // 2. Allocate and copy log buffer
    let log_offset = ffi::wasm_alloc(log_len);
    assert_ne!(log_offset, 0);
    let log_ptr = allocator::get_absolute_ptr(log_offset).unwrap();
    unsafe {
        std::ptr::copy_nonoverlapping(ocel_buf.as_ptr(), log_ptr, log_len as usize);
    }

    // 3. Allocate and copy query
    let query_str = "create_order,approve_order,10000";
    let query_len = query_str.len() as u32;
    let query_offset = ffi::wasm_alloc(query_len);
    assert_ne!(query_offset, 0);
    let query_ptr = allocator::get_absolute_ptr(query_offset).unwrap();
    unsafe {
        std::ptr::copy_nonoverlapping(query_str.as_ptr(), query_ptr, query_len as usize);
    }

    // 4. Query execution
    let res_encoded = ffi::wasm_parse_and_query(log_offset, log_len, query_offset, query_len);
    let res_offset = (res_encoded >> 32) as u32;
    let res_len = (res_encoded & 0xFFFFFFFF) as u32;

    assert_ne!(res_offset, 0);
    assert_ne!(res_len, 0);

    let res_ptr = allocator::get_absolute_ptr(res_offset).unwrap();
    let res_slice = unsafe { std::slice::from_raw_parts(res_ptr, res_len as usize) };
    let res_str = std::str::from_utf8(res_slice).unwrap();

    assert!(res_str.contains("\"match_count\": 1"));
    assert!(res_str.contains("\"event_1_id\":\"e1\""));
    assert!(res_str.contains("\"event_2_id\":\"e2\""));
}

#[test]
fn test_e2e_otel_trace_verification() {
    let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let trace_id = "4a7b744ce58b88cd28148b5dfbe984f9";

    // 1. Build a valid span chain
    let s0_id = "0000000000000001";
    let s0_name = "StartProcess";
    let s0_start = 1000i64;
    let s0_end = 2000i64;
    let s0_ic = 500i64;

    let hash0 = hash_span(
        None,
        trace_id,
        s0_id,
        None,
        s0_name,
        s0_start,
        s0_end,
        s0_ic,
    );
    let hash0_hex = hex_encode(&hash0);

    let s1_id = "0000000000000002";
    let s1_parent = Some(s0_id);
    let s1_name = "ExecuteStep";
    let s1_start = 1200i64;
    let s1_end = 1800i64;
    let s1_ic = 1200i64;

    let hash1 = hash_span(
        Some(&hash0),
        trace_id,
        s1_id,
        s1_parent,
        s1_name,
        s1_start,
        s1_end,
        s1_ic,
    );
    let hash1_hex = hex_encode(&hash1);

    let valid_json = format!(
        "{{\n  \"trace_id\": \"{}\",\n  \"event_chain_root\": \"{}\",\n  \"spans\": [\n    {{\n      \"span_id\": \"{}\",\n      \"parent_span_id\": null,\n      \"span_name\": \"{}\",\n      \"start_time_unix_us\": {},\n      \"end_time_unix_us\": {},\n      \"instruction_count\": {},\n      \"blake3_receipt\": \"{}\"\n    }},\n    {{\n      \"span_id\": \"{}\",\n      \"parent_span_id\": \"{}\",\n      \"span_name\": \"{}\",\n      \"start_time_unix_us\": {},\n      \"end_time_unix_us\": {},\n      \"instruction_count\": {},\n      \"blake3_receipt\": \"{}\"\n    }}\n  ]\n}}",
        trace_id, hash1_hex,
        s0_id, s0_name, s0_start, s0_end, s0_ic, hash0_hex,
        s1_id, s0_id, s1_name, s1_start, s1_end, s1_ic, hash1_hex
    );

    // Verify valid trace
    let trace = OtelTrace::parse_from_str(&valid_json).unwrap();
    let res = verify_otel_trace(&trace);
    assert!(res.is_ok());
    assert!(res.unwrap());

    // 2. Blake3 receipt mismatch (tampering detection)
    let tampered_json = valid_json.replace(&format!("\"instruction_count\": {}", s1_ic), "\"instruction_count\": 1201");
    let tampered_trace = OtelTrace::parse_from_str(&tampered_json).unwrap();
    let tampered_res = verify_otel_trace(&tampered_trace);
    assert!(tampered_res.is_err());
    assert!(tampered_res.unwrap_err().contains("Span BLAKE3 receipt mismatch"));

    // 3. Parent-child timing constraint violation
    let invalid_timing_json = format!(
        "{{\n  \"trace_id\": \"{}\",\n  \"event_chain_root\": \"{}\",\n  \"spans\": [\n    {{\n      \"span_id\": \"{}\",\n      \"parent_span_id\": null,\n      \"span_name\": \"{}\",\n      \"start_time_unix_us\": {},\n      \"end_time_unix_us\": {},\n      \"instruction_count\": {},\n      \"blake3_receipt\": \"{}\"\n    }},\n    {{\n      \"span_id\": \"{}\",\n      \"parent_span_id\": \"{}\",\n      \"span_name\": \"{}\",\n      \"start_time_unix_us\": 900,\n      \"end_time_unix_us\": {},\n      \"instruction_count\": {},\n      \"blake3_receipt\": \"{}\"\n    }}\n  ]\n}}",
        trace_id, hash1_hex,
        s0_id, s0_name, s0_start, s0_end, s0_ic, hash0_hex,
        s1_id, s0_id, s1_name, s1_end, s1_ic, hash1_hex
    );
    let invalid_timing_trace = OtelTrace::parse_from_str(&invalid_timing_json).unwrap();
    let invalid_timing_res = verify_otel_trace(&invalid_timing_trace);
    assert!(invalid_timing_res.is_err());
    assert!(invalid_timing_res.unwrap_err().contains("Parent-child timing constraint violated"));

    // 4. Cyclic parent-child dependency detection
    let cyclic_json = format!(
        "{{\n  \"trace_id\": \"{}\",\n  \"event_chain_root\": \"{}\",\n  \"spans\": [\n    {{\n      \"span_id\": \"{}\",\n      \"parent_span_id\": \"{}\",\n      \"span_name\": \"{}\",\n      \"start_time_unix_us\": {},\n      \"end_time_unix_us\": {},\n      \"instruction_count\": {},\n      \"blake3_receipt\": \"{}\"\n    }},\n    {{\n      \"span_id\": \"{}\",\n      \"parent_span_id\": \"{}\",\n      \"span_name\": \"{}\",\n      \"start_time_unix_us\": {},\n      \"end_time_unix_us\": {},\n      \"instruction_count\": {},\n      \"blake3_receipt\": \"{}\"\n    }}\n  ]\n}}",
        trace_id, hash1_hex,
        s0_id, s1_id, s0_name, s0_start, s0_end, s0_ic, hash0_hex,
        s1_id, s0_id, s1_name, s0_start, s0_end, s1_ic, hash1_hex
    );
    let cyclic_trace = OtelTrace::parse_from_str(&cyclic_json).unwrap();
    let cyclic_res = verify_otel_trace(&cyclic_trace);
    assert!(cyclic_res.is_err());
    assert!(cyclic_res.unwrap_err().contains("Cyclic parent-child dependency detected"));
}
