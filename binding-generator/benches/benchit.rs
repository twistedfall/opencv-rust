use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchit(c: &mut Criterion) {
	let mut group = c.benchmark_group("compare");
	group.bench_function("reference", |b| b.iter(reference));
	group.bench_function("new", |b| b.iter(new));
	group.finish()
}

criterion_group!(benches, benchit);
criterion_main!(benches);

fn reference() {
	black_box(true);
}

fn new() {
	black_box(false);
}
