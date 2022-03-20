use std::rc::Rc;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use libchordr::prelude::SearchIndex;
use libchordr::test_helpers::get_test_catalog;

fn criterion_benchmark(c: &mut Criterion) {
    let catalog = Rc::new(get_test_catalog());
    c.bench_with_input(
        BenchmarkId::new("Search songs in a catalog", "catalog"),
        &catalog,
        |b, c| {
            let index = SearchIndex::build_for_catalog(c.clone());
            b.iter(|| {
                index.search_by_term(black_box("Toni"));
            })
        },
    );

    let catalog = Rc::new(get_test_catalog());
    c.bench_with_input(
        BenchmarkId::new("Build search-index for catalog", "catalog"),
        &catalog,
        |b, c| {
            b.iter(|| {
                SearchIndex::build_for_catalog(black_box(c.clone()));
            })
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
