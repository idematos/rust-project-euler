// Problem #25: 1000-digit Fibonacci Number
// https://projecteuler.net/problem=25

use num_bigint::BigInt;
use num_traits::FromPrimitive;

fn get_fibonacci_term(n: usize) -> u32 {
    let mut term_index = 3;
    let mut num_digits: usize = 1;

    let mut first_fib_num: BigInt = FromPrimitive::from_u32(1).unwrap();
    let mut second_fib_num: BigInt = FromPrimitive::from_u32(1).unwrap();
    let mut third_fib_num: BigInt = FromPrimitive::from_u32(2).unwrap();
    while num_digits < n {
        if n > 1 {
            first_fib_num = second_fib_num.clone();
            second_fib_num = third_fib_num.clone();
            third_fib_num = first_fib_num.clone() + second_fib_num.clone();

            num_digits = third_fib_num
                .to_string()
                .chars()
                .collect::<Vec<_>>()
                .len();

            term_index += 1;
        }
    }

    term_index
}

fn main() {
    let n = 1000;

    println!("\nThe {}th is the first term in the Fibonacci sequence to contain {} digits.", get_fibonacci_term(n), n);
}