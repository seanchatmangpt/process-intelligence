# Lifecycle: Define Monitoring-State Process Intelligence

The **Monitoring Stage** is the active execution audit phase, where live event streams are captured and analyzed for conformance against designed process rules.

## Autonomic MAPE-K Mapping
* **Loop Role**: **Monitor**
* **Responsibility**: In the Monitor phase, raw system events are converted into structured trace streams. The system continuously measures trace alignments, tracking deviations, checking declarative temporal constraints, and raising alerts if compliance or performance falls below thresholds.
* **Actuation Trigger**: When the alignment-based fitness drops below a critical threshold (e.g., $f_{align} < 0.90$) or compliance alerts trigger, a transition initiates the **Repair Stage** to execute autonomic process corrections. See the [Autonomic Knowledge Actuation Map](file:///Users/sac/process-intelligence/lifecycle/define_autonomic_knowledge_actuation_map.md) for trigger definitions.

---

## Conformance Verification Mathematics

Monitoring checks the agreement between the event log $L$ and the model (Petri Net / Declare rules).

### 1. Token-Based Replay Fitness (van der Aalst 2016)
During replay of trace $\sigma$, four token counters are maintained for each place:
* $p$: produced tokens.
* $c$: consumed tokens.
* $m$: missing tokens (added when a transition fires without sufficient input tokens).
* $r$: remaining tokens (left in the net after replay completes).

For a log $L$ containing multiple traces with frequencies $L(\sigma)$, the overall fitness is:
$$f(L, N) = \frac{1}{2} \left( 1 - \frac{\sum_{\sigma \in L} L(\sigma) \cdot m(\sigma, N)}{\sum_{\sigma \in L} L(\sigma) \cdot c(\sigma, N)} \right) + \frac{1}{2} \left( 1 - \frac{\sum_{\sigma \in L} L(\sigma) \cdot r(\sigma, N)}{\sum_{\sigma \in L} L(\sigma) \cdot p(\sigma, N)} \right)$$

### 2. Alignment-Based Conformance (Adriansyah 2014)
Alignment-based conformance maps log traces to model transitions. Let $\Sigma$ be the alphabet of activities. A move is represented as $(a, t)$ where:
* $(a, t)$ is a **synchronous move** if $a \in \Sigma$, $t \in T$, and the label of $t$ is $a$ (cost = 0).
* $(a, \gg)$ is a **move on log** (cost $> 0$, typically 1).
* $(\gg, t)$ is a **move on model** (cost $> 0$, typically 1).

An alignment $\gamma$ is a sequence of such moves. An optimal alignment $\gamma_{opt}$ is one that minimizes the total cost function:
$$\text{cost}(\sigma, \gamma_{opt}) = \min_{\gamma \in \Gamma} \text{cost}(\gamma)$$
where $\Gamma$ is the set of all valid alignments.

The alignment fitness of a trace $\sigma$ is:
$$\text{fitness}(\sigma, N) = 1 - \frac{\text{cost}(\sigma, \gamma_{opt})}{\text{worst-cost}(\sigma, N)}$$
where $\text{worst-cost}(\sigma, N)$ is the cost of aligning $\sigma$ with the empty path (representing total mismatch).

### 3. Declarative Conformance (LTL Declare Rules & Vacuous Satisfaction)
In addition to imperative flows, declarative temporal rules are checked on finite traces using interpretations from the [declare_placement.md](file:///Users/sac/process-intelligence/standards/declare_placement.md) standard.

Let $\sigma = e_1 e_2 \dots e_m$ be a trace of length $m$. Let $\alpha_{\phi}$ represent the activation condition for constraint $\phi$.
- **Response(A, B)**: $\phi = \Box(A \implies \lozenge B)$. The activation condition is $\alpha_{\phi} = A$. The constraint is satisfied vacuously if $A$ never occurs:
  $$\sigma \models_{\text{vac}} \phi \iff \sigma \models \phi \quad \land \quad \text{Acts}(\sigma, A) = \emptyset$$
- **Precedence(A, B)**: $\phi = \neg B \mathbin{\mathcal{W}} A$. The activation condition is $\alpha_{\phi} = B$. The constraint is satisfied vacuously if $B$ never occurs:
  $$\sigma \models_{\text{vac}} \phi \iff \sigma \models \phi \quad \land \quad \text{Acts}(\sigma, B) = \emptyset$$

To prevent false audits, the monitor flags `is_vacuously_satisfied: true` when a rule evaluates to true but has zero activations ($|\text{Acts}(\sigma, \alpha_{\phi})| = 0$). Global unary rules (e.g. `Existence(A)`, `Absence(A)`) are treated as always active ($\alpha_{\phi} = \text{True}$) and are never vacuously satisfied.

---

## Sandbox Bounds for Runtime Verification
All online replay and alignment search algorithms execute in a WebAssembly sandbox enclave to prevent uncontrolled host process resource utilization. The runtime enforces safety invariants defined in [sandbox.rs](file:///Users/sac/process-intelligence/sources/wasm4pm/src/sandbox.rs):
- **Fuel Gas-Metering**: A deterministic `GasMeter` limits execution to a maximum of $10\text{M}$ cycles ($10,000,000$). Running out of fuel immediately traps with error code `0xFB01` (ERR_CYCLE_OVERFLOW).
- **Recursion Guard**: Stack frame execution is capped at a maximum of $100$ nested levels in `RecursionGuard`. Violations trap with error code `0xFB05` (ERR_LIFECYCLE_VIOLATION).
- **Shredding**: Post-execution, the memory space is zeroed via a 3-pass ChaCha20 Oblivion Protocol to destroy trace details.

---

## Standards Alignment

* **XES (eXtensible Event Stream)**: A XML-based IEEE standard for process mining event logs. Events represent activity executions mapped to discrete timestamps, case IDs, and resources.
* **OCEL 2.0 (Object-Centric Event Logs)**: Designed for multi-entity systems. Unlike XES which requires flat case IDs, OCEL 2.0 represents events related to multiple objects of different types (e.g. an order object, an item object, and a customer object), avoiding duplication and artificial log flattening.

---

## M&A Due Diligence Claims
Monitoring represents the **Execution Audit Trail** in M&A.
* **Buyer Reliance**: The buyer relies on monitoring data to confirm the actual volume of operations and trace compliance.
* **Slide-to-Receipt Map**: Slides stating "Our internal order-to-cash process is 98% compliant with regulatory guidelines" must link directly to an alignment conformance run receipt proving $f_{align} \ge 0.98$ on OCEL logs of the past 12 months.

---

## Related Documents
* See the [Simulation Stage](file:///Users/sac/process-intelligence/lifecycle/define_simulation-state_process_intelligence.md) for pre-activation properties.
* See the [Repair Stage](file:///Users/sac/process-intelligence/lifecycle/define_repair-state_process_intelligence.md) for handling violations.
* Back to [Lifecycle README](file:///Users/sac/process-intelligence/lifecycle/docs-law__lifecycle_readme.md).