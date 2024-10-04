// Project Euler #46: Goldbach's Other Conjecture
// https://projecteuler.net/problem=46

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

fn can_be_written_as_prime_and_twice_square(n: usize, primes: &HashSet<usize>) -> bool {
    for prime in primes.iter() {
        if *prime >= n {
            break;
        }
        let remainder = n - prime;
        let square = (remainder / 2) as f64;
        let square_root = square.sqrt();
        if square_root == square_root.floor() {
            return true; 
        }
    }
  
    false 
}

fn main() {
    let limit = 10000; 
    let primes = sieve_of_eratosthenes(limit);
    let prime_set: HashSet<usize> = primes.iter().cloned().collect();
    
    for n in (9..=limit).step_by(2) {
        if !prime_set.contains(&n) { 
            if !can_be_written_as_prime_and_twice_square(n, &prime_set) {
                println!("The smallest odd composite that cannot be written as the sum of a prime and twice a square is {}", n);
                break; 
            }
        }
    }
}
