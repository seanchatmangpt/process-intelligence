# POWL and ChoiceGraph (POWL 2.0) — Deep Formal Analysis

**Analyst:** Dr. OCEL Specialist (AGI)
**Date:** 2026-05-31
**Source:** Kourani, van der Aalst — "POWL: Partially Ordered Workflow Language" (ICPM 2023); Kourani, Park, van der Aalst — "POWL 2.0: ChoiceGraph for Flexible Process Discovery" (arXiv:2505.07052, 2025)

---

## Formal Objects (POWL 1.0)

### StrictPartialOrder
- `nodes: Set<PowlNode>` — set of sub-models (activities or operators)
- `order: Set<(PowlNode, PowlNode)>` — strict partial order (irreflexive, asymmetric, transitive)
- `concurrency` is implicit: nodes with no order relation may execute in any order or concurrently
- Constraint: `order` must be a DAG (no cycles)
- Semantic: executions are all topological sorts of `order` over `nodes`

### OperatorPOWL (discriminated union)
- `XorChoice(children: Vec<PowlNode>)` — exactly one child executes (exclusive choice)
- `Loop(do: PowlNode, redo: PowlNode)` — `do` executes, then optionally `redo` + `do` repeats
- `Parallel(children: Vec<PowlNode>)` — all children execute in any order (derived from StrictPartialOrder with empty order)

### SilentTransition
- `tau: ()` — an invisible activity; used in loops and optional paths
- May appear as a child of `XorChoice` to represent optional execution

### PowlNode (recursive)
- `Activity(label: ActivityName)` — leaf node
- `SilentTransition` — invisible leaf
- `OperatorPOWL(op)` — composite node
- `StrictPartialOrder(spo)` — composite with partial order

---

## Formal Objects (POWL 2.0 — ChoiceGraph)

### ChoiceGraph
- A **directed acyclic graph (DAG)** where nodes are `XorChoice` decision points
- Edges encode which choice at node X constrains the available choices at node Y
- Captures **decision dependencies** between XOR choices that a flat POWL tree cannot express
- Key insight: in real processes, the choice at one point (e.g., "approve" vs "reject") constrains what happens later — ChoiceGraph makes this explicit

### mineDG Algorithm (POWL 2.0 discovery)
- Input: event log (DFG + directly-follows with decision context)
- Step 1: Identify all XOR decision points as candidate ChoiceGraph nodes
- Step 2: For each pair of decision points (X, Y), test whether the choice at X statistically constrains the distribution at Y
- Step 3: Add directed edge X → Y if the constraint is confirmed (e.g., via chi-squared test or conditional entropy)
- Step 4: Prune transitive redundancies
- Output: `ChoiceGraph` as the top-level POWL 2.0 model

**Significance:** mineDG is the first algorithm that recovers **inter-decision dependencies** from process logs without requiring a causal model as input.

---

## wasm4pm-compat Coverage

| POWL Concept | wasm4pm-compat Module | Coverage |
|---|---|---|
| StrictPartialOrder shape | `src/powl.rs` — `StrictPartialOrder` | Full |
| OperatorPOWL shapes | `src/powl.rs` — XOR, Loop types | Full |
| SilentTransition | `src/powl.rs` | Full |
| TreeProjectable trait | `src/powl.rs` — sealed trait | Full |
| ProcessTree conversion path | `src/powl.rs` → `src/process_tree.rs` | Structural only |
| ChoiceGraph shape (POWL 2.0) | Not present | None |
| mineDG algorithm | Not present | None |

wasm4pm-compat defines all POWL 1.0 shapes. POWL 2.0 ChoiceGraph is absent.

---

## wasm4pm Gap Assessment

**Status: POWL discovery is MISSING from wasm4pm.**

Current wasm4pm:
- Has no POWL discovery implementation (neither 1.0 nor 2.0)
- Cannot produce `StrictPartialOrder` or `ChoiceGraph` from an event log
- Cannot exercise the `TreeProjectable` path in wasm4pm-compat

PM4Py gap: The standard PM4Py package does not include POWL 2.0 / mineDG. A custom fork (Kourani's research fork) has mineDG. This means **neither wasm4pm nor PM4Py currently ships POWL 2.0**.

**Strategic opportunity:** Implementing mineDG in wasm4pm before PM4Py mainline would provide a genuine capability advantage.

---

## PM4Py Coverage Assessment

| Capability | PM4Py Module | Maturity |
|---|---|---|
| POWL 1.0 discovery | `pm4py.discovery.discover_powl` | Available in recent versions |
| POWL visualization | Custom visualizer | Limited |
| ChoiceGraph / mineDG (POWL 2.0) | Not in mainline | Research fork only |

---

## Gap Action

### Phase 1: POWL 1.0 Discovery
- Input: `Admitted<OcelLog, Ocel20>`
- Algorithm: Inductive Miner variant adapted for partial order output
- Output: `StrictPartialOrder` typed as `Admitted<PowlModel, PowlPaper>`
- Prerequisite: Inductive Miner must be implemented first (see INDUCTIVE_MINER_ANALYSIS.md)

### Phase 2: POWL 2.0 ChoiceGraph (mineDG)
- Add `ChoiceGraph` shape to wasm4pm-compat `src/powl.rs`
- Implement mineDG in wasm4pm:
  1. DFG construction from `Admitted<OcelLog, Ocel20>`
  2. XOR decision point identification
  3. Pairwise dependency testing (chi-squared or conditional entropy)
  4. DAG construction and transitive pruning
- Output: `ChoiceGraph` typed as `Admitted<PowlModel, ChoiceGraphPaper>`
- Receipt: `Receipt<ChoiceGraph, ChoiceGraphPaper>` for board claim attestation

---

## Action Items

| Priority | Action | Owner |
|---|---|---|
| P0 | Add `ChoiceGraph` shape to wasm4pm-compat `src/powl.rs` | wasm4pm-compat |
| P0 | Add `ChoiceGraphPaper` witness to `src/witness.rs` | wasm4pm-compat |
| P1 | Implement POWL 1.0 discovery in wasm4pm | wasm4pm |
| P1 | Implement mineDG (POWL 2.0) in wasm4pm | wasm4pm |
| P2 | Implement `StrictPartialOrder → WF-Net` conversion | wasm4pm |
