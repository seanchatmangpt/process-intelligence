# Conformance Authority Map

This document defines the execution requirements for the conformance checking and alignment capabilities implemented in the Rust-based WebAssembly (`wasm4pm`) execution engine. It specifies the FFI boundaries, algorithms, mathematical equations, and verification procedures.

## 1. FFI Boundary and Memory Architecture for Conformance

The conformance engine validates process traces against a Petri Net model. To ensure deterministic performance and zero heap leaks:
- **Net & Log Ingestion**: The host passes pointers to the serialized Petri Net (soundness verified) and the event log (admitted by the compat layer).
- **Static Graph Layout**: The Petri Net is constructed in a fixed-size contiguous vector. Place-to-transition and transition-to-place mappings are indexed using raw offsets.
- **Trace Replay Memory**: Replay state (marking, token lists) is stored in a reusable linear memory arena to avoid allocation overhead per trace.

---

## 2. Petri Net Soundness Verification ($WF$-net Constraints)

Before conformance checking or alignment execution, the WASM engine verifies that the target model is a valid Workflow Net ($WF$-net) according to van der Aalst 1998:

1. **Structure Verification**:
   - Locate place $i$ such that $\bullet i = \emptyset$. Raise `WFNetValidationError` if count $\ne 1$.
   - Locate place $o$ such that $o \bullet = \emptyset$. Raise `WFNetValidationError` if count $\ne 1$.
   - Construct a directed graph representing the places, transitions, and arcs. Ensure every place $p \in P$ and transition $t \in T$ is on a path from $i$ to $o$ using a depth-first search (DFS).
2. **Behavioral Analysis (Liveness and Boundedness)**:
   - Construct a coverability graph starting from the initial marking $M_0 = [i^1]$ (one token in place $i$, all other places empty).
   - Verify that the net is bounded (specifically 1-bounded, representing safe nets). If any node in the coverability graph contains an $\omega$ (unbounded token count), the net is rejected.
   - Verify liveness: check that from any reachable marking, it is possible to enable any transition.

---

## 3. Token Game Replay and Fitness Calculation

For each trace $\sigma = \langle e_1, e_2, \dots, e_k \rangle$, the WASM engine simulates the firing of transitions.

### Token Bookkeeping
During the replay of a trace, the engine maintains four counters:
- $p$: Produced tokens (tokens created in output places when transitions fire).
- $c$: Consumed tokens (tokens removed from input places when transitions fire).
- $m$: Missing tokens (tokens artificially created in input places to enable a transition that was not enabled by the current marking).
- $r$: Remaining tokens (tokens left in the net at the end of the trace execution, excluding the single token expected in the sink place $o$).

### Replay Rules
1. Initialize the marking to $M = [i^1]$, with counters $p = 1, c = 0, m = 0, r = 0$.
2. For each event $e_j$ in the trace:
   - Identify the transition $t_j$ labeled with $e_j$.
   - For each input place $p \in \bullet t_j$:
     - If $M(p) > 0$, decrement $M(p)$ and increment $c$.
     - If $M(p) == 0$, increment $m$ and increment $c$ (simulate token creation).
   - For each output place $q \in t_j \bullet$:
     - Increment $M(q)$ and increment $p$.
3. When the trace ends, attempt to consume the token from the sink place $o$:
   - If $M(o) > 0$, decrement $M(o)$ and increment $c$.
   - If $M(o) == 0$, increment $m$ and increment $c$.
4. Check all places in the net:
   - For any place $p \ne o$, if $M(p) > 0$, add $M(p)$ to $r$.

### Trace and Log Fitness Equations
- **Trace Fitness** $f(\sigma, N)$:
  $$f(\sigma, N) = 1 - \frac{m}{c} - \frac{r}{p}$$
- **Log Fitness** $f(L, N)$ (weighted by trace frequency $L(\sigma)$):
  $$f(L, N) = 1 - \frac{\sum_{\sigma \in L} L(\sigma) \cdot m(\sigma, N)}{\sum_{\sigma \in L} L(\sigma) \cdot c(\sigma, N)} - \frac{\sum_{\sigma \in L} L(\sigma) \cdot r(\sigma, N)}{\sum_{\sigma \in L} L(\sigma) \cdot p(\sigma, N)}$$

---

## 4. Optimal Alignments via $A^*$ Search (Adriansyah 2014)

When traces do not fit the model exactly, the engine computes optimal alignments.

### State Space Formulation
Let $\sigma$ be the trace and $N = (P, T, F)$ be the Petri Net. An alignment is a sequence of moves.
We define a cost function $K$ for moves:
- **Move on Log**: $K(a, \gg) = 1$ (event $a$ occurs in log but transition is not fired).
- **Move on Model**: $K(\gg, t) = 1$ (transition $t$ fires but no matching event is in the log).
  - *Exception*: Silent/invisible transitions $\tau$ have zero cost: $K(\gg, \tau) = 0$.
- **Synchronous Move**: $K(a, t) = 0$ if $\text{label}(t) == a$. If $\text{label}(t) \ne a$, the cost is $\infty$.

### Algorithm Execution
1. The engine executes the $A^*$ search on a state graph where each state node $n$ consists of:
   - $n.\text{marking}$: The current marking of the Petri Net.
   - $n.\text{idx}$: The index of the next event to align in trace $\sigma$.
2. The search uses an open priority queue ordered by:
   $$f(n) = g(n) + h(n)$$
   - $g(n)$: The exact cost of moves accumulated from the start state to $n$.
   - $h(n)$: The heuristic cost, calculated as the shortest path distance in the Petri Net from $n.\text{marking}$ to the target marking $[o^1]$, ignoring log sync constraints. This heuristic is guaranteed to be admissible and consistent.
3. Keep track of visited states in a closed hash map to avoid loops.
4. When the goal state is reached (trace index == trace length AND marking == $[o^1]$), reconstruct the optimal alignment path.

---

## 5. Non-Forgeable Conformance Receipts

The WASM engine returns a structured JSON/FlatBuffer conformance receipt:
- Contains log hash, model hash, log fitness, and trace-by-trace alignment records.
- Signs the receipt using an internal cryptographic key pair, generating a signature that can be verified by downstream buyers or board auditors.

---
*Back to [execution-authority-atlas.md](file:///Users/sac/process-intelligence/sources/wasm4pm/execution-authority-atlas.md)*
