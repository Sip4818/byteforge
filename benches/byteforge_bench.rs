use byteforge::tokenizer::BpeTokenizer;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn benchmarks(c: &mut Criterion) {
    let corpus = "Hello Hello, Rust is fast and memory safe. ".repeat(1000);

    // Train once, outside the encoding/decoding benchmarks.
    let tokenizer = BpeTokenizer::train(&corpus, 300);

    // Encode once, outside the decoding benchmark.
    let tokens = tokenizer.encode(&corpus);

    c.bench_function("training", |b| {
        b.iter(|| BpeTokenizer::train(black_box(&corpus), 300))
    });

    c.bench_function("encoding", |b| {
        b.iter(|| tokenizer.encode(black_box(&corpus)))
    });

    c.bench_function("decoding", |b| {
        b.iter(|| tokenizer.decode(black_box(&tokens)))
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
