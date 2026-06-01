# Lattice Monotonicity Verification — Doctrine

> Every state transition must monotonically increase the alignment witness within the partial order lattice. Any descent or contradiction is a structural violation.

Source: [paper-to-type-law.md](file:///Users/sac/process-intelligence/sources/papers/paper-to-type-law.md) & [workflow-ledger.md](file:///Users/sac/process-intelligence/sources/papers/workflow-ledger.md) — adapted for the ALIVE_001 compliance framework.

---

## 1. The Monotonicity Axiom (Axiom 2)

In process mining and runtime verification, soundness is not merely the final state conformance score. Soundness is the step-by-step monotonicity of the witness trajectory. 

Let a process state machine be defined by $(S, \to)$, where $S$ is the state space and $\to$ represents valid transitions. Let $W$ be the alignment witness space structured as an $n$-dimensional bounded lattice $(W, \sqsubseteq, \sqcup, \sqcap, \bot, \top)$ representing the satisfaction state of $n$ Declare constraints, as formalized in [declare-satisfaction-lattice.md](file:///Users/sac/process-intelligence/sources/papers/declare-satisfaction-lattice.md).

For every execution step:
$$S_1 \xrightarrow{t} S_2 \quad \implies \quad W_1 \sqsubseteq W_2 \quad \text{where} \quad W_1 \sqcup W_2 = W_2$$

If at any point the witness state $W$ transitions to a state containing a $\top$ component, a conflict is detected, and execution must be halted immediately.

---

## 2. Mathematical Definition of the Witness Lattice

We define the single-constraint satisfaction lattice $(S_W, \sqsubseteq, \sqcup, \sqcap, \bot, \top)$ over the set of possible logical evaluations:
$$S_W = \{ \text{Unknown}, \text{Satisfied}, \text{Violated}, \top \}$$

where:
- $\bot = \text{Unknown}$ represents the absence of information.
- $\top$ represents logical contradiction/conflict.
- The partial order $\sqsubseteq$ represents information progression:
  $$\bot \sqsubseteq \text{Satisfied} \sqsubseteq \top$$
  $$\bot \sqsubseteq \text{Violated} \sqsubseteq \top$$
  with $\text{Satisfied}$ and $\text{Violated}$ being incomparable ($\text{Satisfied} \not\sqsubseteq \text{Violated}$ and $\text{Violated} \not\sqsubseteq \text{Satisfied}$).

The $n$-dimensional witness space $W = S_W^n$ is the coordinate-wise Cartesian product of this lattice. For any $W_A, W_B \in W$:
- $W_A \sqsubseteq W_B \iff \forall i \in \{1, \dots, n\}, W_A[i] \sqsubseteq W_B[i]$
- $W_A \sqcup W_B = (W_A[1] \sqcup W_B[1], \dots, W_A[n] \sqcup W_B[n])$
- $W_A \sqcap W_B = (W_A[1] \sqcap W_B[1], \dots, W_A[n] \sqcap W_B[n])$

---

## 3. Lattice Axiom Verifications

### 3.1 Idempotency
By definition of the join ($\sqcup$) and meet ($\sqcap$) operators on $S_W$, idempotency is strictly preserved:
$$\forall x \in S_W, \quad x \sqcup x = x \quad \text{and} \quad x \sqcap x = x$$

Pointwise verification:
- $\text{Unknown} \sqcup \text{Unknown} = \text{Unknown}$, $\text{Unknown} \sqcap \text{Unknown} = \text{Unknown}$
- $\text{Satisfied} \sqcup \text{Satisfied} = \text{Satisfied}$, $\text{Satisfied} \sqcap \text{Satisfied} = \text{Satisfied}$
- $\text{Violated} \sqcup \text{Violated} = \text{Violated}$, $\text{Violated} \sqcap \text{Violated} = \text{Violated}$
- $\top \sqcup \top = \top$, $\top \sqcap \top = \top$

### 3.2 Monotonicity
The join operator is monotonic with respect to $\sqsubseteq$:
$$\forall x, y, z \in S_W, \quad x \sqsubseteq y \implies x \sqcup z \sqsubseteq y \sqcup z$$

We verify this for all possible relations:
1. If $x = y$, then $x \sqcup z = y \sqcup z$, satisfying $x \sqcup z \sqsubseteq y \sqcup z$.
2. If $x = \bot$ ($\text{Unknown}$):
   - For $y = \text{Satisfied}$: $\bot \sqcup z = z \sqsubseteq \text{Satisfied} \sqcup z$ holds because $z \sqsubseteq \text{Satisfied} \sqcup z$ is an identity of the join LUB. Specifically:
     - $z = \bot \implies \bot \sqsubseteq \text{Satisfied}$ (True)
     - $z = \text{Satisfied} \implies \text{Satisfied} \sqsubseteq \text{Satisfied}$ (True)
     - $z = \text{Violated} \implies \text{Violated} \sqsubseteq \top$ (True)
     - $z = \top \implies \top \sqsubseteq \top$ (True)
   - For $y = \text{Violated}$: Symmetric to the case above.
   - For $y = \top$: $\bot \sqcup z = z \sqsubseteq \top \sqcup z = \top$ (True).
3. If $x = \text{Satisfied}$:
   - The only valid $y \sqsupseteq x$ is $y = \text{Satisfied}$ or $y = \top$.
   - For $y = \top$: $\text{Satisfied} \sqcup z \sqsubseteq \top \sqcup z = \top$ (True).
4. If $x = \text{Violated}$: Symmetric to $x = \text{Satisfied}$.
5. If $x = \top$: The only valid $y \sqsupseteq x$ is $y = \top$, making it trivial.

### 3.3 Absorption Rules
The lattice satisfies the dual absorption laws:
1. $x \sqcup (x \sqcap y) = x$
2. $x \sqcap (x \sqcup y) = x$

We verify both rules exhaustively across the element space $S_W \times S_W$:

#### Table 1: Verification of $x \sqcup (x \sqcap y) = x$
| $x$ | $y$ | $x \sqcap y$ | $x \sqcup (x \sqcap y)$ | Result |
|---|---|---|---|---|
| $\text{Unknown}$ | $\text{Unknown}$ | $\text{Unknown}$ | $\text{Unknown} \sqcup \text{Unknown} = \text{Unknown}$ | Verified |
| $\text{Unknown}$ | $\text{Satisfied}$ | $\text{Unknown}$ | $\text{Unknown} \sqcup \text{Unknown} = \text{Unknown}$ | Verified |
| $\text{Unknown}$ | $\text{Violated}$ | $\text{Unknown}$ | $\text{Unknown} \sqcup \text{Unknown} = \text{Unknown}$ | Verified |
| $\text{Unknown}$ | $\top$ | $\text{Unknown}$ | $\text{Unknown} \sqcup \text{Unknown} = \text{Unknown}$ | Verified |
| $\text{Satisfied}$ | $\text{Unknown}$ | $\text{Unknown}$ | $\text{Satisfied} \sqcup \text{Unknown} = \text{Satisfied}$ | Verified |
| $\text{Satisfied}$ | $\text{Satisfied}$ | $\text{Satisfied}$ | $\text{Satisfied} \sqcup \text{Satisfied} = \text{Satisfied}$ | Verified |
| $\text{Satisfied}$ | $\text{Violated}$ | $\text{Unknown}$ | $\text{Satisfied} \sqcup \text{Unknown} = \text{Satisfied}$ | Verified |
| $\text{Satisfied}$ | $\top$ | $\text{Satisfied}$ | $\text{Satisfied} \sqcup \text{Satisfied} = \text{Satisfied}$ | Verified |
| $\text{Violated}$ | $\text{Unknown}$ | $\text{Unknown}$ | $\text{Violated} \sqcup \text{Unknown} = \text{Violated}$ | Verified |
| $\text{Violated}$ | $\text{Satisfied}$ | $\text{Unknown}$ | $\text{Violated} \sqcup \text{Unknown} = \text{Violated}$ | Verified |
| $\text{Violated}$ | $\text{Violated}$ | $\text{Violated}$ | $\text{Violated} \sqcup \text{Violated} = \text{Violated}$ | Verified |
| $\text{Violated}$ | $\top$ | $\text{Violated}$ | $\text{Violated} \sqcup \text{Violated} = \text{Violated}$ | Verified |
| $\top$ | $\text{Unknown}$ | $\text{Unknown}$ | $\top \sqcup \text{Unknown} = \top$ | Verified |
| $\top$ | $\text{Satisfied}$ | $\text{Satisfied}$ | $\top \sqcup \text{Satisfied} = \top$ | Verified |
| $\top$ | $\text{Violated}$ | $\text{Violated}$ | $\top \sqcup \text{Violated} = \top$ | Verified |
| $\top$ | $\top$ | $\top$ | $\top \sqcup \top = \top$ | Verified |

#### Table 2: Verification of $x \sqcap (x \sqcup y) = x$
| $x$ | $y$ | $x \sqcup y$ | $x \sqcap (x \sqcup y)$ | Result |
|---|---|---|---|---|
| $\text{Unknown}$ | $\text{Unknown}$ | $\text{Unknown}$ | $\text{Unknown} \sqcap \text{Unknown} = \text{Unknown}$ | Verified |
| $\text{Unknown}$ | $\text{Satisfied}$ | $\text{Satisfied}$ | $\text{Unknown} \sqcap \text{Satisfied} = \text{Unknown}$ | Verified |
| $\text{Unknown}$ | $\text{Violated}$ | $\text{Violated}$ | $\text{Unknown} \sqcap \text{Violated} = \text{Unknown}$ | Verified |
| $\text{Unknown}$ | $\top$ | $\top$ | $\text{Unknown} \sqcap \top = \text{Unknown}$ | Verified |
| $\text{Satisfied}$ | $\text{Unknown}$ | $\text{Satisfied}$ | $\text{Satisfied} \sqcap \text{Satisfied} = \text{Satisfied}$ | Verified |
| $\text{Satisfied}$ | $\text{Satisfied}$ | $\text{Satisfied}$ | $\text{Satisfied} \sqcap \text{Satisfied} = \text{Satisfied}$ | Verified |
| $\text{Satisfied}$ | $\text{Violated}$ | $\top$ | $\text{Satisfied} \sqcap \top = \text{Satisfied}$ | Verified |
| $\text{Satisfied}$ | $\top$ | $\top$ | $\text{Satisfied} \sqcap \top = \text{Satisfied}$ | Verified |
| $\text{Violated}$ | $\text{Unknown}$ | $\text{Violated}$ | $\text{Violated} \sqcap \text{Violated} = \text{Violated}$ | Verified |
| $\text{Violated}$ | $\text{Satisfied}$ | $\top$ | $\text{Violated} \sqcap \top = \text{Violated}$ | Verified |
| $\text{Violated}$ | $\text{Violated}$ | $\text{Violated}$ | $\text{Violated} \sqcap \text{Violated} = \text{Violated}$ | Verified |
| $\text{Violated}$ | $\top$ | $\top$ | $\text{Violated} \sqcap \top = \text{Violated}$ | Verified |
| $\top$ | $\text{Unknown}$ | $\top$ | $\top \sqcap \top = \top$ | Verified |
| $\top$ | $\text{Satisfied}$ | $\top$ | $\top \sqcap \top = \top$ | Verified |
| $\top$ | $\text{Violated}$ | $\top$ | $\top \sqcap \top = \top$ | Verified |
| $\top$ | $\top$ | $\top$ | $\top \sqcap \top = \top$ | Verified |

---

## 4. Runtime Verification Architecture

The host environment runs a sidecar validator that intercepts all state change events from the WASM guest engine. The sidecar maintains the running witness $W_{current}$.

1. **Interception**: Guest fires transition $t$ targeting state $S_2$.
2. **State Projection**: Sidecar projects target state $S_2$ and calculates target witness $W_{target}$.
3. **Monotonicity Verification Check**:
   - Check if $W_{current} \sqsubseteq W_{target}$, which is algebraically equivalent to $W_{current} \sqcup W_{target} = W_{target}$.
   - If $W_{current} \sqcup W_{target} \neq W_{target}$, reject the transition as non-monotonic (retroactive state change attempt).
4. **Validation Conflict Check**:
   - If $W_{target}$ contains any $\top$ component, reject transition and trigger immediate Refusal Pathway 6 (Lattice Violation).
5. **Commit State**:
   - If both checks pass, commit transition, updating $W_{current} \leftarrow W_{target}$.

---

## 5. Cryptographic Receipt Binding

At trace termination, the final witness $W_{final}$ is bound with the trace identifier and serialized. The auditor signs the resulting payload, creating a receipt-shaped evidence token.

$$\mathcal{H} = \text{BLAKE3}(T \parallel State_{final} \parallel W_{final})$$

Any modification of the log or the model post-execution will invalidate the cryptographic signature.

---

## 6. Citations and References

- **van der Aalst, W. M. P.**: *Process Mining: Data Science in Action*. Springer, 2016. Cited in [paper-to-type-law.md](file:///Users/sac/process-intelligence/sources/papers/paper-to-type-law.md).
- **Chatman, S.**: *Workflow Ledgers and Trustless Alignment Verification*. Journal of Process Compliance, 2025. Cited in [workflow-ledger.md](file:///Users/sac/process-intelligence/sources/papers/workflow-ledger.md).
