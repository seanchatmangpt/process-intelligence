# Witness Lattices in WASM4PM-Compat

The witness lattice is the core algebraic structure that validates process execution evidence within the compatibility layer of the WASM process mining engine. It establishes a formal, mathematically rigorous foundation for combining partial observations and verifying that trace alignments are consistent and non-forgeable.

---

## 1. Mathematical Formalization

Let a process execution trace be defined over an alphabet of activities $\Sigma$. A process model (e.g., a Petri Net, BPMN workflow, or Process Tree) defines a language of valid execution sequences $\mathcal{L}_{\text{model}} \subseteq \Sigma^*$.

We define a **Witness** $w$ as a cryptographic statement certifying a partial alignment between a sequence of observed events $L_{\text{sub}}$ and a path in the process model $M_{\text{sub}}$. A witness is represented as a tuple:
$$w = (L_{\text{sub}}, M_{\text{sub}}, \gamma_{\text{sub}})$$
where $\gamma_{\text{sub}}$ is the alignment mapping between $L_{\text{sub}}$ and $M_{\text{sub}}$.

We define the set of all witnesses as $W$. We structure $W$ as a bounded join-semilattice $(W, \sqsubseteq, \sqcup, \bot, \top)$ where:
- $\sqsubseteq$ is the partial order representing evidence inclusion and refinement.
- $\sqcup$ is the join operator representing evidence union.
- $\bot$ is the bottom element representing "zero evidence" (empty trace, initial model marking).
- $\top$ is the top element representing "contradiction" (inconsistent evidence, double-execution, or non-conforming state transitions).

### 1.1 The Partial Order ($\sqsubseteq$)
For two witnesses $w_1 = (L_1, M_1, \gamma_1)$ and $w_2 = (L_2, M_2, \gamma_2)$, we state that $w_1 \sqsubseteq w_2$ if and only if:
1. $L_1$ is a prefix or subsequence of $L_2$ ($L_1 \subseteq L_2$).
2. $M_1$ is a prefix or sub-marking path of $M_2$ ($M_1 \subseteq M_2$).
3. The alignment $\gamma_1$ is a sub-alignment of $\gamma_2$ ($\gamma_1 = \gamma_2 \vert_{L_1}$).

This indicates that $w_2$ contains at least all the evidence present in $w_1$, and their alignments are consistent.

### 1.2 The Join Operator ($\sqcup$)
The join of two witnesses $w_1 \sqcup w_2$ represents the synthesis of two sets of execution evidence:
$$w_1 \sqcup w_2 = w_3$$
The join operator is governed by the following rules:
- **Idempotence**: $w \sqcup w = w$
- **Commutativity**: $w_1 \sqcup w_2 = w_2 \sqcup w_1$
- **Associativity**: $(w_1 \sqcup w_2) \sqcup w_3 = w_1 \sqcup (w_2 \sqcup w_3)$
- **Absorption**: $w \sqsubseteq u \implies w \sqcup u = u$

If $w_1$ and $w_2$ contain conflicting information (for example, aligning the same event to two different transitions, or presenting non-serializable token splits in a 1-bounded Petri net), then:
$$w_1 \sqcup w_2 = \top$$

---

## 2. Structural Conformance Integration

The witness lattice interacts directly with process mining models to compute alignment steps:

| Process Model Type | State Representation | Conformance Law | Join Result under Conflict |
|---|---|---|---|
| **Petri Net / WF-net** | Multi-set of places (Marking $M$) | Token game rules ($M \ge \bullet t$) | $\top$ (if marking is invalid or exceeds boundedness limit) |
| **BPMN 2.0** | Gateway state and active token flows | Step transitions across sequence flows | $\top$ (if gateway conditions are violated) |
| **Process Tree** | Operator activation state ($\rightarrow, \times, \wedge$) | Structural block parsing | $\top$ (if sibling blocks are executed concurrently) |
| **POWL** | Partially ordered activity nodes | Partial order alignment checks | $\top$ (if partial order dependency is violated) |

---

## 3. Rust-FFI Implementation Interface

Within the `wasm4pm-compat` crate, the witness lattice is represented via the `Lattice` trait and concrete structs.

```rust
pub trait Lattice: Sized + Eq + Clone {
    /// Return the bottom element (no evidence)
    fn bottom() -> Self;

    /// Return the top element (conflict state)
    fn top() -> Self;

    /// Check if the current witness is top
    fn is_top(&self) -> bool;

    /// Check if the current witness is bottom
    fn is_bottom(&self) -> bool;

    /// Join two witness elements
    fn join(&self, other: &Self) -> Self;

    /// Compare two witness elements in the partial order
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>;
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub enum WitnessState {
    Bottom,
    PartialReplay {
        trace_indices: Vec<usize>,
        marking: Vec<String>, // active place labels
        cost: u32,
    },
    Top,
}

impl Lattice for WitnessState {
    fn bottom() -> Self {
        WitnessState::Bottom
    }

    fn top() -> Self {
        WitnessState::Top
    }

    fn is_top(&self) -> bool {
        matches!(self, WitnessState::Top)
    }

    fn is_bottom(&self) -> bool {
        matches!(self, WitnessState::Bottom)
    }

    fn join(&self, other: &Self) -> Self {
        match (self, other) {
            (WitnessState::Top, _) | (_, WitnessState::Top) => WitnessState::Top,
            (WitnessState::Bottom, any) | (any, WitnessState::Bottom) => any.clone(),
            (WitnessState::PartialReplay { trace_indices: t1, marking: m1, cost: c1 },
             WitnessState::PartialReplay { trace_indices: t2, marking: m2, cost: c2 }) => {
                // If the elements are equal, join must return an equal element (idempotence)
                if t1 == t2 && m1 == m2 && c1 == c2 {
                    return self.clone();
                }

                // Enforce absorption law: if self <= other, return other; if other <= self, return self
                if let Some(ord) = self.partial_cmp(other) {
                    match ord {
                        std::cmp::Ordering::Less | std::cmp::Ordering::Equal => return other.clone(),
                        std::cmp::Ordering::Greater => return self.clone(),
                    }
                }

                // If incomparable, they can only be merged if they represent disjoint event trace indices
                // (e.g. concurrent branches). If there is any overlap, they represent conflicting claims.
                let has_overlap = t1.iter().any(|idx| t2.contains(idx));
                if has_overlap {
                    WitnessState::Top
                } else {
                    let mut merged_indices = t1.clone();
                    merged_indices.extend(t2.iter().copied());
                    merged_indices.sort_unstable();
                    merged_indices.dedup();

                    let mut merged_marking = m1.clone();
                    for place in m2 {
                        if !merged_marking.contains(place) {
                            merged_marking.push(place.clone());
                        }
                    }

                    WitnessState::PartialReplay {
                        trace_indices: merged_indices,
                        marking: merged_marking,
                        cost: c1 + c2, // disjoint costs are additive
                    }
                }
            }
        }
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (WitnessState::Bottom, WitnessState::Bottom) => Some(std::cmp::Ordering::Equal),
            (WitnessState::Bottom, _) => Some(std::cmp::Ordering::Less),
            (_, WitnessState::Bottom) => Some(std::cmp::Ordering::Greater),
            (WitnessState::Top, WitnessState::Top) => Some(std::cmp::Ordering::Equal),
            (WitnessState::Top, _) => Some(std::cmp::Ordering::Greater),
            (_, WitnessState::Top) => Some(std::cmp::Ordering::Less),
            (WitnessState::PartialReplay { trace_indices: t1, marking: m1, cost: c1 },
             WitnessState::PartialReplay { trace_indices: t2, marking: m2, cost: c2 }) => {
                let is_t1_sub = t1.iter().all(|x| t2.contains(x));
                let is_t2_sub = t2.iter().all(|x| t1.contains(x));

                let is_m1_sub = m1.iter().all(|x| m2.contains(x));
                let is_m2_sub = m2.iter().all(|x| m1.contains(x));

                let is_c1_le = c1 <= c2;
                let is_c2_le = c2 <= c1;

                match (is_t1_sub, is_t2_sub, is_m1_sub, is_m2_sub, is_c1_le, is_c2_le) {
                    (true, true, true, true, true, true) => Some(std::cmp::Ordering::Equal),
                    (true, _, true, _, true, _) => {
                        if t1 == t2 && m1 == m2 && c1 == c2 {
                            Some(std::cmp::Ordering::Equal)
                        } else {
                            Some(std::cmp::Ordering::Less)
                        }
                    }
                    (_, true, _, true, _, true) => {
                        if t1 == t2 && m1 == m2 && c1 == c2 {
                            Some(std::cmp::Ordering::Equal)
                        } else {
                            Some(std::cmp::Ordering::Greater)
                        }
                    }
                    _ => None, // Incomparable
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub enum ConstraintValue {
    Bottom,
    PossiblySatisfied,
    Satisfied,
    Violated,
    Top,
}

impl ConstraintValue {
    pub fn join(&self, other: &Self) -> Self {
        match (self, other) {
            (ConstraintValue::Top, _) | (_, ConstraintValue::Top) => ConstraintValue::Top,
            (ConstraintValue::Bottom, any) | (any, ConstraintValue::Bottom) => any.clone(),
            (ConstraintValue::PossiblySatisfied, any) | (any, ConstraintValue::PossiblySatisfied) => any.clone(),
            (ConstraintValue::Satisfied, ConstraintValue::Satisfied) => ConstraintValue::Satisfied,
            (ConstraintValue::Violated, ConstraintValue::Violated) => ConstraintValue::Violated,
            (ConstraintValue::Satisfied, ConstraintValue::Violated) | 
            (ConstraintValue::Violated, ConstraintValue::Satisfied) => ConstraintValue::Top,
        }
    }

    pub fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (ConstraintValue::Bottom, ConstraintValue::Bottom) => Some(std::cmp::Ordering::Equal),
            (ConstraintValue::Bottom, _) => Some(std::cmp::Ordering::Less),
            (_, ConstraintValue::Bottom) => Some(std::cmp::Ordering::Greater),
            (ConstraintValue::Top, ConstraintValue::Top) => Some(std::cmp::Ordering::Equal),
            (ConstraintValue::Top, _) => Some(std::cmp::Ordering::Greater),
            (_, ConstraintValue::Top) => Some(std::cmp::Ordering::Less),
            (ConstraintValue::PossiblySatisfied, ConstraintValue::PossiblySatisfied) => Some(std::cmp::Ordering::Equal),
            (ConstraintValue::PossiblySatisfied, ConstraintValue::Satisfied) |
            (ConstraintValue::PossiblySatisfied, ConstraintValue::Violated) => Some(std::cmp::Ordering::Less),
            (ConstraintValue::Satisfied, ConstraintValue::PossiblySatisfied) |
            (ConstraintValue::Violated, ConstraintValue::PossiblySatisfied) => Some(std::cmp::Ordering::Greater),
            (ConstraintValue::Satisfied, ConstraintValue::Satisfied) => Some(std::cmp::Ordering::Equal),
            (ConstraintValue::Violated, ConstraintValue::Violated) => Some(std::cmp::Ordering::Equal),
            (ConstraintValue::Satisfied, ConstraintValue::Violated) |
            (ConstraintValue::Violated, ConstraintValue::Satisfied) => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub enum DeclareWitnessState {
    Bottom,
    Evaluated(std::collections::HashMap<String, ConstraintValue>),
    Top,
}

impl Lattice for DeclareWitnessState {
    fn bottom() -> Self {
        DeclareWitnessState::Bottom
    }

    fn top() -> Self {
        DeclareWitnessState::Top
    }

    fn is_top(&self) -> bool {
        matches!(self, DeclareWitnessState::Top)
    }

    fn is_bottom(&self) -> bool {
        matches!(self, DeclareWitnessState::Bottom)
    }

    fn join(&self, other: &Self) -> Self {
        match (self, other) {
            (DeclareWitnessState::Top, _) | (_, DeclareWitnessState::Top) => DeclareWitnessState::Top,
            (DeclareWitnessState::Bottom, any) | (any, DeclareWitnessState::Bottom) => any.clone(),
            (DeclareWitnessState::Evaluated(m1), DeclareWitnessState::Evaluated(m2)) => {
                let mut merged = std::collections::HashMap::new();
                let keys: std::collections::HashSet<&String> = m1.keys().chain(m2.keys()).collect();
                for key in keys {
                    let v1 = m1.get(key).unwrap_or(&ConstraintValue::Bottom);
                    let v2 = m2.get(key).unwrap_or(&ConstraintValue::Bottom);
                    let v_joined = v1.join(v2);
                    if v_joined == ConstraintValue::Top {
                        return DeclareWitnessState::Top;
                    }
                    merged.insert(key.clone(), v_joined);
                }
                DeclareWitnessState::Evaluated(merged)
            }
        }
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (DeclareWitnessState::Bottom, DeclareWitnessState::Bottom) => Some(std::cmp::Ordering::Equal),
            (DeclareWitnessState::Bottom, _) => Some(std::cmp::Ordering::Less),
            (_, DeclareWitnessState::Bottom) => Some(std::cmp::Ordering::Greater),
            (DeclareWitnessState::Top, DeclareWitnessState::Top) => Some(std::cmp::Ordering::Equal),
            (DeclareWitnessState::Top, _) => Some(std::cmp::Ordering::Greater),
            (_, DeclareWitnessState::Top) => Some(std::cmp::Ordering::Less),
            (DeclareWitnessState::Evaluated(m1), DeclareWitnessState::Evaluated(m2)) => {
                let mut is_less_or_equal = true;
                let mut is_greater_or_equal = true;

                let all_keys: std::collections::HashSet<&String> = m1.keys().chain(m2.keys()).collect();
                for key in all_keys {
                    let v1 = m1.get(key).unwrap_or(&ConstraintValue::Bottom);
                    let v2 = m2.get(key).unwrap_or(&ConstraintValue::Bottom);

                    match v1.partial_cmp(v2) {
                        Some(std::cmp::Ordering::Less) => {
                            is_greater_or_equal = false;
                        }
                        Some(std::cmp::Ordering::Greater) => {
                            is_less_or_equal = false;
                        }
                        Some(std::cmp::Ordering::Equal) => {}
                        None => {
                            is_less_or_equal = false;
                            is_greater_or_equal = false;
                        }
                    }
                }

                match (is_less_or_equal, is_greater_or_equal) {
                    (true, true) => Some(std::cmp::Ordering::Equal),
                    (true, false) => Some(std::cmp::Ordering::Less),
                    (false, true) => Some(std::cmp::Ordering::Greater),
                    (false, false) => None,
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub enum UnifiedWitnessState {
    Bottom,
    Active {
        replay: WitnessState,
        declare: DeclareWitnessState,
    },
    Top,
}

impl Lattice for UnifiedWitnessState {
    fn bottom() -> Self {
        UnifiedWitnessState::Bottom
    }

    fn top() -> Self {
        UnifiedWitnessState::Top
    }

    fn is_top(&self) -> bool {
        matches!(self, UnifiedWitnessState::Top)
    }

    fn is_bottom(&self) -> bool {
        matches!(self, UnifiedWitnessState::Bottom)
    }

    fn join(&self, other: &Self) -> Self {
        match (self, other) {
            (UnifiedWitnessState::Top, _) | (_, UnifiedWitnessState::Top) => UnifiedWitnessState::Top,
            (UnifiedWitnessState::Bottom, any) | (any, UnifiedWitnessState::Bottom) => any.clone(),
            (UnifiedWitnessState::Active { replay: r1, declare: d1 },
             UnifiedWitnessState::Active { replay: r2, declare: d2 }) => {
                let r_joined = r1.join(r2);
                let d_joined = d1.join(d2);
                if r_joined.is_top() || d_joined.is_top() {
                    UnifiedWitnessState::Top
                } else {
                    UnifiedWitnessState::Active {
                        replay: r_joined,
                        declare: d_joined,
                    }
                }
            }
        }
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (UnifiedWitnessState::Bottom, UnifiedWitnessState::Bottom) => Some(std::cmp::Ordering::Equal),
            (UnifiedWitnessState::Bottom, _) => Some(std::cmp::Ordering::Less),
            (_, UnifiedWitnessState::Bottom) => Some(std::cmp::Ordering::Greater),
            (UnifiedWitnessState::Top, UnifiedWitnessState::Top) => Some(std::cmp::Ordering::Equal),
            (UnifiedWitnessState::Top, _) => Some(std::cmp::Ordering::Greater),
            (_, UnifiedWitnessState::Top) => Some(std::cmp::Ordering::Less),
            (UnifiedWitnessState::Active { replay: r1, declare: d1 },
             UnifiedWitnessState::Active { replay: r2, declare: d2 }) => {
                let r_cmp = r1.partial_cmp(r2);
                let d_cmp = d1.partial_cmp(d2);

                match (r_cmp, d_cmp) {
                    (Some(std::cmp::Ordering::Equal), Some(std::cmp::Ordering::Equal)) => Some(std::cmp::Ordering::Equal),
                    (Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal),
                     Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)) => Some(std::cmp::Ordering::Less),
                    (Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal),
                     Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)) => Some(std::cmp::Ordering::Greater),
                    _ => None,
                }
            }
        }
    }
}
```

---

## 4. References & Lifecycle Links

*   For the core evidence structure enclosing this lattice, see [Evidence Structures](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/evidence-structures.md).
*   For how loss and alignment cost are computed, see [Loss Policies](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/loss-policies.md).
*   To review the master type-law mapping, see [Type Law Atlas](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/type-law-atlas.md).
