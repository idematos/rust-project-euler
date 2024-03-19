// Problem #97: Large Non-Mersenne Prime
// https://projecteuler.net/problem=97

use num_bigint::BigInt;
use num_traits::FromPrimitive;

fn find_last_ten_digits() -> String {
    let two: BigInt = FromPrimitive::from_u32(2).unwrap();
    let power: BigInt = num_traits::pow(two, 7830457 as usize);
    let factor: BigInt = FromPrimitive::from_u32(2843).unwrap();
    let one: BigInt = FromPrimitive::from_u32(1).unwrap();

    (factor * power + one).to_string()[0..10].to_string()
}

fn main() {
    println!("\nThe last ten digits of the prime number are {}", find_last_ten_digits());
}