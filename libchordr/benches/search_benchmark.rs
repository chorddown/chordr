use std::rc::Rc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use libchordr::prelude::SearchIndex;
use libchordr::test_helpers::get_test_catalog;

fn criterion_benchmark(c: &mut Criterion) {
    let catalog = Rc::new(get_test_catalog());
    c.bench_function("Search songs in a catalog", |b| {
        let index = SearchIndex::build_for_catalog(catalog.clone());
        b.iter(|| {
            index.search_by_term(black_box("Toni"));
        })
    });

    c.bench_function("Build search-index for catalog", |b| {
        b.iter(|| {
            SearchIndex::build_for_catalog(black_box(catalog.clone()));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
