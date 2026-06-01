# Algorithm Taxonomy Across Repos

This taxonomy maps every process mining algorithm family to its input types, output types,
ownership (compat = structure only; wasm4pm = execution), PM4Py coverage, and gap status.

**Law:** No algorithm that traverses data belongs in wasm4pm-compat.
Structure belongs in compat. Execution belongs in wasm4pm.

---

## Discovery Algorithms

### Alpha Miner (van der Aalst, Weijters, Maruster — 2004)

| Item | Detail |
|---|---|
| Input types | `EventLog` / `XesLog` (flat, case-centric) |
| Output types | `WfNetConst<SOUNDNESS>` with typed bipartite arcs |
| Structural surface | `PlaceToTransitionArc`, `TransitionToPlaceArc`, `WfNetConst` — owned by **compat** |
| Execution ownership | **wasm4pm** — footprint matrix computation, causal relation derivation, net construction |
| PM4Py coverage | `pm4py.discover_petri_net_alpha` |
| wasm4pm coverage | Gap — not yet implemented |
| Gap | Alpha Miner execution; `wasm4pm` has no Alpha Miner intake |

### Inductive Miner family (Leemans, Fahland, van der Aalst — 2013/2014/2022)

Variants: IM (2013), IMf (infrequent, 2014), IMc (incompleteness), scalable IM (Leemans 2022).

| Item | Detail |
|---|---|
| Input types | `EventLog` / `XesLog` (flat); directly-follows relation `Dfg` |
| Output types | `ProcessTree` with `TypedLoopNode<ARITY>` and `ProcessOperator` |
| Structural surface | `ProcessTree`, `TypedLoopNode<2>`, `ProcessOperator` — owned by **compat**; `Require<{ ARITY == 2 }>: IsTrue` enforced at compile time |
| Execution ownership | **wasm4pm** — IM cut detection (sequence/exclusive-choice/parallel/loop), base case handling, recursion over log partition |
| PM4Py coverage | `pm4py.discover_process_tree_inductive`, `pm4py.discover_petri_net_inductive` |
| wasm4pm coverage | Gap — not yet implemented |
| Gap | IM execution; noise-threshold parameter (IMf); incompleteness handling (IMc) |

### Heuristics Miner (Weijters, Ribeiro — 2011)

| Item | Detail |
|---|---|
| Input types | `EventLog` / `XesLog` |
| Output types | `CausalNet` (heuristics net) — **structural type missing from compat** |
| Structural surface | `Dfg`, `DfgEdge`, `DfgWeight` in compat; `CausalNet` shape not yet typed |
| Execution ownership | **wasm4pm** — dependency measure computation, threshold-based arc selection, short-loop handling |
| PM4Py coverage | `pm4py.discover_heuristics_net` |
| wasm4pm coverage | Gap — not yet implemented |
| Gap | `CausalNet` structural type needed in compat; Heuristics Miner execution in wasm4pm |

### POWL / ChoiceGraph (Kourani, van der Aalst — 2023/2026)

| Item | Detail |
|---|---|
| Input types | `EventLog` / `XesLog`; DFG |
| Output types | `PowlNodeKind` tree with `ChoiceGraphEdge` and `OrderEdge` |
| Structural surface | `PowlNodeKind`, `ChoiceGraphEdge`, `OrderEdge`, `SeparableWfNet`, `WfNet2PowlWitness` — owned by **compat** |
| Execution ownership | **wasm4pm** — POWL discovery from DFG/log, ChoiceGraph construction, WF-net→POWL transformation |
| PM4Py coverage | `pm4py.discover_powl` |
| wasm4pm coverage | Gap — not yet implemented |
| Gap | POWL discovery execution; WF-net→POWL conversion execution |

### mineDFG (van der Aalst — 2019)

| Item | Detail |
|---|---|
| Input types | `EventLog` / `XesLog` |
| Output types | `Dfg` with frequency/performance `DfgWeight` |
| Structural surface | `Dfg`, `DfgNode`, `DfgEdge`, `DfgWeight` — owned by **compat** |
| Execution ownership | **wasm4pm** — directly-follows relation computation, threshold-based filtering |
| PM4Py coverage | `pm4py.discover_dfg`, `pm4py.filter_dfg` |
| wasm4pm coverage | Gap — not yet implemented |
| Gap | DFG computation from event log; threshold-based filtering |

### OC-DFG Mining (Berti, van der Aalst — 2020)

| Item | Detail |
|---|---|
| Input types | `OcelLog` (object-centric) |
| Output types | `OcDfgEdge<ObjectType>` annotated DFG |
| Structural surface | `Dfg`, `DfgEdge`, `DfgWeight` in compat; `OcDfgEdge<ObjectType>` with `PhantomData` marker needed |
| Execution ownership | **wasm4pm** — OC-DFG computation from OCEL, per-object-type DFG aggregation |
| PM4Py coverage | `pm4py.discover_oc_dfg` |
| wasm4pm coverage | Gap — not yet implemented |
| Gap | `OcDfgEdge<ObjectType>` structural type in compat; OC-DFG computation in wasm4pm |

---

## Conformance Checking Algorithms

### Token Replay (Berti, van der Aalst — 2019)

| Item | Detail |
|---|---|
| Input types | `EventLog` + `WfNetConst<SOUNDNESS>` |
| Output types | `Metric<FITNESS>` with `Between01<NUM,DEN>` bounds; `AlignmentResult` shape |
| Structural surface | `FitnessConst`, `Between01`, `AlignmentResult` shape — owned by **compat** |
| Execution ownership | **wasm4pm** — fire transitions, count missing/remaining tokens, compute fitness formula |
| PM4Py coverage | `pm4py.conformance_diagnostics_token_based_replay` |
| wasm4pm coverage | Gap — not yet implemented |
| Gap | Token firing execution; missing/remaining token counters; fitness formula evaluation |

### Alignment-Based Conformance (Adriansyah, van Dongen, Munoz-Gama, Carmona — 2011)

| Item | Detail |
|---|---|
| Input types | `EventLog` + process model (Petri net or process tree) |
| Output types | `Metric<FITNESS>`, `Metric<PRECISION>`, `Metric<F1>` with `Between01` bounds; move sequences |
| Structural surface | `FitnessConst`, `PrecisionConst`, `F1Const`, `Between01` — owned by **compat** |
| Execution ownership | **wasm4pm** — synchronous product net construction, A* cost search, alignment trace computation |
| PM4Py coverage | `pm4py.conformance_diagnostics_alignments` |
| wasm4pm coverage | Gap — not yet implemented |
| Gap | Synchronous product net construction; A* cost search; alignment computation |

### Log Skeleton (Verbeek — 2018)

| Item | Detail |
|---|---|
| Input types | `EventLog` |
| Output types | `DeclareConstraint` set derived from log (always-after, always-before, etc.) |
| Structural surface | `DeclareConstraint`, `DeclareTemplate` variants — owned by **compat** |
| Execution ownership | **wasm4pm** — constraint derivation from log, frequency-bound computation |
| PM4Py coverage | `pm4py.discover_log_skeleton` |
| wasm4pm coverage | Gap — not yet implemented |
| Gap | Log skeleton derivation execution |

### DECLARE Checking (Pesic, van der Aalst — 2006)

| Item | Detail |
|---|---|
| Input types | `EventLog` + `DeclareConstraint` set |
| Output types | Per-constraint violation sets |
| Structural surface | `DeclareConstraint`, `DeclareTemplate`, `DeclareWitness` — owned by **compat** |
| Execution ownership | **wasm4pm** — LTL formula evaluation per trace, violation set construction |
| PM4Py coverage | `pm4py.conformance_diagnostics_declare` |
| wasm4pm coverage | Gap — not yet implemented |
| Gap | LTL constraint evaluation execution |

---

## Enhancement Algorithms

### Performance Annotation

Annotates discovered process models with time/cost data from event logs.

| Item | Detail |
|---|---|
| Input types | `EventLog` + discovered model (`WfNetConst`, `ProcessTree`, `Dfg`) |
| Output types | Performance-annotated model shapes |
| Structural surface | `DfgWeight` (performance variant), `Timestamp` — owned by **compat** |
| Execution ownership | **wasm4pm** — replay traces against model, aggregate performance statistics |
| PM4Py coverage | `pm4py.enhance_with_performance_map` |
| wasm4pm coverage | Gap |
| Gap | Performance aggregation execution |

### Organizational Mining

Discovers roles, groups, and social networks from the resource perspective.

| Item | Detail |
|---|---|
| Input types | `XesLog` with resource attribute |
| Output types | Role clusters, social-network edge types |
| Structural surface | `ResourceAttribute` (XES extension) — owned by **compat**; role/social-network result shapes needed |
| Execution ownership | **wasm4pm** — clustering, hand-off graph computation |
| PM4Py coverage | `pm4py.discover_organizational_roles` |
| wasm4pm coverage | Gap |
| Gap | Role cluster and social-network result shapes; organizational mining execution |

### Prediction (Teinemaa et al. — 2019; Compliance-Aware PPM 2026)

| Item | Detail |
|---|---|
| Input types | `EventLog` with prefix traces |
| Output types | `PredictionTarget` (outcome/remaining-time/next-activity) |
| Structural surface | `PredictionTarget` — owned by **compat** |
| Execution ownership | **wasm4pm** — prefix-based feature extraction, ML inference |
| PM4Py coverage | Partial — external libraries |
| wasm4pm coverage | Gap |
| Gap | `PredictionHorizon`/`PrefixLength` const-generic params in compat; ML inference in wasm4pm |

---

## Object-Centric Algorithms

### OC-Petri Net Mining (van der Aalst — 2020)

| Item | Detail |
|---|---|
| Input types | `OcelLog` |
| Output types | `PlaceToTransitionArc`/`TransitionToPlaceArc` with object-type inscription markers |
| Structural surface | Typed bipartite arcs with PhantomData object-type markers — owned by **compat** |
| Execution ownership | **wasm4pm** — per-object-type Petri net discovery, arc inscription assignment |
| PM4Py coverage | `pm4py.discover_oc_petri_net` |
| wasm4pm coverage | Gap |
| Gap | OC-Petri net discovery execution; arc inscription assignment |

### OCPQ Evaluation (Küsters, van der Aalst — 2025)

| Item | Detail |
|---|---|
| Input types | `OcelLog` + `OcpqQuery` |
| Output types | `OcpqResult` with per-object-type violation sets |
| Structural surface | `OcpqQuery`, `OcpqResult`, predicate witnesses — owned by **compat** |
| Execution ownership | **wasm4pm** — nested query evaluation over OCED, constraint checking |
| PM4Py coverage | Gap — OCPQ is new (2025) |
| wasm4pm coverage | Gap |
| Gap | OCPQ query execution; constraint satisfaction checking |

---

## Coverage Summary

| Algorithm family | compat structural coverage | wasm4pm execution | PM4Py | Gap |
|---|---|---|---|---|
| Alpha Miner | Complete | Missing | Yes | wasm4pm intake |
| Inductive Miner | Complete | Missing | Yes | wasm4pm intake |
| Heuristics Miner | Partial — `CausalNet` missing | Missing | Yes | `CausalNet` type + wasm4pm |
| POWL / ChoiceGraph | Complete | Missing | Yes | wasm4pm intake |
| mineDFG | Complete | Missing | Yes | wasm4pm intake |
| OC-DFG | Partial — `OcDfgEdge` missing | Missing | Yes | `OcDfgEdge` type + wasm4pm |
| Token Replay | Complete | Missing | Yes | wasm4pm intake |
| Alignment-based | Complete | Missing | Yes | wasm4pm A* search |
| Log Skeleton | Complete | Missing | Yes | wasm4pm intake |
| DECLARE Checking | Complete | Missing | Yes | wasm4pm intake |
| Performance Annotation | Partial | Missing | Yes | wasm4pm intake |
| Organizational Mining | Partial | Missing | Yes | role shapes + wasm4pm |
| Prediction | Partial | Missing | Partial | const-generic params + wasm4pm |
| OC-Petri Mining | Partial | Missing | Yes | arc inscription + wasm4pm |
| OCPQ Evaluation | Partial | Missing | No | wasm4pm intake |
