use criterion::{black_box, criterion_group, criterion_main, Criterion};

use libchordr::prelude::{Converter, Format, Formatting, Node};
use libchordr::test_helpers::{get_test_ast, get_test_metadata};

fn convert_to_chorddown(node: Node) -> () {
    let converter = Converter::get_converter(Format::Chorddown);
    assert!(converter
        .convert(&node, &get_test_metadata(), Formatting::default())
        .is_ok());
}

fn convert_to_html(node: Node) -> () {
    let converter = Converter::get_converter(Format::HTML);
    assert!(converter
        .convert(&node, &get_test_metadata(), Formatting::default())
        .is_ok());
}

fn convert_to_text(node: Node) -> () {
    let converter = Converter::get_converter(Format::Text);
    assert!(converter
        .convert(&node, &get_test_metadata(), Formatting::default())
        .is_ok());
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Convert Swing Low Sweet Chariot to Chorddown", |b| {
        b.iter(|| convert_to_chorddown(black_box(get_test_ast())))
    });

    c.bench_function("Convert Swing Low Sweet Chariot to HTML", |b| {
        b.iter(|| convert_to_html(black_box(get_test_ast())))
    });

    c.bench_function("Convert Swing Low Sweet Chariot to Text", |b| {
        b.iter(|| convert_to_text(black_box(get_test_ast())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
