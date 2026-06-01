# Type Crosswalk: Declare Constraints to wasm4pm Models

This document defines the mapping between declarative templates in Declare (expressed via Linear Temporal Logic) and the concrete type-safe data models executed within the `wasm4pm` WebAssembly virtual machine.

---

## 1. Declarative Type Mapping Matrix

| Declare Template | LTL Representation | wasm4pm Rust Witness Representer | Initial State | Terminal Compliant Condition |
|---|---|---|---|---|
| **Precedence** | $\mathbf{G}(B \implies \mathbf{F}^{-1}(A))$ | `WitnessPrecedence` | `a_seen: false` | `true` (Always compliant if not explicitly violated) |
| **Response** | $\mathbf{G}(A \implies \mathbf{F}(B))$ | `WitnessResponse` | `pending_b_count: 0` | `pending_b_count == 0` |
| **Coexistence** | $\mathbf{F}(A) \leftrightarrow \mathbf{F}(B)$ | `WitnessCoexistence` | `a_seen: false, b_seen: false` | `a_seen == b_seen` |
| **Succession** | $\mathbf{G}(A \implies \mathbf{F}(B)) \land \mathbf{G}(B \implies \mathbf{F}^{-1}(A))$ | `WitnessSuccession` | `pending_a_for_b: 0, violated: false` | `pending_a_for_b == 0 && violated == false` |

---

## 2. Concrete Rust Structures and Transition Functions

The `wasm4pm` runtime processes events sequentially. For a trace $\sigma$, each event $e$ containing activity $x$ updates the witness state $W$ via a step transition function.

### 2.1 Precedence Constraint
The `precedence(A, B)` template requires that activity $B$ cannot fire unless activity $A$ has occurred at least once previously.

**Struct Definition:**
```rust
pub struct WitnessPrecedence {
    pub a_seen: bool,
    pub violated: bool,
}
```

**Step Transition Function:**
```rust
impl WitnessPrecedence {
    pub fn step(&mut self, activity: &str, a: &str, b: &str) {
        if self.violated {
            return;
        }
        if activity == a {
            self.a_seen = true;
        } else if activity == b {
            if !self.a_seen {
                self.violated = true; // B fired before A was seen
            }
        }
    }
}
```

### 2.2 Response Constraint
The `response(A, B)` template requires that if activity $A$ occurs, activity $B$ must eventually occur afterward. Multiple occurrences of $A$ accumulate pending responses.

**Struct Definition:**
```rust
pub struct WitnessResponse {
    pub pending_b_count: u32,
}
```

**Step Transition Function:**
```rust
impl WitnessResponse {
    pub fn step(&mut self, activity: &str, a: &str, b: &str) {
        if activity == a {
            self.pending_b_count = self.pending_b_count.saturating_add(1);
        } else if activity == b {
            if self.pending_b_count > 0 {
                self.pending_b_count -= 1;
            }
        }
    }
}
```

### 2.3 Coexistence Constraint
The `coexistence(A, B)` template requires that if either $A$ or $B$ occurs, then both must occur in the trace.

**Struct Definition:**
```rust
pub struct WitnessCoexistence {
    pub a_seen: bool,
    pub b_seen: bool,
}
```

**Step Transition Function:**
```rust
impl WitnessCoexistence {
    pub fn step(&mut self, activity: &str, a: &str, b: &str) {
        if activity == a {
            self.a_seen = true;
        } else if activity == b {
            self.b_seen = true;
        }
    }

    pub fn is_compliant(&self) -> bool {
        self.a_seen == self.b_seen
    }
}
```

### 2.4 Succession Constraint
The `succession(A, B)` template combines precedence and response. It requires that $B$ occurs if and only if $A$ occurs before it. This means $B$ cannot fire without a preceding $A$, and every $A$ must eventually be followed by a $B$.

**Struct Definition:**
```rust
pub struct WitnessSuccession {
    pub pending_a_for_b: u32,
    pub violated: bool,
}
```

**Step Transition Function:**
```rust
impl WitnessSuccession {
    pub fn step(&mut self, activity: &str, a: &str, b: &str) {
        if self.violated {
            return;
        }
        if activity == a {
            self.pending_a_for_b = self.pending_a_for_b.saturating_add(1);
        } else if activity == b {
            if self.pending_a_for_b > 0 {
                self.pending_a_for_b -= 1;
            } else {
                self.violated = true; // B occurred without a preceding A (precedence violation)
            }
        }
    }

    pub fn is_compliant(&self) -> bool {
        !self.violated && self.pending_a_for_b == 0
    }
}
```

---

## 3. Lattice Merge (Join) Operations

To support distributed process mining, independent witness states can be merged. The merge operation corresponds to the join ($\sqcup$) in the witness semilattice.

```rust
impl WitnessSuccession {
    pub fn join(&self, other: &Self) -> Self {
        Self {
            pending_a_for_b: self.pending_a_for_b.max(other.pending_a_for_b),
            violated: self.violated || other.violated,
        }
    }
}
```

If the join produces a state where `violated == true`, the runtime rejects the combined evidence chain as a `LatticeViolation`.

---

## 4. Related System Reference Documents

Refer to these specifications for more details on mapping declarations and data execution:
- [Declare Compliance Standard Reference](file:///Users/sac/process-intelligence/standards/declare.md) - Contains LTL definitions and templates.
- [Witness Lattices Specification](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/witness-lattices.md) - Outlines formal algebraic laws for join/meet operations.
- [Type-Law Atlas](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/type-law-atlas.md) - Details evidence structures and WebAssembly boundaries.
