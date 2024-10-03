// Problem #27: Quadratic Primes
// https://projecteuler.net/problem=27

fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn consecutive_primes(a: i64, b: i64) -> usize {
    let mut n = 0;
    while is_prime(n * n + a * n + b) {
        n += 1;
    }
    n as usize
}

fn main() {
    let mut max_primes = 0;
    let mut product_of_coefficients = 0;

    for a in -999..1000 {
        for b in -1000..=1000 {
            let num_primes = consecutive_primes(a, b);
            if num_primes > max_primes {
                max_primes = num_primes;
                product_of_coefficients = a * b;
            }
        }
    }

    println!("The product of the coefficients is {}", product_of_coefficients);
}
