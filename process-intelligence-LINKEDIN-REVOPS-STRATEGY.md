# process-intelligence LinkedIn RevOps Strategy

## Standing and claim ceiling

This repository explicitly states that **RevOps is a proof domain for CodeManufactory** and defines `wasm4pm` as the downstream execution authority for process discovery, conformance, replay, receipts, and benchmarks. That makes `process-intelligence` the **research/ontology authority** for modeling and evaluating a LinkedIn-originated revenue process, not the LinkedIn publisher or CRM actuator.

Current strategy standing: **DIRECT-TO-REVOPS RESEARCH / PARTIAL_ALIVE**. No direct LinkedIn API capability is claimed.

## Research question

The August 31 10k campaign creates a falsifiable process-intelligence experiment:

> Can an evidence-grounded LinkedIn teaching event produce qualified enterprise opportunities through a deterministic, observable, optimizable revenue process without collapsing engagement into revenue claims?

## Canonical RevOps process

```text
ContentConstructed
-> PublicationObserved
-> EngagementObserved
-> LeadCaptured
-> AssessmentStarted
-> AssessmentCompleted
-> MQLAdmitted | LeadDisqualified
-> SQLAdmitted | MQLDisqualified
-> POVProposed
-> POVAccepted | POVDeclined
-> POVExecuted
-> OutcomeVerified
-> CustomerWon | OpportunityLost
-> ExpansionObserved
```

Each transition needs an exact object identity, timestamp, source, evidence boundary, and typed refusal where applicable.

## Object-centric model

A LinkedIn revenue process is inherently object-centric. One event can relate to multiple objects:

- Person
- Account
- Campaign
- ContentAsset
- Assessment
- Problem
- Opportunity
- POV/Experiment
- Outcome

OCEL-style modeling is therefore preferable to forcing the entire funnel into one case ID. For example, one Account may contain multiple People, ContentAsset touches, assessments, and opportunities over time.

## Challenger Sale process law

The process should retain the teaching mechanism, not only the commercial stage:

```text
Teach -> Tailor -> TakeControl
```

Useful event attributes include `teaching_thesis`, `persona`, `identified_constraint`, `economic_consequence`, `authority_path`, and `falsifiable_outcome`. These make it possible to test whether Challenger-style diagnosis actually correlates with SQL/POV advancement rather than merely increasing engagement.

## August 31 experiment design

Campaign identity: `10k_august_2026`.

Primary hypotheses:

1. The synchronization-tax reframe creates more qualified-account progression than commit-count framing alone.
2. Assessment completion is a stronger predictor of SQL than LinkedIn engagement.
3. Opportunities with an explicit falsifiable POV outcome advance faster than generic demo requests.
4. Account-level object-centric traces expose loops and handoffs hidden by ordinary funnel tables.

Primary outcome measures:

```text
content -> assessment conversion
assessment completion
MQL admission rate
SQL admission rate
SQL -> POV acceptance
POV cycle time
POV -> customer conversion
attributable pipeline
realized revenue
rework / loop frequency
manual synchronization events
```

## Conformance and refusals

The reference process must refuse invalid promotions. Examples:

- `EngagementObserved -> SQLAdmitted` without qualification evidence.
- `LeadCaptured -> POVAccepted` without an exact opportunity/problem.
- `OutcomeVerified` without evidence identity.
- `CustomerWon` inferred from proposal or verbal interest.
- revenue attributed to LinkedIn when campaign/content provenance is absent.

These should become conformance violations, not silently accepted variants.

## Handoff to wasm4pm

`process-intelligence` should define the event/object ontology, reference process, conformance laws, experiments, and benchmark questions. `wasm4pm` should execute discovery, conformance, object-centric analysis, replay, and receipt manufacture against exported event data.

The research repo should never borrow `ALIVE` standing from a wasm4pm run on a different exact subject or dataset.

## Next admitted increments

1. Define the LinkedIn RevOps OCEL event/object taxonomy and attribute contract.
2. Add the reference POWL/BPMN/process model for the admitted funnel.
3. Add conformance laws for MQL, SQL, POV, customer, and attribution transitions.
4. Define the August 31 experiment protocol and comparison cohorts.
5. Define synchronization-tax process metrics alongside standard funnel metrics.
6. Add privacy/data-minimization rules for person-level event traces.
7. Manufacture downstream wasm4pm fixtures and benchmark prompts from the canonical research artifacts.

## Falsifiers

The RevOps proof fails if stages cannot be reconstructed from evidence, if process variants depend on undocumented manual interpretation, if revenue attribution cannot be linked back to campaign/content objects, or if the analysis requires conflating LinkedIn engagement with commercial standing.

The desired output is not a prettier funnel dashboard. It is a replayable explanation of how demand became—or failed to become—revenue.
