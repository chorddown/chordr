use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use libchordr::prelude::{CatalogBuilder, Converter, FileType, Format, Formatting, Node};
use libchordr::test_helpers::{get_test_ast, get_test_metadata};

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
