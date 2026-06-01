# Conformance Authority Map v30.1.2: wasm4pm Execution Engine

**Version:** 30.1.2  
**Authority:** Execution Agent  
**Classification:** Core Execution Specification  
**Date:** 2026-05-31

---

## Executive Summary

Conformance Authority within wasm4pm governs **alignment between process models and event logs, fitness/precision metrics, admission gates, and the Evidence<T, State, Witness> type-law boundary**. Unlike v30.1.1's abstract framing, **v30.1.2 specifies the concrete conformance operations and formal guarantees** wasm4pm must implement.

The core doctrine: **If a model claims to explain the log, but alignment reveals mismatches, the model fails admission.** Fitness metrics are not advisory; they are gatekeeping thresholds.

---

## 1. Alignment and Fitness: Mandatory Algorithms

### 1.1 Optimal Alignment (A* Search)

**Authority Claim:** wasm4pm must implement A* search for optimal trace-to-model alignment per Adriansyah (2014).

**Formal Specification:**
- **Input:** 
  - Trace $\sigma = \langle e_1, \ldots, e_n \rangle$ (sequence of event activities over alphabet $\Sigma$)
  - Petri Net $N = (P, T, F)$ with labeling function $\lambda: T \to \Sigma \cup \{\tau\}$, initial marking $M_0 = [i]$, and final marking $M_f = [o]$
- **Output:** 
  - Optimal alignment $\gamma = \langle (a_1, t_1), \ldots, (a_m, t_m) \rangle$ where $a_k \in \Sigma \cup \{\gg\}$ and $t_k \in T \cup \{\gg\}$ (excluding $(\gg, \gg)$) representing log moves, model moves, or synchronous moves, satisfying $\pi_{\text{log}}(\gamma) = \sigma$ and $[i] \xrightarrow{\pi_{\text{model}}(\gamma)} [o]$
  - Cost: $\operatorname{Cost}(\gamma) = \sum_{k=1}^{m} \operatorname{cost}(a_k, t_k)$
  - Witness: Proof that no cheaper alignment exists

**Cost Function:**
$$\operatorname{cost}(a, t) = \begin{cases} 0 & \text{if } a \in \Sigma, \, t \in T \text{ and } \lambda(t) = a \\ 1 & \text{if } a = \gg, \, t \in T \text{ and } \lambda(t) \in \Sigma \\ 0 & \text{if } a = \gg, \, t \in T \text{ and } \lambda(t) = \tau \\ 1 & \text{if } a \in \Sigma \text{ and } t = \gg \\ \infty & \text{otherwise} \end{cases}$$

**Alignment-Based Fitness (Adriansyah 2014):**
For a trace $\sigma$ and Petri Net $N$, the alignment fitness $f_{\text{align}}(\sigma, N)$ is computed using the optimal alignment $\gamma^*$:
$$f_{\text{align}}(\sigma, N) = 1 - \frac{\operatorname{Cost}(\gamma^*)}{|\sigma| + d_{\text{model}}([i], [o])}$$
where $|\sigma|$ is the trace length and $d_{\text{model}}([i], [o])$ is the shortest path cost from the initial marking $[i]$ to the final marking $[o]$ using only model moves.

**Algorithm:**
- **Open set:** Priority queue of `(f_score, g_cost, h_estimate, state)` ordered by ascending `f_score = g_cost + h_estimate`.
- **Closed set:** Visited marking+position pairs `(marking, trace_index)`.

### 1.1.1 Reachability Heuristic
- **Heuristic $h_{\text{reach}}(M, p)$:** 
  $$h_{\text{reach}}(M, p) = \max(0, d_{\text{min}}(M, M_o) - (n - p))$$
  where $d_{\text{min}}(M, M_o)$ is the shortest path distance (counting only visible transitions) in the Petri Net from the current marking $M$ to the final marking $M_o$, and $n - p$ is the remaining trace length (where $n = |\sigma|$ and $p$ is the current event index being aligned).
  
  *Audit Note on Admissibility & Consistency:*
  The previous heuristic (remaining trace length $h(M, p) = n - p$) was **inadmissible** and **inconsistent**. Because the cost of a move on both is $0$, a sequence of $n - p$ remaining events matching the model perfectly could be aligned with an actual cost of $0$, which is strictly less than the estimated remaining trace length ($n - p > 0$).
  
  The audited heuristic $h_{\text{reach}}(M, p) = \max(0, d_{\text{min}}(M, M_o) - (n - p))$ is mathematically guaranteed to be:
  1. **Admissible:** It never overestimates the actual remaining cost $h^*(M, p)$. Let $N_L$, $N_M$, and $N_B$ be the number of moves on log, model (visible), and both, respectively, in the optimal remaining alignment to the goal.
     - The remaining cost is $C_{\text{rem}} = 1 \cdot N_L + 1 \cdot N_M + 0 \cdot N_B = N_L + N_M$.
     - The remaining trace length is $n - p = N_L + N_B$, which implies $N_L = (n - p) - N_B$.
     - The number of visible transitions fired is $V = N_M + N_B \ge d_{\text{min}}(M, M_o)$, which implies $N_M = V - N_B$.
     - Since $N_L \ge 0$, we have $C_{\text{rem}} \ge N_M = V - N_B$.
     - Since $V \ge d_{\text{min}}(M, M_o)$ and $N_B \le n - p \implies -N_B \ge -(n - p)$, we have:
       $$C_{\text{rem}} \ge V - N_B \ge d_{\text{min}}(M, M_o) - (n - p)$$
     - Since cost is non-negative, $C_{\text{rem}} \ge 0$.
     - Combining these: $C_{\text{rem}} \ge \max(0, d_{\text{min}}(M, M_o) - (n - p)) = h_{\text{reach}}(M, p)$, proving admissibility.
  2. **Consistent (Monotonic):** For any step from $u = (M, p)$ to $v = (M', p')$ with step cost $c$:
     Let $d = d_{\text{min}}(M, M_o)$, $d' = d_{\text{min}}(M', M_o)$, $r = n - p$, and $r' = n - p'$. We prove $h_{\text{reach}}(u) \le c + h_{\text{reach}}(v)$ case-by-case:
     - **Move on Log:** Here, $p' = p + 1 \implies r' = r - 1$, $M' = M \implies d' = d$, and $c = 1$. Since $d - r < d - r + 1$, we have:
       $$\max(0, d - r) \le \max(0, d - (r - 1)) \le 1 + \max(0, d - r')$$
     - **Move on Model (Visible):** Here, $p' = p \implies r' = r$, $M \xrightarrow{t} M'$ (visible transition) $\implies d \le d' + 1$, and $c = 1$. Since $d - r \le d' - r + 1$, we have:
       $$\max(0, d - r) \le \max(0, d' - r + 1) \le 1 + \max(0, d' - r')$$
     - **Move on Model (Silent):** Here, $p' = p \implies r' = r$, $M \xrightarrow{\tau} M'$ (silent transition) $\implies d \le d'$, and $c = 0$. Since $d - r \le d' - r$, we have:
       $$\max(0, d - r) \le \max(0, d' - r) = 0 + \max(0, d' - r')$$
     - **Move on Both:** Here, $p' = p + 1 \implies r' = r - 1$, $M \xrightarrow{t} M'$ (visible transition) $\implies d \le d' + 1$, and $c = 0$. Since $d - r \le d' - r + 1 = d' - r'$, we have:
       $$\max(0, d - r) \le \max(0, d' - r') = 0 + \max(0, d' - r')$$

### 1.1.2 State Equation Heuristic
- **Heuristic $h_{\text{SE}}(M, p)$:** 
  The state equation heuristic computes a lower bound by relaxing the integer reachability problem to a Linear Program (LP). Let $Y \in \mathbb{Z}^{|P| \times |T|}$ be the incidence matrix of the Petri Net, and let $T_{\text{vis}}$ and $T_{\text{sil}}$ represent visible and silent transitions, respectively. For each activity $a \in \Sigma$, let $n(a, p)$ be the number of remaining occurrences of activity $a$ in the suffix trace from index $p$ to $n$.
  
  The LP is defined as:
  $$h_{\text{SE}}(M, p) = \min_{x_m, x_b, x_\tau} \left( (n - p) - \sum_{t \in T_{\text{vis}}} x_b(t) + \sum_{t \in T_{\text{vis}}} x_m(t) \right)$$
  subject to:
  $$Y \cdot (x_m + x_b + x_\tau) = M_o - M$$
  $$\sum_{t \in T_{\text{vis}}: label(t) = a} x_b(t) \le n(a, p) \quad \forall a \in \Sigma$$
  $$x_m(t) = 0 \quad \forall t \in T_{\text{sil}}$$
  $$x_b(t) = 0 \quad \forall t \in T_{\text{sil}}$$
  $$x_\tau(t) = 0 \quad \forall t \in T_{\text{vis}}$$
  $$x_m, x_b, x_\tau \ge 0$$
  
  *Audit Note on Admissibility & Consistency:*
  1. **Admissibility:** Every valid remaining alignment from state $(M, p)$ to $(M_o, n)$ maps to a sequence of transitions $x \in \mathbb{N}^{|T|}$ that must satisfy the Petri net state equation $Y \cdot x = M_o - M$. Firing sequences are split into Moves on Model ($x_m$, visible only), Moves on Both ($x_b$, visible only), and silent steps ($x_\tau$). The number of synchronous moves $x_b$ for any activity $a$ cannot exceed the trace occurrences $n(a, p)$. Since any real alignment path forms a valid integer solution to the above constraints, the optimal value of the LP relaxation $h_{\text{SE}}(M, p)$ is guaranteed to be a lower bound on the actual minimum remaining cost $h^*(M, p)$, proving admissibility.
  2. **Consistency (Monotonicity):** Let $u = (M, p)$ and $v = (M', p')$ be search states, and $u \xrightarrow{c} v$ be a move with step cost $c$. Let $(x_m^*, x_b^*, x_\tau^*)$ be the optimal LP solution for state $v$. We construct a feasible LP solution $(\tilde{x}_m, \tilde{x}_b, \tilde{x}_\tau)$ for state $u$:
     - **Move on Log:** $M' = M$, $p' = p + 1$, $c = 1$. Let $(\tilde{x}_m, \tilde{x}_b, \tilde{x}_\tau) = (x_m^*, x_b^*, x_\tau^*)$. Since $M_o - M = M_o - M'$ and $n(a, p) \ge n(a, p')$, this is feasible for $u$. Its cost is:
       $$\tilde{C} = (n - p) - \sum \tilde{x}_b(t) + \sum \tilde{x}_m(t) = (n - p' + 1) - \sum x_b^*(t) + \sum x_m^*(t) = h_{\text{SE}}(v) + 1$$
       Therefore, $h_{\text{SE}}(u) \le h_{\text{SE}}(v) + 1 = h_{\text{SE}}(v) + c$.
     - **Move on Model (Visible transition $t_0$):** $M \xrightarrow{t_0} M'$ ($t_0 \in T_{\text{vis}}$), $p' = p$, $c = 1$. Let $\tilde{x}_m = x_m^* + e_{t_0}$, $\tilde{x}_b = x_b^*$, $\tilde{x}_\tau = x_\tau^*$. This is feasible for $u$ because $Y \cdot (\tilde{x}_m + \tilde{x}_b + \tilde{x}_\tau) = Y \cdot (x_m^* + x_b^* + x_\tau^*) + Y \cdot e_{t_0} = M_o - M' + M' - M = M_o - M$. Its cost is:
       $$\tilde{C} = (n - p) - \sum \tilde{x}_b(t) + \sum \tilde{x}_m(t) = h_{\text{SE}}(v) + 1$$
       Therefore, $h_{\text{SE}}(u) \le h_{\text{SE}}(v) + 1 = h_{\text{SE}}(v) + c$.
     - **Move on Model (Silent transition $\tau_0$):** $M \xrightarrow{\tau_0} M'$ ($\tau_0 \in T_{\text{sil}}$), $p' = p$, $c = 0$. Let $\tilde{x}_m = x_m^*$, $\tilde{x}_b = x_b^*$, $\tilde{x}_\tau = x_\tau^* + e_{\tau_0}$. This is feasible for $u$ because $Y \cdot (\tilde{x}_m + \tilde{x}_b + \tilde{x}_\tau) = M_o - M' + M' - M = M_o - M$. Its cost is $\tilde{C} = h_{\text{SE}}(v)$. Therefore, $h_{\text{SE}}(u) \le h_{\text{SE}}(v) = h_{\text{SE}}(v) + c$.
     - **Move on Both (Synchronous transition $t_0$ with label $a_0$):** $M \xrightarrow{t_0} M'$ ($t_0 \in T_{\text{vis}}$), $p' = p + 1$, $c = 0$. Let $\tilde{x}_m = x_m^*$, $\tilde{x}_b = x_b^* + e_{t_0}$, $\tilde{x}_\tau = x_\tau^*$. This is feasible for $u$ because the state equation balances ($M_o - M' + M' - M = M_o - M$) and the activity limit holds as $n(a_0, p) = n(a_0, p') + 1 \ge \sum x_b^*(t) + 1$. Its cost is:
       $$\tilde{C} = (n - p' + 1) - \left( \sum x_b^*(t) + 1 \right) + \sum x_m^*(t) = h_{\text{SE}}(v)$$
       Therefore, $h_{\text{SE}}(u) \le h_{\text{SE}}(v) = h_{\text{SE}}(v) + c$.

- **Termination:** When goal marking $M_o$ is reached with no events remaining ($p = n$).

**Receipt Structure:**
```json
{
  "algorithm": "optimal_alignment_a_star",
  "trace_id": "<case_id>",
  "log_hash": "<SHA-256>",
  "model_hash": "<SHA-256>",
  "alignment_length": <m>,
  "total_cost": <C(γ)>,
  "move_on_both": <count>,
  "move_on_model": <count>,
  "move_on_log": <count>,
  "search_states_explored": <count>,
  "heuristic_accuracy": <h_avg / actual_remaining>,
  "timestamp": "<ISO 8601>",
  "signature": "<Ed25519>"
}
```

### 1.2 Fitness Metric: Standard van der Aalst Equation

**Authority Claim:** wasm4pm computes fitness per van der Aalst's standard token-based replay equation (applied to all traces in a log, using forced-firing replay).

**Formal Definition:**

For a single trace $\sigma$ during token-based replay (not using alignments):
- $p$ = tokens produced during replay
- $c$ = tokens consumed during replay
- $m$ = missing tokens (had to be artificially injected to fire next transition)
- $r$ = remaining tokens (left in net after trace completes, excluding sink place)

$$\text{fitness}(\sigma, N) = \frac{1}{2} \left( 1 - \frac{m}{c} \right) + \frac{1}{2} \left( 1 - \frac{r}{p} \right)$$

**Log-Level Fitness (frequency-weighted):**

$$\text{fitness}(L, N) = \frac{1}{2} \left( 1 - \frac{\sum_{\sigma \in L} \text{freq}(\sigma) \cdot m(\sigma)}{\sum_{\sigma \in L} \text{freq}(\sigma) \cdot c(\sigma)} \right) + \frac{1}{2} \left( 1 - \frac{\sum_{\sigma \in L} \text{freq}(\sigma) \cdot r(\sigma)}{\sum_{\sigma \in L} \text{freq}(\sigma) \cdot p(\sigma)} \right)$$

**Boundary Conditions:**
- $\text{fitness} = 1.0$: Perfect alignment (every event matched, no missing/remaining tokens)
- $\text{fitness} \geq 0.95$: Admissible for operations (Blue River Dam Gate 3)
- $0.85 \leq \text{fitness} < 0.95$: Admissible only with Executive Board override signature
- $\text{fitness} < 0.85$: **Never admissible** (hard floor)

**Receipt Structure:**
```json
{
  "algorithm": "fitness_van_der_aalst",
  "log_hash": "<SHA-256>",
  "model_hash": "<SHA-256>",
  "trace_count": <traces analyzed>,
  "avg_fitness": <f(L, N)>,
  "fitness_distribution": {
    "perfect_1_0": <count>,
    "high_0_95_to_1_0": <count>,
    "medium_0_85_to_0_95": <count>,
    "low_below_0_85": <count>
  },
  "admission_verdict": "<PASS | CONDITIONAL | FAIL>",
  "board_override_required": <bool>,
  "timestamp": "<ISO 8601>",
  "signature": "<Ed25519>"
}
```

### 1.3 Precision Metric: Escaping Transitions (ETC) Precision

**Authority Claim:** wasm4pm computes precision (inverse of false positives) to prevent overfitting using the Escaping Transitions (ETC) algorithm.

**Formal Definition:**

Let $S$ be the set of states in the prefix automaton constructed from the log $L$. For each state $s \in S$:
- $w(s)$ is the visit frequency of state $s$ in the log.
- $E_{\text{total}}(s)$ is the set of transitions enabled by the model $N$ when in the marking corresponding to state $s$.
- $E_{\text{non-escaping}}(s)$ is the set of transitions enabled by the model $N$ at state $s$ that are actually observed in the event log $L$ for that prefix.

$$\text{precision}(L, N) = \frac{\sum_{s \in S} w(s) \cdot \frac{|E_{\text{non-escaping}}(s)|}{|E_{\text{total}}(s)|}}{\sum_{s \in S} w(s)}$$

**Practical Implementation:**
- Build prefix automaton of the log using alignments.
- For each prefix state, evaluate the Petri Net's enabled transitions.
- Mark enabled transitions not taken in the log as escaping edges, penalizing the precision score.

**Receipt Structure:**
```json
{
  "algorithm": "precision_etc",
  "model_hash": "<SHA-256>",
  "prefix_states_count": <count>,
  "precision_score": <0.0 to 1.0>,
  "escaping_edges_count": <count>,
  "timestamp": "<ISO 8601>",
  "signature": "<Ed25519>"
}
```

### 1.4 Boundary Conditions, Memory Isolation, & Non-Determinism Invariants

To guarantee mathematical safety within the sandboxed WASM runtime, the conformance engine must enforce the following structural limits:

1. **Integer Overflow Hardening**:
   - Replay counters ($p$, $c$, $m$, $r$) are tracked using 64-bit unsigned integers (`u64`). 
   - Every token addition or subtraction check must use overflow-checked arithmetic (e.g., `checked_add` and `checked_sub` in Rust).
   - If any counter exceeds `u64::MAX`, the replay terminates immediately with error code `0xFB03` to prevent wrapping exploits.

2. **A* Search Space Boundaries**:
   - **Trace Length Boundary**: Max allowed trace length is $\sigma_{\max} = 10,000$ activities. Longer traces are rejected with `0xFC03`.
   - **Exploration Queue Boundary**: The A* open priority queue is limited to $S_{\max} = 1,000,000$ states to prevent JIT JOP/ROP Heap-smashing or Out-of-Memory (OOM) failures. If $S_{\max}$ is reached, the search aborts with `0xFB03`.

3. **Floating-Point Determinism**:
   - WASM JIT compilers can generate non-deterministic float arithmetic results due to NaN bit-pattern propagation across differing host architectures (x86_64 vs ARM64).
   - The conformance engine prevents NaN and rounding drift by executing all fitness, precision, and generalization calculations using fixed-point integer arithmetic (64-bit integers where the lower 32 bits represent the fractional component).
   - Direct float (`f32`/`f64`) operations are restricted on FFI boundaries.

4. **FFI Pointer and Memory Isolation**:
   - Pointers passed by the host (such as `trace_ptr` or `model_ptr`) are validated against WASM linear memory boundaries (`0` to `wasm_memory_size_in_bytes`). 
   - All dereferences must be bounded to the allocated guest heap buffer. Out-of-bounds pointers trigger an immediate WASM trap.
   - The guest memory space allocated for alignments must be completely zeroed out after receipt generation.

---

---

## 2. Type-Law Boundary: Evidence<T, State, Witness>

### 2.1 Evidence Container Enforcement

**Authority Claim:** wasm4pm enforces strict type wrapping of all process artifacts through the Evidence<T, State, Witness> container.

**Type Parameters:**
- **T:** Payload type
  - `XesLog`: Flat event trace (IEEE 1849-2016)
  - `Ocel2Log`: Object-centric event log (ISO/IEC 23745)
  - `BpmnInstanceData`: BPMN execution instance
  - `PetriNetMarking`: State snapshot of net execution
- **State:** Execution state type
  - `InitialMarking`: $M_0$ for Petri Net
  - `IntermediateMarking`: Some $M$ reachable via trace replay
  - `FinalMarking`: Sink marking after trace completes
  - `GatewayTokenConfiguration`: BPMN token state
- **Witness:** Proof of correctness type
  - `ReplayReceipt`: Contains moves (on both, on log, on model), tokens (p, c, m, r)
  - `AlignmentProof`: Optimal alignment cost, A* closed set size
  - `LatticeWitness`: Witness state from wasm4pm-compat lattice theory

### 2.2 Admissibility Axioms (Restated for Clarity)

**Axiom 1: Cryptographic Binding (Non-Forgeability)**

Every Evidence block must contain:
```rust
pub struct Evidence<T, State, Witness> {
    pub payload: T,
    pub state: State,
    pub witness: Witness,
    pub epoch: u64,  // Monotonic, prevents replay attacks
    pub signature: Ed25519Signature,
    pub hash: Blake3Hash,
}

// Invariant:
// hash == BLAKE3(serialize(payload) || serialize(state) || serialize(witness) || serialize(epoch) || serialize(signature))
```

**Enforcement:** Any mutation of fields invalidates the hash. System rejects admission.

**Axiom 2: Replay Soundness (Lattice Monotonicity)**

Sequential evidence blocks $E_1$ and $E_2$ with transition activity $t$ must satisfy:
$$S_1 \xrightarrow{t} S_2 \quad \text{(valid state transition)}$$
$$W_1 \sqsubseteq W_2 \quad \text{(witness monotonically increases)}$$

**Enforcement:**
```rust
fn admit_sequential_evidence<T, S: PartialOrd, W: Lattice>(
    e1: &Evidence<T, S, W>,
    e2: &Evidence<T, S, W>,
) -> Result<(), RefusalReport> {
    // Verify transition is valid
    verify_transition(&e1.state, &e2.state)?;
    
    // Verify witness is monotonic (lattice join property)
    if e1.witness.join(&e2.witness) == e2.witness {
        Ok(())  // e1 ⊆ e2 in lattice
    } else {
        Err(RefusalReport::LatticeViolation {
            context: format!("witness_2 not ⊇ witness_1"),
        })
    }
}
```

**Axiom 3: Signature Admissibility (Authority Binding)**

```
verify_signature(public_key[authority_role], signature, hash) ≡ True
```

**Enforcement:**
- Maintain registry: `HashMap<Role, Vec<PublicKey>>` for authorized signers
- Roles: `Runner`, `Auditor`, `MiningAuthority`, `ConformanceAuthority`, `BoardOverride`
- Reject signatures from unknown/revoked keys
- Epoch prevents replay of same signature across contexts

### 2.3 Receipt-Shaped Objects: Terminal State

An Evidence<T, State, Witness> becomes **receipt-shaped** (and eligible for archival) when:
1. **State reaches terminal marking** (sink place in Petri Net, final gateway in BPMN)
2. **Witness indicates complete replay** (fitness ≥ threshold OR explicit auditor approval)
3. **Signature is from Auditor role** (not just a runner)

```rust
pub fn is_receipt_shaped<T, S: IsTerminal, W: IsComplete>(
    evidence: &Evidence<T, S, W>,
) -> bool {
    evidence.state.is_terminal() 
        && evidence.witness.is_complete()
        && authority_key_is_auditor(&evidence.signature)
}
```

---

## 3. Admission Gate Enforcement: Blue River Dam Gate 3

### 3.1 Conformance Admissibility Rule

**Rule (from lifecycle/define_blue_river_dam_lifecycle_gate_map.md):**

$$\operatorname{admissible}(\sigma) \iff \operatorname{fitness}(\sigma, N) \geq 0.95 \lor \left(\operatorname{fitness}(\sigma, N) \geq 0.85 \land \operatorname{override}(\sigma)\right)$$

**Where:**
- $\sigma$ = a trace/instance from the live event log
- $N$ = the approved process model
- $\operatorname{override}(\sigma)$ = valid Board signature over $\operatorname{hash}(\sigma)$
- **No trace with fitness < 0.85 is ever admitted** (hard floor)

### 3.2 Enforcement Logic

```rust
pub fn enforce_admission_gate(
    trace: &Trace,
    model: &PetriNet,
    board_public_keys: &[PublicKey],
) -> Result<Receipt, RefusalReport> {
    let fitness = compute_alignment_fitness(trace, model)?;
    
    if fitness >= 0.95 {
        // PASS: Automatic admission
        return Ok(Receipt::admitted(trace, fitness, "automatic"));
    }
    
    if fitness >= 0.85 {
        // CONDITIONAL: Requires board override
        let override_sig = trace.metadata.get("board_override_signature")?;
        let override_hash = hash(&trace);
        
        for key in board_public_keys {
            if verify_signature(key, override_sig, override_hash)? {
                return Ok(Receipt::admitted(trace, fitness, "board_override"));
            }
        }
        
        // Override signature invalid or missing
        return Err(RefusalReport::ConditionalRejectMissingOverride {
            trace_id: trace.id(),
            fitness,
        });
    }
    
    // fitness < 0.85: HARD REJECT
    Err(RefusalReport::HardReject {
        trace_id: trace.id(),
        fitness,
        policy: "van_der_aalst_0_85_floor",
    })
}
```

### 3.3 Refusal Report Generation

When a trace is rejected at Gate 3, wasm4pm emits a structured refusal:

```json
{
  "refusal_code": "0xFC03",
  "refusal_type": "ConformanceAdmissionFailure",
  "trace_id": "<case_id>",
  "trace_hash": "<SHA-256>",
  "model_hash": "<SHA-256>",
  "computed_fitness": <fitness_value>,
  "threshold_required": 0.95,
  "override_required": <bool>,
  "override_signature_valid": <bool>,
  "alignment_moves": {
    "move_on_both": <count>,
    "move_on_log": <count>,
    "move_on_model": <count>
  },
  "tokens": {
    "produced": <p>,
    "consumed": <c>,
    "missing": <m>,
    "remaining": <r>
  },
  "timestamp": "<ISO 8601>",
  "auditor_action_required": "Manual review of low-fitness trace"
}
```

---

## 4. Raw-Laundering Prevention in Conformance

### 4.1 Definition: Raw-Laundering in Conformance Context

**Raw Laundering** = Submitting a non-conforming model (low fitness) to the conformance checker, then falsifying the resulting receipt to claim fitness ≥ 0.95.

**Prevention Mechanisms:**

1. **Receipt Immutability:** Once a receipt is cryptographically signed, it cannot be mutated. Attempting to modify fitness field invalidates signature.

2. **Audit Log Binding:** Every trace admitted to operations is recorded in an immutable audit ledger (OCEL conformance table) with:
   - Trace ID + log hash (binds to source log)
   - Computed fitness (immutable)
   - Receipt hash (links to full receipt)
   - Admission verdict (PASS | CONDITIONAL | REJECT)

3. **Spot Audit:** Randomly selected traces from operations are re-aligned offline:
   - If re-alignment yields fitness ≠ claimed fitness, escalate to board
   - Provides probabilistic guarantee against systematic laundering

### 4.2 Type-Law Boundary Protection

Rawlog → Evidence wrapping enforces:
- No evidence object can exist without a valid receipt ancestor
- Evidence<T, State, Witness> tracks the full lineage: raw log → parsed → admitted → evidenced
- Any gap in lineage triggers refusal with code `0xFC04` (broken chain of custody)

---

## 5. Conformance Authority vs. Mining Authority: Boundary

**Conformance Authority owns:**
- ✅ A* alignment computation
- ✅ Fitness/precision/generalization metrics
- ✅ Admission gate enforcement (θ_fit ≥ 0.95)
- ✅ Evidence<T, State, Witness> type-law wrapping and lattice enforcement
- ✅ Refusal report generation
- ✅ Audit ledger recording
- ✅ Board override signature verification

**Mining Authority owns:**
- ✅ Discovery algorithm correctness (Inductive Miner, Heuristics Miner)
- ✅ DFG construction
- ✅ Discovery receipt generation

**Delegation:**
Conformance Authority does **not** perform discovery. Mining Authority does **not** check fitness thresholds. If Mining Authority discovers a model with fitness < 0.85, the model fails admission at Gate 3; Mining Authority is not responsible for this rejection.

---

## 6. Duplicated Compat Law to Remove

The following wasm4pm-compat pathways must **not be re-implemented** in wasm4pm conformance authority; they belong in the type-law foundry (wasm4pm-compat layer):

| Compat Pathway | Reason |
|---|---|
| Temporal monotonicity validation (raw log check) | Happens before conformance checking; part of admission pipeline, not conformance metrics |
| Duplicate event detection | Raw-log validation |
| Schema validation (XES vs OCEL) | Type-law foundry responsibility |
| Loss policy thermodynamics | Format-conversion policy; separate from conformance metrics |
| Evidence<T, State, Witness> construction details | wasm4pm-compat owns this; wasm4pm uses it |

---

## 7. Missing Execution Law to Add

### 7.1 Generalization Metric

**Authority Claim:** wasm4pm computes generalization to assess how well the Petri Net model generalizes to unseen behavior of the process system.

**Formal Definition:**

Let $T$ be the set of transitions in the Petri Net model $N$, and let $\text{freq}(t, L)$ be the total number of times transition $t$ is fired during the optimal alignment replay of all traces in log $L$. The generalization metric is defined as:

$$g(L, N) = \begin{cases} \frac{1}{|T|} \sum_{t \in T} \left( 1 - \frac{1}{\text{freq}(t, L) + 1} \right) & \text{if } |T| > 0 \\ 0.0 & \text{if } |T| = 0 \end{cases}$$

This metric evaluates transition coverage such that transitions with high firing frequencies contribute close to $1.0$, while unused transitions ($\text{freq}(t, L) = 0$) contribute $0.0$, resulting in an average metric $\in [0, 1]$ representing the overall robustness of model paths against overfitting.

**WASM Implementation Requirement:**
- Generalization calculations must be executed using fixed-point integer arithmetic to prevent cross-platform NaN drift.
- If $|T| = 0$, generalization evaluates to $0.0$.
- Must be computed alongside fitness and precision.

### 7.2 Conformance Alerting: Fitness Anomaly Detection

**Gap:** No specification for detecting sudden fitness drop in live traces.

**Requirement:**
```rust
pub fn detect_fitness_anomaly(
    trace: &Trace,
    model: &PetriNet,
    historical_baseline: f64,  // e.g., 0.98
    tolerance_sigma: f64,      // e.g., 0.02
) -> AnomalyReport {
    let current_fitness = compute_alignment_fitness(trace, model);
    if current_fitness < historical_baseline - tolerance_sigma {
        // Trace is significantly worse than normal
        // Emit alert, recommend process repair
    }
}
```

### 7.3 Multi-Model Conformance

**Gap:** No specification for checking a trace against multiple candidate models simultaneously.

**Requirement:**
```rust
pub fn multi_model_conformance(
    trace: &Trace,
    models: &[PetriNet],
) -> Vec<(ModelId, Fitness, AlignmentReceipt)> {
    // Returns ranked list of fitness values for each model
    // Useful for variant detection: which process variant does this trace match?
}
```

---

## 8. Alignment Receipts and Cryptographic Proofs

### 8.1 Alignment Receipt Structure

Every A* alignment produces a receipt that can be independently verified:

```json
{
  "alignment_id": "<UUID>",
  "trace_id": "<case_id>",
  "log_hash": "<SHA-256 of full log>",
  "model_hash": "<SHA-256 of model>",
  "alignment": [
    { "log_activity": "A", "model_transition": "t1", "cost": 0 },
    { "log_activity": "B", "model_transition": "≫", "cost": 1 },
    { "log_activity": "≫", "model_transition": "τ (silent)", "cost": 0 }
  ],
  "total_cost": <C>,
  "moves": {
    "move_on_both": <count>,
    "move_on_log": <count>,
    "move_on_model": <count>
  },
  "replay_tokens": {
    "produced": <p>,
    "consumed": <c>,
    "missing": <m>,
    "remaining": <r>
  },
  "fitness_computed": <f>,
  "heuristic_accuracy": <h_quality>,
  "wasm4pm_version": "<version>",
  "timestamp": "<ISO 8601>",
  "conformance_engine_pubkey": "<Ed25519>",
  "signature": "<Ed25519>"
}
```

### 8.2 Independent Verification

A third party (e.g., PM4Py auditor) can verify the receipt without recomputing alignment:
1. Deserialize alignment array
2. Replay it manually on the model, computing moves and tokens
3. Verify fitness calculation matches
4. Check signature against wasm4pm's public key

---

## 9. Related Documents

- [Mining Authority Map](file:///Users/sac/process-intelligence/sources/wasm4pm/mining-authority-map.md) — Discovery algorithms and cycle accounting
- [Replay Authority Map](file:///Users/sac/process-intelligence/sources/wasm4pm/replay-authority-map.md) — Decommissioning and final receipts
- [Lifecycle Authority Map](file:///Users/sac/process-intelligence/sources/wasm4pm/lifecycle-authority-map.md) — State transitions
- [Blue River Dam Gate Map](file:///Users/sac/process-intelligence/lifecycle/define_blue_river_dam_lifecycle_gate_map.md) — Admission thresholds
- [Evidence & Type-Law Atlas](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/type-law-atlas.md) — wasm4pm-compat foundry specification

---

## 10. Authority Verdict

**Status:** EXECUTABLE

Conformance Authority specification is complete and actionable. Implementation priority:
1. **Phase 1 (MVP):** A* alignment + fitness computation (6 weeks, 1 engineer)
2. **Phase 2:** Precision metric + admission gate enforcement (3 weeks)
3. **Phase 3:** Evidence<T, State, Witness> wrapper + lattice enforcement (3 weeks)
4. **Phase 4:** Audit ledger + spot-audit framework (2 weeks)
5. **Phase 5:** Generalization + anomaly detection + multi-model (3 weeks)

**Deployment Gate:** wasm4pm conformance authority is the authoritative source of fitness metrics. All process intelligence decisions (repair, optimization, decommissioning) depend on its receipts.
