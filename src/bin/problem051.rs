// Project Euler #51: Prime Digit Replacements
// https://projecteuler.net/problem=51

use std::collections::HashSet;

fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    let mut primes = Vec::new();

    for i in 2..=limit {
        if is_prime[i] {
            primes.push(i);
            for multiple in (i * i..=limit).step_by(i) {
                is_prime[multiple] = false;
            }
        }
    }
    
    primes
}

fn is_prime(n: usize, prime_set: &HashSet<usize>) -> bool {
    prime_set.contains(&n)
}

fn prime_family_size(prime: usize, digit_to_replace: char, prime_set: &HashSet<usize>) -> usize {
    let prime_str = prime.to_string();
    let mut count = 0;

    for replacement_digit in '0'..='9' {
        let candidate_str = prime_str.replace(digit_to_replace, &replacement_digit.to_string());
        
        if candidate_str.starts_with('0') {
            continue;
        }

        if let Ok(candidate) = candidate_str.parse::<usize>() {
            if is_prime(candidate, prime_set) {
                count += 1;
            }
        }
    }
    
    count
}

fn main() {
    let limit = 1_000_000;
    let primes = sieve_of_eratosthenes(limit);
    let prime_set: HashSet<usize> = primes.iter().cloned().collect(); 
    
    for &prime in &primes {
        let prime_str = prime.to_string();
        
        for digit in '0'..='2' {
            let count = prime_family_size(prime, digit, &prime_set);

            if count == 8 {
                println!("The smallest prime with an 8-prime family is {}", prime);
                return;
            }
        }
    }
}
