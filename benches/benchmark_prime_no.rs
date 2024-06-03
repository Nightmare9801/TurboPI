use criterion::{criterion_group, criterion_main, Criterion};
use std::vec::Vec;

#[inline]
pub fn get_no_of_digits(x: u128) -> u128 {
    let mut p: u128= 10;
    for i in 0..39{
        if x<p {
            return i+1;
        }
        p*=10;
    }
    return 39;
}

pub fn multiply(x: u128, y: u128)-> u128 {
    return x*y;
}

pub fn get_prime_nos(lim: u128)-> Vec<u128>{
    let mut prime_nos: Vec<u128> = Vec::new();
    prime_nos.push(1);
    for i in 2..lim{
            prime_nos.push(i);
    }
    for i in 2..lim {
        if(i*i)<lim{
            prime_nos.retain(|&u| u%i != 0|| u == i);
        }else{
            break;
        }
    }
    return prime_nos;
}


fn bench_prime_nos(c: &mut Criterion) {
    c.bench_function("Prime No Benchmark", |b| b.iter(|| get_prime_nos(1000000)));
}

// Create a benchmark group
criterion_group!(benches, bench_prime_nos);

// Run the benchmarks
criterion_main!(benches);