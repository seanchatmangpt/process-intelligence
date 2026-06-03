//! # c8-receipts
//!
//! Cryptographic receipts, receipt chains, replay validation, and boundary proof
//! for the Construct8 market physics engine.
//!
//! Every graph state transition is accompanied by a BLAKE3-hashed receipt that
//! captures the pre-state, delta, post-state, causal time, and module version.
//! Chains of receipts form an append-only audit log that can be replayed and
//! boundary-proven against expected post-state hashes.

use c8_graph::{Construct8Delta, GraphField};
use serde::{Deserialize, Serialize};

/// 32-byte BLAKE3 receipt hash type.
pub type ReceiptHash = [u8; 32];

/// The module version stamped into every receipt.
const MODULE_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// C8Receipt
// ---------------------------------------------------------------------------

/// Durable proof record of a single Construct8 state transition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C8Receipt {
    pub pre_state_hash: u64,
    pub delta_mask: u8,
    pub delta_len: u8,
    pub post_state_hash: u64,
    pub causal_time: u64,
    pub module_version: u32,
    pub receipt_hash: ReceiptHash,
}

impl C8Receipt {
    /// Construct a new receipt with a BLAKE3 hash over the transition fields.
    ///
    /// Hash input (in order):
    ///   pre_state_hash  (le_bytes u64)
    ///   delta_mask      (u8)
    ///   delta_len       (u8)
    ///   post_state_hash (le_bytes u64)
    ///   causal_time     (le_bytes u64)
    ///   module_version  (le_bytes u32)
    pub fn new(
        pre_state_hash: u64,
        delta: &Construct8Delta,
        post_state_hash: u64,
        causal_time: u64,
    ) -> Self {
        let delta_mask = delta.mask().0;
        let delta_len = delta.len() as u8;

        let receipt_hash = Self::compute_hash(
            pre_state_hash,
            delta_mask,
            delta_len,
            post_state_hash,
            causal_time,
            MODULE_VERSION,
        );

        C8Receipt {
            pre_state_hash,
            delta_mask,
            delta_len,
            post_state_hash,
            causal_time,
            module_version: MODULE_VERSION,
            receipt_hash,
        }
    }

    /// Recompute the BLAKE3 hash and compare against the stored receipt_hash.
    pub fn verify(&self) -> bool {
        let expected = Self::compute_hash(
            self.pre_state_hash,
            self.delta_mask,
            self.delta_len,
            self.post_state_hash,
            self.causal_time,
            self.module_version,
        );
        self.receipt_hash == expected
    }

    fn compute_hash(
        pre_state_hash: u64,
        delta_mask: u8,
        delta_len: u8,
        post_state_hash: u64,
        causal_time: u64,
        module_version: u32,
    ) -> ReceiptHash {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&pre_state_hash.to_le_bytes());
        hasher.update(&[delta_mask, delta_len]);
        hasher.update(&post_state_hash.to_le_bytes());
        hasher.update(&causal_time.to_le_bytes());
        hasher.update(&module_version.to_le_bytes());
        *hasher.finalize().as_bytes()
    }
}

// ---------------------------------------------------------------------------
// ReceiptChain
// ---------------------------------------------------------------------------

/// Append-only ledger of Construct8 state-transition receipts.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReceiptChain {
    pub receipts: Vec<C8Receipt>,
}

impl ReceiptChain {
    pub fn new() -> Self {
        ReceiptChain {
            receipts: Vec::new(),
        }
    }

    /// Append a receipt to the chain.
    pub fn append(&mut self, receipt: C8Receipt) {
        self.receipts.push(receipt);
    }

    /// Return the number of receipts in the chain.
    pub fn len(&self) -> usize {
        self.receipts.len()
    }

    /// Return true if the chain contains no receipts.
    pub fn is_empty(&self) -> bool {
        self.receipts.is_empty()
    }

    /// Verify every receipt in the chain by recomputing each BLAKE3 hash.
    pub fn verify_all(&self) -> bool {
        self.receipts.iter().all(|r| r.verify())
    }

    /// Return the receipt_hash of the last receipt, if any.
    pub fn last_hash(&self) -> Option<&ReceiptHash> {
        self.receipts.last().map(|r| &r.receipt_hash)
    }
}

// ---------------------------------------------------------------------------
// Replay
// ---------------------------------------------------------------------------

/// Apply `delta` to `field` and return whether the resulting state hash equals
/// `expected_post_hash`.
pub fn replay_construct8_delta(
    field: &mut GraphField,
    delta: &Construct8Delta,
    expected_post_hash: u64,
) -> bool {
    let _ = field.apply_construct8(delta);
    field.state_hash() == expected_post_hash
}

// ---------------------------------------------------------------------------
// Receipt persistence
// ---------------------------------------------------------------------------

/// Serialise `chain` as pretty-printed JSON and write it to `path`.
pub fn write_implementation_receipt(path: &str, chain: &ReceiptChain) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(chain).map_err(std::io::Error::other)?;
    std::fs::write(path, json)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use c8_graph::TripleRef;

    // Helpers -----------------------------------------------------------------

    fn empty_delta() -> Construct8Delta {
        Construct8Delta::empty()
    }

    fn delta_with_triple() -> Construct8Delta {
        let mut d = Construct8Delta::empty();
        d.push_checked(TripleRef::new(1, 10, 2)).unwrap();
        d
    }

    // Tests -------------------------------------------------------------------

    #[test]
    fn receipt_hash_changes_with_state() {
        let delta = empty_delta();
        let r1 = C8Receipt::new(100, &delta, 200, 1000);
        let r2 = C8Receipt::new(100, &delta, 300, 1000);
        assert_ne!(
            r1.receipt_hash, r2.receipt_hash,
            "different post_state_hash must yield different receipt_hash"
        );
    }

    #[test]
    fn receipt_chain_verifies() {
        let delta = empty_delta();
        let r1 = C8Receipt::new(100, &delta, 200, 1000);
        let r2 = C8Receipt::new(200, &delta, 300, 1001);

        let mut chain = ReceiptChain::new();
        chain.append(r1);
        chain.append(r2);

        assert_eq!(chain.len(), 2);
        assert!(chain.verify_all(), "chain with valid receipts must verify");
    }

    #[test]
    fn tampered_receipt_fails_verification() {
        let delta = empty_delta();
        let mut r = C8Receipt::new(100, &delta, 200, 1000);
        // Tamper: mutate post_state_hash without recomputing receipt_hash
        r.post_state_hash = 999;
        assert!(
            !r.verify(),
            "tampered post_state_hash must cause verify() to return false"
        );
    }

    #[test]
    fn replay_construct8_delta_reproduces_hash() {
        let delta = delta_with_triple();

        // Capture post-state hash from a fresh application
        let mut field = GraphField::new();
        field.apply_construct8(&delta);
        let post_hash = field.state_hash();

        // Replay on a fresh field and confirm the hash matches
        let mut replay_field = GraphField::new();
        let result = replay_construct8_delta(&mut replay_field, &delta, post_hash);
        assert!(result, "replay must reproduce the expected post-state hash");
    }
}
