use criterion::{black_box, criterion_group, criterion_main, Criterion};

use libchordr::prelude::{build_tokenizer, Tokenizer};

fn tokenize(content: &str) -> () {
    let _ = build_tokenizer().tokenize(content.as_bytes()).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Tokenize Swing Low Sweet Chariot", |b| {
        b.iter(|| {
            tokenize(black_box(include_str!(
                "../tests/resources/swing_low_sweet_chariot.chorddown"
            )))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
