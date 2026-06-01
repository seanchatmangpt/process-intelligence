# Buyer Reliance Requirements for Process Claims

**Doctrine:** Buyer reliance is legally and economically meaningful only when the underlying evidence
is independently verifiable. A buyer who relies on process claims backed by non-replayable evidence
has relied on representation, not proof.

---

## The Two Reliance Tests

For every process claim, a buyer must be able to pass two tests:

**Test 1 — Conformance is real, not cherry-picked logs:**
The buyer must be able to verify that the conformance report reflects the full operational population,
not a curated subset selected to maximize apparent fitness.

**Test 2 — Process model was discovered, not manually crafted:**
The buyer must be able to verify that the process model was produced by a recognized discovery algorithm
run against the event log, not drawn by hand to match what the business intended to do.

If a buyer cannot pass Test 1, the conformance claim may be accurate for the sample but not for the population.
If a buyer cannot pass Test 2, the process model represents aspiration, not execution reality.

---

## Test 1 — Conformance Is Real, Not Cherry-Picked Logs

### What cherry-picking looks like

Cherry-picked logs are event logs that were:
- Filtered to exclude cases with exceptions, deviations, or failures before conformance checking
- Sampled to include only recently closed cases (after a process improvement initiative)
- Exported from a reporting view rather than from the operational event store
- Pre-processed to remove activities that would reduce fitness scores

A conformance fitness of 0.95 on a cherry-picked log may correspond to a fitness of 0.61 on the full population.

### How to detect cherry-picking

**Population completeness check:**
- The event log case count must match the system-of-record transaction count for the same period
- Buyer independently queries the system of record for case count and compares to log case count
- Tolerance: ≤5% difference (accounting for cases in-flight at extraction time)

**Exclusion audit:**
- Seller must provide a written extraction specification: what was included, what was excluded, and why
- Any exclusion must be named and documented (e.g., "test cases excluded", "cancelled orders excluded")
- Buyer must be able to reproduce the extraction with the same exclusion criteria
- Buyer may request a no-exclusion extraction and verify that fitness scores do not materially differ

**Temporal coverage check:**
- Event log must span at least one full process cycle from start to finish
- Log must not be truncated to exclude the post-improvement period's early deviations
- Buyer verifies log start and end timestamps against the seller's claimed analysis period

**Buyer right to full population:**
For conformance claims to be buyer-defensible, the buyer must have the right to request and receive
the full operational population log (not sampled, not filtered) during due diligence.
A seller who refuses full population access for conformance verification is asserting a claim
they are unwilling to have verified. This is a due diligence finding.

---

## Test 2 — Process Model Was Discovered, Not Manually Crafted

### What manually crafted models look like

A manually crafted process model is a process model that was:
- Drawn by a business analyst, process consultant, or operations team
- Created to represent the intended process, not the actual executed process
- Not validated against an event log using discovery algorithms
- Updated based on process change initiatives without re-running discovery

Manually crafted models have no evidential value for conformance claims.
A high fitness score computed by replaying a log against a manually crafted model proves that the log
is consistent with the intentions expressed in the model — it does not prove that the process runs as designed.
It proves that the model was designed to match the log, or that the log was selected to match the model.

### The manufacturing requirement: Inductive Miner against full log

**The only buyer-defensible process model is one discovered by the Inductive Miner
(or Split Miner, Heuristics Miner, or equivalent) run against the full operational log.**

Why Inductive Miner specifically:
- Guarantees a sound process model (sound WF-net or valid process tree) by construction
- Produces a model that represents the actual execution patterns in the log
- Parameters are documented and reproducible
- Output is structurally verifiable (soundness certificate follows from algorithm guarantees)

**wasm4pm-discovered models are buyer-defensible. Hand-drawn models are not.**

The distinction is absolute:
- Inductive Miner run against full log → buyer can re-run discovery and verify model similarity → defensible
- Inductive Miner run against filtered log → buyer can detect filter and assess impact → partially defensible
- Hand-drawn model → buyer cannot determine what execution reality it represents → not defensible

### How to verify discovery provenance

**Discovery receipt:**
Seller must provide:
1. The event log used for discovery (must match the conformance log)
2. The discovery algorithm name and version
3. The algorithm parameters (noise threshold, loop detection, parallel split/join detection)
4. The date of discovery run
5. The output model (must be structurally identical to the model used for conformance)

**Buyer verification:**
1. Buyer receives event log and discovery parameters
2. Buyer runs discovery independently with seller's parameters
3. Buyer compares buyer-discovered model against seller-provided model
4. Model similarity threshold: process tree edit distance ≤ 10% of node count, or WF-net transition overlap ≥ 90%
5. Any deviation above threshold is a due diligence finding

**Manual model detection heuristics:**
A buyer-side analyst can apply these heuristics to identify likely manual models:
- Model contains activities that do not appear in the event log
- Model has zero-frequency paths (paths that are structurally present but never traversed in the log)
- Model fitness is implausibly high (>0.99) for a complex real-world process
- Model contains no silent transitions (real processes almost always require silent transitions for fit)
- Model perfectly matches the org chart structure (organizational chart influence on model design is a signal)

---

## Reliance Framework

### When buyer reliance is warranted

Buyer may rely on a process claim when:
1. Event log covers full operational population (Test 1 passed)
2. Process model has documented discovery provenance (Test 2 passed)
3. Conformance score is buyer-reproducible within tolerance
4. Discovery can be buyer-reproduced with similar results

### When buyer reliance is not warranted

Buyer should not rely on a process claim when:
1. Event log has undisclosed exclusions or sampling
2. Process model has no discovery provenance (hand-drawn, consultant-designed)
3. Conformance score cannot be reproduced by buyer from provided artifacts
4. Seller refuses to provide full population log for buyer verification

### Reliance representation in purchase agreement

Buyers who accept process claims as representations in a purchase agreement should require:
- Log completeness representation: seller certifies that the provided log covers ≥95% of the claimed operational population
- Discovery provenance representation: seller certifies that the process model was produced by named discovery algorithm with documented parameters
- Reproducibility representation: seller certifies that conformance scores are reproducible from provided artifacts within ±0.02 tolerance
- Indemnification trigger: material deviation in any representation triggers indemnification under specified thresholds

---

## Why Most Sellers Cannot Meet These Requirements

| Requirement | Typical State |
|---|---|
| Full population OCEL log | Most companies have no OCEL extraction pipeline; they have CSV exports from BI tools |
| No-exclusion extraction capability | Most companies have no documented extraction specification; analysts filter by intuition |
| Inductive Miner discovery receipt | Most companies have hand-drawn BPMN; no discovery provenance exists |
| Reproducible conformance score | Most companies have one-off pm4py notebooks; no reproducible pipeline |
| Buyer-auditable discovery re-run | Requires wasm4pm or pm4py in a reproducible, parameterized configuration |

**The gap between what buyers require for defensible reliance and what most sellers can produce
is the process intelligence acquisition gap.** Sellers who can close this gap command premium
valuations because their process claims can be independently verified. Sellers who cannot close this gap
expose buyers to process claim risk that must be priced into the deal.

---

## The wasm4pm Distinction

wasm4pm-discovered process models satisfy both reliance tests by construction:

**Test 1:** wasm4pm operates against admitted evidence — evidence admitted through the wasm4pm-compat
admission layer, which requires full population coverage or explicit scoping documentation.
Cherry-picking at the admission layer is refusal-capable: a scoped log that misrepresents coverage
violates the admission criteria and is refused with a named law violation.

**Test 2:** wasm4pm discovery produces a receipt that includes: algorithm name, version, parameters,
input log hash, output model hash, and timestamp. The discovery receipt is a first-class artifact —
not documentation added after the fact but a typed output of the manufacturing pipeline.

A buyer who receives a wasm4pm discovery receipt can:
1. Hash the input log and verify it matches the receipt
2. Re-run the algorithm with receipt parameters and verify model similarity
3. Hash the output model and verify it matches the receipt

This is buyer-defensible by construction.

---

*Grounded in: Blue River Dam doctrine, wasm4pm execution authority, wasm4pm-compat admission layer, Inductive Miner algorithm, OCEL 2.0 standard.*
