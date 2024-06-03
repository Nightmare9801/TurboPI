use std::str::FromStr;
use bigdecimal::BigDecimal;
use criterion::{criterion_group, criterion_main, Criterion};
use num_bigint::{BigInt, BigUint};

#[inline]
fn compute() -> BigDecimal {
    let c= BigDecimal::from(426880)* BigDecimal::from_str(&((10005.0 as f64).sqrt()).to_string()).unwrap();
    let x: BigInt= BigInt::from(-262537412640768000 as i64);
    let mut s: BigDecimal= BigDecimal::from(1);
    
    for i in 0u128..8 {
        //println!("i: {}",i);
        let l= BigDecimal::from((545140134)*BigDecimal::from(i) + BigDecimal::from(13591409));
        let xi = BigDecimal::from(pow(x.clone(), i));
        let m= factorial(BigUint::from(6*i))/(factorial(BigUint::from(3*i))*factorial(BigUint::from(i)));
        //println!("{}", xi);
        s+= ((BigDecimal::from_str(&m.to_string())).unwrap()*l) / xi ;
    }
    
    let pi: BigDecimal = c / (s-1);
    pi
}

#[inline]
fn pow(x: BigInt, y: u128)->BigInt {
    let mut returner: BigInt= BigInt::from(1 as u128);
    for _i in 0..y{
        returner*= x.clone();
    }
    returner
}

#[inline]
fn factorial(k: BigUint) -> BigUint {
    let one= BigUint::from(1 as u128);
    if k <= one {
        one
    } else {
        k.clone() * factorial(k.clone() - one)
    }
}



fn bench_prime_nos(c: &mut Criterion) {
    println!("{}", compute());
    c.bench_function("Prime No Benchmark", |b| b.iter(|| compute()));
}

// Create a benchmark group
criterion_group!(benches, bench_prime_nos);

// Run the benchmarks
criterion_main!(benches);