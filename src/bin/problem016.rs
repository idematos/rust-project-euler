// Problem #16: Power Digit Sum
// https://projecteuler.net/problem=16

use num_bigint::BigInt;
use num_traits::FromPrimitive;

fn get_sum(n: u32) -> u32 {
    let two: BigInt = FromPrimitive::from_u32(2).unwrap();

    num_traits::pow(two, n as usize)
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum()
}

fn main() {
    let n = 1000;

    println!("\nThe sum of the digits of the number 2^{} is {}.", n, get_sum(n));
}
