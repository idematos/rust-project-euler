// Problem #69: Totient Maximum
// https://projecteuler.net/problem=69

fn euler_totient(n: usize, primes: &Vec<usize>) -> usize {
    let mut result = n;
    let mut num = n;

    for &p in primes {
        if p * p > n {
            break;
        }
        if num % p == 0 {
            while num % p == 0 {
                num /= p;
            }
            result -= result / p;
        }
    }
    if num > 1 {
        result -= result / num;
    }
    result
}

fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut sieve = vec![true; limit + 1];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..=(limit as f64).sqrt() as usize {
        if sieve[i] {
            for j in (i * i..=limit).step_by(i) {
                sieve[j] = false;
            }
        }
    }
    sieve.into_iter().enumerate()
        .filter_map(|(i, is_prime)| if is_prime { Some(i) } else { None })
        .collect()
}

fn main() {
    let limit = 1_000_000;
    
    let primes = sieve_of_eratosthenes(limit);

    let mut n_max = 1;
    let mut prime_product = 1;

    for &p in &primes {
        if prime_product * p > limit {
            break;
        }
        prime_product *= p;
        n_max = prime_product;
    }

    println!("The value of n ≤ {} for which n/φ(n) is maximized is {}", limit, n_max);
}
