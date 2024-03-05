// Problem #3: Largest Prime Factor
// https://projecteuler.net/problem=3

use std::io::{self, BufRead};

fn largest_prime_factor(mut n: u32) -> u32 {
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
    println!("How many test cases would you like to execute?");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        println!("\nChoose a natural number:");
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<u32>().unwrap();
        println!("The largest prime factor of {} is {}.", n, largest_prime_factor(n));
    }
}