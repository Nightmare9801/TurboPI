use std::{io, str::FromStr};
use bigdecimal::BigDecimal;
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

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    {
    println!("{}", compute());
    }
    let elapsed = now.elapsed();
    println!("Time Taken: {:.2?}", elapsed);
    println!("Enter any character to exit");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
        }
        Err(error) => println!("error: {error}"),
    }
}
