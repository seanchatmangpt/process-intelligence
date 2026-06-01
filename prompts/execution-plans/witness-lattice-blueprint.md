# Witness Lattice Blueprint v30.1.1
## wasm4pm-compat

### 1. Mathematical Foundation

The `WitnessState` in `wasm4pm-compat` is modeled as a bounded join-semilattice $L = (S, \vee, \bot, \top)$.

*   **Set $S$**: The set of all possible witness states.
*   **Join Operation ($\vee$)**: The least upper bound (LUB) operation, merging two witness states into a state that encapsulates the knowledge of both. It must be commutative, associative, and idempotent.
*   **Bottom ($\bot$)**: The state of zero information (Uninitialized / Empty).
*   **Top ($\top$)**: The state of contradiction or maximal information overflow.

### 2. `WitnessState` Structure

A single `WitnessState` represents a set of verified computational facts or state transitions.

```rust
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitnessState {
    Bottom,
    Partial(BTreeSet<Fact>),
    Top,
}

pub trait Join {
    fn join(self, other: Self) -> Self;
}

impl Join for WitnessState {
    fn join(self, other: Self) -> Self {
        match (self, other) {
            (WitnessState::Top, _) | (_, WitnessState::Top) => WitnessState::Top,
            (WitnessState::Bottom, x) | (x, WitnessState::Bottom) => x,
            (WitnessState::Partial(a), WitnessState::Partial(b)) => {
                let merged: BTreeSet<_> = a.union(&b).cloned().collect();
                // Assumes a check_contradictions(&merged) exists.
                // If it contradicts, it moves to Top.
                WitnessState::Partial(merged)
            }
        }
    }
}
```

### 3. `UnifiedWitnessState` Structure

The `UnifiedWitnessState` represents the global state, constructed by the join of multiple regional `WitnessState`s.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnifiedWitnessState {
    pub state: WitnessState,
    pub vector_clock: VectorClock,
}

impl Join for UnifiedWitnessState {
    fn join(self, other: Self) -> Self {
        UnifiedWitnessState {
            state: self.state.join(other.state),
            vector_clock: self.vector_clock.merge(&other.vector_clock),
        }
    }
}
```
