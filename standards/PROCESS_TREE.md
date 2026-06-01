# Process Tree — Block-Structured Process Representation

**Authority:** Leemans et al. 2013 (Inductive Miner); process tree algebra (van der Aalst 2016)
**Witness key:** `process-tree` — `WitnessFamily::Paper`

---

## Overview

A **process tree** is a recursive, block-structured model built from operators over
activity leaves. Every process tree corresponds to a sound WF-net (the conversion is
structural, not computed). This makes process trees the canonical output of the Inductive
Miner family of discovery algorithms.

---

## Operators

| Operator | Notation | Semantics |
|---|---|---|
| **Sequence** | `→(A, B, ...)` | Execute children left-to-right in total order. |
| **Exclusive Choice (XOR)** | `×(A, B, ...)` | Execute exactly one child branch. |
| **Parallel (AND)** | `∧(A, B, ...)` | Execute all children in any interleaving (true concurrency). |
| **Loop** | `↺(do, redo)` | Execute `do`; optionally repeat via `redo`, then `do` again. Leemans arity: exactly 2 children. |
| **Silent (tau)** | `τ` | An invisible step with no observable activity. No children. |
| **Or (inclusive choice)** | `∨(A, B, ...)` | Execute one or more children in any interleaving. |

### Loop Arity Law

The `Loop` operator requires exactly 2 children: the `do`-body (mandatory) and the
`redo`-branch (optional, may be tau). This is the Leemans definition. A loop node with
fewer than 2 children is structurally inadmissible.

---

## Inductive Miner Output

The Inductive Miner (IM) and its variants (IMf — with noise filtering, IMd — data-aware)
produce a process tree as their primary output. The process tree is then convertible to:

- A WF-net (by structural induction on the tree operators) — guaranteed sound.
- A BPMN diagram (by mapping operators to gateway types).
- A POWL model (by treating the tree as a restricted partial order).

When discovery produces a process tree, the conversion to WF-net is a theorem, not an
engine computation. But **the discovery itself** is an engine responsibility.

---

## Process Tree vs. POWL

| | Process Tree | POWL |
|---|---|---|
| Expressiveness | Block-structured only | Includes partial orders exceeding block structure |
| Concurrency | AND-split/join must be matched | StrictPartialOrder DAG; no required matching |
| Inductive Miner output | Yes | Yes (SplitMiner, IM-POWL variant) |
| Can project to process tree | — | Only if `TreeProjectable`; otherwise `ExceedsBlockStructure` |

---

## wasm4pm-compat Implementation (process_tree.rs)

`src/process_tree.rs` models the process tree **shape** — structure only:

- `ProcessTree` — a tree of `ProcessTreeNode`s with a root node.
- `ProcessTreeNode` — an enum: `Operator(ProcessTreeOperatorNode)` or `Leaf(ActivityLeaf)`.
- `ProcessTreeOperator` — a closed enum: `Sequence`, `Xor`, `Parallel`, `Loop`, `Silent`, `Or`.
- `ActivityLeaf` — a strongly-named `String` activity label.
- `ProcessTreeRefusal` — named refusal surface: `LoopArityViolation`, `EmptyOperator`,
  `UnknownOperator`, `CyclicStructure`.

### TypedLoopNode<ARITY>

The nightly foundry module (`nightly_foundry.rs`) provides `TypedLoopNode<ARITY>` with a
`const` generic arity parameter sealed by:

```rust
Require<{ ARITY == 2 }>: IsTrue
```

This makes `TypedLoopNode<1>` a compile error — the Leemans arity law is enforced at the
type level, not at runtime. A loop node of the wrong arity cannot reach the `wasm4pm`
engine.

### operator_minimum_arity

The `operator_minimum_arity` function is a `const fn` encoding minimum arity for each
operator:

| Operator | Minimum arity |
|---|:---:|
| Sequence | 2 |
| Xor | 2 |
| Parallel | 2 |
| Loop | 2 |
| Silent | 0 |
| Or | 2 |

---

## What wasm4pm Must Provide

| Capability | Graduates to |
|---|---|
| Inductive Miner discovery (IM, IMf, IMd) | `wasm4pm` |
| Process tree → WF-net conversion | `wasm4pm` |
| Process tree replay (token game, alignment) | `wasm4pm` |
| Process tree simplification | `wasm4pm` |
| Process tree fitness/precision metrics | `wasm4pm` |
| POWL → process tree projection (when TreeProjectable) | `wasm4pm` |

---

## Board Placement

Process trees are the gold-standard output of the Inductive Miner — the most widely used
discovery algorithm in academic and commercial process mining. The compile-time arity law
(`TypedLoopNode<ARITY>` with `Require<{ ARITY == 2 }>: IsTrue`) is a concrete example of
wasm4pm-compat's value: a loop node with the wrong arity is a compile error, not a
runtime crash.
