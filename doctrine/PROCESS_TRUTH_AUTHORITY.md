# Process Truth Authority

> "Whoever controls admissible process truth controls the downstream data, audit, governance,
> automation, and intelligence below it." — Blue River Dam doctrine

---

## The Central Claim

Consequential software cannot be governed by downstream interpretation of activity records.
It must manufacture admissible process truth upstream.

Traditional software governance flow:
```
write code → deploy → observe → collect logs → explain later → govern
```

Blue River Dam governance flow:
```
define lawful work → admit evidence → refuse weak claims → execute bounded →
emit receipts → replay consequence → let downstream consume proof
```

The difference is not operational. It is architectural. In the traditional flow, governance
is downstream of execution. In the Blue River Dam flow, governance is upstream — before
execution, not after.

**The dam must be upstream.** Whoever controls admissible process truth controls the
downstream data, audit, governance, automation, and intelligence below it.

---

## Five Levels of Process Authority

### Level 1: Records Activity

**Identity:** Activity recording. No process mining.

**What they can say:** "We log what happens."

**What they cannot say:** Anything about whether what happened was lawful, expected, or
conformant with any process model.

**Governance implication:** Fully dependent on downstream interpretation. Anyone who can
read the logs can reinterpret them. The activity recorder has no authority over what the
logs mean.

**Technical signature:** Activity records in flat databases, application logs, event streams.
No formal process model. No conformance checking. No event log standard.

---

### Level 2: Structures Evidence

**Identity:** `wasm4pm-compat`. Structures evidence.

**What they can say:** "Our process evidence is structured in OCEL 2.0 / XES 1849 with typed
admission, named witnesses, and explicit loss accounting on every format projection."

**What they cannot say:** Conformance metrics, because discovery and replay have not been
executed.

**Governance implication:** Evidence is structured and auditable, but process truth has not
yet been computed. Downstream consumers inherit structured evidence, not computed truth.

**Dependencies:** OCEL 2.0 spec, XES 1849, process model structural types, evidence admission
surface (`Evidence<T, State, W>`, `Admit::admit()`).

**Technical signature:** `Evidence<OcelLog, Admitted, Ocel20>`. Named refusals. `LossReport`
on every OCEL→XES projection. Trybuild compile-fail fixtures confirming type law.

---

### Level 3: Strict Compat Covenant

**Identity:** Strict admission/refusal surfaces. Judges evidence claims.

**What they can say:** "Our process boundary has been strictly audited. Every format claim
names a loss policy. Every admission names a witness. Every refusal names the violated law.
Strict violations are named, not suppressed."

**What they cannot say:** Conformance metrics from execution (still no runtime).

**Governance implication:** The evidence claim surface is adversarial — claims that do not
meet the strict covenant are refused at the type level, not at runtime. Third parties can
audit the type law by reading the compile-fail fixtures.

**Dependencies:** Level 2 + `strict` feature (`ProcessBoundary`, `StrictViolation`,
`StrictCheck`).

**Technical signature:** `ProcessBoundary::fully_attested(kind, name)`. `StrictCheck` audit
passing with zero `StrictViolation`. `StrictViolation::law()` names human-readable law.

---

### Level 4: Graduation Bridge

**Identity:** Graduation bridge. Prepares execution authority.

**What they can say:** "We have identified all structural types that are ready to cross the
bridge to wasm4pm. Each `GraduationCandidate` carries a named `GraduationReason`. The bridge
is defined and typed."

**What they cannot say:** Post-graduation execution results, because wasm4pm has not yet
consumed the `GraduationCandidate`.

**Governance implication:** The boundary between structure and execution is formally defined.
The graduation act is explicit — not implicit through shared code, not accidental through
dependency bleed. The governance claim is: "We know where our structure ends and our
execution begins."

**Dependencies:** Level 3 + `wasm4pm` feature (`GraduationCandidate`, `GraduationReason`,
`GraduateToWasm4pm` trait, `graduation` module).

**Gap (current state):** wasm4pm has no intake function for `GraduationCandidate`. The bridge
is one-sided. Level 4 is structurally defined but not yet fully operational.

**Technical signature:** `impl GraduateToWasm4pm for MyProcessShape`. `GraduationCandidate::is_grounded()` true.

---

### Level 5: Full wasm4pm

**Identity:** Full `wasm4pm`. Adjudicates process truth.

**What they can say:** "We have mined a process model from admitted event evidence. We have
checked conformance via [token replay / alignment-based checking]. Our fitness is [NUM/DEN].
Our precision is [NUM/DEN]. These metrics are computed, not claimed. Every computation
emitted a receipt. Every receipt can be replayed. Our process truth is adversarially judgeable."

**Governance implication:** This is the throne room of process authority. A Level 5 system
can answer any process governance question with a receipted, replayable, formally bounded
answer. No narration, no approximation, no "approximately compliant."

**Dependencies:** Level 4 + wasm4pm execution engine (branchless Rust/WASM; discovery,
conformance, replay, receipts, benchmark gates).

**Technical signature:** `Metric<FITNESS, 97, 100>` from wasm4pm token replay. `Metric<PRECISION, 93, 100>`
from wasm4pm alignment. `Receipt` emitted. Replay confirmed. Adversarial benchmark passing.

---

## The Maturity Matrix (Summary)

| Level | One-Line Authority | Who has it now |
|---|---|---|
| 1 | Records activity | Most enterprise software |
| 2 | Structures evidence | wasm4pm-compat (ALIVE gate) |
| 3 | Judges evidence claims | wasm4pm-compat + `strict` feature |
| 4 | Prepares execution authority | wasm4pm-compat + `wasm4pm` feature (bridge one-sided) |
| 5 | Adjudicates process truth | wasm4pm (exists; bridge gap outstanding) |

---

## The Authority Stack (Blue River Dam)

```
Level 5: Full wasm4pm
  → branchless execution
    → 8-bit bounded state (Need9 = split)
      → typed admission / refusal (Level 2–4)
        → external-witness mapping
          → GALL growth
            → Living LSP author-time observation
              → receipts / replay
                → adversarial benchmark judgment
```

Process maturity = the progressive removal of unresolved process uncertainty from execution.

At Level 1: all process uncertainty is resolved downstream (or not at all).
At Level 5: all process uncertainty is resolved before the hot path. The branchless execution
kernel never encounters unresolved process state.

---

## Governance Consequences

| Question | Level 1–2 answer | Level 5 answer |
|---|---|---|
| "Is our process conformant?" | "We believe so." | "Fitness = 97/100 at [receipt timestamp]." |
| "What happened in case X?" | "Let me check the logs." | "Replay of case X shows [alignment result]." |
| "Did we have divergence?" | "We don't think so." | "DivergenceWitness absent from E2O link structure by construction." |
| "Can you prove it?" | "Here is a report." | "Here is a receipt. You can replay it yourself." |
| "What does the data say?" | "Here is a dashboard." | "Here is an admitted `OcelLog` and its conformance receipt." |

The governance authority gap between Level 1 and Level 5 is not operational — it is
constitutional. Level 1 governs by assertion. Level 5 governs by proof.
