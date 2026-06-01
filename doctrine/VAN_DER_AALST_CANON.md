# Van der Aalst Process Mining Canon

> "Process mining bridges the gap between data mining and process modeling." — van der Aalst

---

## Seven Perspectives of Process Mining

Process mining is not a single activity. It is a multi-perspectival discipline that examines the
same underlying event data from seven distinct analytical angles. Each perspective is a named
law, not a feature toggle.

### 1. Control Flow

What happened, in what order? The control-flow perspective discovers and checks the ordering
of activities as expressed in Petri nets, process trees, BPMN, POWL, and Declare models.
Discovery: Alpha Miner, Inductive Miner, Heuristics Miner, POWL/ChoiceGraph.
Conformance: token replay, alignment-based checking, log skeleton.

This perspective is the **foundation**. All other perspectives presuppose it.

### 2. Data / Decision

At decision points (XOR-splits, Declare conditional activations), which data attributes
determined the route taken? The data perspective mines decision trees at branching nodes and
annotates process models with guard conditions derived from event attributes.

Key structures: `XesEvent` attribute map, `DeclareConstraint` guard expressions,
`GatewayKind::Xor` with data-driven routing condition.

### 3. Resource / Organizational

Who performed the activity? The resource perspective discovers organizational models: roles,
groups, hand-off patterns, social networks derived from the resource attribute in event logs.

Key structures: `ResourceAttribute` (XES extension), role clusters, social-network edge types.
This perspective is the primary source of bottleneck and SLA violation detection.

### 4. Time

When did activities occur, and how long did they take? The time perspective annotates
conformance results and process models with performance data: waiting times, service times,
sojourn times, cycle times.

Key structures: `Timestamp` (nanosecond precision), `DfgWeight` (frequency and performance
variants), performance annotation on conformance result shapes.

### 5. Cost

How much did each activity and case cost? The cost perspective associates monetary values with
activities and resources. It is structurally analogous to the time perspective but carries
currency-denominated weights.

Key structures: cost annotation type (analogous to `DfgWeight` with currency dimension),
cost-aware conformance metric shape.

### 6. Case / Object

What is the unit of analysis? Classic PM: one case per process instance (trace-centric,
XES). Object-centric PM: multiple interacting object types per process execution (OCEL 2.0).
The case/object perspective defines which granularity of evidence is admitted.

Key structures: `XesTrace` (case-centric), `OcelLog` + `EventObjectLink` + `ObjectObjectLink`
(object-centric). This perspective is the **architectural fault line** between classic and
modern process mining.

### 7. Exception / Deviation

Where does actual behavior deviate from the expected model? The exception perspective identifies
non-conforming traces, deviant paths, and systematic rule violations. It transforms conformance
checking results (fitness, precision, alignment costs) into actionable deviation records.

Key structures: `ConformanceViolation`, `AlignmentResult`, `Metric<FITNESS>`, `Metric<PRECISION>`,
`StrictViolation` (from the `strict` feature), `DeclareConstraint` violation sets.

---

## Four Quality Dimensions

Every discovered or declared process model is evaluated on four dimensions. These are the
"compass" of process mining quality (van der Aalst 2011, Carmona et al. 2018).

### Fitness

Does the model allow all observed behavior in the log?
Formula (token replay): `0.5 * (1 - missing/produced) + 0.5 * (1 - remaining/consumed)`
Range: [0, 1]. A fitness value outside [0,1] is a type error, not a runtime exception.

### Precision

Does the model avoid allowing too much behavior not observed in the log?
Formula (ETC, Munoz-Gama & Carmona 2010): escaping-edges ratio.
Range: [0, 1]. High fitness + low precision = an overly permissive ("flower") model.

### Simplicity

Is the model as simple as possible? Occam's razor applied to process models.
Heuristic: minimal number of nodes and arcs that still fits and is precise.

### Generalization

Does the model generalize beyond the specific observed traces, without overfitting?
High generalization = the model describes the process class, not just the logged sample.

### The Tradeoff

Fitness and precision are in tension. Maximizing both simultaneously is NP-hard in the
general case. Every conformance checking system must make this tradeoff explicit — hiding it
in a single aggregate score without decomposition is a defect.

---

## Mapping to wasm4pm-compat vs. wasm4pm

| Concern | wasm4pm-compat owns | wasm4pm owns |
|---|---|---|
| Control flow shapes | `PetriNet`, `WfNetConst<SOUNDNESS>`, `ProcessTree`, `PowlNodeKind`, `DeclareConstraint`, `BpmnElement`, `GatewayKind` | Alpha Miner execution, Inductive Miner execution, Heuristics Miner execution, POWL discovery |
| Data perspective shapes | `XesEvent` attribute map, `GatewayKind::Xor` structural type | Decision mining execution, decision tree induction |
| Resource shapes | `ResourceAttribute` (XES extension namespace) | Organizational mining, social network computation |
| Time perspective shapes | `Timestamp`, `DfgWeight` (performance variant) | Performance annotation, cycle-time computation |
| Cost shapes | Cost annotation structural type (to be typed) | Cost-aware replay execution |
| Case/Object perspective | `XesTrace` (case-centric), `OcelLog`/`EventObjectLink`/`ObjectObjectLink` (OC) | OC-DFG computation, OC-Petri net discovery |
| Exception/Deviation shapes | `Metric<FITNESS>`, `Metric<PRECISION>`, `ConformanceViolation`, `AlignmentResult` shape, `StrictViolation` | Token replay execution, alignment-based conformance, A* cost search |

The boundary law is absolute: **no algorithm that traverses data belongs in wasm4pm-compat.**
Structure belongs in compat. Execution belongs in wasm4pm.

---

## Foundational References

- van der Aalst (2011/2016). *Process Mining: Discovery, Conformance and Enhancement of
  Business Processes*. Springer. The foundational PM textbook; every canonical structural shape
  in this codebase traces its type-law lineage here.
- Carmona, van Dongen, Solti, Weidlich (2018). *Conformance Checking*. Springer.
- Munoz-Gama, Carmona (2010). Measuring Precision of Modeled Behavior.
- Berti, van Zelst, Schuster (2023). PM4Py. Software Impacts.
