# AGENT 9 — Receipts, Replay, and Boundary Proof

## Mission

Implement cryptographic receipt validation, transition hash linking using BLAKE3,
and deterministic delta replay verification for the Construct8 market physics engine.

## Files Inspected

- `crates/c8-receipts/Cargo.toml`
- `crates/c8-receipts/src/lib.rs`
- `crates/c8-graph/src/lib.rs` (TripleRef, Construct8Delta, GraphField API)
- `crates/c8-core/src/lib.rs` (Construct8Mask public field layout)

## Files Created/Updated

- `crates/c8-receipts/Cargo.toml` — workspace deps: c8-core (path), c8-graph (path), blake3, serde, serde_json
- `crates/c8-receipts/src/lib.rs` — full implementation
- `crates/c8-bench/Cargo.toml` — added missing `[[bench]]` declaration (unblocked workspace parse)
- `docs/agents/AGENT_09_RECEIPTS.md` — this file

## Public API

### Types

```rust
pub type ReceiptHash = [u8; 32];
```

### C8Receipt

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C8Receipt {
    pub pre_state_hash: u64,
    pub delta_mask: u8,
    pub delta_len: u8,
    pub post_state_hash: u64,
    pub causal_time: u64,
    pub module_version: u32,
    pub receipt_hash: ReceiptHash,
}
```

- `new(pre_state_hash, delta, post_state_hash, causal_time) -> Self`
  BLAKE3 over: pre_state_hash.le + [delta_mask, delta_len] + post_state_hash.le + causal_time.le + module_version.le
- `verify(&self) -> bool` — recompute BLAKE3, compare against stored receipt_hash

### ReceiptChain

```rust
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReceiptChain { pub receipts: Vec<C8Receipt> }
```

- `new() -> Self`
- `append(&mut self, receipt: C8Receipt)`
- `len(&self) -> usize`
- `is_empty(&self) -> bool`
- `verify_all(&self) -> bool`
- `last_hash(&self) -> Option<&ReceiptHash>`

### Free functions

- `replay_construct8_delta(field: &mut GraphField, delta: &Construct8Delta, expected_post_hash: u64) -> bool`
- `write_implementation_receipt(path: &str, chain: &ReceiptChain) -> std::io::Result<()>`

## Implementation Notes

- `Construct8Delta::mask()` returns `Construct8Mask` — inner `u8` accessed via `.0` (public field).
- `Construct8Delta::len()` returns `u8` directly.
- `GraphField::apply_construct8` is infallible (returns `GraphApplyResult`, not `Result<>`).
- Workspace dep syntax used for blake3, serde, serde_json rather than pinned inline versions.
- `c8-bench/Cargo.toml` was missing its `[[bench]]` section, which caused workspace manifest
  parse failure and blocked `cargo check -p c8-receipts`; fixed as a side effect.

## Tests

| Test | Assertion |
|---|---|
| `receipt_hash_changes_with_state` | Two receipts with different post_state_hash have distinct receipt_hash |
| `receipt_chain_verifies` | Append 2 receipts; verify_all() == true |
| `tampered_receipt_fails_verification` | Mutate post_state_hash after construction; verify() == false |
| `replay_construct8_delta_reproduces_hash` | Apply delta, capture hash, replay on fresh field returns true |

## Cargo Check Result

**PASS** — `cargo check -p c8-receipts` exits 0 with `Finished dev profile`.

## Verdict

**ALIVE** — Cryptographic receipt hashing, chain verification, and deterministic replay
modules compile cleanly and all four required tests are present.
