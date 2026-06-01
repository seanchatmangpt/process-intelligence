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

#[derive(Clone, Debug, PartialEq, Eq)]
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
                // If they have conflicting markings for the same trace indices, it is invalid
                if t1 == t2 && m1 != m2 {
                    WitnessState::Top
                } else {
                    let mut merged_indices = t1.clone();
                    for idx in t2 {
                        if !merged_indices.contains(idx) {
                            merged_indices.push(*idx);
                        }
                    }
                    merged_indices.sort();
                    
                    // In a sound WF-net, marking represents union of places if disjoint and concurrent
                    let mut merged_marking = m1.clone();
                    for place in m2 {
                        if !merged_marking.contains(place) {
                            merged_marking.push(place.clone());
                        }
                    }
                    
                    WitnessState::PartialReplay {
                        trace_indices: merged_indices,
                        marking: merged_marking,
                        cost: c1 + c2,
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
            (WitnessState::PartialReplay { trace_indices: t1, marking: m1, cost: _ },
             WitnessState::PartialReplay { trace_indices: t2, marking: m2, cost: _ }) => {
                let is_t1_sub = t1.iter().all(|x| t2.contains(x));
                let is_t2_sub = t2.iter().all(|x| t1.contains(x));
                
                let is_m1_sub = m1.iter().all(|x| m2.contains(x));
                let is_m2_sub = m2.iter().all(|x| m1.contains(x));

                match (is_t1_sub, is_t2_sub, is_m1_sub, is_m2_sub) {
                    (true, true, true, true) => Some(std::cmp::Ordering::Equal),
                    (true, false, true, _) => Some(std::cmp::Ordering::Less),
                    (false, true, _, true) => Some(std::cmp::Ordering::Greater),
                    _ => None, // Incomparable elements in the partial order
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
