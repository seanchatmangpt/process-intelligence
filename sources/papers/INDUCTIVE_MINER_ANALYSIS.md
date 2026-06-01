# Inductive Miner — Deep Formal Analysis

**Analyst:** Dr. OCEL Specialist (AGI)
**Date:** 2026-05-31
**Source:** Leemans, Fahland, van der Aalst — "Discovering Block-Structured Process Models from Event Logs" (2013)

---

## Formal Objects

### DFG (Input)
- `nodes: Set<Activity>` — alphabet of observed activities
- `edges: Map<(Activity, Activity), Frequency>` — directly-follows relations with counts
- `start_activities: Map<Activity, Frequency>` — activities that begin traces
- `end_activities: Map<Activity, Frequency>` — activities that end traces
- Derived from: an event log by scanning consecutive activity pairs

### ProcessTree (Output)
- Recursive algebraic data type:
  - `Leaf(Activity)` — atomic task node
  - `Sequence([children])` — `→` operator — children execute in order
  - `ExclusiveChoice([children])` — `×` operator — exactly one child executes
  - `Parallel([children])` — `∧` operator — all children execute in any order
  - `Loop(do, redo, exit)` — `↺` operator — body executes, then optionally redo
- **Critical:** Output is a `ProcessTree`, NOT a PetriNet directly

### WF-Net Conversion
- `ProcessTree → WF-Net` is a second step, separate from discovery
- Each operator maps to a structured subnet:
  - `Sequence` → chain of transitions
  - `ExclusiveChoice` → split + join with XOR routing
  - `Parallel` → AND-split + AND-join
  - `Loop` → back-arc with silent transition
- **Soundness is inherited:** a well-structured ProcessTree always yields a sound WF-net
- This is the key soundness guarantee the Alpha Miner lacks

### Cut Operators (Discovery Algorithm)
1. **Sequence cut** — partition activities where all DFG edges go left-to-right
2. **Exclusive choice cut** — partition activities with no cross-partition DFG edges
3. **Parallel cut** — partition activities where all cross-partition edges are bidirectional
4. **Loop cut** — partition into do-body and redo-body based on start/end overlap
5. **Fall-through** — if no cut applies: flower model (admits all behavior)

---

## Key Insight: ProcessTree is Not PetriNet

The Inductive Miner outputs a `ProcessTree`. To obtain a PetriNet (WF-net), a **separate conversion step** is required. This has critical implications:

- Conformance checking algorithms operating on PetriNets (token replay, alignment) require the conversion
- The conversion is deterministic but adds silent transitions
- Fitness metrics must account for silent transitions in alignment costs
- Skipping the conversion and treating ProcessTree as PetriNet is a semantic error

**Obligation for wasm4pm:** The execution engine must expose both:
1. `discover_process_tree(Admitted<OcelLog, Ocel20>) -> ProcessTree`
2. `process_tree_to_wf_net(ProcessTree) -> Admitted<WfNet, WfNetSoundnessPaper>`

---

## wasm4pm-compat Coverage

| Inductive Miner Concept | wasm4pm-compat Module | Coverage |
|---|---|---|
| ProcessTree output shape | `src/process_tree.rs` — `TypedLoopNode<ARITY>`, operator types | Partial — shape only, no discovery |
| WF-Net shape | `src/petri.rs` — `WfNetConst<SOUNDNESS>` | Structural shape, no conversion |
| Loop arity constraint | `Require<{ ARITY == 2 }>: IsTrue` | Full — type law enforced |
| Tree projectability | `src/powl.rs` — `TreeProjectable` | Structural only |

wasm4pm-compat defines the **output shapes** and **type laws**. It does not implement the discovery algorithm.

---

## wasm4pm Gap Assessment

**Status: Inductive Miner is MISSING from wasm4pm.**

Current wasm4pm uses heuristics-based discovery (heuristic miner) as its primary process discovery algorithm. This is a significant gap because:

1. Heuristic miner does not guarantee a sound WF-net output
2. Heuristic miner does not produce a ProcessTree (no block-structured output)
3. Without a ProcessTree, the loop arity type law in `process_tree.rs` cannot be exercised
4. Board claims about "sound process models" are unattested when discovery is heuristic

### Gap Action

wasm4pm must implement Inductive Miner:
- Input: `Admitted<OcelLog, Ocel20>` (not raw EventLog)
- Algorithm: DFG construction → recursive cut detection → ProcessTree assembly
- Output: `Admitted<ProcessTree, InductiveMinerPaper>` with witness threading
- Conversion: `process_tree_to_wf_net` yielding `WfNetConst<Sound>`

---

## PM4Py Coverage Assessment

| Capability | PM4Py Module | Maturity |
|---|---|---|
| Inductive Miner (flat log) | `pm4py.discovery.discover_process_tree_inductive` | Mature — handles noise via IMf variant |
| Inductive Miner (OCEL) | `pm4py.discovery.discover_process_tree_inductive` with OCEL flattening | Limited — requires flattening first |
| ProcessTree → PetriNet | `pm4py.convert.convert_to_petri_net` | Mature |
| Inductive Miner Infrequent (IMi) | Available | Mature |
| Inductive Miner with Filtering (IMf) | Available | Mature |

PM4Py's Inductive Miner is the reference implementation. wasm4pm must match or exceed its behavior on the typed `Admitted<OcelLog, Ocel20>` input, not a flattened proxy.

---

## Board Claim Implications

Any board claim of the form "our process model was discovered using a sound algorithm" requires:
1. Inductive Miner (not heuristic miner) as the discovery algorithm
2. Output typed as `Admitted<ProcessTree, InductiveMinerPaper>`
3. ProcessTree → WF-Net conversion producing `WfNetConst<Sound>`
4. Receipt chain showing the full lineage

Without Inductive Miner, this claim is **unattested**.

---

## Action Items

| Priority | Action | Owner |
|---|---|---|
| P0 | Implement Inductive Miner consuming `Admitted<OcelLog, Ocel20>` | wasm4pm |
| P0 | Implement `process_tree_to_wf_net` yielding `WfNetConst<Sound>` | wasm4pm |
| P1 | Add `InductiveMinerPaper` witness to wasm4pm-compat `src/witness.rs` | wasm4pm-compat |
| P1 | Add IMf (infrequent) variant for noise handling | wasm4pm |
