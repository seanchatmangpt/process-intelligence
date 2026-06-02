//! # Hot Path Module
//!
//! Real-time constraint validation and decision logic for Construct8.
//! This module provides the core hot path execution primitives.

use crate::bounds::Construct8Len;
use crate::verdicts::HotPathVerdict;

/// ColdPathExplanation: Reasoning attached to non-terminal hot verdicts.
///
/// When the hot path cannot reach a terminal verdict (Accept or Reject),
/// it defers to the cold (warm) path with an explanation. This type
/// carries the reasoning in a bounded, auditable form.
///
/// # Variants
///
/// - **InsufficientConstraintData**: The hot path has incomplete information.
/// - **LoopDetected**: A circular dependency or constraint loop was detected.
/// - **MultipleFeasibleSolutions**: More than one solution satisfies constraints.
/// - **EscalateToWarmPath**: Generic escalation signal.
///
/// # Examples
///
/// ```
/// use c8_core::ColdPathExplanation;
///
/// let reason = ColdPathExplanation::InsufficientConstraintData;
/// match reason {
///     ColdPathExplanation::InsufficientConstraintData => println!("Need more info"),
///     _ => {}
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColdPathExplanation {
    /// Insufficient data to reach a terminal verdict.
    InsufficientConstraintData,

    /// A circular dependency or loop was detected.
    LoopDetected,

    /// Multiple feasible solutions exist; cold path must select one.
    MultipleFeasibleSolutions,

    /// Generic escalation to warm path.
    EscalateToWarmPath,
}

impl ColdPathExplanation {
    /// Human-readable label for this explanation.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::ColdPathExplanation;
    ///
    /// let reason = ColdPathExplanation::LoopDetected;
    /// assert_eq!(reason.label(), "LoopDetected");
    /// ```
    pub fn label(&self) -> &'static str {
        match self {
            ColdPathExplanation::InsufficientConstraintData => "InsufficientConstraintData",
            ColdPathExplanation::LoopDetected => "LoopDetected",
            ColdPathExplanation::MultipleFeasibleSolutions => "MultipleFeasibleSolutions",
            ColdPathExplanation::EscalateToWarmPath => "EscalateToWarmPath",
        }
    }

    /// Check if this is insufficient data.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::ColdPathExplanation;
    ///
    /// let reason = ColdPathExplanation::InsufficientConstraintData;
    /// assert!(reason.is_insufficient_data());
    /// ```
    pub fn is_insufficient_data(&self) -> bool {
        matches!(self, ColdPathExplanation::InsufficientConstraintData)
    }

    /// Check if this is a loop detection.
    pub fn is_loop_detected(&self) -> bool {
        matches!(self, ColdPathExplanation::LoopDetected)
    }

    /// Check if this is multiple solutions.
    pub fn is_multiple_solutions(&self) -> bool {
        matches!(self, ColdPathExplanation::MultipleFeasibleSolutions)
    }
}

/// HotPathResult: Outcome of a single hot path evaluation.
///
/// A hot path evaluation yields either a terminal verdict (Accept/Reject)
/// or a deferral to the cold path with an explanation.
///
/// # Examples
///
/// ```
/// use c8_core::{HotPathVerdict, ColdPathExplanation};
///
/// let verdict = HotPathVerdict::Accept;
/// assert!(verdict.is_terminal());
/// ```
pub type HotPathResult = Result<HotPathVerdict, ColdPathExplanation>;

/// HotPathContext: The minimal state needed for hot path decisions.
///
/// This struct bundles the constraint count and the slot mask, the only
/// information needed for fast, real-time decisions.
///
/// # Examples
///
/// ```
/// use c8_core::{HotPathContext, Construct8Len, Construct8Mask};
///
/// let context = HotPathContext {
///     constraint_count: Construct8Len::new(4).expect("valid"),
///     slot_mask: Construct8Mask::from_bits(0x0F),
/// };
/// assert_eq!(context.constraint_count.value(), 4);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct HotPathContext {
    /// Number of active constraints (0–8).
    pub constraint_count: Construct8Len,

    /// Bitmask of active slots.
    pub slot_mask: crate::bounds::Construct8Mask,
}

impl HotPathContext {
    /// Create a new hot path context.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::{HotPathContext, Construct8Len, Construct8Mask};
    ///
    /// let context = HotPathContext::new(
    ///     Construct8Len::new(3).expect("valid"),
    ///     Construct8Mask::from_bits(0x07),
    /// ).expect("valid context");
    /// assert!(!context.is_empty());
    /// ```
    pub fn new(count: Construct8Len, mask: crate::bounds::Construct8Mask) -> crate::C8Result<Self> {
        // Basic validation: mask bit count should match count
        if mask.count_set() > count.value() {
            return Err(crate::C8Error::NeedNine); // Escalate on mismatch
        }
        Ok(HotPathContext {
            constraint_count: count,
            slot_mask: mask,
        })
    }

    /// Check if context is empty (no constraints).
    pub fn is_empty(&self) -> bool {
        self.constraint_count.is_empty()
    }

    /// Check if context is at maximum capacity.
    pub fn is_full(&self) -> bool {
        self.constraint_count.is_full()
    }
}

#[cfg(test)]
#[allow(clippy::unnecessary_literal_unwrap)]
mod tests {
    use super::*;
    use crate::bounds::Construct8Mask;

    #[test]
    fn test_cold_path_explanation_insufficient_data() {
        let reason = ColdPathExplanation::InsufficientConstraintData;
        assert!(reason.is_insufficient_data());
        assert_eq!(reason.label(), "InsufficientConstraintData");
    }

    #[test]
    fn test_cold_path_explanation_loop() {
        let reason = ColdPathExplanation::LoopDetected;
        assert!(reason.is_loop_detected());
    }

    #[test]
    fn test_cold_path_explanation_multiple() {
        let reason = ColdPathExplanation::MultipleFeasibleSolutions;
        assert!(reason.is_multiple_solutions());
    }

    #[test]
    fn test_hot_path_context_creation() {
        let context = HotPathContext::new(
            Construct8Len::new(4).expect("valid"),
            Construct8Mask::from_bits(0x0F),
        )
        .expect("valid context");
        assert_eq!(context.constraint_count.value(), 4);
        assert!(!context.is_empty());
        assert!(!context.is_full());
    }

    #[test]
    fn test_hot_path_context_empty() {
        let context = HotPathContext::new(
            Construct8Len::new(0).expect("valid"),
            Construct8Mask::from_bits(0x00),
        )
        .expect("valid context");
        assert!(context.is_empty());
    }

    #[test]
    fn test_hot_path_context_full() {
        let context = HotPathContext::new(
            Construct8Len::new(8).expect("valid"),
            Construct8Mask::from_bits(0xFF),
        )
        .expect("valid context");
        assert!(context.is_full());
    }

    #[test]
    fn test_hot_path_result_accept() {
        let result: HotPathResult = Ok(HotPathVerdict::Accept);
        assert!(result.is_ok());
        assert!(result.unwrap().is_accept());
    }

    #[test]
    fn test_hot_path_result_defer() {
        let result: HotPathResult = Err(ColdPathExplanation::InsufficientConstraintData);
        assert!(result.is_err());
    }
}
