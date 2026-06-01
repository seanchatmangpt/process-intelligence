# Crosswalk: WF-net to POWL Structural Projection with Loss Detection

## Structural Relationship

A Workflow Net (WF-net) is a Petri net with a single source place, a single sink
place, and every node on a path from source to sink. POWL (Partially Ordered
Workflow Language) is a tree-structured model where nodes are atoms, partial
orders, exclusive choices, or loops, connected by precedence edges.

The WF-net → POWL direction is **not universally lossless**:
- Block-structured WF-nets have an exact POWL representation.
- Non-block-structured WF-nets (free-choice or general) may not have a POWL
  equivalent without introducing loss.

## Block-Structured Case: No Loss

A block-structured WF-net uses only sequence, XOR-split/join, AND-split/join,
and loop patterns. Each such structural pattern maps exactly to a POWL node type:

| WF-net pattern | POWL node type | Lossless? |
|---|---|:---:|
| Sequence of transitions | `PartialOrder` with total order | Yes |
| XOR-split / XOR-join | `Choice` node | Yes |
| AND-split / AND-join | `PartialOrder` with concurrent children | Yes |
| While-loop (do-redo) | `Loop` node with arity 2 | Yes |
| Single activity | `Atom` | Yes |
| Silent transition (tau) | `Silent` | Yes |

For block-structured WF-nets, the projection is lossless. The `LossPolicy` for
this case is `RefuseLoss` — if a non-block-structured pattern is detected, the
projection must refuse rather than silently drop structure.

## Non-Block-Structured Case: Named Loss

Non-block-structured WF-nets contain patterns that have no POWL equivalent:
- **Duplicate activities**: the same activity label appears on multiple
  transitions (cannot be represented as a single `Atom` node).
- **Implicit places**: routing logic embedded in place structure, not in
  visible split/join patterns.
- **Non-local dependencies**: an arc that skips a block boundary.
- **Or-joins / inclusive choice**: no direct POWL counterpart.

For these, the projection must use `LossPolicy::AllowLossWithReport`:

```rust
use wasm4pm_compat::loss::{LossPolicy, ProjectionName, LossReport};

let policy = LossPolicy::AllowLossWithReport;
// The report must enumerate: duplicate_activities, implicit_places_dropped,
// or_join_approximations, non_local_arc_count.
```

## wasm4pm-compat Enforcement

`src/formats.rs` + `src/powl.rs` + `src/petri.rs` together enforce:

1. A `WfNet` must be admitted (`Admission<WfNet, WfNetSoundnessPaper>`) before
   projection begins. An unadmitted `WfNet` cannot enter the projection path.

2. The projection code must explicitly decide its `LossPolicy` before examining
   the WF-net structure. This is the "policy decided before loss occurs" covenant
   from `src/loss.rs`:
   > A `LossPolicy` decided before loss occurs.

3. If `RefuseLoss` is chosen and a non-block-structured pattern is detected,
   the projection returns a `Refusal<WfNetToPoWlRefusal::NonBlockStructured, W>`.

4. If `AllowLossWithReport` is chosen, a `LossReport` is mandatory. No silent
   structural drop.

## Loss Detection: What to Check

The projection must scan the WF-net for non-block-structured markers before
deciding whether loss will occur:

| Check | How to detect | Loss type if present |
|---|---|---|
| Duplicate activity labels | Two transitions with same label | `DuplicateActivityLabel` |
| Implicit places | Place with no direct split/join pair | `ImplicitPlace` |
| Non-local arc | Arc weight > 1 bypassing a block | `NonLocalArc` |
| Or-join pattern | Merge of AND and XOR routing | `OrJoinApproximated` |
| Soundness unknown | `SoundnessUnknown` typestate token | `SoundnessNotAttested` |

In wasm4pm-compat, `WfNet<SoundnessUnknown>` and `WfNet<SoundnessWitnessed>`
are different types. A projection that requires a soundness witness can only
accept `WfNet<SoundnessWitnessed>` — passing `WfNet<SoundnessUnknown>` is a
compile error.

## POWL Witness Markers After Projection

After a successful projection, the resulting `Powl` node carries witness markers:

- `TreeProjectable` — the POWL node can be further projected to a process tree
  (block-structured, no partial order siblings at the same level).
- `ExceedsProcessTree` — the POWL node contains partial order structure that
  cannot be represented in a block-structured process tree.

These markers are set during the WF-net → POWL projection and prevent
downstream code from silently treating a `TreeProjectable` POWL as
`ExceedsProcessTree` or vice versa.

## PM4Py Comparison

PM4Py has no direct WF-net → POWL projection. PM4Py's POWL discovery
(SHS-Miner) operates directly on event logs. A PM4Py user who discovers a WF-net
(via Alpha or Inductive Miner) and wants POWL must rediscover from the log —
there is no `wf_net_to_powl(net)` function.

wasm4pm-compat names this projection as a first-class contract with an explicit
loss policy. The conversion is not a re-discovery; it is a structural crosswalk
with declared loss.

## Summary

| Scenario | LossPolicy | Result |
|---|---|---|
| Block-structured WF-net | `RefuseLoss` | Lossless `Powl` with `TreeProjectable` witnesses |
| Block-structured WF-net with partial order | `RefuseLoss` | Lossless `Powl` with `ExceedsProcessTree` where needed |
| Non-block-structured WF-net | `AllowLossWithReport` | `Powl` + `LossReport` enumerating structural drops |
| Non-block-structured, loss refused | `RefuseLoss` | `Refusal<NonBlockStructured, W>` — no silent drop |
