// Problem #35: Circular Primes
// https://projecteuler.net/problem=35

use std::collections::HashSet;

fn sieve_of_eratosthenes(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    for p in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[p] {
            for multiple in (p * p..=limit).step_by(p) {
                is_prime[multiple] = false;
            }
        }
    }
    
    is_prime
}

fn rotations(n: usize) -> Vec<usize> {
    let digits = n.to_string();
    let mut rotations = Vec::new();
    
    for i in 0..digits.len() {
        let rotation = digits[i..].to_string() + &digits[..i];
        rotations.push(rotation.parse::<usize>().unwrap());
    }
    
    rotations
}

fn is_circular_prime(n: usize, is_prime: &Vec<bool>) -> bool {
    for rotation in rotations(n) {
        if !is_prime[rotation] {
            return false;
        }
    }
  
    true
}

fn main() {
    let limit = 1_000_000;
    let is_prime = sieve_of_eratosthenes(limit);
    
    let mut circular_primes = HashSet::new();
    
    for num in 2..limit {
        if is_prime[num] && is_circular_prime(num, &is_prime) {
            circular_primes.insert(num);
        }
    }
    
    println!("The number of circular primes below one million is {}", circular_primes.len());
}
