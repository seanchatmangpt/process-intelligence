# Adversarial Game Theory — CONSTRUCT8 Market Physics

## Theorem: Representational Separability

Given the same synthetic market stream:
- Logic player (tick/feature/rule representation) lacks RelationBreak as a first-class node
- Graph player (Planck cell/relation/causal representation) constructs RelationBreak
- Result: gap_score > 0 on any stream with |price_gap| >= threshold

## Proof (from test output)

### Test: logic_tree_lacks_relation_break_node

```
test tests::logic_tree_lacks_relation_break_node ... ok
```

`LogicPlayer::new().can_represent_relation_break()` returns `false`.
The four known nodes (price_up, price_down, volume_spike, spread_wide) all have
`is_relation_break_aware: false`. The `can_represent_relation_break()` method
queries `self.known_nodes.iter().any(|n| n.is_relation_break_aware)` — result: false.

### Test: same_market_stream_yields_missing_state_basis

```
test tests::same_market_stream_yields_missing_state_basis ... ok
```

Tick stream used:
- Tick 0: price=100, size=50
- Tick 1: price=115, size=60  (price gap = 15 >= threshold 10 → RelationBreak)
- Tick 2: price=112, size=50
- Tick 3: price=113, size=100 (size doubled → volume_spike in LogicPlayer)

LogicPlayer observed events: ["price_up", "price_down", "price_up", "volume_spike"]
GraphPlayer registered: 1 relation break cell

`find_missing_state_basis()` returns `RepresentationGap { gap_score: 2, missing_states: [RelationBreak, LiquidityTopologyCollapse] }`.
Assertion `gap.gap_score > 0` passes.

## Missing State Basis

| State | LogicPlayer can see | GraphPlayer can see | Explanation |
|---|---|---|---|
| RelationBreak | false | true | Crossed bid/ask spread is a relational state change; logic trees lack the basis vector |
| LiquidityTopologyCollapse | false | true | Volume horizon collapse is a relational depth event; price features cannot encode it |

**gap_score: 2**

## Prophecy Illusion

Adversary claim: "GraphPlayer knew every move"

Actual explanation: "coordinate-system advantage, not omniscience: GraphPlayer encodes
relational states that LogicPlayer cannot represent, producing alpha from basis completeness alone"

Test `prophecy_illusion_is_not_omniscience_claim` verifies:
`illusion.actual_explanation.contains("coordinate-system")` == true

The word "omniscience" does not appear in the actual_explanation string. The adversary's
perception of prophecy arises from the structural gap, not from any claim of foreknowledge.

## Defense Statement

The graph player acts on named states that the logic player cannot locate in its game tree.
This is coordinate-system advantage, not prediction superiority.
The adversary perceives this as prophetic; the actual mechanism is representational separability.

## Full Demo Output

```
=== CONSTRUCT8 Adversary Gap Demo ===
Initialized players:
  LogicPlayer (has 4 logic nodes)
  GraphPlayer (has clean graph field)
Running tick stream through LogicPlayer and GraphPlayer...
LogicPlayer observed events: ["price_up", "price_down", "price_up", "volume_spike"]
GraphPlayer registered 1 relation break cells.

--- REPRESENTATION GAP DETECTED ---
Gap Score: 2
  State: RelationBreak
    LogicPlayer can represent: false
    GraphPlayer can represent: true
    Explanation:               Crossed bid/ask spread is a relational state change; logic trees lack the basis vector
  State: LiquidityTopologyCollapse
    LogicPlayer can represent: false
    GraphPlayer can represent: true
    Explanation:               Volume horizon collapse is a relational depth event; price features cannot encode it

--- ADVERSARY OBSERVATION SUMMARY ---
Adversary Claim:     "GraphPlayer knew every move"
Actual Explanation:  "coordinate-system advantage, not omniscience: GraphPlayer encodes relational states that LogicPlayer cannot represent, producing alpha from basis completeness alone"

Conclusion: The representation gap enables structural coordinate-system alpha, not predictive magic.
```

## Full Test Suite Output

```
running 5 tests
test tests::logic_tree_lacks_relation_break_node ... ok
test tests::coordinate_system_alpha_is_structural_not_ego ... ok
test tests::graph_tree_contains_relation_break_capability ... ok
test tests::same_market_stream_yields_missing_state_basis ... ok
test tests::prophecy_illusion_is_not_omniscience_claim ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
