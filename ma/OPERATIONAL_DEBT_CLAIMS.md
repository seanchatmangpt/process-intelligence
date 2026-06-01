# Operational Debt Taxonomy from Process Mining Evidence

**Doctrine:** Operational debt is process structure that accumulates cost, risk, or compliance exposure
through repeated execution of non-conforming, bypassed, or inefficient process patterns.
Unlike technical debt, operational debt is invisible to code review. It is only visible in event logs.

> "We quantify operational debt from event logs, not from interviews."
> This is the acquirer's claim. It is the structural advantage.

---

## Why Interviews Cannot Measure Operational Debt

Interview-based process assessment produces:
- The process as people believe it runs (cognitive bias toward declared process)
- The process as people want it to appear to run (presentation bias in M&A context)
- The process as it ran during the last audit (recency bias, audit-triggered behavior change)

Log-based process assessment produces:
- The process as it actually ran, across the full execution history
- Deviations that occurred at 2am when no one was watching
- Patterns that were too common to be noticed as deviations

Operational debt that cannot be seen in interviews is the operational debt that matters most in M&A.
It is the debt that will manifest post-acquisition when audit pressure is removed.

---

## Type 1 — Hidden Loops (Rework)

**Definition:** Cases that revisit an activity more than once in a single process execution.
Rework loops represent work that was done, found insufficient or incorrect, and repeated.

**What it looks like in the log:**
- Activity `A` appears twice or more in the same case trace
- Activities follow the pattern: `A → B → C → A` (return to A after downstream steps)
- Case duration is inflated relative to the modal trace duration

**How wasm4pm detects it:**
- Process variant analysis identifies traces with self-loops and extended variants
- Rework rate: fraction of cases with at least one activity repetition
- Rework activity map: which activities are most frequently repeated, with frequency distribution
- Rework cost estimate: median additional case duration added by each rework loop

**wasm4pm-compat type surfaces:**
- `Evidence<ProcessVariant, Admitted, InductiveMinerWitness>` — variants with loop structure
- `Metric<ReworkRate, NUM, DEN>` — fraction of cases with rework
- Named law violation: `HiddenLoop<ActivityName>` — rework loop at a specific activity

**Acquirer claim:**
"We identified $2.3M in annual rework cost embedded in your order fulfillment process
through process variant analysis of your OCEL log. This rework is invisible in your ERP metrics
because the ERP records only final completion, not intermediate attempts."

**Detectable in:**
- Order fulfillment (order-item-pick-pack-ship-invoice)
- Customer onboarding (document submission and resubmission cycles)
- Claims processing (adjudication and re-adjudication loops)
- Software deployment (failed deployment → rollback → re-deploy loops)

---

## Type 2 — Bypass Patterns (Compliance Risk)

**Definition:** Cases that skip mandatory process steps, taking a direct path that circumvents
required activities. Bypasses represent compliance risk because skipped activities often carry
approval, validation, or control obligations.

**What it looks like in the log:**
- The modal (most frequent) trace includes activity `B`
- A variant exists where `A → C` occurs without `B` in between
- The bypassing variant appears with non-trivial frequency (>1% of cases)

**How wasm4pm detects it:**
- Variant analysis identifies traces that are strict subsets of the declared model
- Declare constraint checking: `Response(A, B)` — if A occurs, B must eventually occur
- Bypass rate: fraction of cases where mandatory activities are absent
- Bypass frequency map: which activities are most frequently bypassed

**wasm4pm-compat type surfaces:**
- `Admission<DeclareModel, DeclareWitness>` — formalized mandatory activity constraints
- Named law violation: `BypassPattern<MandatoryActivity>` — named activity was bypassed
- `Metric<BypassRate, NUM, DEN>` — fraction of cases with at least one bypass

**Compliance risk quantification:**
Each bypassed activity type carries a specific compliance risk profile:
- Approval bypass: authorization control failure
- Validation bypass: data quality risk
- Notification bypass: regulatory disclosure failure
- Audit trail bypass: audit integrity risk

**Acquirer claim:**
"4.7% of your purchase orders were processed without three-way match validation.
This represents a compliance exposure that your internal controls framework did not detect
because it is not visible in your ERP approval logs — only in the object-centric event log
that shows the relationship between PO, GR, and Invoice lifecycle events."

**Detectable in:**
- Procure-to-pay (purchase order to invoice to payment)
- HR (hiring approval chains)
- Finance (payment authorization controls)
- Healthcare (prior authorization and clinical review steps)

---

## Type 3 — Synchronization Failures (Object Lifecycle Violations)

**Definition:** Cases where two or more object types that should progress in synchronized lifecycle
states instead diverge — one object advances while another remains in an earlier state,
or an event on one object occurs before its causally required predecessor on another object.

**What it looks like in the log (OCEL only — requires object-centric log):**
- Invoice is emitted before the corresponding shipment confirmation event
- Payment is processed before the corresponding approval event
- A customer object transitions to "Active" before the associated contract object has been countersigned

**How wasm4pm detects it:**
- Object-centric conformance checking: replay across object types simultaneously
- Temporal precedence analysis: for each object pair with declared dependency, verify temporal ordering
- Lifecycle soundness: verify that each object type's lifecycle is individually sound and mutually consistent with dependent objects

**wasm4pm-compat type surfaces:**
- `Admission<OcelLog, Ocel20Witness>` — admitted OCEL log with E2O and O2O relations
- Named law violation: `SynchronizationFailure<ObjectTypeA, ObjectTypeB>` — lifecycle divergence
- `Evidence<ObjectLifecycle, Admitted, Ocel20Witness>` — per-object lifecycle evidence

**Why this is invisible in XES logs:**
XES logs flatten object-centric data into case sequences. A synchronization failure between
an order and its line items is invisible in a flattened XES log because both object types
are collapsed into a single case sequence, masking the temporal precedence violations.

**Acquirer claim:**
"We found 312 instances over the past 12 months where invoices were emitted before shipment confirmation.
These are not ERP errors — the ERP accepted them. They are process-level synchronization failures
that are only visible in the object-centric event log connecting invoice events to shipment events.
Each represents a revenue recognition risk under ASC 606."

**Detectable in:**
- Revenue recognition (shipment-invoice synchronization)
- Financial close (accrual-payment synchronization)
- Supply chain (PO-GR-Invoice three-way match)
- Healthcare (encounter-claim-authorization synchronization)

---

## Type 4 — Resource Conflicts (Org Mining)

**Definition:** Cases where the organizational execution of a process violates declared role boundaries:
activities executed by unauthorized roles, role concentration that creates single-point-of-failure risk,
or role distribution that creates hand-off overhead without accountability benefit.

**What it looks like in the log:**
- Activity `Approve` is executed by the same resource as activity `Submit` in the same case (segregation of duty violation)
- A single resource handles >40% of cases of a specific type (key person concentration)
- Cases requiring hand-off between 5+ resources show systematically higher duration and rework rates

**How wasm4pm detects it:**
- Org mining: extract resource-to-activity assignment from event log
- SoD violation detection: cases where Submit and Approve roles are held by the same resource
- Concentration analysis: per-activity resource distribution (Herfindahl-Hirschman Index from log)
- Hand-off analysis: correlation between hand-off count and case duration/rework rate

**wasm4pm-compat type surfaces:**
- `Evidence<OrgModel, Admitted, OrgMiningWitness>` — discovered organizational model
- Named law violation: `SoDViolation<SubmitRole, ApproveRole>` — same resource in conflicting roles
- `Metric<RoleConcentration, NUM, DEN>` — HHI of resource distribution per activity type

**Acquirer claim:**
"Org mining of your procurement event log shows that 23% of purchase orders were approved by the
same person who initiated them. This is a segregation of duty violation that your access controls
framework did not catch because SoD enforcement is configured at the system level but the same
individual holds both roles in your identity management system."

**Detectable in:**
- Finance (payment initiation and approval)
- Procurement (requisition and purchase order approval)
- HR (hiring request and candidate selection)
- IT (access request and access grant)

---

## Operational Debt Quantification Framework

Each debt type is quantifiable from the event log:

| Debt Type | Quantification Metric | Unit | Source |
|---|---|---|---|
| Hidden loops | Rework rate × average rework cost per case | $ annual | Log-derived duration × volume |
| Bypass patterns | Bypass rate × regulatory fine exposure per bypass | $ risk-adjusted | Compliance risk schedule |
| Synchronization failures | Failure count × average resolution cost | $ annual | Log count × remediation data |
| Resource conflicts | SoD violation count × audit finding cost | $ risk-adjusted | Log count × audit cost schedule |

**Total operational debt = sum of all four quantified types, derived from the event log.**

This is not an estimate based on interviews or process documentation.
This is a computation from evidence that a buyer can independently verify.

---

## The Acquirer Claim

> "We quantify operational debt from event logs, not from interviews."

This claim is the structural advantage of process intelligence in M&A.

An acquirer equipped with wasm4pm can:
1. Receive the target's OCEL log during due diligence
2. Run operational debt analysis within 5 business days
3. Produce a quantified operational debt report across all four debt types
4. Present the report with receipts that the target's own team can verify
5. Use the debt quantification as grounds for deal repricing or integration budget adjustment

An acquirer without process mining capability cannot make this claim.
Their operational debt assessment is based on interviews, which the target can manage.

---

## Traceability to wasm4pm-compat Type Surfaces

Each debt type has a corresponding wasm4pm-compat type surface:

| Debt Type | wasm4pm-compat Surface |
|---|---|
| Hidden loops | `Evidence<ProcessVariant, Admitted, W>` with loop structure; `Metric<ReworkRate, N, D>` |
| Bypass patterns | `Admission<DeclareModel, DeclareWitness>`; named `BypassPattern<A>` violations |
| Synchronization failures | `Admission<OcelLog, Ocel20Witness>` required; named `SynchronizationFailure<A, B>` |
| Resource conflicts | `Evidence<OrgModel, Admitted, OrgMiningWitness>`; named `SoDViolation<R1, R2>` |

The type surfaces enforce that debt evidence is admitted, typed, witnessed, and named.
Debt that cannot be named and typed is not detectable by wasm4pm — it remains invisible operational risk.

---

*Grounded in: Van der Aalst process mining, org mining (resource-aware conformance), OCEL 2.0 object-centric conformance, Blue River Dam doctrine, wasm4pm-compat type surfaces.*
