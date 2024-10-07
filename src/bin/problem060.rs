// Problem #60: Prime Pair Sets
// https://projecteuler.net/problem=60

use itertools::Itertools; 
use std::collections::HashSet;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn concatenation_is_prime(p1: u64, p2: u64) -> bool {
    let concat1 = format!("{}{}", p1, p2);
    let concat2 = format!("{}{}", p2, p1);
    is_prime(concat1.parse::<u64>().unwrap()) && is_prime(concat2.parse::<u64>().unwrap())
}

fn main() {
    let limit = 10000; 
    let primes: Vec<u64> = (2..limit).filter(|&n| is_prime(n)).collect();
    let prime_set: HashSet<u64> = primes.iter().cloned().collect();
    let mut min_sum = u64::MAX;

    for comb in primes.iter().combinations(5) {
        let mut valid_set = true;

        for (i, &p1) in comb.iter().enumerate() {
            for &p2 in &comb[i + 1..] {
                if !concatenation_is_prime(*p1, *p2) {
                    valid_set = false;
                    break;
                }
            }
            if !valid_set {
                break;
            }
        }

        if valid_set {
            let sum: u64 = comb.iter().map(|&&p| p).sum();
            if sum < min_sum {
                min_sum = sum;
            }
        }
    }

    println!("The minimum sum of a set of five primes is {}", min_sum);
}
