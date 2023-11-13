use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchit(c: &mut Criterion) {
	let mut group = c.benchmark_group("func");
	group.bench_function("func1", |b| {
		b.iter(|| true);
	});
	group.bench_function("func2", |b| {
		b.iter(|| true);
	});
}

criterion_group!(benches, benchit);
criterion_main!(benches);
