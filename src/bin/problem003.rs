// Problem #3: Largest Prime Factor
// https://projecteuler.net/problem=3

fn largest_prime_factor(mut n: u64) -> u64 {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            n = n/i;
        } else {
            i += 1;
        }
    }

    n
}

fn main() {
    let n = 600851475143;

    println!("\nThe largest prime factor of {} is {}.", n, largest_prime_factor(n));
}