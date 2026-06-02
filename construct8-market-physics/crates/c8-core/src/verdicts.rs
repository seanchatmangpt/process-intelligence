//! # Verdicts Module
//!
//! Bounded decision outcomes for the hot path. No open-ended strings, no catch-alls.
//! Every verdict is named and auditable.

/// HotPathVerdict: A bounded decision outcome for real-time constraint validation.
///
/// The hot path makes exactly four distinct verdicts, no more. Each verdict is a
/// closed enum variant—never a string, never dynamic. This ensures all decisions
/// are auditable and typed.
///
/// # Variants
///
/// - **Accept**: Constraint is satisfied; proceed.
/// - **Reject**: Constraint is violated; refuse the order or transaction.
/// - **DeferToWarm**: Insufficient information for a hot verdict; escalate to warm path.
/// - **NeedNineInformed**: The decision requires nine or more dimensions; beyond scope.
///
/// # Why This Is Bounded
///
/// Logic-chaos thrives on open-ended verdicts (string results, catch-all variants).
/// `HotPathVerdict` is **closed**: only four cases exist. Every verdict path is known,
/// testable, and auditable. Adding a fifth variant requires a deliberate code change
/// and review—not a silent string addition.
///
/// # Examples
///
/// ```
/// use c8_core::HotPathVerdict;
///
/// let verdict = HotPathVerdict::Accept;
/// match verdict {
///     HotPathVerdict::Accept => println!("Proceed"),
///     HotPathVerdict::Reject => println!("Stop"),
///     HotPathVerdict::DeferToWarm => println!("Escalate"),
///     HotPathVerdict::NeedNineInformed => println!("Out of scope"),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotPathVerdict {
    /// Constraint satisfied; proceed with order execution.
    Accept,

    /// Constraint violated; refuse order.
    Reject,

    /// Hot path is inconclusive; defer to warm path analysis.
    DeferToWarm,

    /// Decision space exceeds eight dimensions; not handled by hot path.
    NeedNineInformed,
}

impl HotPathVerdict {
    /// Check if this verdict is an acceptance.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::HotPathVerdict;
    ///
    /// let verdict = HotPathVerdict::Accept;
    /// assert!(verdict.is_accept());
    /// ```
    pub fn is_accept(self) -> bool {
        matches!(self, HotPathVerdict::Accept)
    }

    /// Check if this verdict is a rejection.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::HotPathVerdict;
    ///
    /// let verdict = HotPathVerdict::Reject;
    /// assert!(verdict.is_reject());
    /// ```
    pub fn is_reject(self) -> bool {
        matches!(self, HotPathVerdict::Reject)
    }

    /// Check if this verdict defers to warm path.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::HotPathVerdict;
    ///
    /// let verdict = HotPathVerdict::DeferToWarm;
    /// assert!(verdict.is_defer());
    /// ```
    pub fn is_defer(self) -> bool {
        matches!(self, HotPathVerdict::DeferToWarm)
    }

    /// Check if this verdict signals need for higher-dimensional logic.
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::HotPathVerdict;
    ///
    /// let verdict = HotPathVerdict::NeedNineInformed;
    /// assert!(verdict.is_need_nine());
    /// ```
    pub fn is_need_nine(self) -> bool {
        matches!(self, HotPathVerdict::NeedNineInformed)
    }

    /// Check if this verdict is terminal (Accept or Reject).
    ///
    /// # Examples
    ///
    /// ```
    /// use c8_core::HotPathVerdict;
    ///
    /// assert!(HotPathVerdict::Accept.is_terminal());
    /// assert!(HotPathVerdict::Reject.is_terminal());
    /// assert!(!HotPathVerdict::DeferToWarm.is_terminal());
    /// ```
    pub fn is_terminal(self) -> bool {
        matches!(self, HotPathVerdict::Accept | HotPathVerdict::Reject)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verdict_accept() {
        let v = HotPathVerdict::Accept;
        assert!(v.is_accept());
        assert!(!v.is_reject());
        assert!(v.is_terminal());
    }

    #[test]
    fn test_verdict_reject() {
        let v = HotPathVerdict::Reject;
        assert!(v.is_reject());
        assert!(!v.is_accept());
        assert!(v.is_terminal());
    }

    #[test]
    fn test_verdict_defer() {
        let v = HotPathVerdict::DeferToWarm;
        assert!(v.is_defer());
        assert!(!v.is_terminal());
    }

    #[test]
    fn test_verdict_need_nine() {
        let v = HotPathVerdict::NeedNineInformed;
        assert!(v.is_need_nine());
        assert!(!v.is_terminal());
    }

    #[test]
    fn test_verdict_exhaustive() {
        let verdicts = [
            HotPathVerdict::Accept,
            HotPathVerdict::Reject,
            HotPathVerdict::DeferToWarm,
            HotPathVerdict::NeedNineInformed,
        ];
        assert_eq!(verdicts.len(), 4);
        for v in &verdicts {
            // Each verdict is distinct
            for v2 in &verdicts {
                if std::ptr::eq(v, v2) {
                    assert_eq!(v, v2);
                } else {
                    assert_ne!(v, v2);
                }
            }
        }
    }

    #[test]
    fn test_verdict_match_all_arms() {
        let v = HotPathVerdict::Accept;
        match v {
            HotPathVerdict::Accept => (),
            HotPathVerdict::Reject => unreachable!(),
            HotPathVerdict::DeferToWarm => unreachable!(),
            HotPathVerdict::NeedNineInformed => unreachable!(),
        }
    }
}
