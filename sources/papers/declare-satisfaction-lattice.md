# LTL-Based Declare Constraint Satisfaction Lattices in Process Mining

**Authors**: Conformance Agent Research Team  
**Stage**: Published Research / Thesis Reference  
**Abstract**: We formalize Declare constraint satisfaction as a bounded join-semilattice and full lattice. By mapping Linear Temporal Logic (LTL) satisfaction states to partial order elements under finite trace semantics ($\text{LTL}_f$), we provide a unified mathematical framework for combining structural process model alignments with declarative compliance rules. We introduce a 5-valued evaluation lattice that tracks progress during trace replay, incorporating weak-until LTL-to-DFA transitions and explicit vacuous satisfaction logic.

---

## 1. Introduction and Formalization

Declarative process models specify *what* should be done rather than *how* it should be done using Declare templates. In this paper, we map a set of Declare constraints $C = \{c_1, c_2, \dots, c_n\}$ to a satisfaction lattice.

### 1.1 The Static Information Lattice ($S_W$)
For a completed trace $\sigma$, we define the single-constraint static satisfaction lattice $(S_W, \sqsubseteq, \sqcup, \sqcap, \bot, \top)$ over the set of possible logical evaluations:
$$S_W = \{ \text{Unknown}, \text{Satisfied}, \text{Violated}, \top \}$$

where:
- $\bot = \text{Unknown}$ represents the absence of evaluation information.
- $\top$ represents logical contradiction/conflict.
- The partial order $\sqsubseteq$ represents information progression:
  $$\bot \sqsubseteq \text{Satisfied} \sqsubseteq \top$$
  $$\bot \sqsubseteq \text{Violated} \sqsubseteq \top$$
  with $\text{Satisfied}$ and $\text{Violated}$ being incomparable ($\text{Satisfied} \not\sqsubseteq \text{Violated}$ and $\text{Violated} \not\sqsubseteq \text{Satisfied}$).

The join operator $\sqcup$ is defined pointwise:
- $x \sqcup \bot = x$
- $x \sqcup x = x$
- $\text{Satisfied} \sqcup \text{Violated} = \top$
- $x \sqcup \top = \top$

### 1.2 The Dynamic Valuation Lattice ($\mathcal{V}$)
To support runtime verification over trace prefixes, we extend the static lattice to a 5-valued dynamic valuation lattice $(\mathcal{V}, \sqsubseteq, \sqcup, \sqcap, \bot, \top)$ as defined in [Witness Lattices](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/witness-lattices.md) and [Type Law Atlas](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/type-law-atlas.md):
$$\mathcal{V} = \{ \bot, \text{PossiblySatisfied}, \text{Satisfied}, \text{Violated}, \top \}$$

where:
- $\bot$ (Bottom) represents uninitialized / not yet evaluated.
- $\text{PossiblySatisfied}$ represents a pending state where the constraint is satisfied on the current prefix but could be violated in the future, or requires future events to be permanently satisfied.
- $\text{Satisfied}$ represents permanent satisfaction (immutable).
- $\text{Violated}$ represents permanent violation (immutable).
- $\top$ (Top) represents contradiction.

The partial order $\sqsubseteq$ for $(\mathcal{V}, \sqsubseteq)$ is defined by the reflexive transitive closure of:
$$\bot \sqsubseteq \text{PossiblySatisfied}$$
$$\text{PossiblySatisfied} \sqsubseteq \text{Satisfied}$$
$$\text{PossiblySatisfied} \sqsubseteq \text{Violated}$$
$$\text{Satisfied} \sqsubseteq \top$$
$$\text{Violated} \sqsubseteq \top$$

The join operator $\sqcup: \mathcal{V} \times \mathcal{V} \to \mathcal{V}$ is defined as:
- $x \sqcup \bot = x$ for all $x \in \mathcal{V}$
- $x \sqcup \top = \top$ for all $x \in \mathcal{V}$
- $x \sqcup x = x$ for all $x \in \mathcal{V}$
- $\text{PossiblySatisfied} \sqcup x = x$ for all $x \in \{\text{PossiblySatisfied}, \text{Satisfied}, \text{Violated}, \top\}$
- $\text{Satisfied} \sqcup \text{Violated} = \top$

This partial order and join operation are verified in [Lattice Monotonicity Verification](file:///Users/sac/process-intelligence/doctrine/lattice-monotonicity-verification.md).

For a set of constraints $C = \{c_1, \dots, c_n\}$, the global witness state $w \in W = \mathcal{V}^n$ is the Cartesian product of the coordinate-wise lattices.

---

## 2. Linear Temporal Logic on Finite Traces ($\text{LTL}_f$) and Weak-Until Mappings

Declare templates are interpreted under finite trace semantics ($\text{LTL}_f$) to account for trace completion. Let $\sigma = e_1 e_2 \dots e_m$ be a finite trace of length $m$ over alphabet $\Sigma$. Let $\sigma, i \models \phi$ denote that the formula $\phi$ holds at index $i \in \{1, \dots, m\}$:
*   $\sigma, i \models p \iff p = e_i$ (for atomic proposition $p \in \Sigma$)
*   $\sigma, i \models \neg \phi \iff \sigma, i \not\models \phi$
*   $\sigma, i \models \phi_1 \lor \phi_2 \iff \sigma, i \models \phi_1 \text{ or } \sigma, i \models \phi_2$
*   $\sigma, i \models \bigcirc \phi \iff i < m \text{ and } \sigma, i+1 \models \phi$ (Strong Next: false at the end of the trace)
*   $\sigma, i \models \widetilde{\bigcirc} \phi \iff i = m \text{ or } \sigma, i+1 \models \phi$ (Weak Next: true at the end of the trace)
*   $\sigma, i \models \phi_1 \mathbin{\mathcal{U}} \phi_2 \iff \exists j \in \{i, \dots, m\} \text{ s.t. } \sigma, j \models \phi_2 \text{ and } \forall k \in \{i, \dots, j-1\}, \sigma, k \models \phi_1$
*   $\sigma, i \models \lozenge \phi \iff \exists j \in \{i, \dots, m\} \text{ s.t. } \sigma, j \models \phi$
*   $\sigma, i \models \Box \phi \iff \forall j \in \{i, \dots, m\}, \sigma, j \models \phi$

The **Weak-Until** operator $\mathbin{\mathcal{W}}$ is defined as:
$$\phi_1 \mathbin{\mathcal{W}} \phi_2 \equiv (\phi_1 \mathbin{\mathcal{U}} \phi_2) \lor \Box \phi_1$$

### 2.1 Precedence(A, B) Mapping
The `Precedence(A, B)` constraint specifies that activity $B$ cannot occur unless activity $A$ has occurred before it.

*   **$\text{LTL}_f$ Representation**:
    $$\phi_{\text{Precedence}} = \neg B \mathbin{\mathcal{W}} A \equiv (\neg B \mathbin{\mathcal{U}} A) \lor \Box \neg B$$
*   **Automaton Compilation**: The DFA $\mathcal{A}_{\text{Precedence}} = (\Sigma, Q, q_0, \delta, F)$ has:
    - $Q = \{q_0, q_1, q_{\text{viol}}\}$
    - $q_0$ (initial state, accepting): $A$ and $B$ have not occurred.
    - $q_1$ (accepting): $A$ has occurred.
    - $q_{\text{viol}}$ (sink state, non-accepting): $B$ occurred before $A$.
    - Transition function $\delta$:
      - $\delta(q_0, A) = q_1$
      - $\delta(q_0, B) = q_{\text{viol}}$
      - $\delta(q_0, c) = q_0 \quad \forall c \in \Sigma \setminus \{A, B\}$
      - $\delta(q_1, c) = q_1 \quad \forall c \in \Sigma$
      - $\delta(q_{\text{viol}}, c) = q_{\text{viol}} \quad \forall c \in \Sigma$
*   **Runtime Valuation Mapping $val(\sigma_{\le k}, \phi_{\text{Precedence}})$**:
    - At state $q_0$: Valuation is $\text{PossiblySatisfied}$ (satisfied but can become violated if $B$ occurs).
    - At state $q_1$: Valuation is $\text{Satisfied}$ (permanently satisfied).
    - At state $q_{\text{viol}}$: Valuation is $\text{Violated}$ (permanently violated).

### 2.2 Response(A, B) Mapping
The `Response(A, B)` constraint specifies that if activity $A$ occurs, activity $B$ must occur at or after it.

*   **$\text{LTL}_f$ Representation**:
    $$\phi_{\text{Response}} = \Box(A \implies \lozenge B)$$
*   **Automaton Compilation**: The DFA $\mathcal{A}_{\text{Response}} = (\Sigma, Q, q_0, \delta, F)$ has:
    - $Q = \{q_0, q_1\}$
    - $q_0$ (initial state, accepting): No pending $A$ (either $A$ has not occurred, or $A$ occurred and was followed by $B$).
    - $q_1$ (non-accepting): $A$ occurred, waiting for $B$.
    - Transition function $\delta$ (assuming $A \neq B$):
      - $\delta(q_0, A) = q_1$
      - $\delta(q_0, c) = q_0 \quad \forall c \in \Sigma \setminus \{A\}$
      - $\delta(q_1, B) = q_0$
      - $\delta(q_1, c) = q_1 \quad \forall c \in \Sigma \setminus \{B\}$
*   **Runtime Valuation Mapping $val(\sigma_{\le k}, \phi_{\text{Response}})$**:
    - Since any state during trace execution can be extended to satisfy or violate the constraint, the runtime evaluation is $\text{PossiblySatisfied}$ in both $q_0$ and $q_1$.
    - Upon trace termination at length $m$:
      - If $\delta^*(q_0, \sigma) = q_0$: Valuation resolves to $\text{Satisfied}$.
      - If $\delta^*(q_0, \sigma) = q_1$: Valuation resolves to $\text{Violated}$.

---

## 3. Vacuous Satisfaction Logic

A common vulnerability in compliance checking is **vacuous truth**, where a constraint is reported as satisfied simply because its activation condition never occurred.

### 3.1 Mathematical Formulation of Vacuity
For a relation constraint $\phi$, let $\alpha_{\phi}$ be its activation condition and $\beta_{\phi}$ be its target condition. Let the activation set of trace $\sigma$ be:
$$\text{Acts}(\sigma, \alpha_{\phi}) = \{ i \in \{1, \dots, m\} \mid \sigma, i \models \alpha_{\phi} \}$$

*   **Satisfied Non-Vacuously (Active Fulfillment)**:
    $$\sigma \models_{\text{act}} \phi \iff \sigma, 1 \models \phi \quad \text{and} \quad \text{Acts}(\sigma, \alpha_{\phi}) \neq \emptyset$$
*   **Satisfied Vacuously**:
    $$\sigma \models_{\text{vac}} \phi \iff \sigma, 1 \models \phi \quad \text{and} \quad \text{Acts}(\sigma, \alpha_{\phi}) = \emptyset$$

### 3.2 Activation Conditions for Core Templates
To support full-conformance checking, the activation condition $\alpha_{\phi}$ is defined for each template:

*   **Unary Constraints**: Always active ($\alpha_{\phi} = \text{True}$). Since they specify global occurrence frequency or positioning rather than conditional logic, they are never vacuously satisfied.
    - `Existence(A)`, `Existence2(A)`, `Existence3(A)`, `Absence(A)`, `Absence2(A)`, `Absence3(A)`, `Init(A)`, `ExclusiveChoice(A, B)` (ExclusiveChoice restricts the entire trace, acting as a global logical constraint).
*   **Binary Relation Constraints**:
    - `Response(A, B)`: $\alpha_{\phi} = A$.
    - `Precedence(A, B)`: $\alpha_{\phi} = B$.
    - `RespondedExistence(A, B)`: $\alpha_{\phi} = A$.
    - `CoExistence(A, B)`: $\alpha_{\phi} = A \lor B$.
    - `Succession(A, B)`: $\alpha_{\phi} = A \lor B$.
    - `AlternateResponse(A, B)`: $\alpha_{\phi} = A$.
    - `AlternatePrecedence(A, B)`: $\alpha_{\phi} = B$.
    - `AlternateSuccession(A, B)`: $\alpha_{\phi} = A \lor B$.
    - `ChainResponse(A, B)`: $\alpha_{\phi} = A$.
    - `ChainPrecedence(A, B)`: $\alpha_{\phi} = B$.
    - `ChainSuccession(A, B)`: $\alpha_{\phi} = A \lor B$.
*   **Negative / Exclusion Constraints**:
    - `NotCoExistence(A, B)`: $\alpha_{\phi} = A \lor B$.
    - `NotSuccession(A, B)`: $\alpha_{\phi} = A$.
    - `NotChainSuccession(A, B)`: $\alpha_{\phi} = A$.

The runtime engine in [Declare Placement Standard](file:///Users/sac/process-intelligence/standards/declare_placement.md) tracks activations and sets `is_vacuously_satisfied: true` when $\sigma, 1 \models \phi$ but $|\text{Acts}(\sigma, \alpha_{\phi})| = 0$, preventing false-positive compliance audits.

---

## 4. Dynamic Monotonicity

During trace execution, the evaluation vector progresses from $(\bot, \dots, \bot)$ to a terminal state. Let $w_k = val(\sigma_{\le k}, C)$ be the witness vector at step $k$. 

Lattice monotonicity requires that for all $k \ge 0$:
$$w_k \sqsubseteq w_{k+1}$$

which is algebraically equivalent to:
$$w_k \sqcup w_{k+1} = w_{k+1}$$

If the system attempts to transition to a state $w_{k+1}$ that violates this (e.g. attempting to revert from $\text{Violated}$ back to $\text{PossiblySatisfied}$), the check $w_k \sqcup w_{k+1} = w_{k+1}$ fails, and the transition is rejected. If a contradiction occurs, the state transitions to $\top$, halting the VM.

This algebraic check allows low-overhead runtime compliance verification inside resource-constrained environments like WebAssembly.

---

## 5. References and Related Material
- Pesic, M., Schonenberg, H., & van der Aalst, W. M. P. (2007). *DECLARATIVE: A Declarative Workflow Management System*. BPM 2007.
- For concrete implementation details, see [Witness Lattices](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/witness-lattices.md) and [Declare Placement Standard](file:///Users/sac/process-intelligence/standards/declare_placement.md).
- For test validation, see [Declare Violation Sample](file:///Users/sac/process-intelligence/experiments/declare_violation_sample.md).
