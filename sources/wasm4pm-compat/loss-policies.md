# Loss Policies and Loss Reports in WASM4PM-Compat

Process execution event logs often contain minor deviations from their reference process models. To handle these deviations without immediately rejecting entire logs, `wasm4pm-compat` implements a formal `LossPolicy` framework. The engine evaluates traces against these policies, producing a cryptographically signed `LossReport`.

---

## 1. Mathematical Conformance Replay Equations

We compute discrepancies using alignment-based conformance checking.

### 1.1 Alignment Fitness Calculation
Given a trace $\sigma$ from an event log $L$ and a process model $N$, an alignment $\gamma$ maps activities in the log to transitions in the model.
The alignment can contain:
1. **Sync-moves**: $(a, t)$ where activity $a$ corresponds to transition $t$ (cost $= 0$).
2. **Log-only moves**: $(a, \gg)$ where activity $a$ occurs in the log but is not fired in the model (cost $= w_L(a)$).
3. **Model-only moves**: $(\gg, t)$ where transition $t$ fires in the model but is missing from the log (cost $= w_M(t)$).

Let the weight function be defined as $w(x, y)$. The optimal alignment $\gamma_{\text{opt}}$ minimizes the total alignment cost:
$$\gamma_{\text{opt}}(\sigma) = \arg\min_{\gamma} \sum_{(x, y) \in \gamma} w(x, y)$$

The trace-level alignment fitness $f_{\text{align}}(\sigma, N)$ is defined as:
$$f_{\text{align}}(\sigma, N) = 1 - \frac{w(\gamma_{\text{opt}}(\sigma))}{w(\theta_{\text{worst}}(\sigma))}$$
where $\theta_{\text{worst}}(\sigma)$ represents aligning the trace $\sigma$ completely with log-only moves, and running the model to its final marking using model-only moves.

For the entire log $L$, the aggregate alignment fitness is:
$$\operatorname{fitness}(L, N) = 1 - \frac{\sum_{\sigma \in L} L(\sigma) \cdot w(\gamma_{\text{opt}}(\sigma))}{\sum_{\sigma \in L} L(\sigma) \cdot w(\theta_{\text{worst}}(\sigma))}$$

---

## 2. Loss Policy Structure

A `LossPolicy` specifies the threshold of acceptable divergence between the observed traces and the reference models:

```rust
pub struct LossPolicy {
    /// The maximum allowed log-only moves as a percentage of total events
    pub max_log_moves_pct: f64,

    /// The maximum allowed model-only moves as a percentage of total model transitions
    pub max_model_moves_pct: f64,

    /// The minimum required fitness score (between 0.0 and 1.0)
    pub min_fitness_threshold: f64,

    /// Weight assigned to log-only moves in alignment calculations
    pub log_move_weight: f64,

    /// Weight assigned to model-only moves in alignment calculations
    pub model_move_weight: f64,
}
```

---

## 3. Loss Report Structure

A `LossReport` certifies the conformance quality of a specific replay task:

```rust
pub struct LossReport {
    /// The unique identifier of the analyzed trace
    pub trace_id: String,

    /// The list of log-only moves (insertions/extra events)
    pub log_moves: Vec<String>,

    /// The list of model-only moves (omissions/skipped transitions)
    pub model_moves: Vec<String>,

    /// The total calculated alignment cost
    pub total_cost: f64,

    /// The final fitness score (from 0.0 to 1.0)
    pub fitness: f64,

    /// True if the fitness score and move ratios satisfy the LossPolicy
    pub is_compliant: bool,
}
```

---

## 4. Rust Implementation Interface

```rust
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct LossPolicy {
    pub max_log_moves_pct: f64,
    pub max_model_moves_pct: f64,
    pub min_fitness_threshold: f64,
    pub log_move_weight: f64,
    pub model_move_weight: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct LossReport {
    pub trace_id: String,
    pub log_moves: Vec<String>,
    pub model_moves: Vec<String>,
    pub total_cost: f64,
    pub fitness: f64,
    pub is_compliant: bool,
}

impl LossPolicy {
    /// Evaluate a trace replay sequence and generate a LossReport
    pub fn evaluate_replay(
        &self,
        trace_id: &str,
        log_events_count: usize,
        model_transitions_count: usize,
        log_moves: Vec<String>,
        model_moves: Vec<String>,
    ) -> LossReport {
        // Calculate alignment costs
        let log_cost = log_moves.len() as f64 * self.log_move_weight;
        let model_cost = model_moves.len() as f64 * self.model_move_weight;
        let total_cost = log_cost + model_cost;

        // Calculate worst-case cost (all log events are log-only, all model transitions are model-only)
        let worst_cost = (log_events_count as f64 * self.log_move_weight)
            + (model_transitions_count as f64 * self.model_move_weight);

        let fitness = if worst_cost > 0.0 {
            1.0 - (total_cost / worst_cost)
        } else {
            1.0
        };

        // Check percentages
        let log_moves_pct = if log_events_count > 0 {
            (log_moves.len() as f64 / log_events_count as f64) * 100.0
        } else {
            0.0
        };

        let model_moves_pct = if model_transitions_count > 0 {
            (model_moves.len() as f64 / model_transitions_count as f64) * 100.0
        } else {
            0.0
        };

        // Compliance check
        let is_compliant = fitness >= self.min_fitness_threshold
            && log_moves_pct <= self.max_log_moves_pct
            && model_moves_pct <= self.max_model_moves_pct;

        LossReport {
            trace_id: trace_id.to_string(),
            log_moves,
            model_moves,
            total_cost,
            fitness,
            is_compliant,
        }
    }
}
```

---

## 5. References & Related Documents

*   For details on witness lattices and their join properties, see [Witness Lattices](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/witness-lattices.md).
*   For the core evidence structure enclosing this lattice, see [Evidence Structures](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/evidence-structures.md).
*   To review the master type-law mapping, see [Type Law Atlas](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/type-law-atlas.md).
