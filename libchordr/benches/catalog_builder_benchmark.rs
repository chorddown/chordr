use criterion::{criterion_group, criterion_main, Criterion};
use libchordr::prelude::{CatalogBuilder, FileType};
use std::path::Path;

fn criterion_benchmark(c: &mut Criterion) {
    let songs_dir = format!("{}/tests/resources", env!("CARGO_MANIFEST_DIR"));
    let songs_dir = Path::new(&songs_dir);

    c.bench_function("Build catalog", |b| {
        b.iter(|| {
            CatalogBuilder::new().build_catalog_for_directory(songs_dir, FileType::Chorddown, false)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
