use criterion::{criterion_group, criterion_main, Criterion};
mod multiply;

fn bench_digits_old(c: &mut Criterion) {
    c.bench_function("add_numbers_benchmark", |b| b.iter(|| multiply::get_no_of_digits(10, 20)));
}

fn bench_digits_new(c: &mut Criterion) {
    c.bench_function("add_numbers_benchmark", |b| b.iter(|| multiply::get_no_of_digits_new(10, 20)));
}

// Create a benchmark group
criterion_group!(benches, bench_digits_old);
criterion_group!(benches, bench_digits_new);

// Run the benchmarks
criterion_main!(benches);