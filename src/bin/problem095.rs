// Problem #95: Amicable Chains
// https://projecteuler.net/problem=95

use std::collections::HashMap;

fn sum_of_proper_divisors(n: usize) -> usize {
    let mut sum = 1; 
    let limit = (n as f64).sqrt() as usize;

    for i in 2..=limit {
        if n % i == 0 {
            sum += i;
            let other_divisor = n / i;
            if other_divisor != i && other_divisor != n {
                sum += other_divisor;
            }
        }
    }

    sum
}

fn main() {
    let limit = 1_000_000;
    let mut longest_chain_length = 0;
    let mut longest_chain_sum = 0;
    let mut chain_lengths = HashMap::new();

    for i in 1..limit {
        let mut seen = Vec::new();
        let mut current = i;

        while !seen.contains(&current) {
            if current >= limit {
                break;
            }
            seen.push(current);
            current = sum_of_proper_divisors(current);
        }

        if let Some(&start) = seen.iter().find(|&&num| num == current) {
            let cycle_start_index = seen.iter().position(|&num| num == start).unwrap();
            let chain = &seen[cycle_start_index..];

            let chain_length = chain.len();

            if chain_length > longest_chain_length {
                longest_chain_length = chain_length;
                longest_chain_sum = chain.iter().copied().sum();
            }
        }
    }

    println!("The sum of the longest amicable chain is {}.", longest_chain_sum);
}
