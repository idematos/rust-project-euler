// Problem #20: Factorial Digit Sum
// https://projecteuler.net/problem=20

use num_bigint::BigInt;
use num_traits::FromPrimitive;

fn get_sum(n: u32) -> u32 {
    let mut factorial: BigInt = FromPrimitive::from_u32(1).unwrap();
    for i in 1..n {
        factorial = factorial * i
    }

    factorial
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum()
}

fn main() {
    let n = 100;

    println!("\nThe sum of the digits of the number {}! is {}.", n, get_sum(n));
}
