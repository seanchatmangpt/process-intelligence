use c8_graph::{Construct8Delta, Construct8Triple, GraphField, NodeId, RelationId};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn populate_graph(node_count: u32) -> GraphField {
    let mut graph = GraphField::new();
    let mut delta = Construct8Delta::empty();

    // Add up to 8 triples from a population of `node_count` nodes
    let triples_to_add = std::cmp::min(8, node_count / 2);
    for i in 0..triples_to_add {
        let triple = Construct8Triple::new(
            NodeId((i % node_count) as u64),
            RelationId(10u32),
            NodeId(((i + 1) % node_count) as u64),
        );
        delta.push_checked(triple).ok();
    }

    graph.apply_construct8(&delta);
    graph
}

fn benchmark_apply_1(c: &mut Criterion) {
    c.bench_function("apply_1_triple_1000_nodes", |b| {
        b.iter(|| {
            let mut graph = black_box(populate_graph(1000));
            let mut delta = Construct8Delta::empty();
            let triple = Construct8Triple::new(NodeId(500u64), RelationId(20u32), NodeId(501u64));
            delta.push_checked(triple).ok();
            graph.apply_construct8(&delta)
        });
    });
}

fn benchmark_apply_2(c: &mut Criterion) {
    c.bench_function("apply_2_triples_1000_nodes", |b| {
        b.iter(|| {
            let mut graph = black_box(populate_graph(1000));
            let mut delta = Construct8Delta::empty();
            delta
                .push_checked(Construct8Triple::new(
                    NodeId(500u64),
                    RelationId(20u32),
                    NodeId(501u64),
                ))
                .ok();
            delta
                .push_checked(Construct8Triple::new(
                    NodeId(502u64),
                    RelationId(20u32),
                    NodeId(503u64),
                ))
                .ok();
            graph.apply_construct8(&delta)
        });
    });
}

fn benchmark_apply_4(c: &mut Criterion) {
    c.bench_function("apply_4_triples_1000_nodes", |b| {
        b.iter(|| {
            let mut graph = black_box(populate_graph(1000));
            let mut delta = Construct8Delta::empty();
            for i in 0u64..4u64 {
                delta
                    .push_checked(Construct8Triple::new(
                        NodeId(500u64 + i),
                        RelationId(20u32),
                        NodeId(501u64 + i),
                    ))
                    .ok();
            }
            graph.apply_construct8(&delta)
        });
    });
}

fn benchmark_apply_8(c: &mut Criterion) {
    c.bench_function("apply_8_triples_1000_nodes", |b| {
        b.iter(|| {
            let mut graph = black_box(populate_graph(1000));
            let mut delta = Construct8Delta::empty();
            for i in 0u64..8u64 {
                delta
                    .push_checked(Construct8Triple::new(
                        NodeId(500u64 + i),
                        RelationId(20u32),
                        NodeId(501u64 + i),
                    ))
                    .ok();
            }
            graph.apply_construct8(&delta)
        });
    });
}

criterion_group!(
    benches,
    benchmark_apply_1,
    benchmark_apply_2,
    benchmark_apply_4,
    benchmark_apply_8
);
criterion_main!(benches);
