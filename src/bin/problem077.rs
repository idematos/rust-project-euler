// Problem #77: Prime Summations
// https://projecteuler.net/problem=77

fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut sieve = vec![true; limit + 1];
    let mut primes = Vec::new();

    sieve[0] = false;
    sieve[1] = false;

    for num in 2..=limit {
        if sieve[num] {
            primes.push(num);
            for multiple in (num * num..=limit).step_by(num) {
                sieve[multiple] = false;
            }
        }
    }

    primes
}

fn prime_summations(limit: usize, target_ways: usize) -> usize {
    let primes = sieve_of_eratosthenes(limit);
    let mut dp = vec![0; limit + 1];
    dp[0] = 1;
  
    for &prime in &primes {
        for num in prime..=limit {
            dp[num] += dp[num - prime];
        }
    }

    for i in 2..=limit {
        if dp[i] > target_ways {
            return i;
        }
    }

    0 
}

fn main() {
    let target_ways = 5000;
    let limit = 100; 
    let result = prime_summations(10000, target_ways); 
    println!("The first number that can be written as the sum of primes in more than {} ways is {}", target_ways, result);
}
