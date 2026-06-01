# Process Lifecycle Ontology (12 Phases)

A process lifecycle is not a software development lifecycle. It describes the full arc of a
formal process from its initial design through eventual decommissioning. Each phase has a
required capability, a specific kind of data produced, a receipt that must be emitted for
closure, and a board claim that is possible once the phase is properly admitted.

---

## Phase 1: Design

**Required capability:** Model a process using a formal notation (BPMN, Petri net, process
tree, POWL, Declare). The model must be structural — not a diagram, not prose.

**Data produced:** A formal process model artifact in a recognized notation.
- `WfNetConst<SOUNDNESS>`, `ProcessTree`, `BpmnElement`/`GatewayKind`, `PowlNodeKind`,
  `DeclareConstraint`

**Receipt needed:** `WfNetSoundnessWitness` (for WF-nets); `TreeProjectable` (for process
trees projectable to POWL); `DeclareWitness` (for Declare models). A model without a witness
type is a diagram, not an admitted formal object.

**Board claim possible:** "We have a formally specified process model in [notation], grounded
in [paper witness]."

---

## Phase 2: Simulation

**Required capability:** Simulate process execution to estimate performance before deployment.
Simulation produces synthetic event logs under defined parameters.

**Data produced:** Synthetic `EventLog` / `OcelLog` with known ground-truth behavior.

**Receipt needed:** Simulation provenance receipt naming the simulation parameters and the
model version used. `Evidence<SyntheticLog, Receipted, SimulationWitness>`.

**Board claim possible:** "We have simulated the process under [parameters] and observed
[performance metrics] with [confidence bounds]."

**wasm4pm graduation boundary:** Simulation execution graduates to wasm4pm. Structure of the
synthetic log remains in compat.

---

## Phase 3: Construction

**Required capability:** Implement the process in software or in a workflow engine. Construction
produces a deployed system whose behavior is expected to follow the formal model.

**Data produced:** Deployed system + initial configuration. No process evidence yet — execution
has not yet occurred.

**Receipt needed:** Construction receipt linking the deployed system version to the formal
model it is intended to implement.

**Board claim possible:** "The system version [X] is intended to implement the formally
specified [model] and has been deployed."

---

## Phase 4: Activation

**Required capability:** Begin process execution. Activation produces the first real events
in the production event log.

**Data produced:** First `OcelEvent` / `XesEvent` entries in the production log. Raw process
evidence begins accumulating.

**Receipt needed:** Activation receipt: timestamp of first event, object types present,
initial marking (for WF-nets) confirmed.

**Board claim possible:** "Process execution has begun. Initial evidence is being admitted."

---

## Phase 5: Operation

**Required capability:** Ongoing process execution. Operation produces the bulk of the event
log. Evidence admission is continuous.

**Data produced:** Growing `OcelLog` / `EventLog` with full E2O links, object changes,
timestamps. Divergence/convergence anomalies may appear (managed by OCEL structure).

**Receipt needed:** Periodic operation receipts confirming continuous log admission under
named witnesses. No silent gaps.

**Board claim possible:** "The process is operating. N events admitted across M object types
since activation."

---

## Phase 6: Monitoring

**Required capability:** Continuously check whether operation conforms to the formal model.
Monitoring applies conformance checking algorithms in near-real-time.

**Data produced:** `Metric<FITNESS>` and `Metric<PRECISION>` values, `ConformanceViolation`
records, deviation traces.

**Receipt needed:** Monitoring receipt: conformance check timestamp, model version, fitness
and precision values (as `Between01`-bounded fractions), violation count.

**Board claim possible:** "The process is operating at fitness [X/Y] and precision [A/B]
against model version [V] as of [timestamp]."

**wasm4pm graduation boundary:** Token replay and alignment computation execute in wasm4pm.
Result shapes (`Metric`, `ConformanceViolation`) are owned by compat.

---

## Phase 7: Repair

**Required capability:** Respond to detected conformance violations by modifying the process
model, the system, or the operating procedures to restore conformance.

**Data produced:** Updated process model artifact + repair event records. The event log
accumulates evidence of the deviation-then-repair pattern.

**Receipt needed:** Repair receipt: naming the violated law, the corrective action taken,
the conformance metric before and after repair.

**Board claim possible:** "Violation of [named law] was detected on [date]. Repair action
[X] was applied. Post-repair fitness is [metric]."

---

## Phase 8: Optimization

**Required capability:** Apply enhancement techniques (performance annotation, organizational
mining, prediction) to improve process efficiency beyond mere conformance.

**Data produced:** Performance-annotated process model, organizational roles, predictive
model targeting next activity or remaining time.

**Receipt needed:** Optimization receipt: baseline metric values, optimization technique
applied, post-optimization metric values.

**Board claim possible:** "Process cycle time was reduced from [X] to [Y] by applying [technique]
under evidence admitted from [log version]."

---

## Phase 9: Board-Projection

**Required capability:** Translate admitted process evidence and conformance results into
board-admissible claims. Claims must be falsifiable and traceable to receipted evidence.

**Data produced:** Board presentation artifact grounded in process receipts. Each claim maps
to a specific `Metric`, `ConformanceViolation` set, or `AlignmentResult`.

**Receipt needed:** Board projection receipt: date of claim, process model version, evidence
log version, metric values at claim time.

**Board claim possible:** This phase IS the board claim surface. A claim without a board
projection receipt is narration, not process intelligence.

---

## Phase 10: Integration

**Required capability:** Connect the formal process to upstream data sources and downstream
consumers. Integration must not break the evidence chain.

**Data produced:** Integration mapping artifacts: `EventObjectLink` derivation from external
systems, format projection receipts for each external format consumed or emitted.

**Receipt needed:** Integration receipt: source format, target format, projection name,
`LossPolicy` applied, `LossReport` if loss occurred.

**Board claim possible:** "Integration with [system] has been established. Event-object links
are admitted under [witness]. Loss policy: [policy]. Loss report: [report]."

---

## Phase 11: Acquisition

**Required capability:** When a process-bearing system is acquired (M&A scenario), audit the
acquired process against the Blue River Dam criteria before claiming process intelligence.

**Data produced:** Acquisition audit report: conformance audit results, evidence chain
traceability assessment, gap analysis against process maturity levels.

**Receipt needed:** Acquisition receipt: conformance audit timestamp, fitness/precision values
for acquired process, identified gaps, remediation commitments.

**Board claim possible:** "Acquired process [X] has been audited. Current maturity level: [L].
Evidence chain: [admitted/partial/missing]. Gap remediation plan: [commitments]."

---

## Phase 12: Decommissioning

**Required capability:** Retire a process with full evidence preservation. The event log and
all receipts must remain available for post-decommissioning audit.

**Data produced:** Final archived `OcelLog` / `EventLog` + complete receipt ledger. Decommissioning
event admits the terminal state.

**Receipt needed:** Decommissioning receipt: final log snapshot, final conformance metrics,
named decommissioning reason, archival location.

**Board claim possible:** "Process [X] has been decommissioned on [date]. Final fitness: [metric].
Evidence archive: [location]. Reason: [named law or business decision]."

---

## Phase Summary

| Phase | Required Capability | Key Data | Receipt Type | Board Claim Tier |
|---|---|---|---|---|
| 1. Design | Formal model notation | WfNetConst / ProcessTree / POWL / Declare | WitnessReceipt | Model claim |
| 2. Simulation | Synthetic log generation | SyntheticEventLog | SimulationReceipt | Prediction claim |
| 3. Construction | Deployment | System version | ConstructionReceipt | Deployment claim |
| 4. Activation | First event | Initial OcelEvent | ActivationReceipt | Evidence-start claim |
| 5. Operation | Continuous admission | Growing OcelLog | OperationReceipts | Volume claim |
| 6. Monitoring | Conformance checking | Metric<FITNESS/PRECISION> | ConformanceReceipt | Compliance claim |
| 7. Repair | Violation response | Updated model + repair log | RepairReceipt | Improvement claim |
| 8. Optimization | Enhancement techniques | Performance annotations | OptimizationReceipt | Efficiency claim |
| 9. Board-Projection | Claim grounding | Board artifact | ProjectionReceipt | Board claim |
| 10. Integration | Evidence chain preservation | LossReport + integration map | IntegrationReceipt | Coverage claim |
| 11. Acquisition | M&A audit | Audit report + gap analysis | AcquisitionReceipt | M&A due diligence |
| 12. Decommissioning | Evidence archival | Final log + receipt ledger | DecommissionReceipt | Closure claim |
