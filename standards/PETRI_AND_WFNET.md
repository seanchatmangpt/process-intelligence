# Petri Nets and WF-nets — Formal Foundations

**Authority:** Petri net theory (Petri 1962); WF-net soundness (van der Aalst 1997, 2011)
**Witness key:** `wf-net-soundness` — `WitnessFamily::Paper`

---

## Petri Nets

A Petri net N = (P, T, F) is a bipartite directed graph:

- **P** — a finite set of **places** (circles in graphical notation; represent conditions or states).
- **T** — a finite set of **transitions** (rectangles; represent events or activities).
- **F ⊆ (P × T) ∪ (T × P)** — a set of directed **arcs** between places and transitions.
- **M: P → ℕ** — a **marking** assigns token counts to places; represents current system state.

A transition t is **enabled** in marking M if every input place of t holds at least one
token. Firing t removes one token from each input place and adds one token to each output
place, producing a new marking M'.

---

## WF-nets (Workflow Nets)

A **WF-net** (van der Aalst 1997) is a Petri net with:

- Exactly one **source place** `i` (no incoming arcs).
- Exactly one **sink place** `o` (no outgoing arcs).
- Every place and transition is on a path from `i` to `o`.

### WF-net Soundness

A WF-net is **sound** if and only if it satisfies three properties:

1. **Option to complete** — from any reachable marking M, the sink marking [o] is
   reachable.
2. **Proper completion** — when [o] is reached, no other place holds a token (no dangling
   tokens).
3. **No dead transitions** — every transition t can be fired in at least one reachable
   marking (no unreachable activity).

Soundness is a non-trivial decidable property. It requires reachability analysis — a
polynomial-time algorithm for free-choice WF-nets, EXPSPACE-hard in the general case.

---

## wasm4pm-compat Implementation (petri.rs)

`src/petri.rs` models the Petri net / WF-net / OC-Petri-net **shapes** with soundness as
a typestate claim, never a computed proof:

### Key types

- `PetriNet` — set of `Place`s and `Transition`s joined by `Arc`s with a `Marking`.
- `WfNet<SOUNDNESS>` — a `PetriNet` with a declared source and sink, parameterized on a
  soundness typestate token.
- `ObjectCentricPetriNet` — arcs typed by object type and may be variable (per
  Van der Aalst OC-Petri-net formalism).

### Soundness typestate tokens (from `law.rs`)

```
SoundnessUnknown    — default; no claim made.
SoundnessClaimed    — asserted by a human or upstream system; unproven here.
SoundnessWitnessed  — carries a witness obtained from wasm4pm, re-attached here.
```

These are empty enums — `PhantomData` type parameters, zero-cost, never constructed. The
compiler prevents `WfNet<SoundnessWitnessed>` from being constructed without a witness
obtained from the `wasm4pm` engine.

### `WfNetConst<SOUNDNESS>`

The nightly foundry module (`nightly_foundry.rs`) provides `WfNetConst<SOUNDNESS>` where
`SOUNDNESS` is a `ConstParamTy` value. This seals soundness at compile time using
`generic_const_exprs` and `adt_const_params`: a `WfNetConst<{ SoundnessState::Witnessed
}>` is a distinct type from `WfNetConst<{ SoundnessState::Unknown }>` and the two cannot
be confused in function signatures.

### Structural validation

- `PetriNet::validate()` — arcs reference declared nodes; IDs unique.
- `WfNet::validate()` — structural shape: source/sink declared, basic graph consistency.
- The `wasm4pm` engine implements the full dynamic verification of soundness, boundedness, and liveness in [petri.rs](file:///Users/sac/process-intelligence/sources/wasm4pm/src/petri.rs).

---

## Dynamic Verification Algorithms in [petri.rs](file:///Users/sac/process-intelligence/sources/wasm4pm/src/petri.rs)

The `wasm4pm` engine evaluates the following dynamic properties via state space exploration:

### 1. Reachability and Coverability Analysis
- **Algorithm**: Depth-First Search (DFS) state space traversal starts from the initial marking $M_0 = [i]$.
- **Termination**: Ensured by checking coverability. If a marking $M_{curr}$ is reached that covers an ancestor marking $M_{anc}$ (i.e., $M_{curr} \ge M_{anc}$ and $M_{curr} \neq M_{anc}$), the state space is unbounded, and the traversal prunes the branch.
- **State Limit**: Traversal is capped at 100,000 states to prevent state space explosion.

### 2. 1-Boundedness (Safeness) Check
- **Verification**: A Petri net is 1-bounded if no reachable marking contains more than one token in any place.
- **Coverability Check**: If any marking covers an ancestor, it implies unboundedness, violating 1-boundedness.
- **Asserted Invariant**: $M(p) \le 1$ for all places $p$ and all reachable markings $M$.

### 3. Deadlock Detection
- **Definition**: A deadlock is a reachable marking $M$ with no enabled transitions, where $M \neq [o]$ (the sink marking).
- **Verification**: The reachability graph is scanned to verify that all non-sink markings have at least one enabled transition.

### 4. Dead-Transition Detection
- **Definition**: A transition $t$ is dead if it cannot be fired from any reachable marking.
- **Verification**: The analysis accumulates all transitions fired during reachability exploration. Any transition not in this set is flagged as dead.

### 5. Siphon-Trap Properties
- **Siphon**: A subset of places $S$ such that $\bullet S \subseteq S\bullet$. Once empty of tokens, a siphon can never obtain a token again.
- **Trap**: A subset of places $T$ such that $T\bullet \subseteq \bullet T$. Once marked with at least one token, a trap can never become completely empty.
- **Commoner's Theorem**: In a free-choice Petri net, liveness is equivalent to every siphon containing a marked trap.
- **Implementation**: The engine provides `PetriNet::is_siphon`, `PetriNet::is_trap`, `PetriNet::find_siphons`, `PetriNet::find_traps`, and `PetriNet::check_siphon_trap_property` to calculate and check these invariants.

---

## What wasm4pm Provides

| Capability | Module |
|---|---|
| Soundness analysis (reachability graph construction) | [petri.rs](file:///Users/sac/process-intelligence/sources/wasm4pm/src/petri.rs) |
| Token-game replay (alignment, fitness) | `wasm4pm` |
| Boundedness / safeness verification | [petri.rs](file:///Users/sac/process-intelligence/sources/wasm4pm/src/petri.rs) |
| Siphon-trap property checking | [petri.rs](file:///Users/sac/process-intelligence/sources/wasm4pm/src/petri.rs) |
| OC-Petri-net discovery from OCEL | `wasm4pm` |
| WF-net → `SoundnessWitnessed` upgrade (returning a witness) | `wasm4pm` |

---

## Board Placement

WF-net soundness is the mathematical guarantee that a process model will always complete correctly. By sealing soundness as a typestate at the type level, wasm4pm-compat makes it impossible to accidentally use an unsound process model in a safety-critical graduation path. The only legitimate route to a `SoundnessWitnessed` WF-net is through the verification engine in [petri.rs](file:///Users/sac/process-intelligence/sources/wasm4pm/src/petri.rs) — no short-circuit exists.
