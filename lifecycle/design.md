# DESIGN — Process Intelligence Lifecycle Phase

## What is designed

The design phase produces formally grounded process models in standard notations:

- **WF-net (Workflow net):** A Petri net with a single source place and single sink place,
  every transition on a path from source to sink. Soundness is a decidable structural property:
  option to complete, proper completion, no dead transitions.
- **BPMN:** Business Process Model and Notation. Mapped to WF-net semantics for soundness
  verification. Control-flow constructs (sequence, XOR, AND, OR gateways) have formal semantics.
- **POWL (Partially Ordered Workflow Language):** A tree-structured model where nodes are
  activities, XOR choices, parallel branches, or loops. Tree projectability is a type-level
  invariant in `wasm4pm-compat` (`src/powl.rs`: `TreeProjectable` sealed trait).
- **Declare:** Constraint-based process model. Each constraint names a law between activities
  (e.g., `Response(A, B)`, `NotCoexistence(A, B)`). The model is the set of constraints;
  any trace satisfying all constraints is conforming.

## What receipts prove soundness

Soundness is not a claim made in prose. It is a receipt earned by structural verification:

- **WF-net soundness receipt:** `WfNetConst<SOUNDNESS>` in `src/petri.rs` — the non-forgeable
  witness path enforces that a WF-net instance carries its soundness class as a const generic
  parameter. A `WfNetConst<Unsound>` cannot be admitted as sound. The soundness variant is
  encoded at the type level, not asserted at runtime.
- **POWL tree receipt:** `assert_tree_projectable` in `src/powl.rs` — compile-time assertion
  that a POWL node satisfies the tree-projectability law.
- **Typed loop receipt:** `TypedLoopNode<ARITY>` in `src/process_tree.rs` with
  `Require<{ ARITY == 2 }>: IsTrue` — a loop node with wrong arity is a compile error, not a
  runtime panic.
- **Conformance metric receipt:** `Metric<KIND, NUM, DEN>` in `src/conformance.rs` with
  `Between01` bounds — a metric outside [0, 1] is structurally inadmissible.

## Board claim

> Our processes are formally modeled in standard notations with provable soundness.

This claim is admissible only if:
1. Each process model is an instance of a typed structure (WF-net, BPMN, POWL, Declare) —
   not a prose description or diagram image.
2. Soundness was checked by a structural verifier, not declared by a human.
3. A receipt exists: a typed artifact that encodes the soundness verdict and cannot be forged
   by bypassing the verifier.

Without receipts, "formally modeled with provable soundness" is narration. With receipts, it is
a replayable, auditor-admissible claim.

## wasm4pm-compat surfaces

| Type | Module | Design-phase role |
|---|---|---|
| `WfNetConst<SOUNDNESS>` | `src/petri.rs` | Carries WF-net with soundness class |
| `TypedLoopNode<ARITY>` | `src/process_tree.rs` | Enforces loop arity law at compile time |
| `TreeProjectable` | `src/powl.rs` | Seals POWL nodes that can be tree-projected |
| `Metric<KIND, NUM, DEN>` | `src/conformance.rs` | Bounds conformance metrics to [0,1] |
| `ConditionCell<BITS>` | `src/law.rs` | Compile-time law kernel for condition bounds |

## What does NOT belong here

- Discovery (fitting a model to an observed log) belongs in the simulation or operation phase.
- Conformance checking (replaying a log against a model) belongs in the operation phase.
- Repair (modifying the model based on observed violations) belongs in the repair phase.

The design phase closes when a formally sound model exists as a typed, receipted artifact.
