// Problem #49: Prime Permutations
// https://projecteuler.net/problem=49

use std::collections::HashSet;

fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    let mut primes = Vec::new();

    for i in 2..=limit {
        if is_prime[i] {
            if i >= 1000 { 
                primes.push(i);
            }
            for multiple in (i * i..=limit).step_by(i) {
                is_prime[multiple] = false;
            }
        }
    }
    
    primes
}

fn is_permutation(a: usize, b: usize) -> bool {
    let mut a_digits: Vec<char> = a.to_string().chars().collect();
    let mut b_digits: Vec<char> = b.to_string().chars().collect();
    a_digits.sort();
    b_digits.sort();
    a_digits == b_digits
}

fn main() {
    let primes = sieve_of_eratosthenes(9999); 
    let prime_set: HashSet<usize> = primes.iter().cloned().collect();

    for &prime in &primes {
        for diff in 1.. {
            let prime2 = prime + diff;
            let prime3 = prime + 2 * diff;

            if prime_set.contains(&prime2) && prime_set.contains(&prime3) &&
               is_permutation(prime, prime2) && is_permutation(prime, prime3) {
                if prime != 1487 {
                    let result = format!("{}{}{}", prime, prime2, prime3);
                    println!("The 12-digit number formed by concatenating the sequence is {}", result);
                    return;
                }
            }
        }
    }
}
