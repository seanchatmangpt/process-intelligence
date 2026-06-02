    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/examples/adversary_gap_demo`
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

Automated Verification:

    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/lib.rs (target/debug/deps/c8_adversary-1ee0b47661c530ef)

running 5 tests
test tests::coordinate_system_alpha_is_structural_not_ego ... ok
test tests::graph_tree_contains_relation_break_capability ... ok
test tests::logic_tree_lacks_relation_break_node ... ok
test tests::prophecy_illusion_is_not_omniscience_claim ... ok
test tests::same_market_stream_yields_missing_state_basis ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/c8_bench-e8fe6bc5b45db529)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/c8_core-836997efb9e0c6b5)

running 6 tests
test tests::construct8_len_accepts_eight ... ok
test tests::construct8_len_accepts_zero ... ok
test tests::construct8_len_rejects_nine ... ok
test tests::hot_path_verdict_has_no_string_variant ... ok
test tests::mask_operations ... ok
test tests::need9_is_typed_not_string ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/c8_graph-e6bfd6a93da73ef1)

running 4 tests
test tests::apply_same_delta_twice_is_idempotent ... ok
test tests::ninth_triple_refuses_with_need9 ... ok
test tests::eight_triples_succeed ... ok
test tests::one_triple_sets_one_mask_bit ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/c8_instruments-49d3b3e222828cf5)

running 5 tests
test tests::collider_emits_bounded_construct8_delta ... ok
test tests::collider_finds_hidden_body_when_combined_strength_high ... ok
test tests::detect_liquidity_cliff_from_synthetic_depth_collapse ... ok
test tests::event_horizon_delta_has_at_most_8_updates ... ok
test tests::telescope_detects_relation_break_from_gap ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/c8_market-0eaa04d9147b40a5)

running 5 tests
test tests::planck_cell_emits_construct8_delta_with_max_8_triples ... ok
test tests::relation_break_detected_on_large_gap ... ok
test tests::relation_break_not_detected_on_small_gap ... ok
test tests::tick_alone_is_not_planck_cell ... ok
test tests::wave_phase_state_is_graph_state_not_mysticism ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/c8_receipts-ebd9ba8917db57fd)

running 4 tests
test tests::receipt_hash_changes_with_state ... ok
test tests::receipt_chain_verifies ... ok
test tests::replay_construct8_delta_reproduces_hash ... ok
test tests::tampered_receipt_fails_verification ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/c8_time-ba6d4a3583900dd6)

running 6 tests
test tests::independent_lane_ticks_are_concurrent ... ok
test tests::merge_dominates_both_prior_clocks ... ok
test tests::monotonic_time_never_regresses ... ok
test tests::zero_clocks_are_equal ... ok
test tests::tick_lane_creates_causal_after ... ok
test tests::causal_align_distinguishes_concurrent_from_ordered ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests c8_adversary

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests c8_bench

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests c8_core

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests c8_instruments

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests c8_market

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests c8_receipts

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests c8_time

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

