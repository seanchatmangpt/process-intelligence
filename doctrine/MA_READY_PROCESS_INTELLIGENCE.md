# M&A-Ready Process Intelligence Criteria

A process intelligence claim is board-admissible if and only if it satisfies all six criteria
below. A claim that satisfies five of six is not board-admissible — it is narration.

---

## The Six Criteria

### 1. Traces to an Event Log

The claim must trace to an event log from which the evidence was derived.

**Minimum:** An admitted `OcelLog` (object-centric) or `XesLog` (case-centric flat), carried
in `Evidence<T, Admitted, W>` under a named witness.

**Not sufficient:** A dashboard metric, a sprint velocity number, a reported SLA, a manually
compiled table. These are records, not event logs. Records narrate; logs evidence.

**Test:** Can you identify the specific `OcelLog` or `XesLog` from which this claim derives?
Can you replay it? If not, the claim does not trace to an event log.

**Why it matters for M&A:** An acquirer who cannot find the event log behind a process
intelligence claim has nothing to audit. The claim is hearsay.

---

### 2. The Log Can Be Replayed Against a Formal Process Model

The log must be replayable. Replay requires:
- A formal process model (`WfNetConst<SOUNDNESS>`, `ProcessTree`, `PowlNodeKind` graph, etc.)
- A conformance checking algorithm (token replay or alignment-based, owned by wasm4pm)
- A verdict with a `Metric<FITNESS>` value and a `Metric<PRECISION>` value

**Not sufficient:** A log that exists but has no model to replay against. A model that exists
but has never been replayed against the log.

**Test:** Run `pm4py.conformance_diagnostics_token_based_replay` or
`pm4py.conformance_diagnostics_alignments` over the log and the model. Does it complete? Do
the metrics fall in [0, 1]? If not, the claim is not replayable.

**Why it matters for M&A:** If the log cannot be replayed against a model, then no one has
ever verified that the process described matches the process executed. The gap may contain
undetected fraud, systematic deviation, or architectural misrepresentation.

---

### 3. Conformance Metrics Are Calculated, Not Claimed

Fitness and precision must come from executed conformance algorithms. They must not be
asserted, estimated, or manually reported.

**Acceptable:** `Metric<FITNESS, 97, 100>` — a `Between01`-bounded fraction derived from
token replay execution in wasm4pm.

**Not acceptable:** "Our process achieves 97% compliance." — A claim without a receipt.
Claims about compliance percentages that are not backed by a specific `FitnessConst` value
from a specific replay run over a specific log version against a specific model version are
narration.

**Test:** What is the exact `NUM/DEN` of the fitness metric? What replay algorithm produced
it? What log version? What model version? What timestamp?

**Why it matters for M&A:** Compliance claims without computational receipts are the most
common form of process misrepresentation in M&A. "97% conformance" can mean anything from
a rigorous alignment-based calculation to a manually assembled spreadsheet.

---

### 4. Replay Uses Admitted Evidence

The log replayed must have passed admission — not raw data, not data from an untrusted source
without a named witness.

**Minimum:** `Admission<OcelLog, Ocel20>` — the `OcelLog` has been admitted under the
`Ocel20` witness via `Admit::admit()`. Any structural law violated during admission produced
a named `Refusal<NamedLaw, Ocel20>`.

**Not sufficient:** Replaying against raw event records that were extracted from a database
without admission, without witness, without refusal surface. Such records may contain
divergence, convergence, missing timestamps, undeclared event types, or schema violations
that were never detected.

**Test:** Has `Admit::admit()` been called on the log? What witness governs it? Were there
any refusals? If refusals were suppressed or ignored, the evidence is tainted.

**Why it matters for M&A:** Admitted evidence is court-grade. Raw evidence is hearsay.
A process intelligence claim grounded in unadmitted evidence is vulnerable to discovery.

---

### 5. Every Loss Is Named and Reported

If any projection or format conversion was applied to the evidence before or during the
claim, every loss must be named and reported.

**Minimum:** Every `LossyFormatExport` in the evidence chain carried a `ProjectionName`,
a `LossPolicy`, and a `LossReport<From, To, Items>`. No silent flattening. No implicit
case-id derivation from an object-centric log.

**Not sufficient:** "We converted the OCEL log to XES for compatibility." — without naming
the projection, without reporting which events were duplicated (divergence), without
reporting which events were merged (convergence), without a `LossPolicy` that permits this.

**Test:** Are there any `From<OcelLog> for XesLog` conversions in the evidence path that
bypass `LossyFormatExport`? If so, the evidence chain has unnamed loss.

**Why it matters for M&A:** Loss in the evidence chain means the process reality is different
from the process claim. An acquirer who discovers unnamed loss post-close has grounds to
allege misrepresentation.

---

### 6. The Claim Is Falsifiable

A board-admissible process intelligence claim must be falsifiable. It must be possible to
construct a log that would make the claim false.

**Acceptable falsifiability:** "Our process fitness is 0.97 (97/100) under OCEL 2.0 witness,
measured via token replay against WF-net model v3 on log snapshot 2026-05-31. If the log
contained more than 3% of traces with missing final marking transitions, this metric would
fall below 0.94."

**Not falsifiable:** "Our process is highly efficient and conformant." — No metric value, no
model reference, no log reference, no threshold that would make the claim false.

**Test:** What would it take to falsify this claim? What log would produce a metric value
that contradicts it? If no such log is conceivable, the claim is narration.

**Why it matters for M&A:** A claim that cannot be falsified cannot be audited. An auditor
who cannot falsify a claim cannot confirm it. Non-falsifiable claims are contractually
meaningless in M&A representations and warranties.

---

## The Six Criteria as a Checklist

| # | Criterion | Evidence type | Failure mode |
|---|---|---|---|
| 1 | Traces to event log | `Admission<OcelLog\|XesLog, W>` | Dashboard metrics without log reference |
| 2 | Log replayable against model | `WfNetConst` / `ProcessTree` + replay execution | Log exists but no model; model exists but never replayed |
| 3 | Metrics calculated, not claimed | `Metric<FITNESS\|PRECISION, NUM, DEN>` from wasm4pm execution | Asserted percentages without algorithm receipt |
| 4 | Admitted evidence | `Admit::admit()` called; named witness; no suppressed refusals | Raw data replayed without admission |
| 5 | Loss named and reported | `LossReport<From, To, Items>` on every projection | Silent OCEL→XES flattening; unnamed case-id derivation |
| 6 | Falsifiable claim | Specific metric + model + log + threshold | "Highly conformant" without numbers |

---

## M&A Due Diligence Application

In an M&A context, the buyer's process intelligence audit applies these six criteria to the
target's process claims. For each claim:

1. Request the admitted event log (`OcelLog` or `XesLog`).
2. Request the formal process model used for replay.
3. Request the computed conformance metrics (with algorithm, log version, model version).
4. Verify the admission receipts for the event log.
5. Request all loss reports for any format projections in the evidence chain.
6. Identify the falsifiability threshold for each claim.

A target that cannot produce these artifacts for its process intelligence claims has not
achieved Level 2 maturity (wasm4pm-compat / structures evidence). Its claims are unauditable.

A target that can produce all six for every material process claim has achieved Level 3 or
higher (strict compat covenant / graduate bridge). Its claims are auditable and warrant-grade.

**Board claim:** "We require all process intelligence representations in this transaction to
meet the six-criterion standard. Unauditable claims will be treated as unrepresented."
