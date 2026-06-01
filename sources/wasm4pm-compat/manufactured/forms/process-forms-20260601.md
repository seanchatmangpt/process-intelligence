# Process Forms: Type-Law Definition of Lawful Process Structures

**Authority:** wasm4pm-compat Type-Law Atlas
**Generated:** 2026-06-01
**Purpose:** Authoritative definitions of lawful process structures in wasm4pm-compat

---

## Overview

A **Process Form** is a type-law definition of a lawful process structure. Each form specifies:
- **Structure Constraint**: The algebraic or syntactic rule governing structure validity
- **Sound Marking**: The definition of a valid terminal state
- **Admissible States**: The number of distinct valid intermediate states
- **Witness Marker**: The algebraic proof structure required for this form

All process forms enforce strict type safety at the process intelligence boundary.

---

## Workflow Net (WF-net)

**Type-Law Authority:** Van der Aalst 1989
**Structure Constraint:** `1-bounded-net`
**Sound Marking:** `unique-sink`
**Admissible States:** 4

### Definition

A **Workflow Net (WF-net)** is a Petri net where:
1. All places are 1-bounded (at most one token per place at any marking)
2. A unique sink place $o$ exists; all transitions must eventually fire tokens to $o$
3. No transition can fire from $o$; no place before $o$ has external input

### Soundness Property

A WF-net is **sound** if:
- For every possible firing sequence from the initial marking, all tokens eventually reach the sink $o$
- No transition is dead (every transition can fire in some execution)
- No tokens remain in non-sink places after $o$ reaches its terminal marking

### Witness Structure

The witness for WF-net conformance is a **Petri Net Marking**:
```
Marking = {place_id → {0, 1}}  // Each place is 1-bounded
```

---

## BPMN 2.0 Gateway

**Type-Law Authority:** Van der Aalst 1998
**Structure Constraint:** `token-routing`
**Sound Marking:** `exclusive-or-and-synchronization`
**Admissible States:** 7

### Definition

A **BPMN Gateway** is a control flow element that:
1. Routes tokens according to logical rules (AND, XOR, OR)
2. Maintains proper token conservation across join/split transitions
3. Enforces synchronization semantics for AND-join/split
4. Enforces mutual exclusivity for XOR-join/split

### Gateway Types

- **AND-Join**: Synchronizes all incoming tokens; fires when ALL incoming edges have tokens
- **XOR-Join**: Exclusive choice; fires when EXACTLY ONE incoming edge has a token
- **OR-Join**: Complex; fires based on quorum analysis (non-local in BPMN 2.0)

### Witness Structure

The witness for gateway conformance is a **Token Routing State**:
```
TokenState = {edge_id → {Present, Absent}}
```

---

## Object-Centric Event Log (OCEL)

**Type-Law Authority:** Van der Aalst 2016
**Structure Constraint:** `object-instance-mapping`
**Sound Marking:** `complete-event-trace`
**Admissible States:** 5

### Definition

An **Object-Centric Event Log (OCEL)** is a log where:
1. Events are linked to **object instances** (business objects), not just process instances
2. Each event may affect multiple objects (multi-instance tasks)
3. An object instance has a complete lifecycle: created → modified* → terminated
4. Events are timestamped and causally ordered within each object's lifecycle

### Completeness Property

An OCEL is **complete** if:
- Every object instance has a creation event and a termination event
- Every event references at least one valid object instance
- Causality is consistent across object instances (no backward time travel)

### Witness Structure

The witness for OCEL conformance is an **Object Mapping**:
```
OcelMapping = {object_id → {event_id*}}  // Objects map to their event sequences
```

---

## Process Tree

**Type-Law Authority:** Leemans 2013
**Structure Constraint:** `block-structured`
**Sound Marking:** `root-completion`
**Admissible States:** 6

### Definition

A **Process Tree** is a hierarchical process model where:
1. The model is a binary tree with a single root operator
2. Operators are: Sequence (→), Choice (×), Parallel (∧), Loop (← →)
3. Leaves are activities; all internal nodes are operators
4. The tree is **block-structured**: nesting respects proper syntactic boundaries

### Soundness Property

A Process Tree is **sound** if:
- The root operator can always complete execution from any reachable state
- No activity is unreachable from the root
- No activity can lead to a deadlock (except intentional choice branches)

### Witness Structure

The witness for Process Tree conformance is a **Tree Node Marking**:
```
TreeMarking = {node_id → ExecutionState}
ExecutionState = {NotStarted, Running, Completed}
```

---

## Process-Log Alignment

**Type-Law Authority:** Adriansyah 2011
**Structure Constraint:** `traceback-moves`
**Sound Marking:** `fitness-threshold`
**Admissible States:** 3

### Definition

An **Alignment** is a sequence of moves that relates a process model to an event log:
1. **Synchronous Move**: Model transition and log event co-occur
2. **Model Move**: Model transition fires without corresponding log event (invisible transition)
3. **Log Move**: Log event occurs without model transition (non-conforming)

### Fitness Threshold

Alignment fitness is measured as:
$$\text{fitness} = 1 - \frac{\text{cost of alignment}}{\text{ideal cost}}$$

**Board Admissibility** ($\theta_{\text{fit}} \geq 0.95$): Automatic receipt generation
**Conditional Admissibility** ($0.85 \leq \theta_{\text{fit}} < 0.95$): Requires validator signature
**Non-Admissible** ($\theta_{\text{fit}} < 0.85$): Receipt generation forbidden

### Witness Structure

The witness for alignment conformance is an **Alignment Cost Matrix**:
```
AlignmentCost = {move_type → cost}
move_type ∈ {SynchronousMove(0), ModelMove(1), LogMove(1)}
```

---

## Summary Table

| Form | Authority | Constraint | Terminal Marking | States | Witness Marker |
|------|-----------|-----------|------------------|--------|---|
| Workflow Net | van der Aalst 1989 | 1-bounded | Unique sink | 4 | PetriNetMarking |
| BPMN Gateway | van der Aalst 1998 | Token routing | AND/XOR sync | 7 | TokenRoutingState |
| OCEL | van der Aalst 2016 | Object mapping | Complete trace | 5 | ObjectMapping |
| Process Tree | Leemans 2013 | Block-structured | Root completion | 6 | TreeNodeMarking |
| Alignment | Adriansyah 2011 | Traceback moves | Fitness ≥ 0.95 | 3 | AlignmentCostMatrix |

---

## Non-Forgeable Type Enforcement

Each process form is enforced via Rust type invariants:

```rust
// 1-Boundedness Invariant (WF-net)
pub struct Marking {
    places: HashMap<PlaceId, u32>,
    // invariant: ∀p, places[p] ≤ 1
}

// Terminal Marking Requirement
pub struct SoundMarking {
    sink_place: PlaceId,
    final_token: u32,
    // invariant: token ≥ 1 at sink
}

// Object Instance Completeness (OCEL)
pub struct OcelObject {
    object_id: ObjectId,
    creation_event: EventId,
    termination_event: EventId,
    // invariant: creation_time < termination_time
}

// Tree Node Soundness (Process Tree)
pub struct TreeNode {
    operator: Operator,
    children: Vec<TreeNode>,
    // invariant: valid syntactic nesting
}

// Fitness Threshold (Alignment)
pub struct Fitness {
    value: f64,
    threshold: f64,
    // invariant: 0.0 ≤ value ≤ 1.0
}
```

All type enforcement is **non-forgeable** at compile-time via Rust's type system.

---

## See Also

- [Witness Markers](./witnesses/witness-markers.md) — Algebraic witness structures
- [Boundary Law](./boundaries/boundary-law.wit) — Type-safe WASM boundaries
- [Type-Law Atlas](../sources/wasm4pm-compat/type-law-atlas.md) — Complete formalism reference
