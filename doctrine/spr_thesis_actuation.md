# The Sparse Priming Representation (SPR) Thesis Actuation spec (v30.1.1 Ultimate Standard)

This document materializes the v30.1.1 ultimate standard thesis on Process Intelligence, detailing all 27 core sections extracted from the foundational chapters.

---

## Part I: The Type-Law Architecture (Chapter 3)

### Section 1: The Evidence Lifecycle
The central structural invariant of the process-evidence lifecycle is a typed, one-way door. Raw, untrusted input cannot be used as admitted evidence; admitted evidence cannot be un-admitted; refused evidence cannot be rehabilitated.
The lifecycle is defined as a directed state machine over the set of stage tokens:
$$\mathcal{S} = \{\texttt{Raw}, \texttt{Parsed}, \texttt{Admitted}, \texttt{Refused}, \texttt{Projected}, \texttt{Exportable}, \texttt{Receipted}\}$$
with initial state $\texttt{Raw}$ and terminal states $\{\texttt{Refused}, \texttt{Receipted}\}$.

The lawful transitions are:
$$\texttt{Raw} \xrightarrow{\texttt{into\_parsed}} \texttt{Parsed} \xrightarrow{\texttt{Admit::admit}} \texttt{Admitted} \to \begin{cases} 
\texttt{Projected} \xrightarrow{\texttt{into\_receipted}} \texttt{Receipted} \\ 
\texttt{Exportable} \xrightarrow{\texttt{into\_receipted}} \texttt{Receipted} \\ 
\texttt{Receipted} \quad \text{(terminal)} 
\end{cases}$$

Refuse paths are available before admission:
$$\texttt{Raw} \xrightarrow{\texttt{refuse}} \texttt{Refused}, \qquad \texttt{Parsed} \xrightarrow{\texttt{into\_refused}} \texttt{Refused}$$

Type-level safety enforces that cross-state substitution is a compiler error:
$$\forall T, S_1, S_2, W. \quad S_1 \neq S_2 \implies \text{Evidence}\langle T, S_1, W \rangle \not\leq \text{Evidence}\langle T, S_2, W \rangle$$

---

### Section 2: The Witness Marker System
A witness marker is an uninhabited type (an empty enum) representing a specific named authority. Witnesses carry no data and do no work; their sole function is to thread a named authority through the type signature.
A witness marker $W \in \mathcal{W}$ implements the sealed `Witness` trait:
$$\text{Witness} \implies \begin{cases}
\texttt{KEY}: \&'\text{static str} \\
\texttt{FAMILY}: \text{WitnessFamily} \\
\texttt{TITLE}: \&'\text{static str} \\
\texttt{YEAR}: \text{Option}\langle u16 \rangle
\end{cases}$$

Let $\iota: \mathcal{W} \times \mathcal{W} \to \{0, 1\}$ be the compile-time nominal type identity:
$$\iota(W_1, W_2) = \begin{cases} 1 & \text{if } W_1 \equiv W_2 \text{ as Rust types} \\ 0 & \text{otherwise} \end{cases}$$
Because generic type constructors are nominal and injective:
$$\iota(W_1, W_2) = 0 \implies \text{Admission}\langle T, W_1 \rangle \not\equiv \text{Admission}\langle T, W_2 \rangle$$

---

### Section 3: Const-Generic Law Machinery
We map boolean const-generic expressions to compile-time proof obligations via:
$$\text{Assert}\langle\text{const OK: bool}\rangle; \qquad \text{IsTrue for Assert}\langle\text{true}\rangle; \qquad \text{Require}\langle\text{const OK: bool}\rangle = \text{Assert}\langle\text{OK}\rangle$$
A where-bound $\text{Require}\langle\text{EXPR}\rangle: \text{IsTrue}$ compiles successfully if and only if $\text{EXPR}$ evaluates to $\text{true}$.

The type-level rational metric bounds $\text{Between01}\langle n, d \rangle$ is defined as:
$$\text{Between01}\langle\text{const NUM: u64, const DEN: u64}\rangle \quad \text{where} \quad d > 0 \land n \leq d$$
Its soundness is stated as:
$$\forall n, d \in \mathbb{N}, \quad \text{Between01}\langle n, d \rangle \text{ is well-formed} \iff d > 0 \land n \leq d$$

The "Need-9 means split" law enforces that a single condition cell holds at most 8 primary bits:
$$\text{ConditionCell}\langle\text{const BITS: usize}\rangle \quad \text{where} \quad \text{Require}\langle\text{BITS} \leq 8\rangle: \text{IsTrue}$$

---

### Section 4: Non-Forgeability via Sealed Constructors
A workflow net soundness state is tracked via the enum:
$$\text{SoundnessState} = \{\texttt{Unknown}, \texttt{Claimed}, \texttt{Witnessed}\}$$
The non-forgeability of the $\texttt{Witnessed}$ state is enforced by a private constructor seal:
$$\text{WfNetConst}\langle\text{const SOUNDNESS: SoundnessState}\rangle$$
Only the function `witness_soundness`, requiring a `SoundnessProof` token generated inside the engine, can produce $\text{WfNetConst}\langle\texttt{Witnessed}\rangle$.
The single-type design prevents the forgery surface introduced by conversion functions in a two-type design (e.g., separating parameterized and unparameterized nets).

---

### Section 5: The LossPolicy Chain
Loss policies determine how information loss during translation is handled:
$$\text{LossPolicy} = \{\texttt{RefuseLoss}, \texttt{AllowNamedProjection}, \texttt{AllowLossWithReport}\}$$
They form a lattice under permissiveness.
A static name is defined by $\text{ProjectionName}(\&'\text{static str})$.
A $\text{LossReport}\langle\text{From, To, Items}\rangle$ is emitted. The `Project` trait is defined as:
$$\text{Project}(e, p) \to \text{Result}\langle\text{LossReport}\langle\text{From, To, Lost}\rangle, \text{Reason}\rangle$$

**Algorithm: Lawful Lossy Projection Protocol:**
1. Let $e$ be the source evidence of type $\text{Evidence}\langle T, \texttt{Admitted}, W \rangle$.
2. Let $p$ be the target $\text{LossPolicy}$ and $n$ the $\text{ProjectionName}$.
3. Identify dropped items: $\text{dropped} \leftarrow \text{Identify\_Loss}(e)$.
4. If $p = \texttt{RefuseLoss}$ and $\text{dropped} \neq \emptyset$, return $\text{Err}(\text{RefusalReason})$.
5. Otherwise, project: $e' \leftarrow \text{Apply\_Projection}(e, n)$.
6. Construct report: $r \leftarrow \text{LossReport::new}(n, p, \text{dropped})$.
7. Return $\text{Ok}(e'.\text{into\_projected}(), r)$.

---

### Section 6: The Three-Layer Summary
The compile-time checks of the type system are structured into three distinct layers:

| Layer | Mechanism | Invariant Enforced |
| :--- | :--- | :--- |
| **State Tokens** | Empty enums as `PhantomData` | Cross-stage substitution is a type error |
| **Witness Markers** | Uninhabited enums + sealed `Witness` trait | Cross-authority substitution is a type error |
| **Const-Generic Law** | `Require<{EXPR}>: IsTrue` bounds | Out-of-bound metric or arity parameters do not compile |

---

## Part II: The Manufacturing Doctrine (Chapter 4)

### Section 7: Commits as Receipts
A git commit is treated as a manufacturing transition $\tau = (c, \rho)$ where $c$ is the commit hash and $\rho$ is a receipt class:
$$\rho \in \mathcal{R} = \{\texttt{paper-ledger}, \texttt{paper-law}, \texttt{type-law}, \texttt{fixture-pass}, \texttt{fixture-fail}, \texttt{stderr}, \texttt{ledger}, \texttt{audit}, \texttt{docs-law}, \texttt{checkpoint}, \texttt{tag}\}$$
Commit validity is structural, meaning the changes in $c$ must exactly produce the type surface or compiler diagnostic claimed by the receipt class $\rho$.

---

### Section 8: The ALIVE Gate
A codebase state $\mathcal{R}$ satisfies the ALIVE certification level if and only if:
$$\text{ALIVE}_{004}(\mathcal{R}) \iff \bigwedge_{i=1}^{10} G_i(\mathcal{R})$$
where:
* $G_1(\mathcal{R})$: Paper Coverage ($\geq 80$ paper families ledgered).
* $G_2(\mathcal{R})$: No Missing Type Law.
* $G_3(\mathcal{R})$: Compile-Pass Fixtures ($\geq 200$).
* $G_4(\mathcal{R})$: Compile-Fail Fixtures ($\geq 160$).
* $G_5(\mathcal{R})$: Stderr Parity ($|\text{compile\_fail}| = |\text{.stderr}|$).
* $G_6(\mathcal{R})$: Audit Script Coverage ($\geq 20$).
* $G_7(\mathcal{R})$: Master Audit Gate (exit 0).
* $G_8(\mathcal{R})$: Cargo Tests (exit 0).
* $G_9(\mathcal{R})$: Clippy Warnings (exit 0).
* $G_{10}(\mathcal{R})$: Format check (exit 0).

---

### Section 9: The PARTIAL Checkpoint Pattern
A PARTIAL checkpoint is a tagged commit recording:
1. Satisfied gates: $\{i \mid G_i(\mathcal{R}) = \top\}$
2. Residual gates: $\{i \mid G_i(\mathcal{R}) = \bot\}$
3. Bill of Materials (BOM) specifying the exact backlog of receipt-bearing commits required to close the residuals:
$$\text{BOM}(G_i) = \{ n \text{ commits of class } \rho \}$$
The workflow is: $\text{PARTIAL} \to \text{residual inventory} \to \text{targeted closure} \to \text{recomputed gate} \to \text{ALIVE}$.

---

### Section 10: Anti-Regression Audit Mesh
The repository enforces invariants via a mesh of 23 audit scripts divided into:
* **Hard Audits**: Immediate blockers (e.g. no engine creep, feature boundaries).
* **Soft Audits**: Quality metrics and coverage warning thresholds.

**Algorithm: Crown Audit Protocol (`audit\_crown\_gate\_all.sh`):**
1. Let $\mathcal{S} = \{s_1, \ldots, s_{22}\}$ be the subordinate audit scripts.
2. Initialize $\text{FAIL} \leftarrow 0$, $\text{defects} \leftarrow \emptyset$.
3. For each $s_i \in \mathcal{S}$:
   * If $\text{exit}(s_i(\mathcal{R})) \neq 0$:
     * $\text{FAIL} \leftarrow \text{FAIL} + 1$
     * $\text{defects} \leftarrow \text{defects} \cup \{s_i\}$
4. If $\text{FAIL} = 0$, return exit 0. Else, print defects and return exit 1.

---

### Section 11: Velocity and Scale
Parallel manufacturing of compile-fail/pass fixture pairs is enabled by Claude Code dynamic workflows using the schema-free agent pattern. Returning results as plain text with simple patterns avoids structured-output latency timeouts:
$$\text{Latency}_{\text{schema-free}} \ll \text{Latency}_{\text{schema-validated}}$$
The manufacturing pipeline follows:
$$\text{Paper intake} \to \text{Type-law assignment} \to \text{Fixture pair manufacturing} \to \text{Audit gate} \to \text{Checkpoint / Tag}$$

---

### Section 12: Paper as Law
Every paper in the process mining corpus is mapped to one of five classifications:
$$\text{Classification} \in \{\texttt{COVERED\_BY\_TYPE}, \texttt{COVERED\_BY\_GRADUATION\_BOUNDARY}, \texttt{PARTIAL\_WITH\_REASON}, \texttt{DUPLICATE\_OR\_BACKGROUND}, \texttt{OUT\_OF\_SCOPE\_WITH\_REASON}\}$$
The status $\texttt{MISSING\_TYPE\_LAW}$ is strictly forbidden. Gate $G_2$ ensures that:
$$|\{m \in \text{modules} \mid \text{MISSING\_TYPE\_LAW}(m)\}| = 0$$

---

## Part III: Process Mining Canon as Type Law (Chapter 5)

### Section 13: OCEL 2.0 Formal Objects
An OCEL 2.0 log is defined as a bipartite directed graph:
$$\mathcal{L} = (\mathcal{E}, \mathcal{O}, \text{E2O}, \text{O2O}, \Delta)$$
where:
* $\mathcal{E}$ is the set of events.
* $\mathcal{O}$ is the set of objects.
* $\text{E2O} \subseteq \mathcal{E} \times \mathcal{O} \times \mathcal{Q}$ are event-to-object links.
* $\text{O2O} \subseteq \mathcal{O} \times \mathcal{O} \times \mathcal{Q}$ are object-to-object links.
* $\Delta \subseteq \mathcal{O} \times \text{Attr} \times \text{Val} \times \mathbb{T}$ tracks attribute evolution.

A log $\mathcal{L}$ is valid if and only if:
$$\forall (e, o, q) \in \text{E2O}, \quad e \in \mathcal{E} \land o \in \mathcal{O} \qquad \text{and} \qquad \forall (o_1, o_2, q) \in \text{O2O}, \quad o_1, o_2 \in \mathcal{O}$$
The dimensional schema is defined as a triple $(\mathcal{OT}, \mathcal{AT}, \lambda)$ where $\lambda: \mathcal{AT} \to 2^{\mathcal{OT}}$ maps activity types to allowed object types.

---

### Section 14: XES and Case-Centric Evidence
The IEEE 1849 XES standard enforces single-case-notion event semantics. The type separation is strict:
$$\text{XesLog} \not\equiv \text{OcelLog}$$
The boundary is marked by the zero-sized `CaseCentricMarker`.
The XES extension prefix law requires non-empty prefixes:
$$\text{XesExtension}(n, p, u) \quad \text{is valid} \iff p \neq ""$$
Violation of this law yields `XesRefusal::InvalidExtension`.

---

### Section 15: Petri Net Bipartite Arc Law
A Petri net is a tuple $N = (P, T, F, W, M_0)$ where $P$ and $T$ are disjoint finite sets of places and transitions.
The flow relation $F$ must satisfy:
$$F \subseteq (P \times T) \cup (T \times P) \qquad \text{and} \qquad F \cap (P \times P) = \emptyset \land F \cap (T \times T) = \emptyset$$
In `wasm4pm-compat`, this is structurally guaranteed because the only arc constructors are:
$$\text{PlaceToTransitionArc}\langle P, T, Weight \rangle \qquad \text{and} \qquad \text{TransitionToPlaceArc}\langle T, P, Weight \rangle$$
No place-to-place arc can be constructed in the type system.

---

### Section 16: WF-Net Soundness
A workflow net (WF-net) has source place $i$, sink place $o$, and every node is on a path from $i$ to $o$.
Soundness states are tracked at the type level:
$$\text{SoundnessState} \in \{\texttt{Unknown}, \texttt{Claimed}, \texttt{Witnessed}\}$$
The `WfNetConst<SOUNDNESS>` type restricts execution capabilities based on this parameter, requiring a cryptographically signed `SoundnessProof` from the engine to promote to $\texttt{Witnessed}$.

---

### Section 17: POWL Projection Law
A Partially Ordered Workflow Language (POWL) model $M$ is process-tree projectable if and only if:
$$\mathcal{L}(M) = \mathcal{L}(T) \quad \text{for some block-structured process tree } T$$
If $M$ contains irreducible partial orders, it exceeds the process tree language. The `TreeProjectable` trait is sealed and implemented only for `ProcessTreeProjectable`.
Acyclic partial orders are verified using the `AcyclicPartialOrder` witness.

---

### Section 18: Conformance Metrics
The four quality dimensions (fitness, precision, generalization, simplicity) and F1 score are represented as rational numbers $\frac{p}{q} \in [0, 1]$.
This is compile-enforced via the where-bounds:
$$\text{Metric}\langle\text{const KIND: QualityMetricKind, const NUM: u64, const DEN: u64}\rangle \quad \text{where} \quad \text{Between01}\langle\text{NUM, DEN}\rangle: \text{IsTrue}$$

---

### Section 19: New Modules
We formalize three additional modules:
1. **Process Cube**: Dimensions are encoded as distinct types using const-strings:
$$\text{CubeDimension}\langle\text{const NAME: \&'static str}\rangle$$
2. **Temporal Ordering**: Four-valued temporal ordering relation:
$$\text{TemporalOrder} = \{\texttt{Before}, \texttt{After}, \texttt{Concurrent}, \texttt{Unknown}\}$$
3. **Object Lifecycle**: Phase transitions enforced through typestate methods:
$$\text{PHASE} \in \{\texttt{Created}, \texttt{Active}, \texttt{Modified}, \texttt{Archived}, \texttt{Deleted}\}$$
To prevent the nightly Rust compiler E0391 variance cycle, intermediate type aliases are used:
$$\text{type ActiveToModified}\langle T \rangle = \text{LifecycledObject}\langle T, \texttt{Modified} \rangle$$

---

## Part IV: Formal Foundations and Future Directions (Chapter 8)

### Section 20: The Process Canon Summary
The formal objects of the process mining canon are summarized in the following table:

| Canon Object | Paper Authority | Rust Type | Law Enforced |
| :--- | :--- | :--- | :--- |
| **OCEL 2.0** | Berti & van der Aalst (2023) | `OcelLog` | Bipartite graphs, typed links |
| **XES** | IEEE 1849 (2016) | `XesLog` | Single-case notion, non-empty prefix |
| **Petri Net** | Murata (1989) | `WfNetConst<S>` | Disjoint sets $P, T$; bipartite arcs |
| **WF-Net** | van der Aalst (1998) | `WfNetConst<S>` | Non-forgeable soundness witness |
| **POWL** | Kourani (2024) | `ProcessTreeProjectable` | Irreducible partial order check |
| **Metrics** | Carmona (2018) | `FitnessConst<N,D>` | Value in $[0,1]$ via `Between01` |
| **Lifecycle** | Berti (2023) | `LifecycledObject<T,P>` | Phase-transition DAG |

---

### Section 21: The Evidence Lifecycle as a Partial Order
The set of stage tokens $S$ forms a partially ordered set $(S, \preceq)$ where:
$$s_1 \preceq s_2 \iff \text{there exists a lawful transition sequence } s_1 \to \cdots \to s_2$$
The relation is reflexive, antisymmetric, and transitive.

**Theorem: Monotonicity of Evidence Transitions:**
Let $f: \text{Evidence}\langle T, s_1, W \rangle \to \text{Evidence}\langle T, s_2, W \rangle$ be any lawful transition. Then:
$$s_1 \preceq s_2$$
**Proof:**
By exhaustive inspection of the public constructors of `Evidence`. Transition methods are strictly one-way (e.g. `into_parsed`, `into_projected`, `into_receipted`), mapping from an earlier stage to a later stage. The constructors for advanced states are `pub(crate)` or sealed, meaning external callers cannot bypass the order. Hence, $s_1 \preceq s_2$. $\square$

---

### Section 22: Witness Markers as a Free Monoid
Let $\mathcal{W}$ be the set of types implementing the `Witness` trait. The nominal type system ensures that:
$$\iota(W_1, W_2) = 0 \implies \text{Admission}\langle T, W_1 \rangle \not\equiv \text{Admission}\langle T, W_2 \rangle$$
This nominal separation means witness markers act as unique coordinates in the type-level authority space. No coercion exists between different witness coordinates.

---

### Section 23: The Between01 Lattice and Conformance Arithmetic
Let $\mathbb{Q}_{01} = \{ \frac{p}{q} \in \mathbb{Q} \mid 0 \leq \frac{p}{q} \leq 1, q > 0 \}$. $\mathbb{Q}_{01}$ is a bounded lattice under:
$$\frac{p_1}{q_1} \wedge \frac{p_2}{q_2} = \min\left(\frac{p_1}{q_1}, \frac{p_2}{q_2}\right), \qquad \frac{p_1}{q_1} \vee \frac{p_2}{q_2} = \max\left(\frac{p_1}{q_1}, \frac{p_2}{q_2}\right)$$
with bounds $0/1$ and $1/1$.

For trace token-replay fitness of $\tau_i$ against WF-net $N$:
$$f(\tau_i, N) = \frac{1}{2}\left(1 - \frac{m_i}{c_i}\right) + \frac{1}{2}\left(1 - \frac{r_i}{p_i}\right) \in [0, 1]$$
Aggregate fitness of $L = \{\tau_1, \ldots, \tau_n\}$ is:
$$F(L, N) = \frac{\sum_{i=1}^n |\tau_i| \cdot f(\tau_i, N)}{\sum_{i=1}^n |\tau_i|}$$

Precision via the escaping-edges estimator is:
$$\text{prec}(L, N) = \frac{\sum_{\hat\sigma \in \text{Pref}(L)} | \text{EN}(N, \hat\sigma) \cap A(L) |}{\sum_{\hat\sigma \in \text{Pref}(L)} | \text{EN}(N, \hat\sigma) |} \in [0, 1]$$

---

### Section 24: WF-Net Soundness as a Reachability Decision Problem
Deciding whether an arbitrary Petri net is sound is PSPACE-complete.
Block-structured nets are certified statically via their construction invariants. The incidence matrix $C$ is used to compute place invariants $A$:
$$A \in \ker(C^\top) \implies A \cdot \mathbf{m} = A \cdot M_0 \quad \text{for all reachable markings } \mathbf{m}$$
Soundness is verified by checking that the minimal $P$-invariants cover all places in $P$.

---

### Section 25: Algorithm: The ALIVE Certification Protocol
Let $\mathcal{C}$ be the codebase, $\mathcal{F}^+$ compile-pass fixtures, $\mathcal{F}^-$ compile-fail fixtures, and $\mathcal{D}$ expected stderr diagnostics.

**Algorithm: ALIVE Protocol:**
1. Initialize $R \leftarrow \emptyset$.
2. For each $f^+ \in \mathcal{F}^+$:
   * If $\text{Compile}(\mathcal{C} \cup \{f^+\}) \neq \text{Ok}$, then $R \leftarrow R \cup \{(\text{PASS\_FAIL}, f^+)\}$.
3. For each $(f^-, d) \in \mathcal{F}^- \times \mathcal{D}$:
   * Let $res \leftarrow \text{Compile}(\mathcal{C} \cup \{f^-\})$.
   * If $res = \text{Ok}$, $R \leftarrow R \cup \{(\text{FAIL\_PASS}, f^-)\}$.
   * Else if $\text{diagnostic}(res) \neq d$, $R \leftarrow R \cup \{(\text{WRONG\_LAW}, f^-, d)\}$.
4. If $R = \emptyset$, return $\text{ALIVE}$. Else, return $\text{PARTIAL}(R)$.

---

### Section 26: Loss-Policy Algebra
The three loss policies form a semiring $(\mathcal{L}, \oplus, \otimes)$ ordered by permissiveness:
$$\text{RefuseLoss} \leq \text{AllowNamedProjection} \leq \text{AllowLossWithReport}$$
where $p_1 \oplus p_2 = \max(p_1, p_2)$ and $p_1 \otimes p_2 = \min(p_1, p_2)$.

A loss report is a signed measure $\ell: \mathcal{I} \to \mathbb{N}_0$ over the item universe $\mathcal{I}$ of dropped structures. Total loss is the $L_1$ norm:
$$\|\ell\|_1 = \sum_{x \in \mathcal{I}} \ell(x) \in \mathbb{N}_0$$
A projection is lossless if and only if $\|\ell\|_1 = 0$.

---

### Section 27: Algorithm: Admission Gate and the Graduation Functor
The structural admission gate maps:
$$\text{Admit}(v, W) \to \text{Result}\langle\text{Admission}\langle T, W \rangle, \text{Refusal}\langle R, W \rangle\rangle$$
where $R$ is a named structural law. The validation check runs in $O(|\Lambda_W| \cdot |v|)$ time.

The graduation map is a functor $\mathcal{G}: \mathbf{Struct} \to \mathbf{Exec}$ mapping structural types in $\mathbf{Struct}$ to their corresponding runtime representations in $\mathbf{Exec}$.

**Algorithm: Receipt-Bearing Commit Validation:**
1. Given range $[c_1, c_2]$ in repository $\mathcal{R}$, initialize $\text{violations} \leftarrow [ ]$.
2. For each commit $c \in [c_1, c_2]$:
   * Let $msg \leftarrow \text{CommitMessage}(c)$.
   * If "Law:" $\notin msg$ or the class prefix is not in $\{\texttt{type-law}, \texttt{fixture-pass}, \texttt{fixture-fail}, \texttt{paper-ledger}, \texttt{audit}\}$, add $c$ to $\text{violations}$.
3. Return $\text{violations}$.

**Differential Analysis of Certification State:**
Let $\mathbf{g} = (g_1, \ldots, g_{10}) \in \mathbb{N}^{10}$ be the gate counts. The certification step is monotone if:
$$\Delta\mathbf{g}^{(k)} = \mathbf{g}^{(k+1)} - \mathbf{g}^{(k)} \geq \mathbf{0}$$
which ensures that counts of fixtures, papers, and audits never decrease during development.
