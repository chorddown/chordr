use criterion::{black_box, criterion_group, criterion_main, Criterion};

use libchordr::prelude::{Parser, ParserTrait, Token};
use libchordr::test_helpers::get_test_tokens;

fn parse(test_tokens: Vec<Token>) -> () {
    let mut parser = Parser::new();
    assert!(parser.parse(test_tokens).is_ok());
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Parse Swing Low Sweet Chariot", |b| {
        b.iter(|| parse(black_box(get_test_tokens())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
