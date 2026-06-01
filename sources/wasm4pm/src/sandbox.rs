use crate::crypto::{ChaCha20, Sha256};
use crate::allocator;

// Error codes defined in audits/audit-execution-boundaries.md
pub const ERR_CYCLE_OVERFLOW: u32 = 0xFB01;
pub const ERR_QUERY_TIMEOUT: u32 = 0xFB02;
pub const ERR_CONFORMANCE_VIOLATION: u32 = 0xFB03;
pub const ERR_REPLAY_ATTESTATION: u32 = 0xFB04;
pub const ERR_LIFECYCLE_VIOLATION: u32 = 0xFB05;

pub struct RecursionGuard {
    current_depth: u32,
    max_depth: u32,
}

impl RecursionGuard {
    pub fn new(max_depth: u32) -> Self {
        Self {
            current_depth: 0,
            max_depth,
        }
    }

    pub fn enter(&mut self) -> Result<(), u32> {
        if self.current_depth >= self.max_depth {
            return Err(ERR_LIFECYCLE_VIOLATION); // Call stack depth violation
        }
        self.current_depth += 1;
        Ok(())
    }

    pub fn exit(&mut self) {
        if self.current_depth > 0 {
            self.current_depth -= 1;
        }
    }
}

pub struct GasMeter {
    allocated: u64,
    consumed: u64,
}

impl GasMeter {
    pub fn new(allocated: u64) -> Self {
        Self {
            allocated,
            consumed: 0,
        }
    }

    pub fn consume(&mut self, cycles: u64) -> Result<(), u32> {
        self.consumed = self.consumed.checked_add(cycles).ok_or(ERR_CYCLE_OVERFLOW)?;
        if self.consumed > self.allocated {
            return Err(ERR_CYCLE_OVERFLOW);
        }
        Ok(())
    }

    pub fn consumed(&self) -> u64 {
        self.consumed
    }
}

// Oblivion Protocol: Overwrites linear memory using 3 passes of CSPRNG bytes
pub fn execute_oblivion_protocol(seed: &[u8; 32]) {
    let nonce = [0u8; 12];
    let mut prng = ChaCha20::new(seed, &nonce);

    // Shred the global allocator's memory buffer
    allocator::shred_global_arena(&mut || prng.next_block());
}

// Cycle attestation: Verify proof-of-work
// SHA-256(Challenge || Proof) < Target
pub fn verify_cycle_proof_of_work(
    challenge: &[u8; 32],
    proof: u64,
    difficulty_threshold: &[u8; 32],
) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(challenge);
    hasher.update(&proof.to_le_bytes());
    let hash = hasher.finalize();
    
    // Check if hash < difficulty_threshold lexicographically
    hash < *difficulty_threshold
}
