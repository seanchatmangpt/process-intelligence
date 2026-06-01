# BPMN OR-Join Smart-Completion — Doctrine

> An OR-Join gateway must synchronize only the incoming branches that are active or can potentially become active. The evaluation of future potential is a structural graph check.

Source: [paper-to-execution-law.md](file:///Users/sac/process-intelligence/sources/papers/paper-to-execution-law.md) — compiled for runtime gateway semantics.

---

## 1. The OR-Join Problem and Petri Net Formalism

BPMN 2.0 OR-Join gateways are notoriously difficult to implement deterministically because they require non-local lookahead. To determine if an OR-Join should fire, the runtime must verify whether any token currently in the process flow can still reach one of the waiting incoming branches.

Let $N = (P, T, F)$ be a Workflow Net (WF-net), where:
- $P$ is a finite set of places.
- $T$ is a finite set of transitions.
- $F \subseteq (P \times T) \cup (T \times P)$ is the set of directed flow relations.
- $i \in P$ is the unique source place with $\bullet i = \emptyset$.
- $o \in P$ is the unique sink place with $o\bullet = \emptyset$.
- Every node $n \in P \cup T$ is on a path from $i$ to $o$.

Let $g \in T$ be a transition representing an inclusive OR-Join gateway.
The preset of $g$, denoted by $\bullet g = \{p \in P \mid (p, g) \in F\}$, represents the set of incoming branches to the gateway.
Let $M : P \to \mathbb{N}$ represent the current marking (token distribution) of $N$, where $M(p)$ is the number of tokens at place $p$.

The gateway $g$ is designed to synchronize only the incoming branches that are currently active or can potentially become active. Under the Smart-Completion policy, we define the firing rule and safety conditions using structural reachability.

---

## 2. Smart-Completion Policy and Firing Rule

Under the Smart-Completion policy, the OR-Join transition $g$ is enabled at marking $M$, denoted by $M \vdash_{SC} [g\rangle$, if and only if two conditions are met:

1. **Token Presence**: At least one incoming branch has a token.
   $$\sum_{p \in \bullet g} M(p) \ge 1$$
2. **No Incoming Token Potential (No Blocking Tokens)**: There is no active token in the net that can still reach an incoming branch of $g$ without passing through $g$.
   
   Let $N \setminus \{g\} = (P, T \setminus \{g\}, F \cap ((P \cup T \setminus \{g\}) \times (P \cup T \setminus \{g\})))$ be the subnet where transition $g$ is removed.
   We define the reflexive transitive closure of the flow relation in the subnet as $\to_{N \setminus \{g\}}^*$. For places $p_1, p_2 \in P$, $p_1 \to_{N \setminus \{g\}}^* p_2$ if there is a path from $p_1$ to $p_2$ in the directed graph of $N \setminus \{g\}$.
   We define the transitive closure (excluding the 0-length path) as $\to_{N \setminus \{g\}}^+$.
   
   A place $p' \in P$ is a **blocking place** for $g$ under marking $M$ if there exists $p \in \bullet g$ such that $p' \to_{N \setminus \{g\}}^* p$, and at least one of the following structural conditions holds:
   - **External Token Path**: $p' \notin \bullet g$ (the token is outside the input places of $g$ and can reach an input).
   - **Alternative Join Destination**: $p' \in \bullet g \land \exists p'' \in \bullet g \setminus \{p'\} : p' \to_{N \setminus \{g\}}^* p''$ (the token is at one input place but can still reach another input place of $g$).
   - **Cyclic Feedback (Loop)**: $p' \in \bullet g \land p' \to_{N \setminus \{g\}}^+ p'$ (the token is at an input place but is part of a cycle/loop that does not contain $g$, allowing it to flow out and return to the same input place).

   Thus, the Smart-Completion safety condition is:
   $$\forall p' \in P \quad \left( M(p') > 0 \implies p' \text{ is not a blocking place for } g \right)$$

If the safety condition is met, no further tokens can arrive at the gateway. The OR-Join completes ("smart completion") and executes immediately.

---

## 3. Loop-Join Execution Paths and Decidability

In processes containing cyclic control flows (e.g., feedback loops returning to an OR-Join), tokens can cycle repeatedly.
- Let $L \subseteq P \cup T$ be a loop structure. If $L \cap \bullet g \neq \emptyset$, any token in $L$ has a path to $\bullet g$.
- Since $L$ is a cycle, for any $p_{input} \in L \cap \bullet g$, the relation $p_{input} \to_{N \setminus \{g\}}^+ p_{input}$ holds.
- Therefore, any active token within $L$ is classified as a blocking place under the Smart-Completion policy. The OR-Join must wait until the token exits the loop (i.e., transitions to a place $p_{out}$ from which $\bullet g$ is structurally unreachable in $N \setminus \{g\}$).
- **Decidability**: The structural reachability check $\to_{N \setminus \{g\}}^*$ is equivalent to transitive closure reachability on a directed graph. For a net with $|P|$ places and $|T|$ transitions, this check is computable in $O(|P|(|P| + |T|))$ time via standard Breadth-First Search (BFS) or Depth-First Search (DFS). The Smart-Completion policy is therefore **decidable in polynomial time**, resolving the general undecidability of OR-joins.

---

## 4. Liveness and Soundness Invariants

For a Workflow Net $N$, soundness requires:
1. **Option to complete**: For every marking $M$ reachable from the initial marking $M_0 = [i]$, there exists a firing sequence leading to the final marking $M_f = [o]$:
   $$\forall M \in [M_0\rangle, \quad M_f \in [M\rangle$$
2. **Proper completion**: For every marking $M$ reachable from $M_0$, if $M \ge M_f$, then $M = M_f$:
   $$\forall M \in [M_0\rangle, \quad M \ge M_f \implies M = M_f$$
3. **No dead transitions**: For every transition $t \in T$, there exists a marking $M$ reachable from $M_0$ that enables $t$:
   $$\forall t \in T, \quad \exists M \in [M_0\rangle \quad \text{s.t. } M \xrightarrow{t}$$

Under the Smart-Completion policy, we establish the following invariants:
- **Liveness Invariant (No Deadlock / Premature Fire)**: If $g$ fires at marking $M$, then all tokens that could ever reach $\bullet g$ before $g$'s consumption have already arrived at $\bullet g$. This prevents premature firing which would leave late-arriving tokens stranded, preserving the "option to complete" and preventing deadlocks.
- **Soundness Invariant (Proper Completion / No Leftover Tokens)**: Premature firing of an OR-join results in tokens being left in the upstream branches of the net after the gateway has already fired. By enforcing the Smart-Completion safety condition, we guarantee that all incoming tokens are synchronized, preventing token accumulation and ensuring proper completion.

---

## 5. Temporal Logic (LTL / CTL) Specifications

We formalize the safety and liveness invariants of the OR-Join using Linear Temporal Logic (LTL) and Computation Tree Logic (CTL). Let $g$ be the transition representing the OR-Join. Let $fired(g)$ be a proposition indicating that $g$ fires in the current transition. Let $M(p) > 0$ be the proposition that place $p$ contains at least one token.

1. **Safety (No Premature Firing)**:
   $$G \left( fired(g) \implies \left( \exists p \in \bullet g : M(p) > 0 \land \forall p' \in P \left( M(p') > 0 \implies \neg \text{Blocking}(p', g) \right) \right) \right)$$
   Where the predicate $\text{Blocking}(p', g)$ is defined as:
   $$\text{Blocking}(p', g) \equiv \exists p \in \bullet g : \left( p' \to_{N \setminus \{g\}}^* p \right) \land \left( p' \notin \bullet g \lor \exists p'' \in \bullet g \setminus \{p'\}: p' \to_{N \setminus \{g\}}^* p'' \lor p' \to_{N \setminus \{g\}}^+ p' \right)$$

2. **Liveness (Eventual Progress)**:
   In any execution path, if there is a token at $\bullet g$ and no blocking tokens exist in the net, then the gateway must eventually fire or the tokens must be consumed by a valid competing transition:
   $$G \left( \left( \exists p \in \bullet g : M(p) > 0 \land \forall p' \in P \left( M(p') > 0 \implies \neg \text{Blocking}(p', g) \right) \right) \implies F \left( fired(g) \lor \bigvee_{t \in T \setminus \{g\}, \bullet t \cap \bullet g \neq \emptyset} fired(t) \right) \right)$$

3. **Liveness (CTL Reachability of Final Marking)**:
   Under the Smart-Completion policy, for all reachable markings $M$:
   $$A G \left( A F ( M = M_f ) \right)$$

---

## 6. Conformance and Alignment Calculations

When aligning an event log $L$ against a process model containing an OR-join $g$, the alignment cost is calculated over the state space of the Petri Net.
- Let $\sigma \in \Sigma^*$ be a trace in the event log, and let $N$ be the process model.
- The alignment problem is to find a sequence of moves that minimizes the alignment cost:
  $$\gamma^* = \operatorname{argmin}_{\gamma \in \text{Align}(\sigma, N)} \sum_{(t, a) \in \gamma} c(t, a)$$
- If the model execution attempts to fire $g$ while the Smart-Completion policy is violated (i.e., some active token is at a blocking place), this transition is illegal in the model under the Smart-Completion semantics.
- Therefore, a log event representing the firing of $g$ cannot be mapped to a synchronous move of $g$ if there are active blocking tokens. Firing $g$ in the log at this point would require either:
  1. A **Log Move** on $g$ (cost $c(\gg, g)$), meaning the log recorded $g$ but the model could not fire it.
  2. Or multiple **Model Moves** (cost $\sum c(t_j, \gg)$) to advance the blocking tokens until they either reach $\bullet g$ or leave the reachability path, thereby satisfying the Smart-Completion safety condition, followed by a synchronous move on $g$.
- In either case, the alignment algorithm automatically penalizes deviations from the Smart-Completion policy, ensuring that offline compliance checks (such as those required by [GAP_002_OR_JOIN_AMBIGUITY.md](file:///Users/sac/process-intelligence/gaps/GAP_002_OR_JOIN_AMBIGUITY.md)) will detect and record the exact mismatch.

---

## 7. Compliance and Verification

- **Pathway 9 (AmbiguousBpmnGateway)**: Any process execution using OR-Joins without an explicit smart completion policy is rejected at the admission boundary. See [structural-gaps.md](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/structural-gaps.md) for transition rules and implementation details.
- **Verification Engine**: The underlying process verification engine in [petri.rs](file:///Users/sac/process-intelligence/sources/wasm4pm/src/petri.rs) performs dynamic reachability/coverability and 1-boundedness checks to ensure that the workflow net does not exhibit unbounded behavior or token accumulation which would invalidate OR-Join synchronization.
- **Siphon-Trap Verification**: The engine verifies siphon-trap properties in [petri.rs](file:///Users/sac/process-intelligence/sources/wasm4pm/src/petri.rs) to confirm structural deadlock-freeness, proving that all siphons contain a marked trap. In the context of OR-joins, this guarantees that tokens cannot become permanently trapped in upstream loops and prevent OR-join completion.
- **Auditing**: Alignment logs must record the reachability state at the moment of OR-Join firing to allow offline auditors to replay and verify correctness.

---

## 8. Citations and References

- **Völzer, H.**: *Semantics of the BPMN OR-Join*. BPM Conference, 2010. Cited in [paper-to-execution-law.md](file:///Users/sac/process-intelligence/sources/papers/paper-to-execution-law.md).
- **van der Aalst, W. M. P.**: *The Application of Petri Nets to Workflow Management*. Journal of Circuits, Systems and Computers, 1998.
