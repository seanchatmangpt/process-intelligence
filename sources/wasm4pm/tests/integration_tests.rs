use wasm4pm::allocator;
use wasm4pm::ocel::ZeroCopyOcel;
use wasm4pm::query::{self, OcpqQuery};
use wasm4pm::sandbox::{self, GasMeter, RecursionGuard};
use wasm4pm::ffi;
use std::sync::Mutex;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

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
    // Entry 0: offset=8, count=1
    buf[200..204].copy_from_slice(&8u32.to_le_bytes());
    buf[204..208].copy_from_slice(&1u32.to_le_bytes());
    // Entry 1: offset=12, count=1
    buf[208..212].copy_from_slice(&12u32.to_le_bytes());
    buf[212..216].copy_from_slice(&1u32.to_le_bytes());
    // E2O Elements (relative offset 8: index 0, offset 12: index 0)
    buf[208..212].copy_from_slice(&0u32.to_le_bytes()); // Element for Event 0 is Object 0
    buf[212..216].copy_from_slice(&0u32.to_le_bytes()); // Element for Event 1 is Object 0
    // Let's correct alignment and offsets for E2O entries:
    // Event count is 2. The entry array size is 2 * 8 = 16 bytes.
    // So elements start at relative offset 16.
    // Event 0: offset=16, count=1.
    // Event 1: offset=20, count=1.
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

#[test]
fn test_allocator_boundaries() {
    let _lock = TEST_MUTEX.lock().unwrap();
    // Test ceiling violation
    let res = allocator::init_global_arena(2_000_000_000); // 2GB (absolute limit is 1GB)
    assert!(res.is_err());

    // Init with 10MB
    let res = allocator::init_global_arena(10 * 1024 * 1024);
    assert!(res.is_ok());

    // Try to allocate large block exceeding permanent boundary (which is half of ceiling = 5MB)
    let layout = std::alloc::Layout::from_size_align(6 * 1024 * 1024, 8).unwrap();
    let ptr = allocator::alloc_permanent(layout);
    assert!(ptr.is_err());

    // Valid allocation
    let layout = std::alloc::Layout::from_size_align(1024, 8).unwrap();
    let ptr = allocator::alloc_permanent(layout);
    assert!(ptr.is_ok());
    let offset = allocator::get_relative_offset(ptr.unwrap());
    assert!(offset.is_some());
    assert_ne!(offset.unwrap(), 0);
}

#[test]
fn test_zero_copy_parsing() {
    let _lock = TEST_MUTEX.lock().unwrap();
    let buf = build_valid_ocel_buffer();
    let ocel = ZeroCopyOcel::parse(&buf).unwrap();
    
    assert_eq!(ocel.events_count(), 2);
    assert_eq!(ocel.objects_count(), 1);

    assert_eq!(ocel.get_event_id(0).unwrap(), "e1");
    assert_eq!(ocel.get_event_activity(0).unwrap(), "create_order");
    assert_eq!(ocel.get_event_timestamp(0).unwrap(), 1000);
    assert_eq!(ocel.get_event_type(0).unwrap(), "Order");

    assert_eq!(ocel.get_event_id(1).unwrap(), "e2");
    assert_eq!(ocel.get_event_activity(1).unwrap(), "approve_order");
    assert_eq!(ocel.get_event_timestamp(1).unwrap(), 5000);

    assert_eq!(ocel.get_object_id(0).unwrap(), "order_1");
    assert_eq!(ocel.get_object_type(0).unwrap(), "Order");

    // Test E2O index retrieval
    let e1_objs = ocel.get_event_objects(0).unwrap();
    assert_eq!(e1_objs.len(), 1);
    assert_eq!(e1_objs[0], 0);

    let e2_objs = ocel.get_event_objects(1).unwrap();
    assert_eq!(e2_objs.len(), 1);
    assert_eq!(e2_objs[0], 0);
}

#[test]
fn test_query_evaluator() {
    let _lock = TEST_MUTEX.lock().unwrap();
    let buf = build_valid_ocel_buffer();
    let ocel = ZeroCopyOcel::parse(&buf).unwrap();
    
    let query_str = "create_order,approve_order,10000";
    let query_ast = OcpqQuery::parse(query_str).unwrap();

    let mut gas_meter = GasMeter::new(10_000);
    let mut recursion_guard = RecursionGuard::new(10);

    let res = query::execute_ocpq_query(&ocel, &query_ast, &mut gas_meter, &mut recursion_guard).unwrap();
    assert_eq!(res.match_count, 1);
    assert_eq!(res.matches[0].event_1_id, "e1");
    assert_eq!(res.matches[0].event_2_id, "e2");
    assert_eq!(res.matches[0].object_id, "order_1");
    assert_eq!(res.matches[0].duration_us, 4000);
}

#[test]
fn test_gas_limit_violation() {
    let _lock = TEST_MUTEX.lock().unwrap();
    let buf = build_valid_ocel_buffer();
    let ocel = ZeroCopyOcel::parse(&buf).unwrap();
    
    let query_str = "create_order,approve_order,10000";
    let query_ast = OcpqQuery::parse(query_str).unwrap();

    // Very low gas limit
    let mut gas_meter = GasMeter::new(10);
    let mut recursion_guard = RecursionGuard::new(10);

    let res = query::execute_ocpq_query(&ocel, &query_ast, &mut gas_meter, &mut recursion_guard);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), sandbox::ERR_CYCLE_OVERFLOW);
}

#[test]
fn test_recursion_limit_violation() {
    let _lock = TEST_MUTEX.lock().unwrap();
    let buf = build_valid_ocel_buffer();
    let ocel = ZeroCopyOcel::parse(&buf).unwrap();
    
    let query_str = "create_order,approve_order,10000";
    let query_ast = OcpqQuery::parse(query_str).unwrap();

    let mut gas_meter = GasMeter::new(10_000);
    // Recursion limit = 0 to trigger immediate violation
    let mut recursion_guard = RecursionGuard::new(0);

    let res = query::execute_ocpq_query(&ocel, &query_ast, &mut gas_meter, &mut recursion_guard);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), sandbox::ERR_LIFECYCLE_VIOLATION);
}

#[test]
fn test_oblivion_protocol_memory_shredding() {
    let _lock = TEST_MUTEX.lock().unwrap();
    allocator::init_global_arena(1 * 1024 * 1024).unwrap(); // 1MB
    
    // Allocate something and write data
    let layout = std::alloc::Layout::from_size_align(100, 8).unwrap();
    let ptr = allocator::alloc_permanent(layout).unwrap();
    unsafe {
        std::ptr::write_bytes(ptr, 0xAA, 100);
    }

    // Verify it contains non-zero
    let slice = unsafe { std::slice::from_raw_parts(ptr, 100) };
    assert_eq!(slice[0], 0xAA);

    // Shred
    let seed = [0u8; 32];
    sandbox::execute_oblivion_protocol(&seed);

    // Verify it has been overwritten (should contain CSPRNG random bytes, i.e., not 0xAA)
    assert_ne!(slice[0], 0xAA);
}

#[test]
fn test_ffi_boundary_safety() {
    let _lock = TEST_MUTEX.lock().unwrap();
    // Initialize FFI with 50MB
    let code = ffi::wasm_init(50 * 1024 * 1024);
    assert_eq!(code, 0);

    let ocel_buf = build_valid_ocel_buffer();
    
    // Allocate space for log via FFI
    let log_len = ocel_buf.len() as u32;
    let log_offset = ffi::wasm_alloc(log_len);
    assert_ne!(log_offset, 0);

    // Write log data to allocated space
    let log_ptr = allocator::get_absolute_ptr(log_offset).unwrap();
    unsafe {
        std::ptr::copy_nonoverlapping(ocel_buf.as_ptr(), log_ptr, log_len as usize);
    }

    // Allocate space for query string
    let query_str = "create_order,approve_order,10000";
    let query_len = query_str.len() as u32;
    let query_offset = ffi::wasm_alloc(query_len);
    assert_ne!(query_offset, 0);

    let query_ptr = allocator::get_absolute_ptr(query_offset).unwrap();
    unsafe {
        std::ptr::copy_nonoverlapping(query_str.as_ptr(), query_ptr, query_len as usize);
    }

    // Execute query via FFI
    let res_encoded = ffi::wasm_parse_and_query(log_offset, log_len, query_offset, query_len);
    let res_offset = (res_encoded >> 32) as u32;
    let res_len = (res_encoded & 0xFFFFFFFF) as u32;

    assert_ne!(res_offset, 0);
    assert_ne!(res_len, 0);

    // Read result JSON
    let res_ptr = allocator::get_absolute_ptr(res_offset).unwrap();
    let res_slice = unsafe { std::slice::from_raw_parts(res_ptr, res_len as usize) };
    let res_str = std::str::from_utf8(res_slice).unwrap();

    assert!(res_str.contains("\"match_count\": 1"));
    assert!(res_str.contains("\"event_1_id\":\"e1\""));
    assert!(res_str.contains("\"event_2_id\":\"e2\""));

    // Test FFI heap shredding
    let seed_offset = ffi::wasm_alloc(32);
    assert_ne!(seed_offset, 0);
    let seed_ptr = allocator::get_absolute_ptr(seed_offset).unwrap();
    unsafe {
        std::ptr::write_bytes(seed_ptr, 0x05, 32);
    }

    let shred_code = ffi::wasm_shred_heap(seed_offset);
    assert_eq!(shred_code, 0);
}

#[test]
fn test_otel_trace_blake3_verification() {
    let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let trace_id = "4a7b744ce58b88cd28148b5dfbe984f9";
    
    // Generate valid span chain
    let s0_id = "0000000000000001";
    let s0_name = "StartProcess";
    let s0_start = 1000i64;
    let s0_end = 2000i64;
    let s0_ic = 500i64;
    
    let hash0 = wasm4pm::otel::hash_span(
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
    
    let hash1 = wasm4pm::otel::hash_span(
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
    
    let root_hex = &hash1_hex;
    
    let valid_json = format!(
        "{{\n  \"trace_id\": \"{}\",\n  \"event_chain_root\": \"{}\",\n  \"spans\": [\n    {{\n      \"span_id\": \"{}\",\n      \"parent_span_id\": null,\n      \"span_name\": \"{}\",\n      \"start_time_unix_us\": {},\n      \"end_time_unix_us\": {},\n      \"instruction_count\": {},\n      \"blake3_receipt\": \"{}\"\n    }},\n    {{\n      \"span_id\": \"{}\",\n      \"parent_span_id\": \"{}\",\n      \"span_name\": \"{}\",\n      \"start_time_unix_us\": {},\n      \"end_time_unix_us\": {},\n      \"instruction_count\": {},\n      \"blake3_receipt\": \"{}\"\n    }}\n  ]\n}}",
        trace_id, root_hex,
        s0_id, s0_name, s0_start, s0_end, s0_ic, hash0_hex,
        s1_id, s0_id, s1_name, s1_start, s1_end, s1_ic, hash1_hex
    );
    
    // 1. Verify parsing and verification of valid trace
    let trace = wasm4pm::otel::OtelTrace::parse_from_str(&valid_json).unwrap();
    let res = wasm4pm::otel::verify_otel_trace(&trace);
    assert!(res.is_ok());
    assert!(res.unwrap());
    
    // 2. Verify via FFI boundary
    let ffi_init_code = ffi::wasm_init(10 * 1024 * 1024);
    assert_eq!(ffi_init_code, 0);
    
    let json_len = valid_json.len() as u32;
    let json_offset = ffi::wasm_alloc(json_len);
    assert_ne!(json_offset, 0);
    
    let json_ptr = allocator::get_absolute_ptr(json_offset).unwrap();
    unsafe {
        std::ptr::copy_nonoverlapping(valid_json.as_ptr(), json_ptr, json_len as usize);
    }
    
    let ffi_res = ffi::wasm_verify_otel_trace(json_offset, json_len);
    assert_eq!(ffi_res, 0);
    
    // 3. Verify tampering detection
    let tampered_json = valid_json.replace(&format!("\"instruction_count\": {}", s1_ic), "\"instruction_count\": 1201");
    let tampered_trace = wasm4pm::otel::OtelTrace::parse_from_str(&tampered_json).unwrap();
    let tampered_res = wasm4pm::otel::verify_otel_trace(&tampered_trace);
    assert!(tampered_res.is_err());
    assert!(tampered_res.unwrap_err().contains("Span BLAKE3 receipt mismatch"));
    
    let tampered_len = tampered_json.len() as u32;
    let tampered_offset = ffi::wasm_alloc(tampered_len);
    assert_ne!(tampered_offset, 0);
    let tampered_ptr = allocator::get_absolute_ptr(tampered_offset).unwrap();
    unsafe {
        std::ptr::copy_nonoverlapping(tampered_json.as_ptr(), tampered_ptr, tampered_len as usize);
    }
    let ffi_tampered_res = ffi::wasm_verify_otel_trace(tampered_offset, tampered_len);
    assert_eq!(ffi_tampered_res, sandbox::ERR_REPLAY_ATTESTATION);
    
    // 4. Verify parent-child timing constraint violation detection
    let invalid_timing_json = format!(
        "{{\n  \"trace_id\": \"{}\",\n  \"event_chain_root\": \"{}\",\n  \"spans\": [\n    {{\n      \"span_id\": \"{}\",\n      \"parent_span_id\": null,\n      \"span_name\": \"{}\",\n      \"start_time_unix_us\": {},\n      \"end_time_unix_us\": {},\n      \"instruction_count\": {},\n      \"blake3_receipt\": \"{}\"\n    }},\n    {{\n      \"span_id\": \"{}\",\n      \"parent_span_id\": \"{}\",\n      \"span_name\": \"{}\",\n      \"start_time_unix_us\": 900,\n      \"end_time_unix_us\": {},\n      \"instruction_count\": {},\n      \"blake3_receipt\": \"{}\"\n    }}\n  ]\n}}",
        trace_id, root_hex,
        s0_id, s0_name, s0_start, s0_end, s0_ic, hash0_hex,
        s1_id, s0_id, s1_name, s1_end, s1_ic, hash1_hex
    );
    let invalid_timing_trace = wasm4pm::otel::OtelTrace::parse_from_str(&invalid_timing_json).unwrap();
    let invalid_timing_res = wasm4pm::otel::verify_otel_trace(&invalid_timing_trace);
    assert!(invalid_timing_res.is_err());
    assert!(invalid_timing_res.unwrap_err().contains("Parent-child timing constraint violated"));
    
    // 5. Verify cyclic dependency detection
    let cyclic_json = format!(
        "{{\n  \"trace_id\": \"{}\",\n  \"event_chain_root\": \"{}\",\n  \"spans\": [\n    {{\n      \"span_id\": \"{}\",\n      \"parent_span_id\": \"{}\",\n      \"span_name\": \"{}\",\n      \"start_time_unix_us\": {},\n      \"end_time_unix_us\": {},\n      \"instruction_count\": {},\n      \"blake3_receipt\": \"{}\"\n    }},\n    {{\n      \"span_id\": \"{}\",\n      \"parent_span_id\": \"{}\",\n      \"span_name\": \"{}\",\n      \"start_time_unix_us\": {},\n      \"end_time_unix_us\": {},\n      \"instruction_count\": {},\n      \"blake3_receipt\": \"{}\"\n    }}\n  ]\n}}",
        trace_id, root_hex,
        s0_id, s1_id, s0_name, s0_start, s0_end, s0_ic, hash0_hex,
        s1_id, s0_id, s1_name, s0_start, s0_end, s1_ic, hash1_hex
    );
    let cyclic_trace = wasm4pm::otel::OtelTrace::parse_from_str(&cyclic_json).unwrap();
    let cyclic_res = wasm4pm::otel::verify_otel_trace(&cyclic_trace);
    assert!(cyclic_res.is_err());
    assert!(cyclic_res.unwrap_err().contains("Cyclic parent-child dependency detected"));
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}


#[test]
fn test_recursion_guard_clamping() {
    let mut guard = RecursionGuard::new(150);
    // Enter 100 times should succeed
    for _ in 0..100 {
        assert!(guard.enter().is_ok());
    }
    // The 101st enter must fail because it is capped at 100
    assert_eq!(guard.enter().unwrap_err(), sandbox::ERR_LIFECYCLE_VIOLATION);
}

#[test]
fn test_gas_meter_clamping() {
    let mut meter = GasMeter::new(20_000_000);
    // Consume 10,000,000 should succeed
    assert!(meter.consume(10_000_000).is_ok());
    // Consuming 1 more cycle should fail because budget is capped at 10,000,000
    assert_eq!(meter.consume(1).unwrap_err(), sandbox::ERR_CYCLE_OVERFLOW);
}

#[test]
fn test_ffi_get_last_error_no_panic() {
    let _lock = TEST_MUTEX.lock().unwrap();
    // Verify calling wasm_get_last_error does not crash and behaves correctly
    let err = ffi::wasm_get_last_error();
    assert!(err == 0 || err == sandbox::ERR_LIFECYCLE_VIOLATION);
}

#[test]
fn test_evidence_lattice_and_typestates() {
    use wasm4pm::evidence::{Evidence, Lattice, WitnessState, IdentitySignature, Blake3Hash};

    // 1. Test WitnessState Lattice Properties
    let bottom = WitnessState::bottom();
    let top = WitnessState::top();

    let p1 = WitnessState::PartialReplay {
        trace_indices: vec![1, 2],
        marking: vec!["p1".to_string()],
        cost: 10,
    };

    let p2 = WitnessState::PartialReplay {
        trace_indices: vec![3],
        marking: vec!["p2".to_string()],
        cost: 20,
    };

    let p3 = WitnessState::PartialReplay {
        trace_indices: vec![2, 4],
        marking: vec!["p1".to_string()],
        cost: 15,
    };

    // Identity and bounds
    assert_eq!(bottom.join(&p1), p1);
    assert_eq!(p1.join(&bottom), p1);
    assert_eq!(top.join(&p1), top);
    assert_eq!(p1.join(&top), top);

    // Disjointness check in join
    let union_ws = p1.join(&p2);
    if let WitnessState::PartialReplay { ref trace_indices, marking: _, cost } = union_ws {
        assert_eq!(trace_indices.len(), 3);
        assert!(trace_indices.contains(&1));
        assert!(trace_indices.contains(&2));
        assert!(trace_indices.contains(&3));
        assert_eq!(cost, 30);
    } else {
        panic!("Expected PartialReplay");
    }

    // Overlap evaluates to Top
    assert_eq!(p1.join(&p3), WitnessState::Top);

    // 2. Test new Evidence struct
    let payload = "artifact_spec".to_string();
    let state = "initial".to_string();
    
    let sig = IdentitySignature {
        public_key: vec![0; 32],
        signature_bytes: vec![0; 64],
    };
    
    let ev1 = Evidence {
        payload: payload.clone(),
        state: state.clone(),
        witness: p1.clone(),
        epoch: 1,
        signature: sig.clone(),
        hash: Blake3Hash([0; 32]),
    };
    
    let hash1 = ev1.calculate_hash();
    let mut ev1_hashed = ev1;
    ev1_hashed.hash = hash1;
    
    let ev2 = Evidence {
        payload: payload.clone(),
        state: state.clone(),
        witness: p1.join(&p2),
        epoch: 2,
        signature: sig,
        hash: Blake3Hash([0; 32]),
    };
    
    let hash2 = ev2.calculate_hash();
    let mut ev2_hashed = ev2;
    ev2_hashed.hash = hash2;

    // Check monotonic transition from ev1 to ev2
    assert!(ev1_hashed.validate_transition(&ev2_hashed).is_ok());

    // Fails on non-monotonic transition (e.g. going back to bottom)
    let ev_bottom = Evidence {
        payload,
        state,
        witness: bottom,
        epoch: 3,
        signature: ev2_hashed.signature.clone(),
        hash: Blake3Hash([0; 32]),
    };
    assert!(ev1_hashed.validate_transition(&ev_bottom).is_err());
}

#[test]
fn test_petri_net_soundness_solver() {
    use wasm4pm::petri::PetriNet;
    use std::collections::{BTreeMap, BTreeSet};

    let make_arc = |place: &str, weight: u32| {
        let mut map = BTreeMap::new();
        map.insert(place.to_string(), weight);
        map
    };

    // Case 1: Sound WF-net
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
    assert!(result_sound.dead_transitions.is_empty());
    assert!(result_sound.proper_completion);
    assert!(result_sound.option_to_complete);
    assert!(!result_sound.state_limit_exceeded);

    // Case 2: Unsound WF-net (Deadlock and Dead Transition)
    let places_deadlock: BTreeSet<String> = vec!["source", "p1", "p2", "sink"].into_iter().map(String::from).collect();
    let transitions_deadlock: BTreeSet<String> = vec!["t1", "t2", "t3"].into_iter().map(String::from).collect();

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

    let net_deadlock = PetriNet::new(places_deadlock, transitions_deadlock, pre_dl, post_dl);
    let result_deadlock = net_deadlock.analyze_soundness();

    assert!(result_deadlock.is_wf_net);
    assert_eq!(result_deadlock.source_place, Some("source".to_string()));
    assert_eq!(result_deadlock.sink_place, Some("sink".to_string()));
    assert!(result_deadlock.has_deadlock);
    assert!(result_deadlock.dead_transitions.contains("t3"));
    assert!(!result_deadlock.option_to_complete);
    assert!(!result_deadlock.state_limit_exceeded);

    // Case 3: Unbounded Petri net
    let places_unbounded: BTreeSet<String> = vec!["source", "p1", "sink"].into_iter().map(String::from).collect();
    let transitions_unbounded: BTreeSet<String> = vec!["t1", "t2"].into_iter().map(String::from).collect();

    let mut pre_ub = BTreeMap::new();
    pre_ub.insert("t1".to_string(), make_arc("source", 1));
    pre_ub.insert("t2".to_string(), make_arc("p1", 1));

    let mut post_ub = BTreeMap::new();
    let mut t1_outputs = BTreeMap::new();
    t1_outputs.insert("p1".to_string(), 1);
    t1_outputs.insert("sink".to_string(), 1);
    post_ub.insert("t1".to_string(), t1_outputs);

    let mut t2_outputs = BTreeMap::new();
    t2_outputs.insert("p1".to_string(), 2);
    t2_outputs.insert("sink".to_string(), 1);
    post_ub.insert("t2".to_string(), t2_outputs);

    let net_unbounded = PetriNet::new(places_unbounded, transitions_unbounded, pre_ub, post_ub);
    let result_unbounded = net_unbounded.analyze_soundness();

    assert!(result_unbounded.is_wf_net);
    assert!(!result_unbounded.is_1_bounded);
    assert!(!result_unbounded.state_limit_exceeded);

    // Case 4: Net with state space limit exceeded (potential state space explosion)
    // 14 parallel places and transitions produce 2^14 = 16,384 states which exceeds MAX_STATES = 10,000.
    let mut places_exp = BTreeSet::new();
    places_exp.insert("source".to_string());
    places_exp.insert("sink".to_string());
    for i in 1..=14 {
        places_exp.insert(format!("p{}", i));
        places_exp.insert(format!("q{}", i));
    }

    let mut transitions_exp = BTreeSet::new();
    transitions_exp.insert("t_split".to_string());
    transitions_exp.insert("t_join".to_string());
    for i in 1..=14 {
        transitions_exp.insert(format!("t{}", i));
    }

    let mut pre_exp = BTreeMap::new();
    pre_exp.insert("t_split".to_string(), make_arc("source", 1));
    
    let mut join_inputs = BTreeMap::new();
    for i in 1..=14 {
        pre_exp.insert(format!("t{}", i), make_arc(&format!("p{}", i), 1));
        join_inputs.insert(format!("q{}", i), 1);
    }
    pre_exp.insert("t_join".to_string(), join_inputs);

    let mut post_exp = BTreeMap::new();
    let mut split_outputs = BTreeMap::new();
    for i in 1..=14 {
        split_outputs.insert(format!("p{}", i), 1);
        post_exp.insert(format!("t{}", i), make_arc(&format!("q{}", i), 1));
    }
    post_exp.insert("t_split".to_string(), split_outputs);
    post_exp.insert("t_join".to_string(), make_arc("sink", 1));

    let net_exp = PetriNet::new(places_exp, transitions_exp, pre_exp, post_exp);
    let result_exp = net_exp.analyze_soundness();

    assert!(result_exp.is_wf_net);
    assert!(result_exp.state_limit_exceeded);
    assert!(!result_exp.is_1_bounded);
    assert!(result_exp.has_deadlock);
    assert!(!result_exp.option_to_complete);
}

#[test]
fn test_ffi_safety_boundary_checks() {
    let _lock = TEST_MUTEX.lock().unwrap();

    let init_res = allocator::init_global_arena(10 * 1024 * 1024);
    assert!(init_res.is_ok());

    let bounds = allocator::get_arena_boundaries().unwrap();
    let base_ptr = bounds.base_addr as *const u8;
    let trans_start_ptr = (bounds.base_addr + bounds.transient_start) as *const u8;

    use wasm4pm::safety::FfiSafetyChecker;

    // Overflow check
    assert!(FfiSafetyChecker::check_overflow(base_ptr, 100));
    let overflow_ptr = usize::MAX - 10;
    assert!(!FfiSafetyChecker::check_overflow(overflow_ptr as *const u8, 20));

    // Alignment matching
    assert!(FfiSafetyChecker::check_alignment(base_ptr, 8));
    let unaligned_ptr = (bounds.base_addr + 3) as *const u8;
    assert!(!FfiSafetyChecker::check_alignment(unaligned_ptr, 8));
    assert!(FfiSafetyChecker::check_alignment(unaligned_ptr, 1));

    // Arena containment
    assert!(FfiSafetyChecker::check_arena_containment(base_ptr, 100));
    let outside_ptr = 0x1000 as *const u8;
    assert!(!FfiSafetyChecker::check_arena_containment(outside_ptr, 100));

    // Memory partition boundaries
    let inside_perm = base_ptr;
    assert!(FfiSafetyChecker::check_partition_boundaries(inside_perm, bounds.transient_start));
    
    let inside_trans = trans_start_ptr;
    assert!(FfiSafetyChecker::check_partition_boundaries(inside_trans, 100));

    // Crossing the partition boundary
    let crossing_ptr = (bounds.base_addr + bounds.transient_start - 50) as *const u8;
    assert!(!FfiSafetyChecker::check_partition_boundaries(crossing_ptr, 100));

    // Disjointness check
    let ptr1 = base_ptr;
    let len1 = 100;
    let ptr2 = (base_ptr as usize + 100) as *const u8;
    let len2 = 50;
    let ptr3 = (base_ptr as usize + 80) as *const u8;
    let len3 = 50;

    assert!(FfiSafetyChecker::check_disjoint(ptr1, len1, ptr2, len2));
    assert!(!FfiSafetyChecker::check_disjoint(ptr1, len1, ptr3, len3));
}

#[test]
fn test_zero_copy_bitmask_projection() {
    let _lock = TEST_MUTEX.lock().unwrap();
    let buf = build_valid_ocel_buffer();
    let ocel = ZeroCopyOcel::parse(&buf).unwrap();

    let mut dfg_matrix = vec![0u32; 4]; // 2 activities * 2 activities = 4
    let mut last_event_for_object = vec![-1i32; 1]; // 1 object
    let bitmask = vec![3u64]; // bits 0 and 1 set: both event 0 and event 1 are active

    // string table offsets for "create_order" (16) and "approve_order" (32)
    let activity_offsets = vec![16, 32];

    ocel.compute_projected_dfg(
        &bitmask,
        &mut dfg_matrix,
        &activity_offsets,
        &mut last_event_for_object,
    ).unwrap();

    // Expect a transition from create_order (index 0) to approve_order (index 1).
    assert_eq!(dfg_matrix[1], 1);
}

#[test]
fn test_heuristics_miner_noisy_trace_hardening() {
    let _lock = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    // 1. Create a buffer that simulates a corrupted/noisy trace with overflowing offsets
    let mut buf = build_valid_ocel_buffer();

    // We modify events_offset to a huge value to trigger overflow in check_bound
    // events_offset is at bytes 8..12
    buf[8..12].copy_from_slice(&0xFFFFFFFFu32.to_le_bytes());

    // Try parsing
    let res = ZeroCopyOcel::parse(&buf);
    assert!(res.is_err());
    assert_eq!(res.err().unwrap(), wasm4pm::ocel::OcelError::OutOfBounds);

    // 2. Create another buffer where string table offsets are corrupted
    let buf2 = build_valid_ocel_buffer();
    // Parse it first (should be valid)
    let ocel2 = ZeroCopyOcel::parse(&buf2).unwrap();
    // Verify valid parsing
    assert_eq!(ocel2.events_count(), 2);

    // Now let's construct a buffer where index tables have overflowing count/offset
    let mut buf3 = build_valid_ocel_buffer();
    // e2o_offset is at bytes 24..28
    // We make events count large but valid, and e2o array_offset overflowing
    // E2O entry 0 offset is at byte 200..204
    // We set array_offset = u32::MAX - 4, count = 2
    buf3[200..204].copy_from_slice(&(u32::MAX - 4).to_le_bytes());
    buf3[204..208].copy_from_slice(&2u32.to_le_bytes());

    let ocel3 = ZeroCopyOcel::parse(&buf3).unwrap();
    // When we call get_event_objects, it should return OutOfBounds instead of overflowing and returning a slice
    let res3 = ocel3.get_event_objects(0);
    assert_eq!(res3.unwrap_err(), wasm4pm::ocel::OcelError::OutOfBounds);

    // 3. FFI Boundary checks
    let code = ffi::wasm_init(10 * 1024 * 1024);
    assert_eq!(code, 0);

    // Allocate space and write the malformed buffer
    let log_len = buf3.len() as u32;
    let log_offset = ffi::wasm_alloc(log_len);
    assert_ne!(log_offset, 0);
    let log_ptr = allocator::get_absolute_ptr(log_offset).unwrap();
    unsafe {
        std::ptr::copy_nonoverlapping(buf3.as_ptr(), log_ptr, log_len as usize);
    }

    let query_str = "create_order,approve_order,10000";
    let query_len = query_str.len() as u32;
    let query_offset = ffi::wasm_alloc(query_len);
    assert_ne!(query_offset, 0);
    let query_ptr = allocator::get_absolute_ptr(query_offset).unwrap();
    unsafe {
        std::ptr::copy_nonoverlapping(query_str.as_ptr(), query_ptr, query_len as usize);
    }

    // Call wasm_parse_and_query. It should catch the out-of-bounds/overflow and return an error code,
    // rather than panicking or executing out-of-bounds read/write.
    let res_encoded = ffi::wasm_parse_and_query(log_offset, log_len, query_offset, query_len);
    let res_offset = (res_encoded >> 32) as u32;
    let res_len = (res_encoded & 0xFFFFFFFF) as u32;

    assert_eq!(res_offset, 0); // Should fail
    assert_eq!(res_len, sandbox::ERR_QUERY_TIMEOUT); // Error code indicating internal query out of bounds
}

#[test]
fn test_m3_sha512_correctness() {
    use wasm4pm::crypto::Sha512;
    // Test vector 1: Empty string
    let mut hasher = Sha512::new();
    hasher.update(b"");
    let result = hasher.finalize();
    let expected = [
        0xcf, 0x83, 0xe1, 0x35, 0x7e, 0xef, 0xb8, 0xbd, 0xf1, 0x54, 0x28, 0x50, 0xd6, 0x6d, 0x80, 0x07,
        0xd6, 0x20, 0xe4, 0x05, 0x0b, 0x57, 0x15, 0xdc, 0x83, 0xf4, 0xa9, 0x21, 0xd3, 0x6c, 0xe9, 0xce,
        0x47, 0xd0, 0xd1, 0x3c, 0x5d, 0x85, 0xf2, 0xb0, 0xff, 0x83, 0x18, 0xd2, 0x87, 0x7e, 0xec, 0x2f,
        0x63, 0xb9, 0x31, 0xbd, 0x47, 0x41, 0x7a, 0x81, 0xa5, 0x38, 0x32, 0x7a, 0xf9, 0x27, 0xda, 0x3e,
    ];
    assert_eq!(result, expected);

    // Test vector 2: "abc"
    let mut hasher = Sha512::new();
    hasher.update(b"abc");
    let result = hasher.finalize();
    let expected = [
        0xdd, 0xaf, 0x35, 0xa1, 0x93, 0x61, 0x7a, 0xba, 0xcc, 0x41, 0x73, 0x49, 0xae, 0x20, 0x41, 0x31,
        0x12, 0xe6, 0xfa, 0x4e, 0x89, 0xa9, 0x7e, 0xa2, 0x0a, 0x9e, 0xee, 0xe6, 0x4b, 0x55, 0xd3, 0x9a,
        0x21, 0x92, 0x99, 0x2a, 0x27, 0x4f, 0xc1, 0xa8, 0x36, 0xba, 0x3c, 0x23, 0xa3, 0xfe, 0xeb, 0xbd,
        0x45, 0x4d, 0x44, 0x23, 0x64, 0x3c, 0xe8, 0x0e, 0x2a, 0x9a, 0xc9, 0x4f, 0xa5, 0x4c, 0xa4, 0x9f,
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_m3_ed25519_signature_canonical_bytes() {
    use wasm4pm::crypto::{self, verify_ed25519_signature};
    // RFC 8032 vector 1: message is empty
    let pk_hex = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a";
    let sig_hex = "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b";
    
    let mut pk = [0u8; 32];
    for i in 0..32 {
        pk[i] = u8::from_str_radix(&pk_hex[i*2..i*2+2], 16).unwrap();
    }
    let mut sig = [0u8; 64];
    for i in 0..64 {
        sig[i] = u8::from_str_radix(&sig_hex[i*2..i*2+2], 16).unwrap();
    }
    
    assert!(verify_ed25519_signature(&pk, &sig, &[]));

    // Test JCS canonicalization and signature verification
    let raw_json_1 = r#"{
      "slide_id": "8c83e135-7eef-b8bd-f154-2850d66d8007",
      "slide_title": "EBITDA",
      "assertion_text": "conforms",
      "target_log_hash": "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce",
      "process_model_hash": "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a",
      "query_definition": "create_order",
      "verification_results": {
        "status": "verified"
      },
      "validator_signature": "e64662bc41e52be887b4b40c14e367c11fc25b725e0ae6472b39a91342e66e69b4c7de0fcd3e8496a86140bca869f3deec2801b62cbe531d3e4f091137513605"
    }"#;

    let raw_json_2 = r#"{"assertion_text":"conforms","process_model_hash":"ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a","query_definition":"create_order","slide_id":"8c83e135-7eef-b8bd-f154-2850d66d8007","slide_title":"EBITDA","target_log_hash":"cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce","validator_signature":"e64662bc41e52be887b4b40c14e367c11fc25b725e0ae6472b39a91342e66e69b4c7de0fcd3e8496a86140bca869f3deec2801b62cbe531d3e4f091137513605","verification_results":{"status":"verified"}}"#;

    let parsed1 = crypto::parse_json(raw_json_1).unwrap();
    let parsed2 = crypto::parse_json(raw_json_2).unwrap();

    let mut map1 = match parsed1 {
        crypto::JsonValue::Object(m) => m,
        _ => panic!("Expected object"),
    };
    let mut map2 = match parsed2 {
        crypto::JsonValue::Object(m) => m,
        _ => panic!("Expected object"),
    };

    map1.remove("validator_signature");
    map2.remove("validator_signature");

    let jcs1 = crypto::JsonValue::Object(map1).to_jcs();
    let jcs2 = crypto::JsonValue::Object(map2).to_jcs();

    assert_eq!(jcs1, jcs2);

    let canonical_sig_hex = "e64662bc41e52be887b4b40c14e367c11fc25b725e0ae6472b39a91342e66e69b4c7de0fcd3e8496a86140bca869f3deec2801b62cbe531d3e4f091137513605";
    let mut canonical_sig = [0u8; 64];
    for i in 0..64 {
        canonical_sig[i] = u8::from_str_radix(&canonical_sig_hex[i*2..i*2+2], 16).unwrap();
    }
    assert!(crypto::verify_jcs_receipt_signature(&pk, &canonical_sig, raw_json_1));
}

#[test]
fn test_m3_typestate_segregation() {
    use wasm4pm::controllers::{ProcessController, GovToken};

    let governor_pk = [1u8; 32];
    let controller = ProcessController::new(governor_pk);

    // Initial state is Init. Let's transition to Active
    let mut active_controller = controller.transition_active();

    // Verify elastic transitions compile and execute without any GovToken
    active_controller.adjust_queue_capacity(250);
    assert_eq!(active_controller.queue_capacity, 250);

    active_controller.set_log_level("DEBUG");
    assert_eq!(active_controller.log_level, "DEBUG");

    // Compliance transition: Quarantine with invalid token should fail
    let mock_sig = [0u8; 64];
    let token_opt = GovToken::verify(&governor_pk, &mock_sig, b"quarantine_me");
    assert!(token_opt.is_none());

    // Construct a correct signature for dummy message (empty message for test_pk)
    let test_pk_hex = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a";
    let test_sig_hex = "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b";
    let mut test_pk = [0u8; 32];
    for i in 0..32 {
        test_pk[i] = u8::from_str_radix(&test_pk_hex[i*2..i*2+2], 16).unwrap();
    }
    let mut test_sig = [0u8; 64];
    for i in 0..64 {
        test_sig[i] = u8::from_str_radix(&test_sig_hex[i*2..i*2+2], 16).unwrap();
    }

    let valid_token = GovToken::verify(&test_pk, &test_sig, &[]).unwrap();

    // Controller initialized with the same test_pk
    let mut active_controller_with_test_pk = ProcessController::new(test_pk).transition_active();
    active_controller_with_test_pk.adjust_queue_capacity(250);

    // Compliance transitions (require GovToken)
    let quarantined_controller = active_controller_with_test_pk.transition_quarantine(&valid_token).unwrap();
    assert_eq!(quarantined_controller.queue_capacity, 250);

    // Transition to decommissioned (require GovToken)
    let decommissioned_controller = quarantined_controller.transition_decommission(&valid_token).unwrap();
    assert_eq!(decommissioned_controller.queue_capacity, 0);
}

#[test]
fn test_m3_memory_scrubbing_wiped() {
    let _lock = TEST_MUTEX.lock().unwrap();
    
    allocator::init_global_arena(1024).unwrap();
    allocator::fill_global_arena_raw_buffer(0xFF);
    
    let raw_buf_before = allocator::get_global_arena_raw_buffer();
    assert_eq!(raw_buf_before[0], 0xFF);

    let seed = [0u8; 32];
    sandbox::execute_oblivion_protocol(&seed);

    let raw_buf_after = allocator::get_global_arena_raw_buffer();

    for &val in &raw_buf_after {
        assert_eq!(val, 0, "Memory was not completely zeroed out");
    }
}

#[test]
fn test_zero_copy_multi_perspective_dfg() {
    let _lock = TEST_MUTEX.lock().unwrap();
    let buf = build_valid_ocel_buffer();
    let ocel = ZeroCopyOcel::parse(&buf).unwrap();

    let mut dfg_matrix = vec![0u32; 4]; // 2 activities * 2 activities = 4
    let mut last_event_for_object = vec![-1i32; 1]; // 1 object
    let bitmask = vec![3u64]; // bits 0 and 1 set: both event 0 and event 1 are active

    // string table offsets for "create_order" (16) and "approve_order" (32)
    let activity_offsets = vec![16, 32];

    // Compute multi-perspective DFG for object type "Order"
    ocel.compute_multi_perspective_dfg(
        &bitmask,
        "Order",
        &mut dfg_matrix,
        &activity_offsets,
        &mut last_event_for_object,
    ).unwrap();

    // Expect a transition from create_order (index 0) to approve_order (index 1) for Order type.
    assert_eq!(dfg_matrix[1], 1);

    // Compute multi-perspective DFG for object type "Item" (which doesn't exist/has no events)
    let mut dfg_matrix_item = vec![0u32; 4];
    let mut last_event_for_object_item = vec![-1i32; 1];
    ocel.compute_multi_perspective_dfg(
        &bitmask,
        "Item",
        &mut dfg_matrix_item,
        &activity_offsets,
        &mut last_event_for_object_item,
    ).unwrap();

    // Expect 0 transitions because there are no Item objects.
    assert_eq!(dfg_matrix_item[1], 0);
}


