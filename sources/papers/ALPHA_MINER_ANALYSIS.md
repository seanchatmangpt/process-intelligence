# Alpha Miner — Deep Formal Analysis

**Analyst:** Dr. OCEL Specialist (AGI)
**Date:** 2026-05-31
**Source:** van der Aalst, Weijters, Maruster — "Workflow Mining: Discovering Process Models from Event Logs" (IEEE TKDE, 2004)

---

## Formal Objects

### DirectlyFollowsRelation
- `>: Activity × Activity → Bool` — `a > b` iff `b` directly follows `a` in at least one trace
- Computed by scanning consecutive pairs in all traces
- Foundation for all derived relations

### CausalRelation (→)
- `a → b` iff `a > b` and NOT `b > a`
- Semantic: `b` always follows `a`, `a` never follows `b` in the log
- Represents a causal dependency from `a` to `b`

### ParallelRelation (∥)
- `a ∥ b` iff `a > b` AND `b > a`
- Semantic: `a` and `b` appear in both orders — evidence of concurrent execution
- Caveat: also triggered by short loops — Alpha Miner conflates loops with parallelism

### XorRelation (#)
- `a # b` iff NOT `a > b` AND NOT `b > a`
- Semantic: `a` and `b` never directly follow each other — evidence of exclusive choice
- Caveat: also triggered by infrequent paths not observed in the log

### Place Construction (Alpha Algorithm Steps)
1. Identify `T_I` (start activities) and `T_O` (end activities)
2. For each pair of activity sets `(A, B)` where all `a ∈ A` have causal relation to all `b ∈ B`, and no two activities in `A` are causally related, and no two in `B` are causally related — create a place `p(A,B)`
3. Remove dominated places (subset pairs)
4. Connect: `a → p(A,B) → b` for all `a ∈ A`, `b ∈ B`
5. Add source place `i` → all `t ∈ T_I`; all `t ∈ T_O` → sink place `o`

### AlphaMinerOutput
- `net: PetriNet` — the discovered Petri net (a WF-net candidate)
- `initial_marking: Marking`
- `final_marking: Marking`

---

## Key Insight: Alpha Miner is Historically Important but Unsound

The Alpha Miner is the original process discovery algorithm and the conceptual foundation for the field. However, it has documented, irremediable limitations:

### Limitation 1: Does Not Handle Noise
- The alpha miner uses ALL directly-follows relations, even those from erroneous or exceptional traces
- A single erroneous trace can add a spurious causal relation and corrupt the model
- No filtering or frequency threshold is applied
- **Consequence:** Real-world logs produce incorrect models

### Limitation 2: Cannot Handle Invisible Transitions
- If the log contains activities that only occur silently (τ-transitions), alpha miner cannot detect them
- Loops requiring silent entry/exit transitions are misrepresented

### Limitation 3: Short Loop Problem
- `a → b → a` patterns are incorrectly interpreted as `a ∥ b` (parallelism)
- Alpha++ and Alpha# variants partially address this but are not the canonical algorithm

### Limitation 4: Output is Not Guaranteed Sound
- The constructed Petri net may not be a sound WF-net
- Dead transitions and livelocks are possible
- **Critical:** Any board claim about "sound process models" discovered by Alpha Miner is unattested

### Historical Value
Despite its limitations, Alpha Miner established:
- The directly-follows relation as the fundamental abstraction
- The causal/parallel/xor relation framework (basis for all subsequent miners)
- The place-construction algorithm as a constructive proof method

---

## wasm4pm Coverage Assessment

### wasm4pm-algos
`wasm4pm-algos/src/alpha.rs` — Alpha Miner implementation is **present** in wasm4pm.

| Alpha Miner Concept | wasm4pm-algos | Coverage |
|---|---|---|
| DirectlyFollowsRelation | `alpha.rs` | Present |
| Causal/Parallel/Xor relations | `alpha.rs` | Present |
| Place construction algorithm | `alpha.rs` | Present |
| Petri net output | `alpha.rs` | Present |
| Soundness validation | Unknown — needs audit | Unknown |
| Noise handling | Not present (by design) | None |
| Short-loop correction | Not present | None |

**Soundness Limitation Flag:** The wasm4pm Alpha Miner implementation produces a Petri net but does not verify soundness. The output **must not** be typed as `WfNetConst<Sound>` without an explicit soundness check. If it is currently typed as sound, this is a **type law defect**.

### wasm4pm-compat
`src/causal_net.rs` — CausalNet output shapes are present.

| Concept | wasm4pm-compat | Coverage |
|---|---|---|
| CausalNet shape | `src/causal_net.rs` | Full |
| DFG shape | `src/dfg.rs` | Full |
| WfNet shape (sound/unsound) | `src/petri.rs` — `WfNetConst<SOUNDNESS>` | Full |

---

## PM4Py Coverage Assessment

| Capability | PM4Py Module | Maturity |
|---|---|---|
| Alpha Miner | `pm4py.discovery.discover_petri_net_alpha` | Mature (reference implementation) |
| Alpha++ | `pm4py.discovery.discover_petri_net_alpha_plus` | Available |
| Alpha# (short loops) | Not in mainline | Research only |

---

## Soundness Limitation — Required Action

The wasm4pm Alpha Miner output must be documented and typed accurately:
1. Output type should be `WfNetConst<Unsound>` or an unparameterized `PetriNet`
2. If `WfNetConst<Sound>` is desired, a soundness checker must be applied after Alpha Miner
3. The soundness checker (WF-net reachability analysis) is a separate capability gap

Any API that returns `WfNetConst<Sound>` from Alpha Miner without an explicit soundness check is a **type lie** and a conformance defect under the wasm4pm-compat type law.

---

## Action Items

| Priority | Action | Owner |
|---|---|---|
| P0 | Audit `wasm4pm-algos/src/alpha.rs` — verify output is NOT typed as `WfNetConst<Sound>` | wasm4pm |
| P0 | Add soundness limitation to all Alpha Miner documentation and rustdoc | wasm4pm |
| P1 | Implement WF-net soundness checker (separate from discovery) | wasm4pm |
| P1 | Add noise filtering variant (frequency threshold) | wasm4pm |
| P2 | Consider deprecating Alpha Miner as primary discovery in favor of Inductive Miner | wasm4pm |
