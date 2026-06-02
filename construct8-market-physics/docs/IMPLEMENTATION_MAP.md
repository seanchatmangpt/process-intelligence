# CONSTRUCT8 Market Physics — Implementation Map

| Primitive | Meaning | Target crate | Key types | Tests |
|---|---|---|---|---|
| Knowledge Hook | (predicate,guard,action) triple; admission membrane | c8-core | HotPathVerdict, C8Error, Construct8Len | Need9 refusal |
| CONSTRUCT8 | Fixed 8-lane bounded graph delta | c8-graph | Construct8Delta, GraphField | 8-slot max |
| Need9 | Typed decomposition signal | c8-core | C8Error::NeedNine | 9th refused |
| MarketPlanckCell | Smallest relational market-state unit | c8-market | MarketPlanckCell, MarketRelationKind | not-a-tick |
| VectorClock8 | 8-lane causal time | c8-time | VectorClock8, VectorClockCompare | concurrent detection |
| MonotonicStamp | Never-regressing time | c8-time | MonotonicStamp | no regression |
| MarketTelescope | Observations to Planck cells | c8-instruments | MarketTelescope | depth collapse |
| EventHorizonTelescope | Liquidity disappearance | c8-instruments | EventHorizonBoundary | emits delta |
| MarketCollider | Hypothesis collision | c8-instruments | ColliderHypothesis | bounded output |
| LogicPlayer | Sees ticks/rules only | c8-adversary | LogicPlayer | missing RelationBreak |
| GraphPlayer | Sees Planck cells/relations | c8-adversary | GraphPlayer, RepresentationGap | finds hidden state |
| C8Receipt | BLAKE3 hash of delta proof | c8-receipts | C8Receipt, ReceiptChain | tamper detection |

## AKA Lifecycle Pipeline

1. TickObservation -> MarketTelescope -> MarketPlanckCell
2. MarketPlanckCell -> to_construct8_delta -> Construct8Delta
3. Construct8Delta + GraphField -> apply_construct8 -> GraphApplyResult
4. GraphApplyResult + causal_time -> C8Receipt
5. C8Receipt -> ReceiptChain::append

## Source Authority

Doctrine source: `/Users/sac/process-intelligence/phd-thesis/research/knowledge-hooks/KNOWLEDGE_HOOKS_AND_AKA_DOCTRINE.md`

Key doctrine laws (sourced verbatim from Truex MANIFESTO):

```
No hook, no consequence.
No receipt, no authority.
No replay, no substrate.
No accounting, no promotion.
```

## Crate Inventory

Workspace root: `/Users/sac/process-intelligence/construct8-market-physics/Cargo.toml`

| Crate | Path | Status |
|---|---|---|
| c8-core | crates/c8-core | exists — HotPathVerdict, C8Error::NeedNine, Construct8Len, Construct8Mask |
| c8-graph | crates/c8-graph | exists — Construct8Delta, GraphField, GraphApplyResult, Construct8Refusal::NeedNine |
| c8-market | crates/c8-market | exists — MarketPlanckCell, MarketRelationKind, TickObservation |
| c8-time | crates/c8-time | exists — VectorClock8, VectorClockCompare, MonotonicStamp |
| c8-instruments | crates/c8-instruments | exists — MarketTelescope, EventHorizonBoundary, MarketCollider, ColliderHypothesis |
| c8-adversary | crates/c8-adversary | exists — LogicPlayer, GraphPlayer, RepresentationGap, MissingStateBasis |
| c8-receipts | crates/c8-receipts | exists — C8Receipt, ReceiptChain, ReplayVerdict |
| c8-bench | crates/c8-bench | exists — benchmark harness |
