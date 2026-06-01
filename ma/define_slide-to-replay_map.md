# Slide-to-Replay Map

For a process intelligence assertion to be verified, it must be backed by a trace-level replay of event logs on a formal process model. This document defines the Slide-to-Replay Map, establishing the mathematical foundations of token replay and alignments used to substantiate slide claims.

## 1. Mathematical Replay Foundations

Simple token game replay checks if a trace can be processed by a Petri Net without blocking. Because real-world process logs contain noise, we use **optimal alignment conformance** (Adriansyah 2014) to map slide assertions to event logs.

### A. Petri Net and Marking Definitions
A Workflow Net (WF-net) is defined as $N = (P, T, F)$, where:
* $P$ is a finite set of places (states).
* $T$ is a finite set of transitions (activities), where $P \cap T = \emptyset$.
* $F \subseteq (P \times T) \cup (T \times P)$ is a set of directed arcs.
* The initial marking is $m_i$ (token in source place $i$), and the final marking is $m_f$ (token in sink place $o$).

### B. Optimal Alignment Mathematics
An alignment between a log trace $\sigma \in L$ and a process model $M$ is a sequence of moves $\gamma \in (T \cup \{\gg\}) \times (\Sigma \cup \{\gg\})$, where $\gg$ represents a skip (no move):
* **Synchronous Move**: $(t, a)$ where $t \in T$, $a \in \Sigma$, and $t$ matches the activity name of $a$.
* **Move in Log**: $(\gg, a)$ where activity $a$ occurs in the log but is skipped by the model (indicates a process deviation).
* **Move in Model**: $(t, \gg)$ where transition $t$ is fired in the model but does not occur in the log (indicates a missed activity).

We assign a cost function $c$ to moves:
* $c(t, a) = 0$ (no cost for synchronized execution).
* $c(\gg, a) > 0$ (cost for log deviation).
* $c(t, \gg) > 0$ (cost for model deviation).

The optimal alignment $\gamma_{\text{opt}}$ minimizes the total deviation cost:
$$\gamma_{\text{opt}} = \operatorname{argmin}_{\gamma} \sum_{(m_x, m_y) \in \gamma} c(m_x, m_y)$$
where $(m_x, m_y)$ represents any move in the alignment.

## 2. Replay Map Specification

The Slide-to-Replay Map links a slide's assertion to its alignment execution parameters:

```json
{
  "slide_id": "8a7f-9b1c",
  "assertion": "98% billing compliance",
  "conformance_method": "optimal_alignment_dijkstra",
  "cost_structure": {
    "move_in_log": 1,
    "move_in_model": 1,
    "sync_move": 0
  },
  "verified_cases": {
    "total_case_count": 14205,
    "conforming_cases": 13921,
    "deviation_log_uri": "file:///Users/sac/process-intelligence/experiments/declare_violation_sample.md"
  }
}
```

## 3. Related M&A Validation Documents

* For the cryptographic receipt containing these replay parameters, see [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md).
* For the auditor's path to executing these replays, see [Auditor Evidence Path](file:///Users/sac/process-intelligence/ma/define_auditor_evidence_path.md).
* For the board rules governing these replays, see [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).
* For the buyer's replication requirements, see [Buyer Reliance Requirements](file:///Users/sac/process-intelligence/ma/define_buyer_reliance_requirements.md).