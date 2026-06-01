use crate::allocator;
use crate::ocel::ZeroCopyOcel;
use crate::query::{self, OcpqQuery};
use crate::sandbox::{self, GasMeter, RecursionGuard};
use crate::crypto::Sha256;
use std::alloc::Layout;
use std::sync::Mutex;

// Global error tracking
static LAST_ERROR: Mutex<u32> = Mutex::new(0);

fn set_last_error(code: u32) {
    let mut guard = LAST_ERROR.lock().unwrap_or_else(|e| e.into_inner());
    *guard = code;
}

#[no_mangle]
pub extern "C" fn wasm_get_last_error() -> u32 {
    let result = std::panic::catch_unwind(|| {
        let guard = LAST_ERROR.lock().unwrap_or_else(|e| e.into_inner());
        *guard
    });
    result.unwrap_or(0)
}

// 1. Initialize the global arena allocator with the memory ceiling
#[no_mangle]
pub extern "C" fn wasm_init(ceiling: u32) -> u32 {
    let result = std::panic::catch_unwind(|| {
        set_last_error(0);
        match allocator::init_global_arena(ceiling as usize) {
            Ok(_) => 0,
            Err(e) => {
                let err_code = match e {
                    allocator::AllocError::CeilingExceeded => sandbox::ERR_LIFECYCLE_VIOLATION,
                    _ => sandbox::ERR_LIFECYCLE_VIOLATION,
                };
                set_last_error(err_code);
                err_code
            }
        }
    });
    result.unwrap_or_else(|_| {
        set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
        sandbox::ERR_LIFECYCLE_VIOLATION
    })
}

// 2. Allocates memory in the global arena. Returns offset or 0 on failure.
#[no_mangle]
pub extern "C" fn wasm_alloc(len: u32) -> u32 {
    let result = std::panic::catch_unwind(|| {
        set_last_error(0);
        // Enforce alignment to 8 bytes for safety
        let layout = match Layout::from_size_align(len as usize, 8) {
            Ok(l) => l,
            Err(_) => {
                set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
                return 0;
            }
        };
        match allocator::alloc_permanent(layout) {
            Ok(ptr) => allocator::get_relative_offset(ptr).unwrap_or(0),
            Err(e) => {
                let code = match e {
                    allocator::AllocError::OutOfMemory => sandbox::ERR_LIFECYCLE_VIOLATION,
                    _ => sandbox::ERR_LIFECYCLE_VIOLATION,
                };
                set_last_error(code);
                0
            }
        }
    });
    result.unwrap_or_else(|_| {
        set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
        0
    })
}

// Helper to encode (offset, length) into u64
fn encode_slice(offset: u32, len: u32) -> u64 {
    ((offset as u64) << 32) | (len as u64)
}

// 3. Executes Object-Centric Process Query (OCPQ) on the zero-copy log
// Returns encoded u64 (upper 32-bit offset, lower 32-bit length) of the output JSON receipt.
// If failed, returns a 64-bit value where the offset is 0, and the length is the error code.
#[no_mangle]
pub extern "C" fn wasm_parse_and_query(
    log_offset: u32,
    log_len: u32,
    query_offset: u32,
    query_len: u32,
) -> u64 {
    let result = std::panic::catch_unwind(|| {
        set_last_error(0);

        // Fetch inputs from relative offsets
        let log_ptr = match allocator::get_absolute_ptr(log_offset) {
            Some(p) => p,
            None => {
                set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
                return encode_slice(0, sandbox::ERR_LIFECYCLE_VIOLATION);
            }
        };
        let query_ptr = match allocator::get_absolute_ptr(query_offset) {
            Some(p) => p,
            None => {
                set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
                return encode_slice(0, sandbox::ERR_LIFECYCLE_VIOLATION);
            }
        };

        // Validate pointer bounds in linear memory using FfiSafetyChecker
        if !crate::safety::FfiSafetyChecker::check_slice(log_ptr, log_len as usize, 1)
            || !crate::safety::FfiSafetyChecker::check_slice(query_ptr, query_len as usize, 1)
            || !crate::safety::FfiSafetyChecker::check_disjoint(log_ptr, log_len as usize, query_ptr, query_len as usize)
        {
            set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
            return encode_slice(0, sandbox::ERR_LIFECYCLE_VIOLATION);
        }

        // Convert to safe Rust slices
        let log_slice = unsafe { std::slice::from_raw_parts(log_ptr, log_len as usize) };
        let query_slice = unsafe { std::slice::from_raw_parts(query_ptr, query_len as usize) };

        let query_str = match std::str::from_utf8(query_slice) {
            Ok(s) => s,
            Err(_) => {
                set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
                return encode_slice(0, sandbox::ERR_LIFECYCLE_VIOLATION);
            }
        };

        // Parse query DSL
        let query_ast = match OcpqQuery::parse(query_str) {
            Ok(ast) => ast,
            Err(_) => {
                set_last_error(sandbox::ERR_CONFORMANCE_VIOLATION);
                return encode_slice(0, sandbox::ERR_CONFORMANCE_VIOLATION);
            }
        };

        // Compute hashes
        let mut log_hasher = Sha256::new();
        log_hasher.update(log_slice);
        let log_hash = log_hasher.finalize();

        let mut query_hasher = Sha256::new();
        query_hasher.update(query_slice);
        let query_hash = query_hasher.finalize();

        // Parse Zero-Copy OCEL 2.0 log
        let ocel = match ZeroCopyOcel::parse(log_slice) {
            Ok(o) => o,
            Err(_) => {
                set_last_error(sandbox::ERR_CONFORMANCE_VIOLATION);
                return encode_slice(0, sandbox::ERR_CONFORMANCE_VIOLATION);
            }
        };

        // Setup runtime sandbox constraints
        let mut gas_meter = GasMeter::new(10_000_000); // 10M cycle limit
        let mut recursion_guard = RecursionGuard::new(100); // 100 max depth

        // Execute query
        let query_res = match query::execute_ocpq_query(&ocel, &query_ast, &mut gas_meter, &mut recursion_guard) {
            Ok(res) => res,
            Err(err_code) => {
                set_last_error(err_code);
                return encode_slice(0, err_code);
            }
        };

        // Generate JSON receipt
        let receipt_str = format_query_receipt(
            &log_hash,
            &query_hash,
            gas_meter.consumed(),
            query_res.match_count,
            &query_res.matches,
        );

        // Allocate result in transient space
        let receipt_len = receipt_str.len();
        let layout = match Layout::from_size_align(receipt_len, 1) {
            Ok(l) => l,
            Err(_) => {
                set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
                return encode_slice(0, sandbox::ERR_LIFECYCLE_VIOLATION);
            }
        };

        let result_ptr = match allocator::alloc_transient(layout) {
            Ok(p) => p,
            Err(_) => {
                set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
                return encode_slice(0, sandbox::ERR_LIFECYCLE_VIOLATION);
            }
        };

        // Copy JSON receipt to linear memory
        unsafe {
            std::ptr::copy_nonoverlapping(receipt_str.as_ptr(), result_ptr, receipt_len);
        }

        let result_offset = allocator::get_relative_offset(result_ptr).unwrap_or(0);
        encode_slice(result_offset, receipt_len as u32)
    });

    result.unwrap_or_else(|_| {
        set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
        encode_slice(0, sandbox::ERR_LIFECYCLE_VIOLATION)
    })
}

// 4. Shred the heap using the Oblivion Protocol
#[no_mangle]
pub extern "C" fn wasm_shred_heap(seed_offset: u32) -> u32 {
    let result = std::panic::catch_unwind(|| {
        set_last_error(0);
        let seed_ptr = match allocator::get_absolute_ptr(seed_offset) {
            Some(p) => p,
            None => {
                set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
                return sandbox::ERR_LIFECYCLE_VIOLATION;
            }
        };
        if !crate::safety::FfiSafetyChecker::check_slice(seed_ptr, 32, 1) {
            set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
            return sandbox::ERR_LIFECYCLE_VIOLATION;
        }

        let seed = unsafe { &*(seed_ptr as *const [u8; 32]) };
        sandbox::execute_oblivion_protocol(seed);
        0
    });
    result.unwrap_or_else(|_| {
        set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
        sandbox::ERR_LIFECYCLE_VIOLATION
    })
}

// Helper to format the query receipt
fn format_query_receipt(
    log_hash: &[u8; 32],
    query_hash: &[u8; 32],
    cycles_consumed: u64,
    matches_count: u32,
    matches: &[query::OcpqMatch],
) -> String {
    let log_hash_hex = hex_encode(log_hash);
    let query_hash_hex = hex_encode(query_hash);
    
    let mut matches_json = String::new();
    matches_json.push('[');
    for (i, m) in matches.iter().enumerate() {
        if i > 0 {
            matches_json.push(',');
        }
        matches_json.push_str(&format!(
            "{{\"event_1_id\":\"{}\",\"event_2_id\":\"{}\",\"object_id\":\"{}\",\"duration_us\":{}}}",
            m.event_1_id.replace("\"", "\\\""),
            m.event_2_id.replace("\"", "\\\""),
            m.object_id.replace("\"", "\\\""),
            m.duration_us
        ));
    }
    matches_json.push(']');

    format!(
        "{{\n  \"source_log_hash\": \"{}\",\n  \"query_ast_hash\": \"{}\",\n  \"execution_metadata\": {{\n    \"cycles_consumed\": {},\n    \"engine_version\": \"30.1.2\"\n  }},\n  \"query_results_summary\": {{\n    \"match_count\": {},\n    \"matches\": {}\n  }}\n}}",
        log_hash_hex, query_hash_hex, cycles_consumed, matches_count, matches_json
    )
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

// 5. Verifies trace telemetry using the OTel-BLAKE3 event chain.
// Returns 0 if verification is successful. Returns error code if validation fails.
#[no_mangle]
pub extern "C" fn wasm_verify_otel_trace(trace_offset: u32, trace_len: u32) -> u32 {
    let result = std::panic::catch_unwind(|| {
        set_last_error(0);
        let trace_ptr = match allocator::get_absolute_ptr(trace_offset) {
            Some(p) => p,
            None => {
                set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
                return sandbox::ERR_LIFECYCLE_VIOLATION;
            }
        };
        if !crate::safety::FfiSafetyChecker::check_slice(trace_ptr, trace_len as usize, 1) {
            set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
            return sandbox::ERR_LIFECYCLE_VIOLATION;
        }

        let trace_slice = unsafe { std::slice::from_raw_parts(trace_ptr, trace_len as usize) };
        let trace_str = match std::str::from_utf8(trace_slice) {
            Ok(s) => s,
            Err(_) => {
                set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
                return sandbox::ERR_LIFECYCLE_VIOLATION;
            }
        };

        let parsed_trace = match crate::otel::OtelTrace::parse_from_str(trace_str) {
            Ok(t) => t,
            Err(_) => {
                set_last_error(sandbox::ERR_REPLAY_ATTESTATION);
                return sandbox::ERR_REPLAY_ATTESTATION;
            }
        };

        match crate::otel::verify_otel_trace(&parsed_trace) {
            Ok(true) => 0,
            _ => {
                set_last_error(sandbox::ERR_REPLAY_ATTESTATION);
                sandbox::ERR_REPLAY_ATTESTATION
            }
        }
    });

    result.unwrap_or_else(|_| {
        set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
        sandbox::ERR_LIFECYCLE_VIOLATION
    })
}

// 6. Decommission the process using GovToken
#[no_mangle]
pub extern "C" fn wasm_decommission_process(
    gov_pk_offset: u32,
    sig_offset: u32,
    msg_offset: u32,
    msg_len: u32,
) -> u32 {
    let result = std::panic::catch_unwind(|| {
        set_last_error(0);
        let pk_ptr = match allocator::get_absolute_ptr(gov_pk_offset) {
            Some(p) => p,
            None => {
                set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
                return sandbox::ERR_LIFECYCLE_VIOLATION;
            }
        };
        let sig_ptr = match allocator::get_absolute_ptr(sig_offset) {
            Some(p) => p,
            None => {
                set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
                return sandbox::ERR_LIFECYCLE_VIOLATION;
            }
        };
        let msg_ptr = match allocator::get_absolute_ptr(msg_offset) {
            Some(p) => p,
            None => {
                set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
                return sandbox::ERR_LIFECYCLE_VIOLATION;
            }
        };

        if !crate::safety::FfiSafetyChecker::check_slice(pk_ptr, 32, 1)
            || !crate::safety::FfiSafetyChecker::check_slice(sig_ptr, 64, 1)
            || !crate::safety::FfiSafetyChecker::check_slice(msg_ptr, msg_len as usize, 1)
        {
            set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
            return sandbox::ERR_LIFECYCLE_VIOLATION;
        }

        let pk = unsafe { &*(pk_ptr as *const [u8; 32]) };
        let sig = unsafe { &*(sig_ptr as *const [u8; 64]) };
        let msg = unsafe { std::slice::from_raw_parts(msg_ptr, msg_len as usize) };

        // Verify signature and create GovToken
        let token = match crate::controllers::GovToken::verify(pk, sig, msg) {
            Some(t) => t,
            None => {
                set_last_error(sandbox::ERR_CONFORMANCE_VIOLATION);
                return sandbox::ERR_CONFORMANCE_VIOLATION;
            }
        };

        // Initialize and run transition
        let controller = crate::controllers::ProcessController::new(*pk);
        let active = controller.transition_active();
        if active.transition_decommission(&token).is_some() {
            0
        } else {
            set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
            sandbox::ERR_LIFECYCLE_VIOLATION
        }
    });

    result.unwrap_or_else(|_| {
        set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
        sandbox::ERR_LIFECYCLE_VIOLATION
    })
}

// 7. Verify JCS canonicalized signature
#[no_mangle]
pub extern "C" fn wasm_verify_jcs_signature(
    gov_pk_offset: u32,
    sig_offset: u32,
    json_offset: u32,
    json_len: u32,
) -> u32 {
    let result = std::panic::catch_unwind(|| {
        set_last_error(0);
        let pk_ptr = match allocator::get_absolute_ptr(gov_pk_offset) {
            Some(p) => p,
            None => {
                set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
                return sandbox::ERR_LIFECYCLE_VIOLATION;
            }
        };
        let sig_ptr = match allocator::get_absolute_ptr(sig_offset) {
            Some(p) => p,
            None => {
                set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
                return sandbox::ERR_LIFECYCLE_VIOLATION;
            }
        };
        let json_ptr = match allocator::get_absolute_ptr(json_offset) {
            Some(p) => p,
            None => {
                set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
                return sandbox::ERR_LIFECYCLE_VIOLATION;
            }
        };

        if !crate::safety::FfiSafetyChecker::check_slice(pk_ptr, 32, 1)
            || !crate::safety::FfiSafetyChecker::check_slice(sig_ptr, 64, 1)
            || !crate::safety::FfiSafetyChecker::check_slice(json_ptr, json_len as usize, 1)
        {
            set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
            return sandbox::ERR_LIFECYCLE_VIOLATION;
        }

        let pk = unsafe { &*(pk_ptr as *const [u8; 32]) };
        let sig = unsafe { &*(sig_ptr as *const [u8; 64]) };
        let json_slice = unsafe { std::slice::from_raw_parts(json_ptr, json_len as usize) };
        
        let json_str = match std::str::from_utf8(json_slice) {
            Ok(s) => s,
            Err(_) => {
                set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
                return sandbox::ERR_LIFECYCLE_VIOLATION;
            }
        };

        if crate::crypto::verify_jcs_receipt_signature(pk, sig, json_str) {
            0
        } else {
            set_last_error(sandbox::ERR_CONFORMANCE_VIOLATION);
            sandbox::ERR_CONFORMANCE_VIOLATION
        }
    });

    result.unwrap_or_else(|_| {
        set_last_error(sandbox::ERR_LIFECYCLE_VIOLATION);
        sandbox::ERR_LIFECYCLE_VIOLATION
    })
}


