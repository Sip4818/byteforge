use criterion::{criterion_group, criterion_main};

fn benchmarks(_c: &mut criterion::Criterion) {}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
