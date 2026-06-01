# POWL 2.0 ChoiceGraph Extension Analysis

**Paper:** Kourani, H., Park, G., van der Aalst, W.M.P. "Unlocking Non-Block-Structured Decisions: Inductive Mining with Choice Graphs." arXiv:2505.07052, 2025.
**Implementation:** ~/chatmangpt/pm4py/pm4py/objects/powl/choice_graph_discovery.py
**Inductive miner variant:** ~/chatmangpt/pm4py/pm4py/algo/discovery/powl/inductive/variants/im_choice_graph.py
**Object model:** ~/chatmangpt/pm4py/pm4py/objects/powl/obj.py (class DecisionGraph, line 576)

---

## What POWL 2.0 ChoiceGraph Adds

### Core Limitation It Solves

Classical POWL (Kourani & van Zelst, BPM 2023) uses block-structured operators: XOR (exclusive choice), loop, sequence, partial order. These can only model decisions where the process branches and rejoins at the same structural level — block-structured control flow. A large class of real-world processes has non-block-structured decisions: the path chosen at one point determines which activities are even possible later, in ways that cannot be expressed as a simple XOR subtree.

**POWL 2.0 introduces the DecisionGraph (choice graph)** to represent these non-block-structured choices.

### The DecisionGraph Type

```python
class DecisionGraph(POWL):
    """
    A POWL model defined over a set of nodes (each node is a POWL model)
    together with a binary relation over these nodes, augmented with two
    artificial nodes: a start node and an end node.
    """
    def __init__(self, order: BinaryRelation, start_nodes, end_nodes, empty_path=False):
        self.children      # POWL sub-models (the "parts")
        self.start_nodes   # subset that can be first
        self.end_nodes     # subset that can be last
        self.start         # sentinel StartNode
        self.end           # sentinel EndNode
        self.order         # BinaryRelation over children + sentinels
        self.empty_path    # True iff ⟨⟩ ∈ L(G)
```

Key properties:
- **Acyclic** — no activity group is mutually reachable with another (Definition 5, condition 5)
- **Ordered** — edges encode which groups can directly follow which other groups (condition 1)
- **Start/end annotated** — which groups can start/end a trace (conditions 2 and 3)
- **Optional empty trace** — start→end direct edge (condition 4)

### The mineDG Algorithm (Algorithm 1)

`mine_dg(log: EventLog) -> List[Set[str]]` is the core partition algorithm:

```python
def mine_dg(log):
    activities = get_activities(log)
    dfg = compute_dfg_relation(log)          # directly-follows pairs
    dfg_tc = compute_transitive_closure(dfg)  # transitive closure

    # Initialize: each activity in its own part
    parts = {activity: {activity} for activity in activities}

    # Merge parts for mutually reachable activities
    for a1 in activities:
        for a2 in activities:
            if (a1, a2) in dfg_tc and (a2, a1) in dfg_tc:
                merged = part_a1 | part_a2
                ...

    return unique_parts  # List[Set[str]]
```

The key insight: two activities must be in the same partition part if they are mutually reachable via the directly-follows transitive closure. This ensures acyclicity of the resulting choice graph. If the algorithm returns a single part containing all activities, no valid choice graph cut exists.

### ChoiceGraphCut and Validity (Definition 4 and 5)

A valid `ChoiceGraphCut(parts, choice_graph)` requires:
1. `(Ai →DFG Aj ∧ Ai ≠ Aj) ⟺ (Ai, Aj) ∈ E` — edges mirror DFG transitions between parts
2. `Ai ∩ L▷ ≠ ∅ ⟺ (▷, Ai) ∈ E` — start activities determine start edges
3. `Ai ∩ L□ ≠ ∅ ⟺ (Ai, □) ∈ E` — end activities determine end edges
4. `⟨⟩ ∈ L ⟺ (▷, □) ∈ E` — empty trace determines direct start→end edge
5. `(Ai →+ Aj ∧ Aj →+ Ai) ⇒ Ai = Aj` — acyclicity (enforced by mineDG merge step)

Given a valid partition, the choice graph is uniquely determined by conditions 1–4.

### InductiveMinerChoiceGraph — Discovery Variant

`InductiveMinerChoiceGraph` extends `IMBasePOWL` to:
1. First try to detect valid choice graph cuts using `mine_dg` and `is_valid_choice_graph_cut`
2. Fall back to classical partial order cuts (StrictPartialOrder)
3. Apply base cases for trivial logs (single activity, empty, etc.)
4. Use flower-model fall-through when no cut is found

This is registered as a POWL discovery variant alongside `MAXIMAL`, `DYNAMIC_CLUSTERING`, `IM_BRUTE_FORCE`, `IM_TREE`.

---

## DSPy LLM Integration

The fork adds `pm4py.algo.dspy.powl.natural_language.PowlPredictor` — a DSPy 2.x module that generates POWL model strings from natural language process descriptions, using Groq `gpt-oss-20b`:

```python
class POWLGenerationSignature(dspy.Signature):
    task: str = dspy.InputField(desc="Natural language process description")
    domain: str = dspy.InputField(desc="Process domain")
    powl_model: str = dspy.OutputField(desc="Valid POWL model string")
    explanation: str = dspy.OutputField(desc="Explanation of structure")
```

This is exposed in the WASM bridge as `get_demos_for_domain(domain)` — few-shot examples for LLM-guided POWL generation. Supported domains: loan_approval, finance, software_release, it, devops, ecommerce, retail, manufacturing, production, healthcare, medical.

The WASM bridge also exposes `generate_code_from_powl(model_str, target)` for the inverse direction: POWL → executable code (n8n JSON, Temporal Go, Camunda BPMN, YAWL v6 XML).

---

## Implications for wasm4pm-compat powl.rs

### Current State of src/powl.rs

`src/powl.rs` currently exposes:
- `TreeProjectable` — sealed trait for models that admit tree projection
- `assert_tree_projectable` — compile-time assertion helper

This covers block-structured POWL (original POWL 1.x: XOR, loop, sequence, partial order). It does NOT cover `DecisionGraph`.

### What Must Be Added for POWL 2.0 Conformance

**1. DecisionGraph type surface**

wasm4pm-compat needs a `DecisionGraph` analog that:
- Carries its acyclicity invariant in the type system (not just runtime checks)
- Uses the existing `BinaryRelation`-like structure from `src/petri.rs`
- Has a non-forgeable construction path (analogous to `Admit::admit()` for evidence)

Proposed type sketch:
```rust
/// A non-block-structured choice graph over POWL sub-models.
/// Invariant: the ordering relation is a strict partial order (irreflexive + acyclic).
/// Construct only via DecisionGraph::from_valid_cut() — the only public path.
pub struct DecisionGraph<W: Witness> {
    parts: Vec<PowlNode>,
    order: StrictPartialOrderWitness<W>,  // non-forgeable
    start_parts: BitSet,
    end_parts: BitSet,
    empty_path: bool,
    _witness: PhantomData<W>,
}
```

**2. ChoiceGraphCut as an admitted type**

The cut validation (Definition 5 conditions 1–5) must be expressed as an admission surface:
```rust
impl Admit<ChoiceGraphCandidate, DecisionGraphCutLaw> for DecisionGraphAdmitter {
    fn admit(candidate: Evidence<ChoiceGraphCandidate, Raw, W>)
        -> Result<Admission<DecisionGraph<W>, W>, Refusal<DecisionGraphCutLaw, W>>;
}
```

Where `DecisionGraphCutLaw` names the specific violation (acyclicity, start-node requirement, end-node requirement, empty-trace mismatch).

**3. Witness for the arXiv paper**

The `src/witness.rs` module should add a witness for the Kourani-Park-van-der-Aalst 2025 paper:
```rust
/// Witness for: Kourani, Park, van der Aalst (2025) arXiv:2505.07052.
/// "Unlocking Non-Block-Structured Decisions: Inductive Mining with Choice Graphs"
pub enum ChoiceGraphPaper2025 {}
impl Witness for ChoiceGraphPaper2025 {
    const KEY: &'static str = "KOURANI_PARK_VDAALST_2025";
    const TITLE: &'static str = "Inductive Mining with Choice Graphs";
    const YEAR: u32 = 2025;
    const FAMILY: WitnessFamily = WitnessFamily::ProcessDiscovery;
}
```

**4. TreeProjectable extension**

POWL models containing `DecisionGraph` nodes are NOT tree-projectable in general (this is the point of the paper — they express non-block-structured behavior). The `TreeProjectable` seal should explicitly exclude `DecisionGraph`:
- `assert_tree_projectable` should fail at compile time if the model type includes `DecisionGraph`
- A separate `assert_choice_graph_model` predicate should be added for models that may contain `DecisionGraph` nodes

**5. mineDG algorithm does NOT belong in wasm4pm-compat**

`mine_dg` is engine logic — it takes an event log and produces a partition. It belongs in `wasm4pm`. wasm4pm-compat's job is only to define the type surface that `mine_dg` writes its output into: `ChoiceGraphCut`, `DecisionGraph<W>`, `DecisionGraphCutLaw`.

---

## Gap Summary

| POWL 2.0 Feature | PM4Py Status | WASM Bridge | wasm4pm-compat Status |
|---|---|---|---|
| `DecisionGraph` type | Implemented in `obj.py` | Not exposed (opaque PowlModel) | MISSING — needs type surface |
| `mine_dg` algorithm | Implemented in `choice_graph_discovery.py` | Not exposed | CORRECTLY ABSENT — engine logic |
| `ChoiceGraphCut` validation | `is_valid_choice_graph_cut()` | Not exposed | MISSING — needs admission surface |
| Acyclicity invariant in types | Runtime `Exception` raise | Not typed | MISSING — needs const-generic enforcement |
| `ChoiceGraphPaper2025` witness | Not applicable (Python) | Not applicable | MISSING — needs witness enum variant |
| `InductiveMinerChoiceGraph` variant | `im_choice_graph.py` | Not exposed | CORRECTLY ABSENT — engine logic |
| DSPy LLM → POWL | `natural_language.py` | `get_demos_for_domain()` | CORRECTLY ABSENT — non-deterministic, no receipt path |

The DecisionGraph type surface is the one structural addition wasm4pm-compat needs to achieve POWL 2.0 paper conformance. Everything else is either engine logic (correctly absent) or LLM tooling (cannot carry a named law receipt).
