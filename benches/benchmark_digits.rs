use criterion::{criterion_group, criterion_main, Criterion};


#[inline]
pub fn get_no_of_digits(x: u128) -> u128 {
    let digits = (x as f64).log10() as u128 + 1;
    return digits;//*/
    /*
    let mut p: u128= 10;
    for i in 0..39{
        if x<p {
            return i+1;
        }
        p*=10;
    }
    return 39;*/
}

fn bench_digits(c: &mut Criterion) {
    c.bench_function("Digits Benchmark 1: ", |b| b.iter(|| get_no_of_digits(100000000)));

    c.bench_function("Digits Benchmark 2: ", |b| b.iter(|| get_no_of_digits(10000000000000000000000000000000000000)));
}

// Create a benchmark group
criterion_group!(benches, bench_digits);
//criterion_group!(benches, bench_digits_new);

// Run the benchmarks
criterion_main!(benches);