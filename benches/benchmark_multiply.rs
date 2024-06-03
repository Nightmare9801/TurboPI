use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;


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

#[inline]
pub fn russian_peasant(mut x: u128, mut y: u128) -> u128 {
    if y == 1 {
        return x;
    }
    if x==1 {
        return y;
    }
    if x==0 || y==0 {
        return 0;
    }
    let mut z: u128 = 0;
    while y>0 {
        if (y&1)!=0 {
            z+= x;
        }
        x= x<<1;
        y= y>>1;
    }
    return z;
}

pub fn karatsuba(a: u128, b: u128)-> u128 {
    if a < 340282366920938463463374607431768211455 || b < 340282366920938463463374607431768211455 {
        return russian_peasant(a, b);
    }
    let n = u128::max(get_no_of_digits(a), get_no_of_digits(b));
    let n2 = (n / 2) + (n % 2);

    let a_high = a / 10u128.pow(n2 as u32);
    let a_low = a % 10u128.pow(n2 as u32);
    let b_high = b / 10u128.pow(n2 as u32);
    let b_low = b % 10u128.pow(n2 as u32);

    let z0 = multiply(a_low, b_low);
    let z2 = multiply(a_high, b_high);

    let z1 = multiply(a_high + a_low, b_high + b_low) - z0 - z2;

    return z2 * 10u128.pow(n2 as u32 * 2) + z1 * 10u128.pow(n2 as u32) + z0;
}

//#[inline]
pub fn multiply(a: u128, b: u128) -> u128 {
    if a<18446744073709551615 || b<18446744073709551615{
        return russian_peasant(a, b);
    }
    return karatsuba(a, b);
}

pub fn get10n(n: u128)-> u128 {
    let mut x= 1;
    for _i in 0..n{
        x*=10;
    }
    return x;
}

pub fn get2n(n: u16)-> u128{
    let mut x= 1;
    for _i in 0..n{
        x*=2;
    }
    return x;
}


fn bench_multiply(c: &mut Criterion) {
    //c.bench_function("Multiply Benchmark 1", |b| b.iter(|| multiply(100000000, 10000000)));
    println!("{}",get2n(512)-1);
    c.bench_function("Multiply Benchmark", |b| b.iter(|| black_box(multiply(45156682, 9223853))));
}

// Create a benchmark group
criterion_group!(benches, bench_multiply);
//criterion_group!(benches, bench_digits_new);

// Run the benchmarks
criterion_main!(benches);