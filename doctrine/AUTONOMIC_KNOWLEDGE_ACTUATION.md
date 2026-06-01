# Autonomic Knowledge Actuation — Doctrine

> Knowledge must actuate, receipt, replay, repair, project, and decommission process life.

Source: ~/chatmangpt/knhk/MAPE-K_AUTONOMIC_INTEGRATION.md — distilled for the research program.

---

## The Central Doctrine

Autonomic knowledge actuation is the closed-loop discipline of self-managing process execution.
It is not monitoring. It is not dashboards. It is not alerting.

Autonomic knowledge actuation is the system that:
1. **Monitors** — continuously observes execution conformance against declared process law
2. **Analyzes** — identifies root causes of model-log divergence using formal process evidence
3. **Plans** — selects remediation actions within authorized elastic subnets
4. **Executes** — actuates corrections with cryptographic receipt emission
5. **Knows** — accumulates a typed knowledge base of lawful and unlawful execution patterns

Every loop must be grounded in process evidence. Every actuation must emit a receipt.
Every repair must replay to prove closure. Every decommissioning must produce a closure receipt.

---

## MAPE-K Architecture

The MAPE-K loop is the canonical architecture for autonomic systems (IBM Autonomic Computing
Manifesto, 2003; extended for process intelligence by this program):

```
Monitor (Observe)  →  Analyze (Understand)
       ↑                        ↓
       └────────────────────────┘

Execute (Act)      ←  Plan (Decide)
       ↑                        ↓
       └────────────────────────┘

              Knowledge Base
              ─────────────
              Lawful patterns
              Unlawful patterns
              Fitness history
              Receipt chains
              Predictive models
              Repair outcomes
```

---

## The Five Components

### Monitor — Continuous Conformance Observation

The monitor component continuously observes process execution and emits typed observations.
In a process intelligence system, monitoring means measuring conformance, not just performance.

**What Monitor tracks:**
- Fitness score: fraction of traces replayable through the declared process model
- Precision score: how much the model over-generalizes observed behavior
- Stage timing: is each lifecycle stage occurring in lawful order?
- Object lifecycle: are artifacts following lawful creation → transition → receipt chains?
- Drift indicator: is fitness decreasing monotonically (process decay)?

**Evidence shape:** Typed observations with timestamps, severity, and affected process elements.
Not raw metrics. Typed, witnessed observations.

**Trigger conditions:**
- Fitness below threshold → Analyze
- Stage ordering violation detected → Analyze immediately (critical)
- Orphaned artifact detected (artifact with no receipt after expected decommission) → Analyze

### Analyze — Root Cause from Process Evidence

The analyzer does not infer from correlation. It mines the actual process from accumulated logs
and compares against the declared model. Root cause = the specific deviation between mined
process and declared model.

**What Analyze does:**
- Converts OTel traces to OCEL event logs
- Runs pm4py process discovery on accumulated log window
- Compares discovered process against declared Petri net / POWL / process tree
- Identifies variant explosion, hidden loops, skipped stages, rework paths
- Computes confidence score for each root cause hypothesis

**Output:** Typed analysis with root cause, affected process elements, confidence score,
and recommended remediations. Not a string. A typed finding.

### Plan — Remediation Within Authorized Bounds

The planner selects actions from the elastic subnet only. It cannot touch the compliance subnet.

**Elastic subnet (autonomous authority):**
- Dynamic resource reallocation (task queue reassignment)
- Safe path selection (branch redirection in exclusive choice operators)
- Elastic rate limiting (input throttling to prevent token accumulation)
- Retry with backoff on transient failures

**Compliance subnet (executive authority — FROZEN):**
- Financial approvals
- Security gate transitions
- KYC verification stages
- Receipt issuance for high-value artifacts

**Hard block:** Any attempt to auto-actuate outside the elastic subnet halts execution and
requires board override. The planner must check subnet membership before selecting any action.

**Plan output:** Ordered action list with risk assessment and subnet membership confirmation
for each action.

### Execute — Receipt-Bearing Actuation

Every execution action emits a receipt. No silent actuation.

**Receipt structure:**
```
Receipt = BLAKE3(action || pre_state || post_state || timestamp || elastic_subnet_proof)
```

Receipt chaining:
```
Receipt_n = BLAKE3(Receipt_{n-1} || new_action || new_state || signature)
```

Executions that do not emit receipts are not closures. They are narration.

**Execution output:** New process state + receipt emitted + monitor re-triggered.

### Knowledge — Typed Learning Base

The knowledge base accumulates typed evidence of lawful and unlawful execution patterns.
It is not a training dataset. It is a typed receipt chain of prior actuations.

**What Knowledge stores:**
- Lawful patterns: execution sequences that achieved conformance targets
- Unlawful patterns: execution sequences that caused fitness degradation
- Repair outcomes: which remediations restored conformance and in how many cycles
- Decommission receipts: final closure proofs for completed process instances
- Predictive models: estimated fitness trajectory given current observed state

**Knowledge is the only thing that may feed Plan.** External data sources, human intuition,
and LLM inference are NOT valid Knowledge inputs without process evidence grounding.

---

## Actuation Lifecycle

```
Process Instance Created
  → Monitor establishes baseline fitness
  → [Execution occurs]
  → Monitor detects deviation
  → Analyze mines actual process
  → Plan selects elastic-subnet remediation
  → Execute actuates with receipt
  → Monitor confirms fitness restored
  → [Cycle continues]
  → Process achieves final marking
  → Receipt chain closed
  → Knowledge updated
  → Process Decommissioned with closure receipt
```

A process is not decommissioned until the closure receipt exists.
A process that cannot replay its receipt chain is PARTIAL, not complete.

---

## Autonomic Bounds in the Research Program

The research program itself operates under autonomic discipline:

1. **Monitor**: Track ALIVE gate criteria counts continuously
2. **Analyze**: When a gap is detected (evidence absent for a claim), issue a PARTIAL finding
3. **Plan**: Select the minimum evidence gathering required to close the gap
4. **Execute**: Gather evidence, write findings, commit with receipts (commit hashes)
5. **Knowledge**: The commit log is the knowledge base; receipts are commit hashes

The research program's autonomic loop is the commit sequence itself.

---

## Knowledge Actuation vs. Knowledge Retrieval

Knowledge retrieval is looking up what you know.
Knowledge actuation is making what you know consequential.

Actuation means:
- Receipt: issue a typed proof that the knowledge was applied
- Replay: the proof can be verified by replaying the evidence chain
- Repair: the knowledge was applied to fix a non-conforming process
- Project: the knowledge was applied to manufacture a board-admissible claim
- Decommission: the knowledge was applied to produce a closure receipt

Knowledge that cannot actuate is documentation. Documentation is PARTIAL.
Knowledge that actuates with receipt and replay is ALIVE.

---

## See Also

- `~/chatmangpt/knhk/MAPE-K_AUTONOMIC_INTEGRATION.md` — Full MAPE-K integration specification
- `doctrine/BLUE_RIVER_DAM.md` — Upstream closure law that bounds autonomic authority
- `doctrine/PROCESS_INTELLIGENCE_DEFINED.md` — Five maturity levels including Level 5 actuation
- `gaps/GAP_001_COMPAT_WASM_BRIDGE.md` — Current gap blocking full actuation chain
