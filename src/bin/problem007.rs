// Problem #7: 10001st prime
// https://projecteuler.net/problem=7

fn nth_prime(n: u64) -> u64 {
    let mut primes = Vec::<u64>::new();
    let mut prime = 2;
    'outer: while primes.len() < n as usize {
        let mut i = 2;
        while i * i <= prime {
            if prime % i == 0 {
                prime += 1;
                continue 'outer;
            }
            i += 1;
        }
        primes.push(prime);
        prime += 1;
    }

    primes[(n-1) as usize]
}

fn main() {
    let n = 10001;

    println!("\nThe {}th prime number is {}.", n, nth_prime(n));
}
