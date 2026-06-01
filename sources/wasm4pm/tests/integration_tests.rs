use wasm4pm::allocator;
use wasm4pm::ocel::ZeroCopyOcel;
use wasm4pm::query::{self, OcpqQuery};
use wasm4pm::sandbox::{self, GasMeter, RecursionGuard};
use wasm4pm::ffi;

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
