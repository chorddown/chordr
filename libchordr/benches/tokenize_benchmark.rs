use criterion::{black_box, criterion_group, criterion_main, Criterion};

use libchordr::prelude::{build_tokenizer, Tokenizer};
use libchordr::test_helpers::get_test_tokens;

fn tokenize(content: &str) -> () {
    let token_lines = build_tokenizer().tokenize(content);
    assert_eq!(token_lines, get_test_tokens());
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
