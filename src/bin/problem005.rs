// Problem #5: Smallest multiple
// https://projecteuler.net/problem=5

use std::collections::HashMap;

fn get_prime_factors(n: u64) -> HashMap<u64, u64> {
    let mut factors = HashMap::new();
    let mut x = n;
    let mut i = 2;
    while i  <= x {
        if x % i == 0 {
            if factors.contains_key(&i) {
                factors.insert(i, factors[&i] + 1);
            } else {
                factors.insert(i, 1);
            }

            x = x / i;
        }
        else {
            i += 1;
        }
    }

    factors
}

fn smallest_multiple(n: u64) -> u64 {
    let mut result_factors = HashMap::new();
    for i in (2..(n + 1)).rev() {
        let factors = get_prime_factors(i);
        for (key, value) in factors {
            if !result_factors.contains_key(&key) || result_factors[&key] < value {
                result_factors.insert(key, value);
            }
        }
    }

    let mut result = 1;
    for (key, value) in result_factors {
        for _ in 1..(value + 1) {
            result *= key;
        }
    }

    result
}

fn main() {
    let n = 20;

    println!("\nThe smallest positive number that is evenly divisible by all of the numbers from 1 to {} is {}.", n, smallest_multiple(n));
}
