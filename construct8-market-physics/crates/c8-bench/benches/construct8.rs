use c8_core::{InstrumentId, VenueId};
use c8_graph::{Construct8Delta, Construct8Triple, GraphField};
use c8_market::{MarketPlanckCell, MarketRelationKind, TickObservation};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn make_delta(n: usize) -> Construct8Delta {
    let mut delta = Construct8Delta::empty();
    for i in 0..n {
        delta
            .push_checked(Construct8Triple {
                subject: c8_core::NodeId(i as u64),
                predicate: c8_core::RelationId(1),
                object: c8_core::NodeId(2),
            })
            .expect("delta capacity exceeded 8");
    }
    delta
}

fn bench_construct8_apply(c: &mut Criterion) {
    let mut group = c.benchmark_group("branchless_mask");
    for n in [1usize, 2, 4, 8] {
        let delta = make_delta(n);
        group.bench_with_input(
            BenchmarkId::new("branchless_mask", n),
            &delta,
            |b, delta| {
                b.iter(|| {
                    let mut field = GraphField::new();
                    let _ = field.apply_construct8(delta);
                });
            },
        );
    }
    group.finish();
}

fn bench_market_planck_cell_emit(c: &mut Criterion) {
    let _prev = TickObservation::new(1, 10, 100, 50, 99, 101, 10, 10, 1000);
    let curr = TickObservation::new(1, 10, 105, 60, 101, 106, 10, 10, 1001);
    let _ = (InstrumentId(1), VenueId(10)); // assert imports resolve

    c.bench_function("market_planck_cell_emit", |b| {
        b.iter(|| {
            let cell = MarketPlanckCell::from_tick_relation(
                &curr,
                MarketRelationKind::RelationBreak,
                12345,
                curr.timestamp,
            );
            let _delta = cell.to_construct8_delta();
        });
    });
}

criterion_group!(
    benches,
    bench_construct8_apply,
    bench_market_planck_cell_emit
);
criterion_main!(benches);
