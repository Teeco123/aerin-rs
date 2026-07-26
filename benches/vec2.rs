use aerin_rs::math::vec2::Vec2;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn vec2_benchmark(c: &mut Criterion) {
    let a = Vec2::new(10.5, 20.25);
    let b = Vec2::new(5.0, 4.75);

    let mut group = c.benchmark_group("Vec2 Addition");

    group.bench_function("Vec2 +", |bench| {
        bench.iter(|| black_box(a + b));
    });

    group.bench_function("Vec2 add", |bench| {
        bench.iter(|| black_box(a.add(b)));
    });

    group.bench_function("Vec2 +=", |bench| {
        bench.iter(|| {
            let mut lhs = a;
            lhs += b;
            black_box(lhs)
        });
    });

    group.bench_function("Vec2 add_assign", |bench| {
        bench.iter(|| {
            let mut lhs = a;
            lhs.add_assign(b);
            black_box(lhs)
        });
    });

    group.finish();
}

criterion_group!(benches, vec2_benchmark);
criterion_main!(benches);
