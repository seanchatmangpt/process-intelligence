//! wasm4pm: High-performance cryptographically sandboxed WASM execution engine for process intelligence
//!
//! This crate implements process discovery and conformance checking algorithms with:
//! - Graduated intake from wasm4pm-compat (manufacturing & type law validation)
//! - Cryptographic receipts and evidence chains
//! - Deterministic process mining with witness markers

// Re-export key types from wasm4pm-compat for bridge integration
pub use wasm4pm_compat::{
    GraduateToWasm4pm, GraduationCandidate, GraduationReason,
    TypedPowl, TreeProjectable, TreeProjection, OperatorKind, PowlNode,
};

// Re-export the bridge intake function
pub use graduation::accept_from_compat;

// Re-export mining types
pub use mining::{PowerMiner, PowerWitness};

pub mod allocator;
pub mod crypto;
pub mod ocel;
pub mod query;
pub mod ocpq_evaluator;
pub mod sandbox;
pub mod ffi;
pub mod otel;
pub mod evidence;
pub mod petri;
pub mod safety;
pub mod replay;
pub mod conformance;
pub mod mining;
pub mod zeroize;
pub mod controllers;
pub mod ltl;
pub mod ocel_v2;
pub mod refusal;

pub mod graduation;

