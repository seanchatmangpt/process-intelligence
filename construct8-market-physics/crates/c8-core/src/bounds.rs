//! # Bounds Module
//!
//! Type-safe boundaries for Construct8: the length constraint (0–8 only) and
//! bit-mask representation for the eight core slots.

use crate::errors::{C8Error, C8Result};

/// Construct8Len: A bounded length type (0–8 inclusive).
///
/// `Construct8Len` enforces the architectural constraint that Construct8 operates
/// on exactly eight logical slots. This type rejects any value outside [0, 8].
///
/// Values 0–8 are lawful; 9+ are refusals emitting `NeedNine`.
///
/// # Why 0–8?
///
/// - **0** (empty): System with no constraints.
/// - **1–8** (partial to full): One through eight slots populated.
/// - **9+** (overflow): Requires a different architecture (future extension).
///
/// # Examples
///
/// ```
/// use c8_core::Construct8Len;
/// use c8_core::C8Error;
///
/// let len = Construct8Len::new(5).expect("5 is valid");
/// assert_eq!(len.value(), 5);
///
/// let result = Construct8Len::new(9);
/// assert!(matches!(result, Err(C8Error::NeedNine)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Construct8Len(u8);

impl Construct8Len {
    /// Create a new `Construct8Len`, rejecting values outside [0, 8].
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::Construct8Len;
    ///
    /// let len = Construct8Len::new(3).expect("3 is valid");
    /// assert_eq!(len.value(), 3);
    /// ```
    pub fn new(value: u8) -> C8Result<Self> {
        if value <= 8 {
            Ok(Construct8Len(value))
        } else {
            Err(C8Error::NeedNine)
        }
    }

    /// Retrieve the inner value.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::Construct8Len;
    ///
    /// let len = Construct8Len::new(7).expect("valid");
    /// assert_eq!(len.value(), 7);
    /// ```
    pub fn value(self) -> u8 {
        self.0
    }

    /// Check if this length is zero (empty).
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::Construct8Len;
    ///
    /// let empty = Construct8Len::new(0).expect("valid");
    /// assert!(empty.is_empty());
    ///
    /// let full = Construct8Len::new(8).expect("valid");
    /// assert!(!full.is_empty());
    /// ```
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Check if this length is at maximum (8).
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::Construct8Len;
    ///
    /// let full = Construct8Len::new(8).expect("valid");
    /// assert!(full.is_full());
    ///
    /// let partial = Construct8Len::new(5).expect("valid");
    /// assert!(!partial.is_full());
    /// ```
    pub fn is_full(self) -> bool {
        self.0 == 8
    }
}

/// Construct8Mask: A bitmask representation of occupied slots (0–255).
///
/// `Construct8Mask` represents which of the eight slots are populated using a single byte.
/// Bit `i` (0–7) is set if slot `i` is active.
///
/// # Examples
///
/// ```
/// use c8_core::Construct8Mask;
///
/// let mask = Construct8Mask::from_bits(0b10101010);
/// assert_eq!(mask.bits(), 0b10101010);
/// assert_eq!(mask.count_set(), 4);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Construct8Mask(u8);

impl Construct8Mask {
    /// Create a mask from a raw byte.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::Construct8Mask;
    ///
    /// let mask = Construct8Mask::from_bits(0xFF);
    /// assert_eq!(mask.bits(), 0xFF);
    /// ```
    pub fn from_bits(bits: u8) -> Self {
        Construct8Mask(bits)
    }

    /// Get the underlying bits.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::Construct8Mask;
    ///
    /// let mask = Construct8Mask::from_bits(0b00001111);
    /// assert_eq!(mask.bits(), 0b00001111);
    /// ```
    pub fn bits(self) -> u8 {
        self.0
    }

    /// Count the number of set bits.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::Construct8Mask;
    ///
    /// let mask = Construct8Mask::from_bits(0b10101010);
    /// assert_eq!(mask.count_set(), 4);
    /// ```
    pub fn count_set(self) -> u8 {
        self.0.count_ones() as u8
    }

    /// Check if a specific bit (slot) is set.
    ///
    /// # Panics
    ///
    /// Panics if `slot >= 8`.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::Construct8Mask;
    ///
    /// let mask = Construct8Mask::from_bits(0b00001100);
    /// assert!(mask.is_set(2));
    /// assert!(mask.is_set(3));
    /// assert!(!mask.is_set(0));
    /// ```
    pub fn is_set(self, slot: u8) -> bool {
        assert!(slot < 8, "slot must be 0–7");
        (self.0 & (1 << slot)) != 0
    }

    /// Set a specific bit (slot).
    ///
    /// # Panics
    ///
    /// Panics if `slot >= 8`.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::Construct8Mask;
    ///
    /// let mut mask = Construct8Mask::from_bits(0);
    /// mask = mask.set(0);
    /// mask = mask.set(7);
    /// assert_eq!(mask.bits(), 0b10000001);
    /// ```
    pub fn set(self, slot: u8) -> Self {
        assert!(slot < 8, "slot must be 0–7");
        Construct8Mask(self.0 | (1 << slot))
    }

    /// Clear a specific bit (slot).
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::Construct8Mask;
    ///
    /// let mask = Construct8Mask::from_bits(0xFF).clear(0);
    /// assert_eq!(mask.bits(), 0b11111110);
    /// ```
    pub fn clear(self, slot: u8) -> Self {
        assert!(slot < 8, "slot must be 0–7");
        Construct8Mask(self.0 & !(1 << slot))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct8_len_valid_range() {
        for i in 0..=8 {
            assert!(Construct8Len::new(i).is_ok());
        }
    }

    #[test]
    fn test_construct8_len_boundary() {
        assert!(Construct8Len::new(0).is_ok());
        assert!(Construct8Len::new(8).is_ok());
        assert!(Construct8Len::new(9).is_err());
    }

    #[test]
    fn test_construct8_len_is_empty() {
        let empty = Construct8Len::new(0).expect("valid");
        let not_empty = Construct8Len::new(1).expect("valid");
        assert!(empty.is_empty());
        assert!(!not_empty.is_empty());
    }

    #[test]
    fn test_construct8_len_is_full() {
        let full = Construct8Len::new(8).expect("valid");
        let partial = Construct8Len::new(7).expect("valid");
        assert!(full.is_full());
        assert!(!partial.is_full());
    }

    #[test]
    fn test_construct8_mask_from_bits() {
        let mask = Construct8Mask::from_bits(0b10101010);
        assert_eq!(mask.bits(), 0b10101010);
    }

    #[test]
    fn test_construct8_mask_count_set() {
        let mask = Construct8Mask::from_bits(0b10101010);
        assert_eq!(mask.count_set(), 4);

        let all = Construct8Mask::from_bits(0xFF);
        assert_eq!(all.count_set(), 8);

        let none = Construct8Mask::from_bits(0x00);
        assert_eq!(none.count_set(), 0);
    }

    #[test]
    fn test_construct8_mask_is_set() {
        let mask = Construct8Mask::from_bits(0b00001100);
        assert!(mask.is_set(2));
        assert!(mask.is_set(3));
        assert!(!mask.is_set(0));
        assert!(!mask.is_set(1));
    }

    #[test]
    fn test_construct8_mask_set() {
        let mask = Construct8Mask::from_bits(0x00);
        let mask = mask.set(0).set(7);
        assert_eq!(mask.bits(), 0b10000001);
    }

    #[test]
    fn test_construct8_mask_clear() {
        let mask = Construct8Mask::from_bits(0xFF).clear(0).clear(7);
        assert_eq!(mask.bits(), 0b01111110);
    }
}
