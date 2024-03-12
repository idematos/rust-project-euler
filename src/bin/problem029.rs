// Problem #29: Distinct Powers
// https://projecteuler.net/problem=29

use std::collections::HashSet;
use num_bigint::BigInt;
use num_traits::FromPrimitive;

fn distinct_terms(n: u32) -> usize {

    let mut distinct_terms: HashSet<BigInt> = HashSet::new();
    for a in 2..=n {
        for b in 2..=n {
            let a: BigInt = FromPrimitive::from_u32(a).unwrap();
            distinct_terms.insert(num_traits::pow(a, b as usize));
        }
    }


    distinct_terms.len()
}

fn main() {
    let n = 100;

    println!("\nThere are {} distinct terms in the sequence.", distinct_terms(n));
}