# AGENT_03_CONSTRUCT8_DELTA — c8-graph Delta Engine

## Mission

Implement the CONSTRUCT8 Delta Engine in `crates/c8-graph`. This crate provides
a fixed-size, mask-driven triple delta structure and an in-memory packed relation
store (`GraphField`) for zero-allocation hot-path graph application.

## Crate

- **Name:** `c8-graph`
- **Path:** `crates/c8-graph`
- **Dependency:** `c8-core` (path = `../c8-core`)

## Types Implemented

### `TripleRef`

```rust
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TripleRef { pub subject: u32, pub predicate: u32, pub object: u32 }
```

Flat u32 triple encoding. No newtype wrappers in the hot path.

### `Construct8Delta`

Fixed 8-slot delta with `Construct8Mask`-based occupancy tracking.

| Field | Type | Purpose |
|---|---|---|
| `triples` | `[TripleRef; 8]` | Fixed slot array — no Vec |
| `valid_mask` | `Construct8Mask` | Which slots are occupied |
| `emit_mask` | `Construct8Mask` | Which slots are emitted on apply |
| `len` | `u8` | Count of occupied slots |

Key methods:
- `empty() -> Self` — zero-initialized delta
- `len(&self) -> u8` — occupied slot count
- `is_empty(&self) -> bool`
- `mask(&self) -> Construct8Mask` — returns `valid_mask`
- `as_fixed_slots(&self) -> &[TripleRef; 8]` — raw slot access
- `push_checked(&mut self, triple: TripleRef) -> C8Result<()>` — returns `Err(C8Error::Need9)` when at capacity

### `Construct8DeltaBuilder`

Consuming builder pattern. `push(self, triple) -> C8Result<Self>` and `build(self) -> Construct8Delta`.

### `BranchlessApplyStats`

```rust
pub struct BranchlessApplyStats {
    pub lanes_applied: u32,
    pub lanes_skipped: u32,
    pub idempotent: bool,
}
```

### `GraphApplyResult`

```rust
pub struct GraphApplyResult {
    pub verdict: HotPathVerdict,
    pub stats: BranchlessApplyStats,
    pub new_state_hash: u64,
}
```

### `GraphField`

In-memory store: `HashMap<(u32, u32, u32), bool>` keyed by `(subject, predicate, object)`.

Key methods:
- `new() -> Self`
- `relation_count(&self) -> usize`
- `contains_relation(&self, s, p, o: u32) -> bool`
- `state_hash(&self) -> u64`
- `apply_construct8(&mut self, delta: &Construct8Delta) -> GraphApplyResult`

## Hot Path Contract

`apply_construct8` MUST:
1. Iterate over all 8 fixed slots using `mask.has(lane)` — no Vec in hot path
2. Update `state_hash` via `wrapping_mul` for each new triple inserted
3. Set `stats.idempotent = true` if no new triples were applied

## Tests

| Test | Assertion |
|---|---|
| `empty_delta_has_len_zero_and_empty_mask` | `len==0`, `mask==EMPTY` |
| `one_triple_sets_one_mask_bit` | lane 0 set, count==1 |
| `eight_triples_succeed` | `mask==FULL` |
| `ninth_triple_refuses_with_need9` | returns `Err(C8Error::Need9)` |
| `apply_same_delta_twice_is_idempotent` | second apply: `lanes_applied==0`, `idempotent==true` |
| `state_hash_changes_after_lawful_apply` | hash differs from initial |
| `state_hash_unchanged_after_idempotent_reapply` | hash stable on re-apply |

## Cargo Check Result

```
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
```

Status: PASS
