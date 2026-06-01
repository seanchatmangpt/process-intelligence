# POWL — Partially Ordered Workflow Language

**Authority:** Kourani et al. 2023 (POWL); POWL 2.0 ChoiceGraph (arXiv:2505.07052)
**Witness key:** `powl` — `WitnessFamily::Paper`

---

## POWL 1.0 (Kourani et al. 2023)

POWL is a partial-order formalism for process models. Unlike block-structured process
trees, POWL can express genuine concurrency without forcing artificial sequentialization:

### Core Nodes

| POWL node | Semantics |
|---|---|
| **Atom** | A single activity (leaf task). |
| **StrictPartialOrder** | A DAG of precedence edges over child nodes. Unordered children may execute concurrently. |
| **OperatorNode(XOR)** | An exclusive choice among child branches. Exactly one branch is taken. |
| **OperatorNode(LOOP)** | A loop: a `do` body and an optional `redo` branch (Leemans semantics). |
| **SilentTransition** | A tau step — no observable activity; used to close choices and loops. |

The key power of POWL is that a `StrictPartialOrder` does not need to impose total order:
two nodes with no precedence edge between them may interleave freely. This cannot be
expressed in a block-structured process tree.

---

## POWL 2.0: ChoiceGraph (arXiv:2505.07052)

POWL 2.0 introduces the **ChoiceGraph** extension: `mineDG` (mine Decision Graph)
discovers the structure of exclusive choices by analyzing data attributes on events. A
choice node in POWL 2.0 carries a ChoiceGraph encoding which data conditions determine
which branch is taken.

This elevates POWL from a purely control-flow formalism to a **data-aware** process model.
The `wasm4pm` engine must provide `mineDG` discovery; wasm4pm-compat carries the structural
shape of a ChoiceGraph node as a bearer type.

---

## wasm4pm-compat Implementation (powl.rs)

`src/powl.rs` models POWL as a **first-class** structural canon:

- `PowlNode` — an enum with variants for each node type: `Atom`, `PartialOrder`,
  `Choice`, `Loop`, `Silent`, and `Irreducible` (a fragment that exceeds process-tree
  expressibility).
- `OrderEdge` — a directed precedence edge between two `PowlNode` IDs within a
  `StrictPartialOrder`.
- `Powl` — the root container holding nodes and edges.

### Witness markers (node-kind markers)

Each POWL fragment type has a corresponding witness marker in `powl.rs`:

```rust
Atom, PartialOrder, Choice, Loop, Silent, Irreducible
```

And two projection markers:

```rust
ProcessTreeProjectable   // This POWL fragment can be expressed as a process tree.
ExceedsProcessTree       // This POWL fragment requires partial-order expressibility.
```

### TreeProjectable (sealed trait)

`TreeProjectable` is a sealed trait in `powl.rs`. Only `PowlNode` fragments that are
block-structured (Atom, Choice, Loop, and PartialOrder nodes that impose total order)
implement `TreeProjectable`. The `assert_tree_projectable` function provides a compile-time
check that a given node can be projected downward.

Projection POWL → ProcessTree is a **named, refusable** operation — never an implicit
coercion. `PowlRefusal::ExceedsBlockStructure` names the law under which it fails.

### `PowlRefusal` surface

Named refusal reasons:
- `ExceedsBlockStructure` — the POWL DAG contains a partial order that cannot be
  block-structured.
- `EmptyPartialOrder` — a `StrictPartialOrder` with no children.
- `CycleInPartialOrder` — a cycle in the precedence DAG (violates partial-order law).
- `InvalidLoopArity` — a loop node with fewer than two children.

---

## What wasm4pm Must Provide

| Capability | Graduates to |
|---|---|
| POWL discovery (SplitMiner, Inductive Miner variant) | `wasm4pm` |
| ChoiceGraph `mineDG` discovery | `wasm4pm` |
| POWL conformance checking (replay, alignment) | `wasm4pm` |
| POWL simplification and pruning | `wasm4pm` |
| POWL → Petri-net translation | `wasm4pm` |
| POWL fitness/precision/generalization metrics | `wasm4pm` |

---

## Board Placement

POWL is the state-of-the-art process representation for real-world logs with concurrency.
It expresses what process trees cannot (partial order), and its POWL 2.0 ChoiceGraph
extension makes it data-aware. wasm4pm-compat enforces POWL structural laws at the type
level — including the sealed TreeProjectable trait — so no invalid POWL shape can reach
the `wasm4pm` discovery engine.