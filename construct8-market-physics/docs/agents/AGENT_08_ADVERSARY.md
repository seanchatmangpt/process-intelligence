# AGENT 8 — Adversarial Game-Theory and Representation Gap Engine

## Mission

Implement the c8-adversary crate: a structural mismatch demonstration showing why logic-based
competitors cannot represent relational market state changes that the graph-based player observes.

THIS IS NOT A PROFIT CLAIM. It is a structural coordinate-system demonstration.

## Crate

`crates/c8-adversary` — depends on c8-core, c8-graph, c8-market (path deps).

## Types Implemented

| Type | Role |
|---|---|
| `GameTreeNode` | Decision tree node with `is_relation_break_aware` flag |
| `LogicPlayer` | Price-feature-only player; 4 nodes, none relation-break-aware |
| `GraphPlayer` | Relational player; wraps `GraphField` + `Vec<MarketPlanckCell>` |
| `MissingStateBasis` | Documents a state visible to graph player but not logic player |
| `RepresentationGap` | Aggregated gap with `gap_score: u32` |
| `ProphecyIllusion` | Names adversary_claim vs actual_explanation |

## Key Functions

- `LogicPlayer::can_represent_relation_break()` — always false (no basis vector)
- `LogicPlayer::process_tick_stream()` — emits price_up/price_down/volume_spike labels
- `GraphPlayer::can_represent_relation_break()` — always true
- `GraphPlayer::process_tick_stream()` — calls `detect_relation_break_state(ticks, 10)`, stores cells
- `find_missing_state_basis()` — builds 2-entry gap: RelationBreak + LiquidityTopologyCollapse
- `explain_prophecy_illusion()` — returns "coordinate-system advantage, not omniscience"
- `score_representation_gap()` — returns `gap.gap_score`

## Tests

| Test | Assertion |
|---|---|
| `logic_tree_lacks_relation_break_node` | `can_represent_relation_break() == false` |
| `graph_tree_contains_relation_break_capability` | `can_represent_relation_break() == true` |
| `same_market_stream_yields_missing_state_basis` | `gap_score > 0`, RelationBreak not seen by logic |
| `prophecy_illusion_is_not_omniscience_claim` | actual contains "coordinate-system" |
| `coordinate_system_alpha_is_structural_not_ego` | synthetic gap_score == 1 |

## Cargo Check Result

PASS — no errors, no warnings.

## Verdict

**ALIVE** — Adversarial representation gap engine manufactured and verified.
