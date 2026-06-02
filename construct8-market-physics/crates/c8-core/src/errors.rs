//! # Errors Module
//!
//! Typed refusal signals for c8-core. Every error is a concrete enum variant,
//! not a string message. Refusals are **evidence**, not exceptions.

/// C8Error: The typed refusal set for Construct8.
///
/// `C8Error` is a closed enum of named refusal reasons. There is no string-based
/// error path, no catch-all, no silent defaults. Every error variant names a
/// specific structural or semantic law violation.
///
/// # Variants
///
/// - **NeedNine**: The operation requires nine or more dimensions; beyond eight-slot scope.
///   This is *not* a failure—it is a **signal** that the decision must escalate.
///
/// # Why This Is Typed
///
/// String-based errors ("NeedNine", "invalid input", etc.) allow silent logic-chaos:
/// typos propagate, handlers miss cases, error codes are confused. Rust enums force
/// exhaustive matching. Every error is named, every handler is explicit.
///
/// # Examples
///
/// ```
/// use c8_core::{Construct8Len, C8Error};
///
/// let result = Construct8Len::new(9);
/// assert!(result.is_err());
/// assert_eq!(result, Err(C8Error::NeedNine));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum C8Error {
    /// Operation requires nine or more dimensions; escalate decision.
    NeedNine,
    /// Invalid GraphSlot accessed.
    InvalidSlot(usize),
    /// Assertion or structural verification failure.
    VerificationFailure(String),
}

impl C8Error {
    /// Human-readable label for this error.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::C8Error;
    ///
    /// assert_eq!(C8Error::NeedNine.label(), "NeedNine");
    /// ```
    pub fn label(&self) -> &'static str {
        match self {
            C8Error::NeedNine => "NeedNine",
            C8Error::InvalidSlot(_) => "InvalidSlot",
            C8Error::VerificationFailure(_) => "VerificationFailure",
        }
    }

    /// Check if this error is a NeedNine signal.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::C8Error;
    ///
    /// let err = C8Error::NeedNine;
    /// assert!(err.is_need_nine());
    /// ```
    pub fn is_need_nine(&self) -> bool {
        matches!(self, C8Error::NeedNine)
    }
}

impl std::fmt::Display for C8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "C8Error::{}", self.label())
    }
}

impl std::error::Error for C8Error {}

/// C8Result: A type alias for Result with C8Error as the error type.
///
/// # Examples
///
/// ```
/// use c8_core::{C8Result, Construct8Len};
///
/// fn validate(n: u8) -> C8Result<u8> {
///     let _len = Construct8Len::new(n)?;
///     Ok(n)
/// }
///
/// assert!(validate(5).is_ok());
/// assert!(validate(9).is_err());
/// ```
pub type C8Result<T> = Result<T, C8Error>;

/// NeedNine: A typed refusal marker (alias for clarity in some contexts).
///
/// This is not a separate type; it is the `C8Error::NeedNine` variant.
/// Provided as a conceptual anchor for developers who reason about
/// "escalation signals" separately from "errors".
///
/// # Examples
///
/// ```
/// use c8_core::C8Error;
///
/// type NeedNine = C8Error;
/// let signal = NeedNine::NeedNine;
/// assert!(signal.is_need_nine());
/// ```
pub type NeedNine = C8Error;

#[cfg(test)]
#[allow(clippy::unnecessary_literal_unwrap)]
mod tests {
    use super::*;

    #[test]
    fn test_c8_error_need_nine() {
        let err = C8Error::NeedNine;
        assert!(err.is_need_nine());
    }

    #[test]
    fn test_c8_error_label() {
        assert_eq!(C8Error::NeedNine.label(), "NeedNine");
    }

    #[test]
    fn test_c8_error_display() {
        let err = C8Error::NeedNine;
        assert_eq!(err.to_string(), "C8Error::NeedNine");
    }

    #[test]
    fn test_c8_error_is_error_trait() {
        use std::error::Error;
        let err: Box<dyn Error> = Box::new(C8Error::NeedNine);
        assert_eq!(err.to_string(), "C8Error::NeedNine");
    }

    #[test]
    fn test_c8_result_ok() {
        let result: C8Result<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_c8_result_err() {
        let result: C8Result<i32> = Err(C8Error::NeedNine);
        assert!(result.is_err());
    }

    #[test]
    fn test_need_nine_alias() {
        let signal: NeedNine = C8Error::NeedNine;
        assert!(signal.is_need_nine());
    }
}
