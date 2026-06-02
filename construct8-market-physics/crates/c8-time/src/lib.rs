//! # c8-time
//!
//! Vector clock and monotonic time engine for CONSTRUCT8 market physics.
//!
//! Provides 8-lane vector clocks for causal ordering, a strictly monotonic
//! wall-clock stamp backed by an atomic counter, and causal observation
//! alignment for distributed market traces.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

// Keep c8-core in scope so the path dep is used (avoids unused-dep warning).
use c8_core as _;

// ---------------------------------------------------------------------------
// VectorClockCompare
// ---------------------------------------------------------------------------

/// Causal comparison outcome between two vector clocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorClockCompare {
    /// self causally precedes other (self <= other component-wise, not equal)
    Before,
    /// self causally follows other (other <= self component-wise, not equal)
    After,
    /// Neither precedes the other; events are causally independent
    Concurrent,
    /// Clocks are identical in every lane
    Equal,
}

// ---------------------------------------------------------------------------
// VectorClock8
// ---------------------------------------------------------------------------

/// Strictly 8-lane vector clock for causal precedence tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VectorClock8 {
    pub lanes: [u64; 8],
}

impl VectorClock8 {
    /// Create a zero-initialized vector clock.
    pub fn zero() -> Self {
        VectorClock8 { lanes: [0u64; 8] }
    }

    /// Increment the logical counter at the given lane index.
    ///
    /// # Panics
    /// Panics if `lane >= 8`.
    pub fn tick_lane(&mut self, lane: usize) {
        assert!(lane < 8, "lane index must be < 8, got {lane}");
        self.lanes[lane] = self.lanes[lane].saturating_add(1);
    }

    /// Return a new clock that is the component-wise maximum of `self` and `other`.
    #[allow(clippy::needless_range_loop)]
    pub fn merge(&self, other: &Self) -> Self {
        let mut result = [0u64; 8];
        for i in 0..8 {
            result[i] = self.lanes[i].max(other.lanes[i]);
        }
        VectorClock8 { lanes: result }
    }

    /// Compare two vector clocks using the standard happens-before relation.
    ///
    /// - `Equal`      — all components equal
    /// - `Before`     — self <= other in every lane (and not equal)
    /// - `After`      — other <= self in every lane (and not equal)
    /// - `Concurrent` — neither dominates the other
    pub fn compare(&self, other: &Self) -> VectorClockCompare {
        // self <= other iff every self.lanes[i] <= other.lanes[i]
        let self_le_other = self
            .lanes
            .iter()
            .zip(other.lanes.iter())
            .all(|(a, b)| a <= b);
        // other <= self iff every other.lanes[i] <= self.lanes[i]
        let other_le_self = other
            .lanes
            .iter()
            .zip(self.lanes.iter())
            .all(|(a, b)| a <= b);

        match (self_le_other, other_le_self) {
            (true, true) => VectorClockCompare::Equal,
            (true, false) => VectorClockCompare::Before,
            (false, true) => VectorClockCompare::After,
            (false, false) => VectorClockCompare::Concurrent,
        }
    }

    /// Return a reference to the underlying lane array.
    pub fn lanes(&self) -> &[u64; 8] {
        &self.lanes
    }
}

// ---------------------------------------------------------------------------
// MonotonicStamp
// ---------------------------------------------------------------------------

/// Strictly monotonic nanosecond timestamp backed by an atomic u64.
///
/// `now_ns` is guaranteed to never decrease: if the measured elapsed time
/// is less than the last recorded value, the last recorded value is returned.
pub struct MonotonicStamp {
    epoch: Instant,
    last_ns: AtomicU64,
}

impl std::fmt::Debug for MonotonicStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MonotonicStamp")
            .field("last_ns", &self.last_ns.load(Ordering::Relaxed))
            .finish()
    }
}

impl MonotonicStamp {
    /// Create a new monotonic stamp anchored at the current instant.
    pub fn new() -> Self {
        MonotonicStamp {
            epoch: Instant::now(),
            last_ns: AtomicU64::new(0),
        }
    }

    /// Return the current monotonic nanosecond offset from the epoch.
    ///
    /// Guaranteed to be non-decreasing: will return at least the previously
    /// observed value even if the OS-level elapsed time appears to regress.
    pub fn now_ns(&self) -> u64 {
        let elapsed = self.epoch.elapsed().as_nanos() as u64;
        // Atomically update: new value is max(elapsed, last_ns)
        let mut prev = self.last_ns.load(Ordering::Relaxed);
        loop {
            let candidate = elapsed.max(prev);
            match self.last_ns.compare_exchange_weak(
                prev,
                candidate,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(_) => return candidate,
                Err(actual) => prev = actual,
            }
        }
    }

    /// Returns `true` if the current monotonic time is at least `prior_ns`.
    pub fn assert_not_before(&self, prior_ns: u64) -> bool {
        self.now_ns() >= prior_ns
    }
}

impl Default for MonotonicStamp {
    fn default() -> Self {
        MonotonicStamp::new()
    }
}

// ---------------------------------------------------------------------------
// CausalObservation
// ---------------------------------------------------------------------------

/// A single causal observation combining a vector clock with a monotonic timestamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CausalObservation {
    pub vector_clock: VectorClock8,
    pub monotonic_ns: u64,
    pub actor_lane: usize,
}

impl CausalObservation {
    /// Align two causal observations and return their vector clock relationship.
    ///
    /// Compares `prior`'s vector clock against `incoming`'s vector clock using
    /// the standard happens-before relation.
    pub fn align(prior: &Self, incoming: &Self) -> VectorClockCompare {
        prior.vector_clock.compare(&incoming.vector_clock)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_clocks_are_equal() {
        let a = VectorClock8::zero();
        let b = VectorClock8::zero();
        assert_eq!(a.compare(&b), VectorClockCompare::Equal);
    }

    #[test]
    fn tick_lane_creates_causal_after() {
        let mut a = VectorClock8::zero();
        let b = VectorClock8::zero();
        a.tick_lane(0);
        // a is causally after b (a > b in lane 0)
        assert_eq!(a.compare(&b), VectorClockCompare::After);
        // b is causally before a
        assert_eq!(b.compare(&a), VectorClockCompare::Before);
    }

    #[test]
    fn independent_lane_ticks_are_concurrent() {
        let mut a = VectorClock8::zero();
        let mut b = VectorClock8::zero();
        a.tick_lane(0);
        b.tick_lane(1);
        assert_eq!(a.compare(&b), VectorClockCompare::Concurrent);
        assert_eq!(b.compare(&a), VectorClockCompare::Concurrent);
    }

    #[test]
    fn merge_dominates_both_prior_clocks() {
        let mut a = VectorClock8::zero();
        let mut b = VectorClock8::zero();
        a.tick_lane(0);
        b.tick_lane(1);
        let merged = a.merge(&b);
        // merged >= a and merged >= b in every lane, so merged is After both
        assert_eq!(merged.compare(&a), VectorClockCompare::After);
        assert_eq!(merged.compare(&b), VectorClockCompare::After);
    }

    #[test]
    fn monotonic_time_never_regresses() {
        let stamp = MonotonicStamp::new();
        let t1 = stamp.now_ns();
        let t2 = stamp.now_ns();
        let t3 = stamp.now_ns();
        assert!(t2 >= t1, "t2={t2} should be >= t1={t1}");
        assert!(t3 >= t2, "t3={t3} should be >= t2={t2}");
    }

    #[test]
    fn causal_align_distinguishes_concurrent_from_ordered() {
        // Ordered: b comes strictly after a (b starts from a's state, then advances)
        let mut vc_a = VectorClock8::zero();
        vc_a.tick_lane(0);
        let mut vc_b = vc_a; // b starts from a's state
        vc_b.tick_lane(0); // b advances lane 0 further

        let obs_a = CausalObservation {
            vector_clock: vc_a,
            monotonic_ns: 100,
            actor_lane: 0,
        };
        let obs_b = CausalObservation {
            vector_clock: vc_b,
            monotonic_ns: 200,
            actor_lane: 0,
        };
        // prior=a, incoming=b: a is Before b
        assert_eq!(
            CausalObservation::align(&obs_a, &obs_b),
            VectorClockCompare::Before
        );

        // Concurrent: c and d each tick an independent lane
        let mut vc_c = VectorClock8::zero();
        let mut vc_d = VectorClock8::zero();
        vc_c.tick_lane(2);
        vc_d.tick_lane(5);

        let obs_c = CausalObservation {
            vector_clock: vc_c,
            monotonic_ns: 300,
            actor_lane: 2,
        };
        let obs_d = CausalObservation {
            vector_clock: vc_d,
            monotonic_ns: 400,
            actor_lane: 5,
        };
        assert_eq!(
            CausalObservation::align(&obs_c, &obs_d),
            VectorClockCompare::Concurrent
        );
    }
}
