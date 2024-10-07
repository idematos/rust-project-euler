// Problem #50: Consecutive Prime Sum
// https://projecteuler.net/problem=50

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

fn main() {
    let limit = 1_000_000;
    let primes = sieve_of_eratosthenes(limit);
    let prime_set: HashSet<usize> = primes.iter().cloned().collect(); 
    
    let mut max_prime = 0;
    let mut max_length = 0;

    for i in 0..primes.len() {
        let mut sum = 0;
        
        for j in i..primes.len() {
            sum += primes[j];
            
            if sum >= limit {
                break; 
            }

            let length = j - i + 1;
            
            if prime_set.contains(&sum) && length > max_length {
                max_length = length;
                max_prime = sum;
            }
        }
    }

    println!("The prime below one million that can be written as the sum of the most consecutive primes is {}", max_prime);
}
