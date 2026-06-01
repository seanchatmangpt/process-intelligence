# Conformance Crosswalk: BPMN 2.0 to Declare Compliance Constraints

This document defines the formal mappings between BPMN 2.0 structural components (gateways, sequence flows, and loop edges) and declarative compliance constraints expressed in Linear Temporal Logic over Finite Traces (LTLf). By defining structural-to-declarative crosswalks, the process intelligence compiler can verify compliance across heterogeneous process representations.

---

## 1. Gateway and Flow Conformance Mappings

| BPMN Structural Component | Equivalent Declare Constraint | LTLf Representation | Lattice State Space | Violation (Top) Condition |
|---|---|---|---|---|
| **Exclusive Choice** (XOR-Split between $A$ and $B$) | `not_coexistence(A, B)` | $\neg (\mathbf{F} A \land \mathbf{F} B)$ | $\{ \bot, \text{Seen}(A), \text{Seen}(B), \top \}$ | $\text{Seen}(A) \sqcup \text{Seen}(B) = \top$ |
| **Parallel Split** (AND-Split triggering $A$ and $B$) | `coexistence(A, B)` | $\mathbf{F} A \leftrightarrow \mathbf{F} B$ | $\{ \bot, \text{Seen}(A), \text{Seen}(B), \text{Coexist} \}$ | Trace termination in $\text{Seen}(A)$ or $\text{Seen}(B)$ |
| **Sequence Flow** ($A \to B$) | `precedence(A, B)` | $\neg B \,\, \mathbf{W} \,\, A$ | $\{ \bot, \text{Seen}(A), \top \}$ | Transition $B$ fires when state is $\bot$ |
| **Loop Backward Edge** ($B \to A$) | `chain_response(B, A)` | $\mathbf{G}(B \implies \mathbf{X} A)$ | $\{ \text{Ok}, \text{Pending}, \top \}$ | Event other than $A$ fires when state is $\text{Pending}$ |

---

## 2. Mathematical Formulations

### 2.1 BPMN Control Flow Graph
A BPMN 2.0 process model is represented as a control flow graph $G = (V, E, T_V)$ where:
- $V = P \cup G_{\text{xor}} \cup G_{\text{and}}$ is a finite set of vertices split into tasks/processes ($P$), exclusive gateways ($G_{\text{xor}}$), and parallel gateways ($G_{\text{and}}$).
- $E \subseteq V \times V$ is the set of directed control-flow sequence edges.
- $T_V: V \to \{ \text{Task}, \text{XORSplit}, \text{XORJoin}, \text{ANDSplit}, \text{ANDJoin} \}$ maps each gateway to its specific gateway type.

### 2.2 Declare Constraints in LTLf
We evaluate Declare constraints on finite execution traces $\sigma = \langle e_1, e_2, \dots, e_n \rangle$ where each event $e_i$ maps to an activity name $x_i \in \Sigma$.

#### A. Exclusive Choice to `not_coexistence(A, B)`
The exclusive choice split mandates that $A$ and $B$ must not occur in the same trace.
- **LTLf Formula:**
  $$\varphi = \mathbf{G}(A \implies \neg \mathbf{F} B) \land \mathbf{G}(B \implies \neg \mathbf{F} A) \equiv \neg (\mathbf{F} A \land \mathbf{F} B)$$
- **Lattice Algebra:**
  Let the witness state space be $W_{\text{xor}} = \{ \bot, \text{Seen}(A), \text{Seen}(B), \top \}$ ordered by:
  $$\bot \sqsubseteq \text{Seen}(A) \sqsubseteq \top, \quad \bot \sqsubseteq \text{Seen}(B) \sqsubseteq \top$$
  The join operator $\sqcup$ is defined as:
  $$w_1 \sqcup w_2 = \begin{cases}
    w_2 & \text{if } w_1 = \bot \\
    w_1 & \text{if } w_2 = \bot \\
    \top & \text{if } w_1 = \text{Seen}(A) \land w_2 = \text{Seen}(B) \\
    \top & \text{if } w_1 = \text{Seen}(B) \land w_2 = \text{Seen}(A) \\
    w_1 & \text{if } w_1 = w_2 \\
    \top & \text{if } w_1 = \top \lor w_2 = \top
  \end{cases}$$
  A join resulting in $\top$ triggers a `LatticeViolation` refusal, signaling that both exclusive branches were executed.

#### B. Parallel Split to `coexistence(A, B)`
The parallel split guarantees that if one branch executes, the sibling branch must also eventually execute.
- **LTLf Formula:**
  $$\varphi = \mathbf{F} A \leftrightarrow \mathbf{F} B$$
- **Lattice Algebra:**
  Let the state space be $W_{\text{and}} = \{ \bot, \text{Seen}(A), \text{Seen}(B), \text{Coexist} \}$ with the ordering:
  $$\bot \sqsubseteq \text{Seen}(A) \sqsubseteq \text{Coexist}, \quad \bot \sqsubseteq \text{Seen}(B) \sqsubseteq \text{Coexist}$$
  The join is:
  $$\text{Seen}(A) \sqcup \text{Seen}(B) = \text{Coexist}$$
  At the end of the trace, the compliance evaluation function $\phi: W_{\text{and}} \to \{ \text{True}, \text{False} \}$ is:
  $$\phi(w) = \begin{cases}
    \text{True} & \text{if } w = \bot \lor w = \text{Coexist} \\
    \text{False} & \text{if } w = \text{Seen}(A) \lor w = \text{Seen}(B)
  \end{cases}$$
  If the trace terminates and $\phi(w) = \text{False}$, the trace is marked non-compliant.

#### C. Sequence Flow to `precedence(A, B)`
A sequence flow $A \to B$ dictates that activity $B$ cannot fire unless activity $A$ has occurred previously.
- **LTLf Formula:**
  $$\varphi = \neg B \,\, \mathbf{W} \,\, A$$
  where $\mathbf{W}$ is the weak until operator, allowing traces where neither occurs.
- **Witness Transition Function:**
  The state is represented by $w \in \{ \bot, \text{Seen}(A), \top \}$. The transition function $\delta: W_{\text{prec}} \times \Sigma \to W_{\text{prec}}$ is:
  $$\delta(w, x) = \begin{cases}
    \text{Seen}(A) & \text{if } x = A \land w \neq \top \\
    \top & \text{if } x = B \land w = \bot \\
    w & \text{otherwise}
  \end{cases}$$
  The execution of $B$ when no preceding $A$ has been recorded forces the state to $\top$, invalidating the trace.

#### D. Loop Backward Edge to `chain_response(B, A)`
A strict loop backward flow requires that returning from $B$ to $A$ must happen immediately, with no intermediate activities.
- **LTLf Formula:**
  $$\varphi = \mathbf{G}(B \implies \mathbf{X} A)$$
- **Witness Transition Function:**
  The state is represented by $w \in \{ \text{Ok}, \text{Pending}, \top \}$. The transition function $\delta: W_{\text{loop}} \times \Sigma \to W_{\text{loop}}$ is:
  $$\delta(w, x) = \begin{cases}
    \text{Pending} & \text{if } x = B \land w \neq \top \\
    \text{Ok} & \text{if } x = A \land w = \text{Pending} \\
    \top & \text{if } x \neq A \land w = \text{Pending} \\
    \text{Ok} & \text{if } x \neq B \land w = \text{Ok}
  \end{cases}$$
  Any event other than $A$ occurring immediately after $B$ transitions the witness state to $\top$, indicating a loop escape violation.

---

## 3. Related Standards and Atlases

For more details on compliance checking, refer to:
- [BPMN 2.0 Standard Specification](file:///Users/sac/process-intelligence/standards/bpmn.md) - Details on gateways and sequence flow behavior.
- [Declare Compliance Standard](file:///Users/sac/process-intelligence/standards/declare.md) - Formal descriptions of Declare LTL rules.
- [Petri Net Formalization](file:///Users/sac/process-intelligence/standards/petri-net.md) - Mathematical representation of token replays.
- [Witness Lattices Specification](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/witness-lattices.md) - Partial orders and algebraic properties of witnesses.
- [Type-Law Atlas](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/type-law-atlas.md) - Inventory of Unified Witness States and typestate boundaries.
