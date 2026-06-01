use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use crate::crypto::Sha256;

// =========================================================================
// 1. Evidence Generic Container and Serialization
// =========================================================================

/// Cryptographic wrapper binding authority and signature bytes
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdentitySignature {
    pub public_key: Vec<u8>,
    pub signature_bytes: Vec<u8>,
}

/// BLAKE3 digest wrapper (holding 32-byte hash)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Blake3Hash(pub [u8; 32]);

impl Blake3Hash {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// Errors returned during evidence validation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvidenceError {
    HashMismatch,
    InvalidSignature,
    InvalidTransition,
    LatticeViolation,
}

/// Helper trait for serialization without serde
pub trait SerializeBytes {
    fn serialize_bytes(&self, buf: &mut Vec<u8>);
}

/// Primitive implementations of SerializeBytes
impl SerializeBytes for u64 {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_le_bytes());
    }
}

impl SerializeBytes for u32 {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_le_bytes());
    }
}

impl SerializeBytes for usize {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&(*self as u64).to_le_bytes());
    }
}

impl SerializeBytes for Vec<u8> {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self);
    }
}

impl SerializeBytes for &[u8] {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self);
    }
}

impl SerializeBytes for String {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self.as_bytes());
    }
}

impl SerializeBytes for &str {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self.as_bytes());
    }
}

impl SerializeBytes for () {
    fn serialize_bytes(&self, _buf: &mut Vec<u8>) {}
}

/// The core Evidence container
#[derive(Clone, Debug)]
pub struct Evidence<T, State, Witness> {
    pub payload: T,
    pub state: State,
    pub witness: Witness,
    pub epoch: u64,
    pub signature: IdentitySignature,
    pub hash: Blake3Hash,
}

impl<T, State, Witness> Evidence<T, State, Witness>
where
    T: SerializeBytes,
    State: SerializeBytes,
    Witness: SerializeBytes + Lattice,
{
    /// Calculate the SHA-256 hash of the Evidence fields (wrapped in Blake3Hash)
    pub fn calculate_hash(&self) -> Blake3Hash {
        let mut hasher = Sha256::new();
        
        let mut buf = Vec::new();
        self.payload.serialize_bytes(&mut buf);
        hasher.update(&buf);
        
        buf.clear();
        self.state.serialize_bytes(&mut buf);
        hasher.update(&buf);
        
        buf.clear();
        self.witness.serialize_bytes(&mut buf);
        hasher.update(&buf);
        
        hasher.update(&self.epoch.to_le_bytes());
        
        buf.clear();
        buf.extend_from_slice(&self.signature.public_key);
        buf.extend_from_slice(&self.signature.signature_bytes);
        hasher.update(&buf);
        
        Blake3Hash(hasher.finalize())
    }

    /// Perform full self-validation checks on the evidence block
    pub fn validate(&self, expected_public_key: &[u8; 32]) -> Result<(), EvidenceError> {
        // 1. Verify cryptographic binding
        let computed_hash = self.calculate_hash();
        if computed_hash != self.hash {
            return Err(EvidenceError::HashMismatch);
        }
        
        // 2. Verify signature using pure-Rust Ed25519 engine
        if self.signature.public_key.len() != 32 || self.signature.signature_bytes.len() != 64 {
            return Err(EvidenceError::InvalidSignature);
        }
        let pk: &[u8; 32] = self.signature.public_key.as_slice().try_into().unwrap();
        let sig: &[u8; 64] = self.signature.signature_bytes.as_slice().try_into().unwrap();
        
        // Validate signature matches expected public key
        if pk != expected_public_key {
            return Err(EvidenceError::InvalidSignature);
        }
        
        if !verify_ed25519_signature(pk, sig, computed_hash.as_bytes()) {
            return Err(EvidenceError::InvalidSignature);
        }

        Ok(())
    }

    /// Validate chronological progression of sequential evidence blocks (Lattice Monotonicity)
    pub fn validate_transition(&self, next: &Self) -> Result<(), EvidenceError> {
        let joined = self.witness.join(&next.witness);
        if joined.is_top() || joined != next.witness {
            return Err(EvidenceError::LatticeViolation);
        }
        Ok(())
    }
}

// =========================================================================
// 2. Information Lattices
// =========================================================================

pub trait Lattice: Sized + Eq + Clone {
    /// Return the bottom element (no evidence / initial state)
    fn bottom() -> Self;

    /// Return the top element (conflict / contradiction state)
    fn top() -> Self;

    /// Check if the element is top
    fn is_top(&self) -> bool;

    /// Check if the element is bottom
    fn is_bottom(&self) -> bool;

    /// Join two lattice elements (least upper bound)
    fn join(&self, other: &Self) -> Self;

    /// Compare two elements in the partial order
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>;
}

/// WitnessState enum tracks Petri net token game alignments, cost, and replayed event trace indices.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WitnessState {
    Bottom,
    PartialReplay {
        trace_indices: Vec<usize>, // Sorted unique event indices replayed
        marking: Vec<String>,       // Petri Net place markings
        cost: u32,                  // Alignment moves cost
    },
    Top,
}

impl Lattice for WitnessState {
    fn bottom() -> Self { WitnessState::Bottom }
    fn top() -> Self { WitnessState::Top }
    fn is_top(&self) -> bool { matches!(self, WitnessState::Top) }
    fn is_bottom(&self) -> bool { matches!(self, WitnessState::Bottom) }

    fn join(&self, other: &Self) -> Self {
        match (self, other) {
            (WitnessState::Top, _) | (_, WitnessState::Top) => WitnessState::Top,
            (WitnessState::Bottom, any) | (any, WitnessState::Bottom) => any.clone(),
            (WitnessState::PartialReplay { trace_indices: t1, marking: m1, cost: c1 },
             WitnessState::PartialReplay { trace_indices: t2, marking: m2, cost: c2 }) => {
                if t1 == t2 && m1 == m2 && c1 == c2 {
                    return self.clone();
                }

                if let Some(ord) = self.partial_cmp(other) {
                    match ord {
                        Ordering::Less | Ordering::Equal => return other.clone(),
                        Ordering::Greater => return self.clone(),
                    }
                }

                // Disjoint checks for indices (overlap represents conflicting claims on same event)
                let has_overlap = t1.iter().any(|idx| t2.contains(idx));
                if has_overlap {
                    WitnessState::Top
                } else {
                    let mut merged_indices = t1.clone();
                    merged_indices.extend(t2.iter().copied());
                    merged_indices.sort_unstable();
                    merged_indices.dedup();

                    let mut merged_marking = m1.clone();
                    for place in m2 {
                        if !merged_marking.contains(place) {
                            merged_marking.push(place.clone());
                        }
                    }
                    merged_marking.sort();

                    WitnessState::PartialReplay {
                        trace_indices: merged_indices,
                        marking: merged_marking,
                        cost: c1 + c2,
                    }
                }
            }
        }
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (WitnessState::Bottom, WitnessState::Bottom) => Some(Ordering::Equal),
            (WitnessState::Bottom, _) => Some(Ordering::Less),
            (_, WitnessState::Bottom) => Some(Ordering::Greater),
            (WitnessState::Top, WitnessState::Top) => Some(Ordering::Equal),
            (WitnessState::Top, _) => Some(Ordering::Greater),
            (_, WitnessState::Top) => Some(Ordering::Less),
            (WitnessState::PartialReplay { trace_indices: t1, marking: m1, cost: c1 },
             WitnessState::PartialReplay { trace_indices: t2, marking: m2, cost: c2 }) => {
                let is_t1_sub = t1.iter().all(|x| t2.contains(x));
                let is_t2_sub = t2.iter().all(|x| t1.contains(x));
                let is_m1_sub = m1.iter().all(|x| m2.contains(x));
                let is_m2_sub = m2.iter().all(|x| m1.contains(x));
                let is_c1_le = c1 <= c2;
                let is_c2_le = c2 <= c1;

                match (is_t1_sub, is_t2_sub, is_m1_sub, is_m2_sub, is_c1_le, is_c2_le) {
                    (true, true, true, true, true, true) => Some(Ordering::Equal),
                    (true, _, true, _, true, _) => Some(Ordering::Less),
                    (_, true, _, true, _, true) => Some(Ordering::Greater),
                    _ => None,
                }
            }
        }
    }
}

impl SerializeBytes for WitnessState {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        match self {
            WitnessState::Bottom => {
                buf.push(0);
            }
            WitnessState::PartialReplay { trace_indices, marking, cost } => {
                buf.push(1);
                buf.extend_from_slice(&(trace_indices.len() as u64).to_le_bytes());
                for &idx in trace_indices {
                    buf.extend_from_slice(&(idx as u64).to_le_bytes());
                }
                buf.extend_from_slice(&(marking.len() as u64).to_le_bytes());
                for m in marking {
                    buf.extend_from_slice(&(m.len() as u64).to_le_bytes());
                    buf.extend_from_slice(m.as_bytes());
                }
                buf.extend_from_slice(&cost.to_le_bytes());
            }
            WitnessState::Top => {
                buf.push(2);
            }
        }
    }
}

/// ConstraintValue representing individual LTLf/Declare constraint satisfaction values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstraintValue {
    Bottom,
    PossiblySatisfied,
    Satisfied,
    Violated,
    Top,
}

impl Lattice for ConstraintValue {
    fn bottom() -> Self { ConstraintValue::Bottom }
    fn top() -> Self { ConstraintValue::Top }
    fn is_top(&self) -> bool { matches!(self, ConstraintValue::Top) }
    fn is_bottom(&self) -> bool { matches!(self, ConstraintValue::Bottom) }

    fn join(&self, other: &Self) -> Self {
        match (self, other) {
            (ConstraintValue::Top, _) | (_, ConstraintValue::Top) => ConstraintValue::Top,
            (ConstraintValue::Bottom, any) | (any, ConstraintValue::Bottom) => any.clone(),
            (ConstraintValue::PossiblySatisfied, any) | (any, ConstraintValue::PossiblySatisfied) => any.clone(),
            (ConstraintValue::Satisfied, ConstraintValue::Satisfied) => ConstraintValue::Satisfied,
            (ConstraintValue::Violated, ConstraintValue::Violated) => ConstraintValue::Violated,
            (ConstraintValue::Satisfied, ConstraintValue::Violated) | 
            (ConstraintValue::Violated, ConstraintValue::Satisfied) => ConstraintValue::Top,
        }
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (ConstraintValue::Bottom, ConstraintValue::Bottom) => Some(Ordering::Equal),
            (ConstraintValue::Bottom, _) => Some(Ordering::Less),
            (_, ConstraintValue::Bottom) => Some(Ordering::Greater),
            (ConstraintValue::Top, ConstraintValue::Top) => Some(Ordering::Equal),
            (ConstraintValue::Top, _) => Some(Ordering::Greater),
            (_, ConstraintValue::Top) => Some(Ordering::Less),
            (ConstraintValue::PossiblySatisfied, ConstraintValue::PossiblySatisfied) => Some(Ordering::Equal),
            (ConstraintValue::PossiblySatisfied, ConstraintValue::Satisfied) |
            (ConstraintValue::PossiblySatisfied, ConstraintValue::Violated) => Some(Ordering::Less),
            (ConstraintValue::Satisfied, ConstraintValue::PossiblySatisfied) |
            (ConstraintValue::Violated, ConstraintValue::PossiblySatisfied) => Some(Ordering::Greater),
            (ConstraintValue::Satisfied, ConstraintValue::Satisfied) => Some(Ordering::Equal),
            (ConstraintValue::Violated, ConstraintValue::Violated) => Some(Ordering::Equal),
            (ConstraintValue::Satisfied, ConstraintValue::Violated) |
            (ConstraintValue::Violated, ConstraintValue::Satisfied) => None,
        }
    }
}

impl SerializeBytes for ConstraintValue {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        let val = match self {
            ConstraintValue::Bottom => 0u8,
            ConstraintValue::PossiblySatisfied => 1u8,
            ConstraintValue::Satisfied => 2u8,
            ConstraintValue::Violated => 3u8,
            ConstraintValue::Top => 4u8,
        };
        buf.push(val);
    }
}

/// DeclareWitnessState maps rule IDs to constraint satisfaction valuations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeclareWitnessState {
    Bottom,
    Evaluated(HashMap<String, ConstraintValue>),
    Top,
}

impl Lattice for DeclareWitnessState {
    fn bottom() -> Self { DeclareWitnessState::Bottom }
    fn top() -> Self { DeclareWitnessState::Top }
    fn is_top(&self) -> bool { matches!(self, DeclareWitnessState::Top) }
    fn is_bottom(&self) -> bool { matches!(self, DeclareWitnessState::Bottom) }

    fn join(&self, other: &Self) -> Self {
        match (self, other) {
            (DeclareWitnessState::Top, _) | (_, DeclareWitnessState::Top) => DeclareWitnessState::Top,
            (DeclareWitnessState::Bottom, any) | (any, DeclareWitnessState::Bottom) => any.clone(),
            (DeclareWitnessState::Evaluated(m1), DeclareWitnessState::Evaluated(m2)) => {
                let mut merged = HashMap::new();
                let keys: HashSet<&String> = m1.keys().chain(m2.keys()).collect();
                for key in keys {
                    let v1 = m1.get(key).unwrap_or(&ConstraintValue::Bottom);
                    let v2 = m2.get(key).unwrap_or(&ConstraintValue::Bottom);
                    let v_joined = v1.join(v2);
                    if v_joined.is_top() {
                        return DeclareWitnessState::Top;
                    }
                    merged.insert(key.clone(), v_joined);
                }
                DeclareWitnessState::Evaluated(merged)
            }
        }
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (DeclareWitnessState::Bottom, DeclareWitnessState::Bottom) => Some(Ordering::Equal),
            (DeclareWitnessState::Bottom, _) => Some(Ordering::Less),
            (_, DeclareWitnessState::Bottom) => Some(Ordering::Greater),
            (DeclareWitnessState::Top, DeclareWitnessState::Top) => Some(Ordering::Equal),
            (DeclareWitnessState::Top, _) => Some(Ordering::Greater),
            (_, DeclareWitnessState::Top) => Some(Ordering::Less),
            (DeclareWitnessState::Evaluated(m1), DeclareWitnessState::Evaluated(m2)) => {
                let mut is_less_or_equal = true;
                let mut is_greater_or_equal = true;

                let all_keys: HashSet<&String> = m1.keys().chain(m2.keys()).collect();
                for key in all_keys {
                    let v1 = m1.get(key).unwrap_or(&ConstraintValue::Bottom);
                    let v2 = m2.get(key).unwrap_or(&ConstraintValue::Bottom);

                    match v1.partial_cmp(v2) {
                        Some(Ordering::Less) => {
                            is_greater_or_equal = false;
                        }
                        Some(Ordering::Greater) => {
                            is_less_or_equal = false;
                        }
                        Some(Ordering::Equal) => {}
                        None => {
                            is_less_or_equal = false;
                            is_greater_or_equal = false;
                        }
                    }
                }

                match (is_less_or_equal, is_greater_or_equal) {
                    (true, true) => Some(Ordering::Equal),
                    (true, false) => Some(Ordering::Less),
                    (false, true) => Some(Ordering::Greater),
                    (false, false) => None,
                }
            }
        }
    }
}

impl SerializeBytes for DeclareWitnessState {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        match self {
            DeclareWitnessState::Bottom => {
                buf.push(0);
            }
            DeclareWitnessState::Evaluated(map) => {
                buf.push(1);
                // Sort keys to ensure deterministic serialization
                let mut keys: Vec<&String> = map.keys().collect();
                keys.sort();
                buf.extend_from_slice(&(keys.len() as u64).to_le_bytes());
                for key in keys {
                    buf.extend_from_slice(&(key.len() as u64).to_le_bytes());
                    buf.extend_from_slice(key.as_bytes());
                    if let Some(val) = map.get(key) {
                        val.serialize_bytes(buf);
                    }
                }
            }
            DeclareWitnessState::Top => {
                buf.push(2);
            }
        }
    }
}

/// UnifiedWitnessState product lattice integrating structural token replay and declarative rules.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnifiedWitnessState {
    Bottom,
    Active {
        replay: WitnessState,
        declare: DeclareWitnessState,
    },
    Top,
}

impl Lattice for UnifiedWitnessState {
    fn bottom() -> Self { UnifiedWitnessState::Bottom }
    fn top() -> Self { UnifiedWitnessState::Top }
    fn is_top(&self) -> bool { matches!(self, UnifiedWitnessState::Top) }
    fn is_bottom(&self) -> bool { matches!(self, UnifiedWitnessState::Bottom) }

    fn join(&self, other: &Self) -> Self {
        match (self, other) {
            (UnifiedWitnessState::Top, _) | (_, UnifiedWitnessState::Top) => UnifiedWitnessState::Top,
            (UnifiedWitnessState::Bottom, any) | (any, UnifiedWitnessState::Bottom) => any.clone(),
            (UnifiedWitnessState::Active { replay: r1, declare: d1 },
             UnifiedWitnessState::Active { replay: r2, declare: d2 }) => {
                let r_joined = r1.join(r2);
                let d_joined = d1.join(d2);
                if r_joined.is_top() || d_joined.is_top() {
                    UnifiedWitnessState::Top
                } else {
                    UnifiedWitnessState::Active {
                        replay: r_joined,
                        declare: d_joined,
                    }
                }
            }
        }
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (UnifiedWitnessState::Bottom, UnifiedWitnessState::Bottom) => Some(Ordering::Equal),
            (UnifiedWitnessState::Bottom, _) => Some(Ordering::Less),
            (_, UnifiedWitnessState::Bottom) => Some(Ordering::Greater),
            (UnifiedWitnessState::Top, UnifiedWitnessState::Top) => Some(Ordering::Equal),
            (UnifiedWitnessState::Top, _) => Some(Ordering::Greater),
            (_, UnifiedWitnessState::Top) => Some(Ordering::Less),
            (UnifiedWitnessState::Active { replay: r1, declare: d1 },
             UnifiedWitnessState::Active { replay: r2, declare: d2 }) => {
                let r_cmp = r1.partial_cmp(r2);
                let d_cmp = d1.partial_cmp(d2);

                match (r_cmp, d_cmp) {
                    (Some(Ordering::Equal), Some(Ordering::Equal)) => Some(Ordering::Equal),
                    (Some(Ordering::Less | Ordering::Equal),
                     Some(Ordering::Less | Ordering::Equal)) => Some(Ordering::Less),
                    (Some(Ordering::Greater | Ordering::Equal),
                     Some(Ordering::Greater | Ordering::Equal)) => Some(Ordering::Greater),
                    _ => None,
                }
            }
        }
    }
}

impl SerializeBytes for UnifiedWitnessState {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        match self {
            UnifiedWitnessState::Bottom => {
                buf.push(0);
            }
            UnifiedWitnessState::Active { replay, declare } => {
                buf.push(1);
                replay.serialize_bytes(buf);
                declare.serialize_bytes(buf);
            }
            UnifiedWitnessState::Top => {
                buf.push(2);
            }
        }
    }
}

// =========================================================================
// 3. Pure-Rust Cryptographic Engine (SHA-512, modular field math, Curve25519)
// =========================================================================

pub struct Sha512 {
    state: [u64; 8],
    buffer: [u8; 128],
    len: u64,
}

impl Sha512 {
    pub fn new() -> Self {
        Self {
            state: [
                0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
                0x510e527fade682d1, 0x9b05688c2b3e6c1f, 0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
            ],
            buffer: [0; 128],
            len: 0,
        }
    }

    pub fn update(&mut self, mut data: &[u8]) {
        let buffer_len = (self.len % 128) as usize;
        self.len += data.len() as u64;

        if buffer_len > 0 {
            let fill = 128 - buffer_len;
            if data.len() >= fill {
                self.buffer[buffer_len..128].copy_from_slice(&data[..fill]);
                let chunk = self.buffer;
                self.transform(&chunk);
                data = &data[fill..];
            } else {
                self.buffer[buffer_len..buffer_len + data.len()].copy_from_slice(data);
                return;
            }
        }

        while data.len() >= 128 {
            let chunk: &[u8; 128] = data[..128].try_into().unwrap();
            self.transform(chunk);
            data = &data[128..];
        }

        if !data.is_empty() {
            self.buffer[..data.len()].copy_from_slice(data);
        }
    }

    pub fn finalize(mut self) -> [u8; 64] {
        let buffer_len = (self.len % 128) as usize;
        self.buffer[buffer_len] = 0x80;
        
        if buffer_len + 1 > 112 {
            self.buffer[buffer_len + 1..128].fill(0);
            let chunk = self.buffer;
            self.transform(&chunk);
            self.buffer[..112].fill(0);
        } else {
            self.buffer[buffer_len + 1..112].fill(0);
        }

        let bits = (self.len * 8).to_be_bytes();
        self.buffer[112..120].copy_from_slice(&[0; 8]);
        self.buffer[120..128].copy_from_slice(&bits);
        let chunk = self.buffer;
        self.transform(&chunk);

        let mut out = [0u8; 64];
        for i in 0..8 {
            out[i * 8..i * 8 + 8].copy_from_slice(&self.state[i].to_be_bytes());
        }
        out
    }

    fn transform(&mut self, chunk: &[u8; 128]) {
        const K: [u64; 80] = [
            0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc,
            0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118,
            0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
            0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694,
            0xe49b69c19ef14ad2, 0xefbe47863fc10196, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
            0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
            0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4,
            0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70,
            0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
            0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
            0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30,
            0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa0703764167b,
            0x19a4c116021c6068, 0x1e376c0819e9e7b4, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8,
            0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f769d65ee, 0x682e6ff3530c5545,
            0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
            0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b,
            0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178,
            0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
            0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c,
            0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817,
        ];

        let mut w = [0u64; 80];
        for i in 0..16 {
            w[i] = u64::from_be_bytes(chunk[i * 8..i * 8 + 8].try_into().unwrap());
        }
        for i in 16..80 {
            let s0 = w[i - 15].rotate_right(1) ^ w[i - 15].rotate_right(8) ^ (w[i - 15] >> 7);
            let s1 = w[i - 2].rotate_right(19) ^ w[i - 2].rotate_right(61) ^ (w[i - 2] >> 6);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];
        let mut f = self.state[5];
        let mut g = self.state[6];
        let mut h = self.state[7];

        for i in 0..80 {
            let s1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
            let ch = (e & f) ^ (!e & g);
            let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
    }
}

/// Modular Field Arithmetic modulo p = 2^255 - 19
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldElement(pub [u64; 4]);

impl FieldElement {
    pub const P: Self = FieldElement([
        0xffffffffffffffed, 0xffffffffffffffff,
        0xffffffffffffffff, 0x7fffffffffffffff
    ]);

    pub fn zero() -> Self { FieldElement([0, 0, 0, 0]) }
    pub fn one() -> Self { FieldElement([1, 0, 0, 0]) }

    pub fn add(self, other: Self) -> Self {
        let mut res = [0u64; 4];
        let mut carry = 0u128;
        for i in 0..4 {
            let sum = (self.0[i] as u128) + (other.0[i] as u128) + carry;
            res[i] = sum as u64;
            carry = sum >> 64;
        }
        if carry > 0 || res_gte_p(res) {
            let mut borrow = 0u128;
            for i in 0..4 {
                let diff = (res[i] as u128) - (Self::P.0[i] as u128) - borrow;
                res[i] = diff as u64;
                borrow = (diff >> 64) & 1;
            }
        }
        FieldElement(res)
    }

    pub fn sub(self, other: Self) -> Self {
        let mut res = [0u64; 4];
        let mut borrow = 0u128;
        for i in 0..4 {
            let diff = (self.0[i] as u128) - (other.0[i] as u128) - borrow;
            res[i] = diff as u64;
            borrow = (diff >> 64) & 1;
        }
        if borrow > 0 {
            let mut carry = 0u128;
            for i in 0..4 {
                let sum = (res[i] as u128) + (Self::P.0[i] as u128) + carry;
                res[i] = sum as u64;
                carry = sum >> 64;
            }
        }
        FieldElement(res)
    }

    pub fn mul(self, other: Self) -> Self {
        let mut product = [0u64; 8];
        for i in 0..4 {
            let mut carry = 0u128;
            for j in 0..4 {
                let val = (product[i + j] as u128) + (self.0[i] as u128) * (other.0[j] as u128) + carry;
                product[i + j] = val as u64;
                carry = val >> 64;
            }
            product[i + 4] = carry as u64;
        }
        reduce_512(product)
    }

    pub fn pow_p_minus_2(self) -> Self {
        let exp: [u64; 4] = [0xffffffffffffffeb, 0xffffffffffffffff, 0xffffffffffffffff, 0x7fffffffffffffff];
        let mut res = FieldElement::one();
        let mut base = self;
        for &word in &exp {
            let mut w = word;
            for _ in 0..64 {
                if w & 1 == 1 {
                    res = res.mul(base);
                }
                base = base.mul(base);
                w >>= 1;
            }
        }
        res
    }
}

fn res_gte_p(res: [u64; 4]) -> bool {
    for i in (0..4).rev() {
        if res[i] > FieldElement::P.0[i] { return true; }
        if res[i] < FieldElement::P.0[i] { return false; }
    }
    true
}

fn reduce_512(product: [u64; 8]) -> FieldElement {
    // Pass 1: split the 512-bit product into lower 255 bits and upper bits,
    // multiply the upper bits by 19, and add.
    let mut high = [0u64; 4];
    let mut carry = (product[3] >> 63) as u128;
    for i in 0..4 {
        let val = product[i + 4] as u128;
        high[i] = ((val << 1) | carry) as u64;
        carry = val >> 63;
    }
    
    let low = [
        product[0],
        product[1],
        product[2],
        product[3] & 0x7fffffffffffffff,
    ];
    
    let mut high_x_19 = [0u64; 5];
    let mut carry_mul = 0u128;
    for i in 0..4 {
        let val = (high[i] as u128) * 19 + carry_mul;
        high_x_19[i] = val as u64;
        carry_mul = val >> 64;
    }
    high_x_19[4] = (carry_mul + carry * 38) as u64; // Handle any remaining overflow bits defensively
    
    let mut sum = [0u64; 5];
    let mut carry_add = 0u128;
    for i in 0..4 {
        let val = (low[i] as u128) + (high_x_19[i] as u128) + carry_add;
        sum[i] = val as u64;
        carry_add = val >> 64;
    }
    sum[4] = (high_x_19[4] as u128 + carry_add) as u64;
    
    // Pass 2: split sum (5 limbs) into lower 255 bits and upper bits,
    // multiply upper bits by 19, and add.
    let sum_high_val = ((sum[3] >> 63) as u128) | ((sum[4] as u128) << 1);
    let sum_high_x_19 = sum_high_val * 19;
    
    let mut final_sum = [0u64; 4];
    final_sum[0] = sum[0];
    final_sum[1] = sum[1];
    final_sum[2] = sum[2];
    final_sum[3] = sum[3] & 0x7fffffffffffffff;
    
    let mut carry_final = sum_high_x_19;
    for i in 0..4 {
        let val = (final_sum[i] as u128) + carry_final;
        final_sum[i] = val as u64;
        carry_final = val >> 64;
    }
    
    while carry_final > 0 || res_gte_p(final_sum) {
        let mut borrow = 0u128;
        for i in 0..4 {
            let diff = (final_sum[i] as u128) - (FieldElement::P.0[i] as u128) - borrow;
            final_sum[i] = diff as u64;
            borrow = (diff >> 64) & 1;
        }
        if carry_final > 0 {
            carry_final = (carry_final - borrow) as u128;
        } else {
            break;
        }
    }
    
    FieldElement(final_sum)
}

/// Twisted Edwards Curve Group operations in Projective Coordinates
#[derive(Clone, Copy, Debug)]
pub struct CurvePoint {
    pub x: FieldElement,
    pub y: FieldElement,
    pub z: FieldElement,
    pub t: FieldElement,
}

impl CurvePoint {
    pub const D: FieldElement = FieldElement([
        0x75eb4dca135978a3, 0x7779e89800700a4d,
        0x8cc740797779e898, 0x52036cee2b6ffe73,
    ]);

    pub const TWO_D: FieldElement = FieldElement([
        0xebe69b9426b2f147, 0xeeeeefd13000e014,
        0x198e80f2eeefd130, 0x2406d9dc56dffce7,
    ]);

    pub const SQRT_M1: FieldElement = FieldElement([
        0x4ee1b274a291954, 0x2f431806ad2fe478,
        0x2b4d00993dfbd7a7, 0x2b8324804fc1df0b
    ]);

    pub fn generator() -> Self {
        CurvePoint {
            x: FieldElement([
                0x11eed3d197e28319, 0x6e8e8154e19069d3,
                0x10f765373a69a473, 0x216936d3cd6e53fe,
            ]),
            y: FieldElement([
                0x6666666666666658, 0x6666666666666666,
                0x6666666666666666, 0x6666666666666666,
            ]),
            z: FieldElement::one(),
            t: FieldElement([
                0x66a6a24911d33405, 0x5cc5d5a7d74db627,
                0xcc06c8cae49c7bc2, 0x5776a30c5e7b2339,
            ]),
        }
    }

    pub fn identity() -> Self {
        CurvePoint {
            x: FieldElement::zero(),
            y: FieldElement::one(),
            z: FieldElement::one(),
            t: FieldElement::zero(),
        }
    }

    pub fn double(self) -> Self {
        let a = self.x.mul(self.x);
        let b = self.y.mul(self.y);
        let c = FieldElement([2, 0, 0, 0]).mul(self.z.mul(self.z));
        let d = FieldElement::zero().sub(a);
        let x_plus_y = self.x.add(self.y);
        let j = x_plus_y.mul(x_plus_y).sub(a).sub(b);
        let e = j;
        let g = d.add(b);
        let f = g.sub(c);
        let h = d.sub(b);
        CurvePoint {
            x: e.mul(f),
            y: g.mul(h),
            z: f.mul(g),
            t: e.mul(h),
        }
    }

    pub fn add(self, other: Self) -> Self {
        let a = (self.y.sub(self.x)).mul(other.y.sub(other.x));
        let b = (self.y.add(self.x)).mul(other.y.add(other.x));
        let c = Self::TWO_D.mul(self.t).mul(other.t);
        let d = (self.z.mul(FieldElement([2, 0, 0, 0]))).mul(other.z);
        let e = b.sub(a);
        let f = d.sub(c);
        let g = d.add(c);
        let h = b.add(a);
        CurvePoint {
            x: e.mul(f),
            y: g.mul(h),
            z: f.mul(g),
            t: e.mul(h),
        }
    }

    pub fn mul(self, scalar: &[u8; 32]) -> Self {
        let mut r = Self::identity();
        let mut p = self;
        for i in 0..256 {
            let byte_idx = i / 8;
            let bit_idx = i % 8;
            if (scalar[byte_idx] >> bit_idx) & 1 == 1 {
                r = r.add(p);
            }
            p = p.double();
        }
        r
    }

    pub fn decompress(bytes: &[u8; 32]) -> Option<Self> {
        let mut y_limbs = [0u64; 4];
        for i in 0..4 {
            y_limbs[i] = u64::from_le_bytes(bytes[i*8..i*8+8].try_into().unwrap());
        }
        let sign = (y_limbs[3] >> 63) & 1;
        y_limbs[3] &= 0x7fffffffffffffff;
        
        // y coordinate must be less than p
        if res_gte_p(y_limbs) {
            return None;
        }
        
        let y = FieldElement(y_limbs);
        let y2 = y.mul(y);
        let u = y2.sub(FieldElement::one());
        let v = Self::D.mul(y2).add(FieldElement::one());
        
        let inv_v = v.pow_p_minus_2();
        let w = u.mul(inv_v);
        
        let mut x = w_pow_2_252_minus_2(w);
        let mut x2 = x.mul(x);
        
        if x2 != w {
            x = x.mul(Self::SQRT_M1);
            x2 = x.mul(x);
            if x2 != w {
                return None;
            }
        }
        
        let x_bytes = x.0[0].to_le_bytes();
        let x_sign = (x_bytes[0] & 1) as u64;
        if x_sign != sign {
            x = FieldElement::zero().sub(x);
        }
        
        let final_x_bytes = x.0[0].to_le_bytes();
        let final_x_sign = (final_x_bytes[0] & 1) as u64;
        if final_x_sign != sign {
            return None;
        }
        
        let t = x.mul(y);
        Some(CurvePoint { x, y, z: FieldElement::one(), t })
    }
}

pub fn w_pow_2_252_minus_2(w: FieldElement) -> FieldElement {
    let mut res = FieldElement::one();
    let mut base = w;
    for i in 0..252 {
        if i > 0 {
            res = res.mul(base);
        }
        base = base.mul(base);
    }
    res
}

pub fn verify_ed25519_signature(
    public_key_bytes: &[u8; 32],
    signature_bytes: &[u8; 64],
    message: &[u8]
) -> bool {
    let r_bytes: &[u8; 32] = signature_bytes[0..32].try_into().unwrap();
    let s_bytes: &[u8; 32] = signature_bytes[32..64].try_into().unwrap();
    
    let r_point = match CurvePoint::decompress(r_bytes) {
        Some(p) => p,
        None => return false,
    };
    
    let pk_point = match CurvePoint::decompress(public_key_bytes) {
        Some(p) => p,
        None => return false,
    };
    
    // Check range of S: [0, L)
    const L_LIMIT: [u64; 4] = [
        0x5812631a5cf5d3ed, 0x14def9dea2f79cd6,
        0x0000000000000000, 0x1000000000000000
    ];
    let mut s_limbs = [0u64; 4];
    for i in 0..4 {
        s_limbs[i] = u64::from_le_bytes(s_bytes[i*8..i*8+8].try_into().unwrap());
    }
    for i in (0..4).rev() {
        if s_limbs[i] > L_LIMIT[i] { return false; }
        if s_limbs[i] < L_LIMIT[i] { break; }
        if i == 0 { return false; } // S == L is illegal
    }
    
    // k = SHA-512(R || PK || Message)
    let mut hasher = Sha512::new();
    hasher.update(r_bytes);
    hasher.update(public_key_bytes);
    hasher.update(message);
    let hash_result = hasher.finalize();
    
    let k_scalar = reduce_sha512_mod_l(&hash_result);
    
    // Check cofactor cleared equation: [8][S]B = [8]R + [8][k]PK
    let sb = CurvePoint::generator().mul(s_bytes);
    let k_pk = pk_point.mul(&k_scalar);
    let r_plus_k_pk = r_point.add(k_pk);
    
    let sb_8 = sb.double().double().double();
    let r_plus_k_pk_8 = r_plus_k_pk.double().double().double();
    
    // Compare Projective coordinates: X1 * Z2 == X2 * Z1 and Y1 * Z2 == Y2 * Z1
    let x1_z2 = sb_8.x.mul(r_plus_k_pk_8.z);
    let x2_z1 = r_plus_k_pk_8.x.mul(sb_8.z);
    let y1_z2 = sb_8.y.mul(r_plus_k_pk_8.z);
    let y2_z1 = r_plus_k_pk_8.y.mul(sb_8.z);
    
    x1_z2 == x2_z1 && y1_z2 == y2_z1
}

/// Bitwise reduction modulo L using subtraction shifts
fn reduce_sha512_mod_l(hash: &[u8; 64]) -> [u8; 32] {
    let mut val = [0u64; 8];
    for i in 0..8 {
        val[i] = u64::from_le_bytes(hash[i * 8..i * 8 + 8].try_into().unwrap());
    }
    
    const L: [u64; 4] = [
        0x5812631a5cf5d3ed,
        0x14def9dea2f79cd6,
        0x0000000000000000,
        0x1000000000000000,
    ];
    
    for shift in (0..=259).rev() {
        let bit_shift = shift % 64;
        let word_shift = shift / 64;
        
        let mut temp = [0u64; 8];
        let mut carry = 0u64;
        for i in 0..8 {
            let val_i = if i < 4 { L[i] } else { 0 };
            temp[i] = (val_i << bit_shift) | carry;
            carry = if bit_shift == 0 { 0 } else { val_i >> (64 - bit_shift) };
        }
        
        let mut l_shifted = [0u64; 8];
        for i in 0..8 {
            if i + word_shift < 8 {
                l_shifted[i + word_shift] = temp[i];
            }
        }
        
        if val_gte(val, l_shifted) {
            let mut borrow = 0u128;
            for i in 0..8 {
                let diff = (val[i] as u128) - (l_shifted[i] as u128) - borrow;
                val[i] = diff as u64;
                borrow = (diff >> 64) & 1;
            }
        }
    }
    
    let mut out = [0u8; 32];
    for i in 0..4 {
        out[i * 8..i * 8 + 8].copy_from_slice(&val[i].to_le_bytes());
    }
    out
}

fn val_gte(a: [u64; 8], b: [u64; 8]) -> bool {
    for i in (0..8).rev() {
        if a[i] > b[i] { return true; }
        if a[i] < b[i] { return false; }
    }
    true
}

// =========================================================================
// 4. Autonomic Lifecycle State Machine
// =========================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutonomicState {
    Design,             // 1. Initial process net structure setup
    Construction,       // 2. Petri Net compiler generation
    Simulation,         // 3. Coverability tree / reachability checks
    Integration,        // 4. Synergistic system mapping
    Activation,         // 5. Ignition key validation / deploying
    Operation,          // 6. Running active case execution
    Monitoring,         // 7. Streaming event logging / conformance checking
    Repair,             // 8. Dynamic S-component repair routing
    Optimization,       // 9. Inductive Miner candidate model search
    Decommission,       // 10. Quarantine and revocation sequences
    Archive,            // 11. Read-only cold ledger storage
    BoardProjection,    // 12. Strategic synergy projection map
}

#[derive(Clone, Debug)]
pub enum AutonomicEvent {
    VerifySoundness(bool),          // Structural proof (Gate 1)
    VerifyReachability(bool),       // 1-boundedness / safety checks (Gate 2)
    ValidateIntegration(bool),      // Enterprise boundary maps
    IgniteVM(bool),                 // Token unlock
    RunStreaming,                   // Start event loop
    CheckMetrics,                   // Run conformance/debt/utility assessment
    CompleteArchiveVerification,    // Seal block hash
    RequestStrategicProjection,     // Slide ledger extraction
}

pub struct AutonomicActuator {
    current_state: AutonomicState,
    pub alignment_fitness: f64,
    pub process_debt: f64,
    pub process_utility: f64,
}

impl AutonomicActuator {
    pub fn new(initial_state: AutonomicState) -> Self {
        Self {
            current_state: initial_state,
            alignment_fitness: 1.0,
            process_debt: 0.0,
            process_utility: 1.0,
        }
    }

    pub fn current_state(&self) -> AutonomicState {
        self.current_state
    }

    /// Process state transition according to quality gates & MAPE-K protocols
    pub fn actuate(&mut self, event: AutonomicEvent) -> Result<AutonomicState, &'static str> {
        let next_state = match (self.current_state, event) {
            // Gate 1: Design -> Construction (requires soundness verification)
            (AutonomicState::Design, AutonomicEvent::VerifySoundness(sound)) => {
                if sound { AutonomicState::Construction } else { return Err("Gate 1 Soundness check failed."); }
            }
            
            // Construction -> Simulation
            (AutonomicState::Construction, AutonomicEvent::VerifyReachability(true)) => {
                AutonomicState::Simulation
            }
            
            // Gate 2: Simulation -> Integration (verify reachability and 1-boundedness bounds)
            (AutonomicState::Simulation, AutonomicEvent::VerifyReachability(true)) => {
                AutonomicState::Integration
            }
            
            // Integration -> Activation
            (AutonomicState::Integration, AutonomicEvent::ValidateIntegration(true)) => {
                AutonomicState::Activation
            }
            
            // Activation -> Operation (Ignition sequence complete)
            (AutonomicState::Activation, AutonomicEvent::IgniteVM(true)) => {
                AutonomicState::Operation
            }
            
            // Operation -> Monitoring
            (AutonomicState::Operation, AutonomicEvent::RunStreaming) => {
                AutonomicState::Monitoring
            }
            
            // Monitoring checks: triggers autonomic actuation laws
            (AutonomicState::Monitoring, AutonomicEvent::CheckMetrics) => {
                // 1. Retirement Actuation: utility drops below 0.50 -> transition to Decommission
                if self.process_utility < 0.50 {
                    AutonomicState::Decommission
                // 2. Compliance Deviation Actuation: fitness drops below 0.85 -> Force Lockdown / Error
                } else if self.alignment_fitness < 0.85 {
                    return Err("Compliance Deviation: Fitness falls below hard floor. Triggering lockdown.");
                // 3. Elastic Deviation Actuation: fitness in [0.85, 0.95) -> transition to Repair
                } else if self.alignment_fitness < 0.95 {
                    AutonomicState::Repair
                // 4. Debt Actuation: process debt > 15% -> transition to Optimization
                } else if self.process_debt > 0.15 {
                    AutonomicState::Optimization
                } else {
                    AutonomicState::Monitoring // Remain stable
                }
            }

            // Repair -> Monitoring (Repaired net returns to operation)
            (AutonomicState::Repair, AutonomicEvent::VerifySoundness(true)) => {
                AutonomicState::Monitoring
            }

            // Optimization -> Monitoring (Discovered model successfully deployed)
            (AutonomicState::Optimization, AutonomicEvent::VerifySoundness(sound)) => {
                if sound { AutonomicState::Monitoring } else { return Err("Optimization yielded unsound model."); }
            }

            // Decommission -> Archive
            (AutonomicState::Decommission, AutonomicEvent::CompleteArchiveVerification) => {
                AutonomicState::Archive
            }

            // Archive -> BoardProjection
            (AutonomicState::Archive, AutonomicEvent::RequestStrategicProjection) => {
                AutonomicState::BoardProjection
            }

            // Fallback: invalid transitions
            _ => return Err("Illegal transition. Transition rejected under Autonomic Law."),
        };

        self.current_state = next_state;
        Ok(next_state)
    }
}

// =========================================================================
// 5. Test Suite
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Lattice Axioms Helper ---
    fn check_lattice_axioms<L: Lattice + std::fmt::Debug>(a: L, b: L, c: L) {
        // 1. Idempotence: a.join(a) == a
        assert_eq!(a.join(&a), a);
        assert_eq!(b.join(&b), b);
        assert_eq!(c.join(&c), c);

        // 2. Commutativity: a.join(b) == b.join(a)
        assert_eq!(a.join(&b), b.join(&a));
        assert_eq!(a.join(&c), c.join(&a));
        assert_eq!(b.join(&c), c.join(&b));

        // 3. Associativity: a.join(b.join(c)) == (a.join(b)).join(c)
        assert_eq!(a.join(&b.join(&c)), (a.join(&b)).join(&c));

        // 4. Monotonicity/Absorption check via partial_cmp
        if let Some(Ordering::Less) = a.partial_cmp(&b) {
            assert_eq!(a.join(&b), b);
        }
        if let Some(Ordering::Greater) = a.partial_cmp(&b) {
            assert_eq!(a.join(&b), a);
        }
    }

    #[test]
    fn test_witness_state_lattice() {
        let bottom = WitnessState::bottom();
        let top = WitnessState::top();

        let w1 = WitnessState::PartialReplay {
            trace_indices: vec![1, 3],
            marking: vec!["p1".to_string()],
            cost: 10,
        };
        let w2 = WitnessState::PartialReplay {
            trace_indices: vec![2, 4],
            marking: vec!["p2".to_string()],
            cost: 20,
        };
        let w3 = WitnessState::PartialReplay {
            trace_indices: vec![1, 2],
            marking: vec!["p1".to_string(), "p2".to_string()],
            cost: 15,
        };

        // Standard axioms
        check_lattice_axioms(bottom.clone(), w1.clone(), top.clone());
        check_lattice_axioms(w1.clone(), w2.clone(), w3.clone());

        // Top convergence on overlap
        let w1_join_w2 = w1.join(&w2);
        assert!(matches!(w1_join_w2, WitnessState::PartialReplay { .. }));
        if let WitnessState::PartialReplay { trace_indices, marking, cost } = w1_join_w2 {
            assert_eq!(trace_indices, vec![1, 2, 3, 4]);
            assert_eq!(marking, vec!["p1".to_string(), "p2".to_string()]);
            assert_eq!(cost, 30);
        }

        // w1 and w3 have overlapping index 1, so they must join to Top
        assert_eq!(w1.join(&w3), WitnessState::Top);
    }

    #[test]
    fn test_constraint_value_lattice() {
        let bot = ConstraintValue::bottom();
        let top = ConstraintValue::top();
        let ps = ConstraintValue::PossiblySatisfied;
        let sat = ConstraintValue::Satisfied;
        let viol = ConstraintValue::Violated;

        check_lattice_axioms(bot.clone(), ps.clone(), top.clone());
        check_lattice_axioms(ps.clone(), sat.clone(), viol.clone());

        assert_eq!(sat.join(&viol), top);
        assert_eq!(ps.join(&sat), sat);
        assert_eq!(ps.join(&viol), viol);
    }

    #[test]
    fn test_declare_witness_lattice() {
        let bot = DeclareWitnessState::bottom();
        let top = DeclareWitnessState::top();

        let mut m1 = HashMap::new();
        m1.insert("c1".to_string(), ConstraintValue::PossiblySatisfied);
        m1.insert("c2".to_string(), ConstraintValue::Satisfied);
        let d1 = DeclareWitnessState::Evaluated(m1);

        let mut m2 = HashMap::new();
        m2.insert("c1".to_string(), ConstraintValue::Satisfied);
        m2.insert("c3".to_string(), ConstraintValue::Violated);
        let d2 = DeclareWitnessState::Evaluated(m2);

        check_lattice_axioms(bot, d1.clone(), top.clone());
        
        let joined = d1.join(&d2);
        if let DeclareWitnessState::Evaluated(ref m) = joined {
            assert_eq!(m.get("c1").unwrap(), &ConstraintValue::Satisfied);
            assert_eq!(m.get("c2").unwrap(), &ConstraintValue::Satisfied);
            assert_eq!(m.get("c3").unwrap(), &ConstraintValue::Violated);
        } else {
            panic!("Expected Evaluated");
        }

        // Conflict check
        let mut m3 = HashMap::new();
        m3.insert("c2".to_string(), ConstraintValue::Violated);
        let d3 = DeclareWitnessState::Evaluated(m3);
        assert_eq!(d1.join(&d3), DeclareWitnessState::top());
    }

    #[test]
    fn test_unified_witness_lattice() {
        let bot = UnifiedWitnessState::bottom();
        let top = UnifiedWitnessState::top();

        let u1 = UnifiedWitnessState::Active {
            replay: WitnessState::PartialReplay { trace_indices: vec![1], marking: vec![], cost: 0 },
            declare: DeclareWitnessState::Bottom,
        };
        let u2 = UnifiedWitnessState::Active {
            replay: WitnessState::PartialReplay { trace_indices: vec![2], marking: vec![], cost: 0 },
            declare: DeclareWitnessState::Bottom,
        };

        check_lattice_axioms(bot, u1.clone(), top);
        assert!(matches!(u1.join(&u2), UnifiedWitnessState::Active { .. }));
    }

    // --- SHA-512 Verification ---
    #[test]
    fn test_sha512_vectors() {
        // Test Vector: empty string
        let hasher = Sha512::new();
        let out = hasher.finalize();
        let expected_empty = [
            0xcf, 0x83, 0xe1, 0x35, 0x7e, 0xef, 0xb8, 0xbd, 0xf1, 0x54, 0x28, 0x50, 0xd6, 0x6d, 0x80, 0x07,
            0xd6, 0x20, 0xe4, 0x05, 0x0b, 0x57, 0x15, 0xdc, 0x83, 0xf4, 0xa9, 0x21, 0xd3, 0x6c, 0xe9, 0xce,
            0x47, 0xd0, 0xd1, 0x3c, 0x5d, 0x85, 0xf2, 0xb0, 0xff, 0x83, 0x18, 0xd2, 0x87, 0x7e, 0xec, 0x2f,
            0x63, 0xb9, 0x31, 0xbd, 0x47, 0x41, 0x7a, 0x81, 0xa5, 0x38, 0x32, 0x7a, 0xf9, 0x27, 0xda, 0x3e
        ];
        assert_eq!(out, expected_empty);

        // Test Vector: "abc"
        let mut hasher = Sha512::new();
        hasher.update(b"abc");
        let out = hasher.finalize();
        let expected_abc = [
            0xdd, 0xaf, 0x35, 0xa1, 0x93, 0x61, 0x7a, 0xba, 0xcc, 0x41, 0x73, 0x49, 0xae, 0x20, 0x41, 0x31,
            0x12, 0xe6, 0xfa, 0x4e, 0x89, 0xa9, 0x7e, 0xa2, 0x0a, 0x9e, 0xee, 0x16, 0x48, 0x20, 0x1a, 0x24,
            0xb0, 0x41, 0x0c, 0x6d, 0x1b, 0x29, 0x31, 0x73, 0x4b, 0x39, 0x7b, 0x49, 0x00, 0x44, 0x37, 0x88,
            0x24, 0x85, 0xdb, 0xb6, 0x88, 0xf8, 0xd7, 0xa3, 0x73, 0xac, 0x47, 0x3b, 0x03, 0x3c, 0x46, 0x8e
        ];
        assert_eq!(out, expected_abc);
    }

    // --- Ed25519 Signature Verification ---
    #[test]
    fn test_ed25519_rfc8032_vector1() {
        // RFC 8032 Section 7.1 Test Vector 1
        let pk_hex = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a";
        let sig_hex = "e5564300c360ac72908f067b40c10b75a38979b0992c275661497cfd9966b8934785397120b31d47e9ab841b0c9520b9e8198641a2c37087bb310159fb9b8700";
        
        let pk_bytes = hex_decode(pk_hex);
        let sig_bytes = hex_decode(sig_hex);
        
        let pk: &[u8; 32] = pk_bytes.as_slice().try_into().unwrap();
        let sig: &[u8; 64] = sig_bytes.as_slice().try_into().unwrap();
        
        // Message is empty
        let message = b"";
        
        let valid = verify_ed25519_signature(pk, sig, message);
        assert!(valid, "Signature verification failed on RFC 8032 Test Vector 1!");

        // Tamper test
        let mut tampered_sig = sig.clone();
        tampered_sig[0] ^= 1;
        let invalid = verify_ed25519_signature(pk, &tampered_sig, message);
        assert!(!invalid, "Tampered signature should fail verification!");
    }

    fn hex_decode(s: &str) -> Vec<u8> {
        let mut res = Vec::new();
        let mut chars = s.chars();
        while let (Some(c1), Some(c2)) = (chars.next(), chars.next()) {
            let s_byte = format!("{}{}", c1, c2);
            res.push(u8::from_str_radix(&s_byte, 16).unwrap());
        }
        res
    }

    // --- Autonomic Actuator Checks ---
    #[test]
    fn test_autonomic_state_machine() {
        let mut actuator = AutonomicActuator::new(AutonomicState::Design);
        assert_eq!(actuator.current_state(), AutonomicState::Design);

        // Transition: Design -> Construction (VerifySoundness(true))
        assert_eq!(actuator.actuate(AutonomicEvent::VerifySoundness(true)).unwrap(), AutonomicState::Construction);

        // Transition: Construction -> Simulation (VerifyReachability(true))
        assert_eq!(actuator.actuate(AutonomicEvent::VerifyReachability(true)).unwrap(), AutonomicState::Simulation);

        // Transition: Simulation -> Integration (VerifyReachability(true))
        assert_eq!(actuator.actuate(AutonomicEvent::VerifyReachability(true)).unwrap(), AutonomicState::Integration);

        // Transition: Integration -> Activation (ValidateIntegration(true))
        assert_eq!(actuator.actuate(AutonomicEvent::ValidateIntegration(true)).unwrap(), AutonomicState::Activation);

        // Transition: Activation -> Operation (IgniteVM(true))
        assert_eq!(actuator.actuate(AutonomicEvent::IgniteVM(true)).unwrap(), AutonomicState::Operation);

        // Transition: Operation -> Monitoring (RunStreaming)
        assert_eq!(actuator.actuate(AutonomicEvent::RunStreaming).unwrap(), AutonomicState::Monitoring);

        // Stable Monitoring
        assert_eq!(actuator.actuate(AutonomicEvent::CheckMetrics).unwrap(), AutonomicState::Monitoring);

        // 1. Elastic Repair Actuation (fitness in [0.85, 0.95))
        actuator.alignment_fitness = 0.90;
        assert_eq!(actuator.actuate(AutonomicEvent::CheckMetrics).unwrap(), AutonomicState::Repair);

        // Repair -> Monitoring
        assert_eq!(actuator.actuate(AutonomicEvent::VerifySoundness(true)).unwrap(), AutonomicState::Monitoring);

        // 2. Debt Actuation (debt > 15%)
        actuator.alignment_fitness = 1.0;
        actuator.process_debt = 0.20;
        assert_eq!(actuator.actuate(AutonomicEvent::CheckMetrics).unwrap(), AutonomicState::Optimization);

        // Optimization -> Monitoring
        assert_eq!(actuator.actuate(AutonomicEvent::VerifySoundness(true)).unwrap(), AutonomicState::Monitoring);

        // 3. Retirement Actuation (utility < 50%)
        actuator.process_debt = 0.0;
        actuator.process_utility = 0.40;
        assert_eq!(actuator.actuate(AutonomicEvent::CheckMetrics).unwrap(), AutonomicState::Decommission);

        // Decommission -> Archive
        assert_eq!(actuator.actuate(AutonomicEvent::CompleteArchiveVerification).unwrap(), AutonomicState::Archive);

        // Archive -> BoardProjection
        assert_eq!(actuator.actuate(AutonomicEvent::RequestStrategicProjection).unwrap(), AutonomicState::BoardProjection);
    }

    #[test]
    fn test_autonomic_lockdown() {
        let mut actuator = AutonomicActuator::new(AutonomicState::Monitoring);
        actuator.alignment_fitness = 0.80; // Compliance Deviation < 0.85
        let res = actuator.actuate(AutonomicEvent::CheckMetrics);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("lockdown"));
    }

    #[test]
    fn test_evidence_hashing_and_validation() {
        let payload = "process_event_trace_log_payload".to_string();
        let state = "petri_net_marking_state".to_string();
        let witness = WitnessState::PartialReplay {
            trace_indices: vec![1, 2],
            marking: vec!["p1".to_string()],
            cost: 0,
        };

        // RFC 8032 Vector 1 keys
        let pk_hex = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a";
        // To construct a valid signature over the computed hash, we must sign it.
        // Since we don't have a private key in the test to sign arbitrarily, we can verify that if we use the signature from RFC 8032 Vector 1,
        // it verifies successfully when the message matches the empty message.
        // Let's create an evidence where the computed hash matches the empty message (i.e. we construct computed hash = 32-byte representation of empty message).
        // Wait, the SHA-256 hash of the evidence fields is what is signed.
        // We can just construct an evidence and verify it fails with an invalid signature,
        // and check that the hash computation works.
        let pk_bytes = hex_decode(pk_hex);
        let pk_arr: [u8; 32] = pk_bytes.as_slice().try_into().unwrap();

        let sig = IdentitySignature {
            public_key: pk_bytes.clone(),
            signature_bytes: vec![0u8; 64],
        };

        let mut evidence = Evidence {
            payload,
            state,
            witness,
            epoch: 42,
            signature: sig,
            hash: Blake3Hash([0u8; 32]),
        };

        let computed_hash = evidence.calculate_hash();
        evidence.hash = computed_hash;

        // Validation must fail because signature is all zeros
        let val_res = evidence.validate(&pk_arr);
        assert_eq!(val_res.unwrap_err(), EvidenceError::InvalidSignature);
    }
}
