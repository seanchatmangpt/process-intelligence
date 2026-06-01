# Crosswalk: Process Tree to WF-net — Lossless Projection

## Why This Direction Is Lossless

A process tree is block-structured by construction. Every node is one of:
sequence (`→`), exclusive choice (`×`), parallel (`∧`), loop (`↺`), or silent
(`τ`). These operators map uniquely to WF-net sub-net patterns. The mapping is
a theorem, not an approximation: van der Aalst & Weijters (2004), Leemans (2022).

The reverse is not lossless: WF-nets can express routing structures (non-block-
structured, implicit places, OR-joins) that have no process tree equivalent.

## The Canonical Mapping

| Process tree operator | WF-net sub-net | Lossless? |
|---|---|:---:|
| `Sequence(A, B, …)` | Places in series: `p₀ →[A]→ p₁ →[B]→ p₂ →…` | Yes |
| `Xor(A, B)` | Split place + two transitions + join place | Yes |
| `Parallel(A, B)` | AND-split transition + sub-nets in parallel + AND-join | Yes |
| `Loop(do, redo)` | Do-body + back-arc via redo-body (Leemans loop law: exactly 2 children) | Yes |
| `Silent` | Silent (tau) transition with no label | Yes |
| `Activity leaf "a"` | Labeled transition `t_a` | Yes |

Every operator has a unique, invertible WF-net sub-net pattern. The resulting
WF-net is guaranteed:
- Block-structured.
- Sound (option to complete, proper completion, no dead transitions) — because
  each operator pattern is individually sound and composition of sound blocks
  is sound.
- One source place, one sink place.

## No LossPolicy Required

Because the projection is lossless, `LossPolicy::RefuseLoss` is the correct
and default policy. There is nothing to report. No `LossReport` is produced.
The projection either succeeds (returning `WfNet<SoundnessClaimed>`) or refuses
for a structural reason (e.g. a `Loop` node with wrong arity).

```rust
// Correct: no LossPolicy argument needed for this direction
let wfnet: WfNet<SoundnessClaimed> = project_process_tree_to_wfnet(admitted_tree)?;
// Returns Ok(WfNet) or Err(Refusal<ProcessTreeToWfNetRefusal, W>)
```

The resulting WF-net carries `SoundnessClaimed` (not `SoundnessWitnessed`)
because this crate does not run soundness checking. The claim is grounded in
the block-structure theorem; engine-level verification graduates to `wasm4pm`.

## The Inductive Miner Connection

The Inductive Miner (PM4Py, and future wasm4pm) produces process trees as output.
The canonical pipeline for conformance checking via alignment is:

```
EventLog
  → [Inductive Miner] → ProcessTree
  → [Process Tree → WF-net, lossless] → WfNet<SoundnessClaimed>
  → [wasm4pm soundness check] → WfNet<SoundnessWitnessed>
  → [wasm4pm alignment] → ConformanceVerdict
```

wasm4pm-compat carries the `ProcessTree` and `WfNet` shapes at each step.
The lossless projection ensures the WF-net faithfully represents the discovered
process tree — no approximation is introduced between discovery and conformance.

## Loop Arity: The Leemans Law

wasm4pm-compat enforces the Leemans loop law at compile time:

```rust
pub struct TypedLoopNode<const ARITY: usize>
where
    Require<{ ARITY == 2 }>: IsTrue,
```

A `TypedLoopNode<3>` does not compile. A loop in a process tree has exactly
two children: the do-body (executed once per iteration) and the redo-body
(executed zero or more times before re-entering the do-body). This is the
Leemans (2022) definition. The compile-time check makes it impossible to
build a malformed loop node.

## Comparison to Other Directions

| Direction | Lossless? | LossPolicy needed? | Notes |
|---|:---:|:---:|---|
| Process Tree → WF-net | **Yes** | No | Canonical lossless direction |
| WF-net → Process Tree | No | Yes | Only block-structured WF-nets project; others refuse |
| Process Tree → POWL | Yes | No | POWL is a strict superset; tree is embeddable |
| POWL → Process Tree | No | Yes | Partial orders exceeding block structure are refused |
| BPMN → WF-net | Partial | Yes | OR-gateways and data objects cause loss |
| WF-net → POWL | Partial | Yes | Non-block-structured WF-nets cause loss |

## PM4Py Comparison

PM4Py performs the process tree → Petri net projection internally as part of
the Inductive Miner pipeline. The projection is implicit — there is no
`process_tree_to_petri_net(tree)` public API. The user calls
`pm4py.discover_petri_net_inductive(log)` and receives a Petri net directly,
with the process tree as an intermediate value that is not exposed.

wasm4pm-compat makes the intermediate `ProcessTree` shape a first-class value
that can be inspected, admitted, and explicitly projected to a WF-net with
a declared soundness claim. The intermediate step is not hidden.

## Summary

Process Tree → WF-net is the one direction in the process model crosswalk
that is universally lossless. No `AllowLossWithReport` is needed. No structural
information is dropped. The Leemans loop arity law is enforced at compile time
by `TypedLoopNode<ARITY>` with `Require<{ ARITY == 2 }>: IsTrue`. The
resulting WF-net carries a `SoundnessClaimed` typestate token grounded in the
block-structure theorem.
