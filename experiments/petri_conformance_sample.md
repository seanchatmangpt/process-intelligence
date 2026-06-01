# Experiment: Petri Net Conformance & Token Game Replay

This experiment verifies the alignment and fitness of event logs against a Petri net model using the classical token game replay algorithm.

## 1. Conformance Replay Doctrine (Token Game)

For a given trace $\sigma$, we replay the sequence of activities on a Petri Net. Let:
- $p$: total number of tokens produced during replay.
- $c$: total number of tokens consumed during replay.
- $m$: number of missing tokens (tokens that had to be artificially added to fire an enabled transition).
- $r$: number of remaining tokens (tokens left in places other than the sink place $o$ at the end of execution).

The fitness metric $f(\sigma, N)$ is calculated as:
$$f(\sigma, N) = \frac{1}{2}\left(1 - \frac{m}{c}\right) + \frac{1}{2}\left(1 - \frac{r}{p}\right)$$

For a set of traces (log $L$), the aggregate fitness is:
$$f(L, N) = \frac{1}{2}\left(1 - \frac{\sum_{\sigma \in L} m_{\sigma}}{\sum_{\sigma \in L} c_{\sigma}}\right) + \frac{1}{2}\left(1 - \frac{\sum_{\sigma \in L} r_{\sigma}}{\sum_{\sigma \in L} p_{\sigma}}\right)$$

## 2. Petri Net Model and Execution Log (JSON Schema)

The following JSON Schema validates the token game execution results for a given trace:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "TokenGameReplayResult",
  "type": "object",
  "properties": {
    "trace_id": { "type": "string" },
    "activity_sequence": {
      "type": "array",
      "items": { "type": "string" }
    },
    "tokens_produced": { "type": "integer", "minimum": 0 },
    "tokens_consumed": { "type": "integer", "minimum": 0 },
    "tokens_missing": { "type": "integer", "minimum": 0 },
    "tokens_remaining": { "type": "integer", "minimum": 0 },
    "fitness": { "type": "number", "minimum": 0.0, "maximum": 1.0 }
  },
  "required": [
    "trace_id",
    "activity_sequence",
    "tokens_produced",
    "tokens_consumed",
    "tokens_missing",
    "tokens_remaining",
    "fitness"
  ]
}
```

## 3. Concrete Replay Execution Data

### Case A: Fully Fitting Trace (Fitness = 1.0)
Trace $\sigma_1 = \langle \text{Register}, \text{Approve}, \text{Ship} \rangle$.
Initial place $i$ starts with 1 token. Transition sequence fires correctly:
- $t_{register}$ consumes 1 from $i$, produces 1 in $p_1$.
- $t_{approve}$ consumes 1 from $p_1$, produces 1 in $p_2$.
- $t_{ship}$ consumes 1 from $p_2$, produces 1 in final place $o$.
At termination, $o$ is consumed, leaving 0 remaining.

```json
{
  "trace_id": "trace_fit_01",
  "activity_sequence": ["Register", "Approve", "Ship"],
  "tokens_produced": 3,
  "tokens_consumed": 3,
  "tokens_missing": 0,
  "tokens_remaining": 0,
  "fitness": 1.0
}
```

### Case B: Non-fitting Trace (Missing and Remaining Tokens)
Trace $\sigma_2 = \langle \text{Approve}, \text{Ship} \rangle$ (Missing "Register").
- $t_{approve}$ tries to fire, but $p_1$ has 0 tokens. 1 missing token is added to $p_1$.
- $t_{approve}$ fires, consuming 1 token and producing 1 token in $p_2$.
- $t_{ship}$ fires, consuming 1 token from $p_2$ and producing 1 token in $o$.
At the end, place $i$ still contains its initial 1 token (remaining).
Stats:
- $c = 3$ (1 from $p_1$, 1 from $p_2$, 1 from $o$)
- $p = 3$ (1 initial in $i$, 1 in $p_2$, 1 in $o$)
- $m = 1$ (missing in $p_1$)
- $r = 1$ (remaining in $i$)

$$f(\sigma_2, N) = \frac{1}{2}\left(1 - \frac{1}{3}\right) + \frac{1}{2}\left(1 - \frac{1}{3}\right) = 0.667$$

```json
{
  "trace_id": "trace_nonfit_02",
  "activity_sequence": ["Approve", "Ship"],
  "tokens_produced": 3,
  "tokens_consumed": 3,
  "tokens_missing": 1,
  "tokens_remaining": 1,
  "fitness": 0.6667
}
```

## 4. Linkages to Standards and M&A Claims

- **Standard Crosswalk**: Fits into the Petri Net placement specification mapped at file:///Users/sac/process-intelligence/standards/petri_net_placement.md.
- **M&A Claims**: Defensibility claims are verified by mapping these replay parameters onto slide-to-replay logs at file:///Users/sac/process-intelligence/ma/define_slide-to-replay_map.md.