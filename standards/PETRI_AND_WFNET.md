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
- Neither checks reachability, boundedness, liveness, or soundness — those are `wasm4pm`.

---

## What wasm4pm Must Provide

| Capability | Graduates to |
|---|---|
| Soundness analysis (reachability graph construction) | `wasm4pm` |
| Token-game replay (alignment, fitness) | `wasm4pm` |
| Boundedness / safeness verification | `wasm4pm` |
| OC-Petri-net discovery from OCEL | `wasm4pm` |
| WF-net → `SoundnessWitnessed` upgrade (returning a witness) | `wasm4pm` |

---

## Board Placement

WF-net soundness is the mathematical guarantee that a process model will always complete
correctly. By sealing soundness as a typestate at the type level, wasm4pm-compat makes it
impossible to accidentally use an unsound process model in a safety-critical graduation
path. The only legitimate route to a `SoundnessWitnessed` WF-net is through the `wasm4pm`
engine — no short-circuit exists.
