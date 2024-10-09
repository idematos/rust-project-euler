// Problem #80: Square Root Digital Expansion
// https://projecteuler.net/problem=80

extern crate num_bigint;
extern crate bigdecimal;

use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use std::str::FromStr;
use std::ops::Div;
use std::cmp::Ordering;
use std::iter::repeat;

fn is_perfect_square(n: u32) -> bool {
    let sqrt_n = (n as f64).sqrt() as u32;
    sqrt_n * sqrt_n == n
}

fn sqrt_bigdecimal(n: u32, precision: usize) -> BigDecimal {
    let big_n = BigDecimal::from(n);
    let mut guess = BigDecimal::from_f64((n as f64).sqrt()).unwrap();

    let precision_limit = BigDecimal::new(BigInt::from(1), -(precision as i64));

    loop {
        let next_guess = (guess.clone() + big_n.clone().div(guess.clone())) / BigDecimal::from(2);
        if (next_guess.clone() - guess.clone()).abs() < precision_limit {
            break;
        }
        guess = next_guess;
    }

    guess.with_prec(precision as u64)
}

fn sum_first_100_digits_of_sqrt(n: u32) -> u32 {
    let precision = 105; 
    let sqrt = sqrt_bigdecimal(n, precision);
    let sqrt_str = sqrt.to_string();

    let mut digit_sum = 0;
    let mut digit_count = 0;

    for ch in sqrt_str.chars() {
        if ch.is_digit(10) {
            digit_sum += ch.to_digit(10).unwrap();
            digit_count += 1;
            if digit_count == 100 {
                break;
            }
        }
    }

    digit_sum
}

fn main() {
    let mut total_sum = 0;

    for n in 1..=100 {
        if !is_perfect_square(n) {
            total_sum += sum_first_100_digits_of_sqrt(n);
        }
    }

    println!("The total sum of the first 100 decimal digits of the square roots of irrational numbers up to 100 is {}", total_sum);
}
