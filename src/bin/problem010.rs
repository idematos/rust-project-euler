// Problem #10: Summation of primes
// https://projecteuler.net/problem=10

fn sum_primes_below(n: i64) -> i64 {
    let mut sum = 0;
    let mut prime = 2;
    'outer: while prime <= n {
        let mut i = 2;
        while i * i <= prime {
            if prime % i == 0 {
                prime += 1;
                continue 'outer;
            }
            i += 1;
        }
        sum += prime;
        prime += 1;
    }

    sum
}

fn main() {
    let n = 2000000;

    println!("The sum of all the primes below {} is {}.", n, sum_primes_below(n));
}
