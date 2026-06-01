# Object-Centric Supremacy over Trace-Based PM

## The Classical Trap

Classic process mining is trace-centric. Every conformance check, every discovery algorithm,
every performance annotation assumes exactly one case identifier per process execution. XES
1849 (IEEE standard) encodes this assumption at the schema level: a `XesTrace` carries a
single case-id, and every event in the trace belongs to that case and only that case.

This works for processes where one ticket, one patient, one order, or one application maps
cleanly to a sequence of events. It fails everywhere else.

---

## The Two Structural Defects of Classic PM

### 1. Divergence

When a single case-id maps to multiple objects, flattening the OCEL data to a trace-centric
log **duplicates events** — once per object. An order-picking event that affects three
packages is logged three times. The discovered DFG shows three times as many "pick" activities.
Discovery algorithms overcount behavior that happened once.

**Van der Aalst and Berti (2020)** named this defect: divergence. It is not a data quality
problem. It is a structural consequence of forcing object-centric data into a case-centric
mold.

In wasm4pm-compat: `OcelLog` + `EventObjectLink` resolves divergence by construction.
The event-to-object link is many-to-many. One event can link to many objects. There is no
duplication. `DivergenceWitness` (to be added to `src/witness.rs`) will name this law.

### 2. Convergence

When multiple case-ids share one object, flattening produces **merged traces** — events from
different cases appear interleaved in the same flattened log. An invoice paid by multiple
orders appears in every one of those orders' traces. Discovery algorithms undercount distinct
behavior and infer spurious sequential dependencies.

**Van der Aalst and Berti (2020)** named this defect: convergence. Again, not a data
quality problem — a structural consequence of case-centric flattening.

In wasm4pm-compat: `ObjectObjectLink` resolves convergence by keeping object identity
first-class. An invoice is an object. An order is an object. Their relation is a typed link,
not an implicit case-id merge.

---

## Why OCEL 2.0 Supersedes XES

OCEL 2.0 (van der Aalst, Berti, Ghahfarokhi, Klijn, Park, Pourbafrani — 2023) is not an
extension of XES. It is a different paradigm.

| Dimension | XES 1849 (classic) | OCEL 2.0 (object-centric) |
|---|---|---|
| Unit of analysis | One trace per case | Multiple interacting object types |
| Event identity | Event belongs to one trace | Event linked to N objects across M types |
| Object-to-object relation | Not representable | `ObjectObjectLink` (first-class) |
| Attribute changes | Event attributes only | `ObjectChange` (typed attribute change at event time) |
| Flattening | Implicit (all events in trace) | Explicit `LossPolicy` + `LossReport` required |
| Divergence handling | Not handled — defect by design | Resolved structurally by E2O link |
| Convergence handling | Not handled — defect by design | Resolved structurally by O2O link |
| PM4Py model | `EventLog`, `Trace` | `OcelLog`, `OcelEvent`, `OcelObject`, `EventObjectLink`, `ObjectObjectLink` |

**OCEL 2.0 does not make XES wrong. It makes XES insufficient for multi-object processes.**

XES remains the canonical format for processes that are genuinely single-case-centric. The
`CaseCentricMarker` in wasm4pm-compat's `src/xes.rs` seals this: `XesTrace` carries a
`CaseCentricMarker` that prevents it from being confused with an OCEL-grounded structure at
the type level.

When a process has multiple object types, XES cannot represent it without loss. That loss
must be explicit — named, policy-governed, and reported. The `LossPolicy::AllowLossWithReport`
path in `src/loss.rs` + the `ocel_to_xes_projection` example in wasm4pm-compat's examples/
directory demonstrates the lawful path.

**Unlawful path (defect):** Flatten OCEL to XES without `ProjectionName`, without
`LossPolicy`, without `LossReport`. This is raw laundering — the exact defect the format
covenant (Blue River Dam, Chapter 4) prohibits.

---

## The Board Claim

> "Our process intelligence is object-centric; classic PM tools are trace-bound."

This claim is board-admissible when and only when:

1. The event data is admitted as `OcelLog` (not flattened to `XesLog`) under `Ocel20` witness.
2. E2O links are first-class (`EventObjectLink`) — not derived from case-id matching.
3. O2O links are first-class (`ObjectObjectLink`) — not inferred from trace merging.
4. Any projection to XES is governed by a named `LossPolicy` and a `LossReport`.
5. Conformance and discovery algorithms operate over the object-centric structure in wasm4pm,
   not over a flattened trace proxy.

---

## The "No AI Without PI" Principle (van der Aalst — 2025)

Van der Aalst's 2025 paper establishes that generative, predictive, and prescriptive AI over
business processes is groundless without an underlying process intelligence (PI) layer. PI
requires the OCED pipeline: OCEL discovery → compliance analysis → performance analysis.

Classic PM provides a trace-centric PI layer. Object-centric PM provides an OCED-grounded
PI layer. The structural gap between them is not a feature difference — it is the difference
between a system that can be audited and a system that cannot.

An AI system that claims to optimize a multi-object business process but grounds its claims
in a flattened, case-centric trace log is operating on distorted evidence. It cannot be
audited at the process level. Its claims are narration.

An AI system grounded in an admitted `OcelLog` with lawful conformance results from wasm4pm
can trace every claim to a receipt. Its claims are process intelligence.

---

## Structural Gap

The following structural work remains outstanding in wasm4pm-compat to fully seal
object-centric supremacy:

1. `DivergenceWitness` and `ConvergenceWitness` named witness types in `src/witness.rs`
   (paper: OC-PM Divergence/Convergence, van der Aalst & Berti 2020).

2. `OcDfgEdge<ObjectType>` with `PhantomData` object-type marker in `src/dfg.rs`
   (paper: OC-DFG, Berti & van der Aalst 2020).

3. Arc inscription as typed `PhantomData<ObjectType>` marker on `PlaceToTransitionArc`
   for OC-Petri nets (paper: Discovering OC-Petri Nets, van der Aalst 2020).

Until these gaps are closed, OCEL 2.0 supremacy is structurally asserted but not fully sealed.
