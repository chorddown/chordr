use criterion::{black_box, criterion_group, criterion_main, Criterion};

use libchordr::models::chord::TransposableTrait;
use libchordr::test_helpers::get_test_ast;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Transpose Swing Low Sweet Chariot", |b| {
        b.iter(|| get_test_ast().transpose(black_box(2)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
