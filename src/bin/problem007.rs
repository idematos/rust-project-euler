// Problem #7: 10001st prime
// https://projecteuler.net/problem=7

use std::io::{self, BufRead};

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
    println!("How many test cases would you like to execute?");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        println!("\nChoose a natural number:");
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<u64>().unwrap();
        println!("The {}th prime number is {}.", n, nth_prime(n));
    }
}
