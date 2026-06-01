# Slide-to-Receipt Map for M&A Decks

**Doctrine:** Every slide in an M&A deck that makes a process claim must have a corresponding receipt.
A receipt is not a footnote citation. A receipt is a machine-verifiable artifact produced by wasm4pm,
grounded in admitted wasm4pm-compat evidence, that a buyer can independently replay.

---

## Overview

M&A decks contain process claims across six slide categories. Each category requires a specific receipt type.
This map defines: the slide type, the claim it typically makes, the receipt required, how the receipt is produced,
and what grounds it in the wasm4pm-compat evidence chain.

---

## 1. Org Chart Slide

**Typical claim:** "Our organization has N process owners across M business units operating the named processes."

**Why the claim is weak without a receipt:**
An org chart describes intended authority, not actual process execution. Org charts are maintained manually
and drift from reality. The person listed as "Order-to-Cash Process Owner" may not be involved in the actual
execution process at all.

**Receipt requirement:**
- HR process event log covering role transitions, case assignments, and escalations
- Role-to-case conformance report: for each named role, what fraction of their assigned cases followed the declared role's process steps?

**Receipt format:**
- Event log: XES or OCEL 2.0 with case IDs mapped to organizational unit identifiers
- Conformance report: per-role fitness score from token-based replay
- Role conformance summary: table of (role, case_count, fitness, deviation_rate)

**How produced:**
1. Extract HR system events: assignments, handoffs, escalations, closures
2. Reconstruct OCEL log with object types: Employee, Role, Case, Unit
3. Discover role-level process models per organizational unit
4. Run conformance checking per role against declared role model
5. wasm4pm emits per-role conformance report with fitness scores

**wasm4pm-compat grounding:**
- Evidence admitted through `Admission<OcelLog, HrProcessWitness>`
- Role models admitted through `Admission<WfNet, OrgProcessWitness>`
- Conformance report grounded in `Metric<ConformanceFitness, NUM, DEN>` with `Between01` bound

**Buyer replay:**
Buyer receives OCEL log + per-role models + conformance report.
Buyer re-runs conformance checking and verifies per-role fitness scores.

---

## 2. Revenue Process Slide

**Typical claim:** "Our revenue process converts leads to closed-won in median 21 days with 34% win rate."

**Why the claim is weak without a receipt:**
Revenue process metrics are almost always derived from CRM system reports, which reflect the CRM's
view of the process (stage timestamps, status fields), not the actual execution process.
CRM stage timestamps are frequently backdated, manually updated, or skipped.

**Receipt requirement:**
- Revenue OCEL log: event log with object types (Lead, Opportunity, Account, Contact, Deal)
- Inductive Miner discovery result: process model discovered from the log, not hand-drawn
- Alignment fitness score: fraction of opportunities that followed the discovered model

**Receipt format:**
- Event log: OCEL 2.0 with E2O relations linking activities to opportunities, accounts, contacts
- Discovery result: process tree or WF-net with soundness certificate
- Conformance report: alignment-based fitness score (alignment is more precise than token-replay for revenue processes)
- Performance metrics: case duration distribution derived from log timestamps (not CRM-reported SLA fields)

**How produced:**
1. Extract CRM events as OCEL 2.0 (activity = CRM event type, objects = linked entities)
2. Reconstruct E2O relations from CRM relationship tables
3. Run Inductive Miner against full opportunity population log
4. Run alignment-based conformance checking
5. Compute case duration distribution from log-derived timestamps
6. wasm4pm emits discovery result + conformance report + performance metrics

**wasm4pm-compat grounding:**
- OCEL log admitted through `Admission<OcelLog, Ocel20Witness>`
- Discovered process tree admitted through `Admission<ProcessTree, InductiveMinerWitness>`
- Conformance metric grounded in `Metric<AlignmentFitness, NUM, DEN>`
- Performance metric grounded in `Metric<CaseDuration, NUM, DEN>`

**Buyer replay:**
Buyer receives OCEL log + discovered model + conformance report.
Buyer re-runs Inductive Miner with seller's parameters and verifies model similarity.
Buyer re-runs alignment conformance and verifies fitness score.
Buyer re-computes case duration distribution and verifies median/P95 claims.

---

## 3. Operational Efficiency Slide

**Typical claim:** "We reduced order processing cycle time by 22% through process improvement in Q3."

**Why the claim is weak without a receipt:**
Cycle time improvements claimed from BI dashboards reflect the BI tool's definition of cycle time,
which may exclude rework loops, failed attempts, and parallel branches.
A 22% improvement in dashboard metrics may correspond to a 5% improvement in log-derived cycle time
if rework and hidden loops are excluded from the BI calculation.

**Receipt requirement:**
- Cycle time analysis from event log: log-derived case duration computation, not BI-reported
- Before/after comparison: event logs from pre-improvement and post-improvement periods
- Variant analysis: confirmation that the improvement reflects genuine process change, not measurement artifact

**Receipt format:**
- Before log + after log (same OCEL structure, different time windows)
- Cycle time distribution comparison: (period, median, P95, rework_rate, bypass_rate)
- Variant comparison: process variants present in before log vs. after log
- Hidden loop analysis: rework loops identified in before log, confirmed absent or reduced in after log

**How produced:**
1. Export OCEL logs for two time windows (pre/post improvement initiative)
2. wasm4pm runs cycle time analysis on both logs independently
3. wasm4pm computes process variant distribution for both periods
4. wasm4pm identifies hidden loops (activities appearing multiple times per case) in both periods
5. wasm4pm produces comparison report with log-derived metrics

**wasm4pm-compat grounding:**
- Both logs admitted through `Admission<OcelLog, Ocel20Witness>`
- Cycle time metrics grounded in `Metric<CaseDuration, NUM, DEN>`
- Variant analysis grounded in `Evidence<ProcessVariant, Admitted, InductiveMinerWitness>`

**Buyer replay:**
Buyer receives both event logs and comparison report.
Buyer re-computes cycle time distributions independently.
Buyer verifies that hidden loops are absent/reduced in post-improvement log.

---

## 4. Compliance Slide

**Typical claim:** "We maintain 99.7% compliance with our documented process controls."

**Why the claim is weak without a receipt:**
Compliance percentages from internal audit or rule engines reflect whether audit rules were satisfied,
not whether the process model was conformed to. A company can pass all audit rules while having
40% of cases deviate from the declared process model.

**Receipt requirement:**
- Declare constraint checking report against the event log
- Declare constraints must be derived from regulatory requirements or internal policy (named constraints)
- Compliance rate must be computed as: fraction of cases satisfying all constraints per log-derived replay

**Receipt format:**
- Declare constraint set: formal constraints in `.decl` format or equivalent typed representation
- Constraint checking report: per-constraint satisfaction rate, per-case constraint violations
- Compliance rate: (constraints_satisfied / (constraints_satisfied + violations)) per case
- Violation taxonomy: which constraints are violated most frequently, with named law labels

**How produced:**
1. Formalize compliance rules as Declare constraints (e.g., `Response(Submit, Approve)`, `Absence(Bypass)`)
2. Export OCEL or XES log covering the compliance period
3. wasm4pm runs Declare constraint checking against full log
4. wasm4pm produces per-constraint violation rates and per-case compliance scores
5. Constraint violations are named (not generic "compliance failure")

**wasm4pm-compat grounding:**
- Event log admitted through `Admission<EventLog, Xes1849>` or `Admission<OcelLog, Ocel20Witness>`
- Declare constraints admitted through `Admission<DeclareModel, DeclareWitness>`
- Violation evidence grounded in named refusal types (e.g., `ConstraintViolation<ResponseConstraintViolated>`)

**Buyer replay:**
Buyer receives event log + Declare constraint set + constraint checking report.
Buyer re-runs constraint checking and verifies violation rates.
Buyer submits known-violating cases and confirms they appear in violation report.

---

## 5. Integration Risk Slide

**Typical claim:** "Integration will take 6 months with low operational disruption."

**Why the claim is weak without a receipt:**
Integration risk claims based on org chart similarity, system compatibility matrices, or consultant assessments
do not reflect process-level integration complexity. Two companies can use the same ERP and have radically
different order-to-cash process variants that are incompatible at the process level.

**Receipt requirement:**
- Process variant analysis: distribution of process variants in seller's log
- Exception rate: fraction of cases that deviate from the dominant process variant
- Object-centric conflict analysis: cases where seller's object lifecycles conflict with buyer's declared process model

**Receipt format:**
- Variant map: top-N process variants with frequency, case count, and fitness score
- Exception rate by process area: (area, total_cases, exception_cases, exception_rate)
- Object-centric conflict report: seller OCEL objects vs. buyer process model — structural conflicts identified
- Integration complexity score: derived from variant entropy + exception rate + object-centric conflict count

**How produced:**
1. Export seller OCEL log
2. wasm4pm runs process variant analysis (clustering by process tree similarity)
3. wasm4pm computes exception rate (cases deviating from dominant variant)
4. wasm4pm runs seller OCEL log against buyer's declared process model (cross-model replay)
5. Object-centric conflicts identified where seller object types have no analog in buyer model
6. wasm4pm emits integration risk assessment with variant map and conflict report

**wasm4pm-compat grounding:**
- Seller OCEL log admitted through `Admission<OcelLog, Ocel20Witness>`
- Buyer process model admitted through `Admission<WfNet, BuyerProcessWitness>`
- Cross-model conformance grounded in `Metric<CrossModelFitness, NUM, DEN>`
- Integration risk receipt: `Evidence<IntegrationRiskAssessment, Receipted, WfNetWitness>`

**Buyer replay:**
Buyer receives seller OCEL log + buyer's own process model + integration risk report.
Buyer re-runs cross-model conformance checking.
Buyer verifies variant map and exception rate claims.
Buyer identifies object-centric conflicts independently.

---

## Receipt Summary Table

| Slide Type | Receipt Type | Produced By | Grounded In |
|---|---|---|---|
| Org chart | HR process event log + role-to-case conformance report | wasm4pm + OCEL extraction | `Admission<OcelLog, HrProcessWitness>`, `Metric<ConformanceFitness, N, D>` |
| Revenue process | Revenue OCEL log + Inductive Miner discovery + alignment fitness | wasm4pm + CRM extraction | `Admission<OcelLog, Ocel20>`, `Metric<AlignmentFitness, N, D>` |
| Operational efficiency | Cycle time analysis from log (before/after) | wasm4pm | `Metric<CaseDuration, N, D>`, variant comparison |
| Compliance | Declare constraint checking report against event log | wasm4pm | `Admission<DeclareModel, DeclareWitness>`, named violation types |
| Integration risk | Process variant analysis + exception rate + object-centric conflict | wasm4pm | `Metric<CrossModelFitness, N, D>`, `Evidence<IntegrationRisk, Receipted, W>` |

---

## The Non-Substitutability Principle

Every receipt in this map is produced by wasm4pm and grounded in admitted wasm4pm-compat evidence.
This is not a tool preference. It is a structural requirement.

A slide claim backed by a Tableau dashboard is **not** backed by a receipt.
A slide claim backed by a consultant's analysis is **not** backed by a receipt.
A slide claim backed by a pm4py notebook is a receipt candidate — if the log is admitted evidence and the notebook is reproducible.

The receipt is non-substitutable because it requires:
1. Admitted evidence (typed, witnessed, refusal-capable)
2. Machine-verifiable computation (reproducible by the buyer)
3. Named law grounding (what law makes this claim valid, what law would make it invalid)

No tool other than wasm4pm (and wasm4pm-compatible pipelines) produces evidence with all three properties.

---

*Grounded in: Blue River Dam doctrine, wasm4pm execution authority, wasm4pm-compat evidence types, OCEL 2.0, Declare formalism, Inductive Miner algorithm.*
