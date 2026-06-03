//! CONSTRUCT8 core type law.
//!
//! Provides the bounded, refusal-aware types that form the zero-cost
//! foundation of the construct8-market-physics engine.

use thiserror::Error;

// ---------------------------------------------------------------------------
// Identifier newtypes
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RelationId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VenueId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InstrumentId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ActorClassId(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GraphSlot(pub u8);

// ---------------------------------------------------------------------------
// Construct8Len — bounded lane count [0, 8]
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Construct8Len(u8);

impl Construct8Len {
    pub const MAX: u8 = 8;

    /// Construct a lane count. Returns `Err(C8Error::Need9)` if `n > 8`.
    pub fn new(n: u8) -> C8Result<Self> {
        if n > Self::MAX {
            Err(C8Error::Need9)
        } else {
            Ok(Self(n))
        }
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn is_full(self) -> bool {
        self.0 == Self::MAX
    }
}

// ---------------------------------------------------------------------------
// Construct8Mask — bitmask over 8 lanes
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Construct8Mask(pub u8);

impl Construct8Mask {
    pub const EMPTY: Self = Self(0);
    pub const FULL: Self = Self(0xFF);

    /// Set the given lane bit (lane must be 0..=7).
    pub fn set(self, lane: u8) -> Self {
        assert!(lane < 8, "lane index must be 0..=7");
        Self(self.0 | (1u8 << lane))
    }

    /// Test whether the given lane bit is set.
    pub fn has(self, lane: u8) -> bool {
        assert!(lane < 8, "lane index must be 0..=7");
        (self.0 >> lane) & 1 == 1
    }

    /// Count the number of set lanes.
    pub fn count(self) -> u32 {
        self.0.count_ones()
    }
}

// ---------------------------------------------------------------------------
// Need9 — zero-size typed decomposition signal
// ---------------------------------------------------------------------------

/// Zero-size typed decomposition signal: emitted when a CONSTRUCT8 structure
/// must be split because it would exceed eight lanes.
pub struct Need9;

// ---------------------------------------------------------------------------
// C8Error / C8Result
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum C8Error {
    #[error("CONSTRUCT8 lane limit exceeded -- decompose (Need9)")]
    Need9,

    #[error("invalid lane index: {0}")]
    InvalidLane(u8),

    #[error("mask overflow on lane {0}")]
    MaskOverflow(u8),

    #[error("receipt mismatch: expected {expected}, actual {actual}")]
    ReceiptMismatch { expected: u64, actual: u64 },

    #[error("verification failure: {0}")]
    VerificationFailure(String),

    #[error("invalid slot index: {0}")]
    InvalidSlot(u8),

    #[error("invalid operation: {0}")]
    InvalidOperation(String),
}

pub type C8Result<T> = Result<T, C8Error>;

// ---------------------------------------------------------------------------
// HotPathVerdict — bounded, #[repr(u8)] verdict enum
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HotPathVerdict {
    Admit = 0,
    Refuse = 1,
    Partial = 2,
}

impl HotPathVerdict {
    pub fn is_admit(self) -> bool {
        matches!(self, Self::Admit)
    }

    pub fn is_refuse(self) -> bool {
        matches!(self, Self::Refuse)
    }

    pub fn is_partial(self) -> bool {
        matches!(self, Self::Partial)
    }
}

// ---------------------------------------------------------------------------
// ColdPathExplanation
// ---------------------------------------------------------------------------

pub struct ColdPathExplanation {
    pub verdict: HotPathVerdict,
    pub reason: &'static str,
    pub module: &'static str,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct8_len_accepts_zero() {
        let len = Construct8Len::new(0).expect("0 is valid");
        assert_eq!(len.value(), 0);
        assert!(len.is_empty());
    }

    #[test]
    fn construct8_len_accepts_eight() {
        let len = Construct8Len::new(8).expect("8 is valid");
        assert_eq!(len.value(), 8);
        assert!(len.is_full());
    }

    #[test]
    fn construct8_len_rejects_nine() {
        let result = Construct8Len::new(9);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, C8Error::Need9);
    }

    #[test]
    fn need9_is_typed_not_string() {
        // C8Error::Need9 is a typed enum variant, not a string
        match C8Error::Need9 {
            C8Error::Need9 => {} // typed match — no string comparison
            _ => panic!("unexpected variant"),
        }
    }

    #[test]
    fn hot_path_verdict_has_no_string_variant() {
        // Exhaustive match proves the enum is closed / bounded
        let verdicts = [
            HotPathVerdict::Admit,
            HotPathVerdict::Refuse,
            HotPathVerdict::Partial,
        ];
        for v in verdicts {
            match v {
                HotPathVerdict::Admit => assert!(v.is_admit()),
                HotPathVerdict::Refuse => assert!(v.is_refuse()),
                HotPathVerdict::Partial => assert!(v.is_partial()),
            }
        }
    }

    #[test]
    fn mask_operations() {
        let mask = Construct8Mask::EMPTY.set(0).set(3).set(7);
        assert!(mask.has(0));
        assert!(mask.has(3));
        assert!(mask.has(7));
        assert!(!mask.has(1));
        assert_eq!(mask.count(), 3);
    }

    #[test]
    #[should_panic(expected = "lane index must be 0..=7")]
    fn mask_set_out_of_bounds_panics() {
        let _ = Construct8Mask::EMPTY.set(8);
    }

    #[test]
    #[should_panic(expected = "lane index must be 0..=7")]
    fn mask_has_out_of_bounds_panics() {
        let _ = Construct8Mask::EMPTY.has(8);
    }
}
