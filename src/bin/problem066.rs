// Problem #66: Diophantine Equation
// https://projecteuler.net/problem=66

use std::collections::HashMap;

fn is_perfect_square(n: u64) -> bool {
    let root = (n as f64).sqrt() as u64;
    root * root == n
}

fn solve_pell_minimal_x(d: u64) -> u64 {
    let sqrt_d = (d as f64).sqrt() as u64;

    let mut m = 0;
    let mut d_ = 1;
    let mut a = sqrt_d;

    let mut num1 = 1;
    let mut num = a;
    let mut denom1 = 0;
    let mut denom = 1;

    while num * num - d * denom * denom != 1 {
        m = d_ * a - m;
        d_ = (d - m * m) / d_;
        a = (sqrt_d + m) / d_;

        let num_next = a * num + num1;
        num1 = num;
        num = num_next;

        let denom_next = a * denom + denom1;
        denom1 = denom;
        denom = denom_next;
    }

    num
}

fn main() {
    let limit = 1000;
    let mut largest_x = 0;
    let mut result_d = 0;

    for d in 2..=limit {
        if is_perfect_square(d) {
            continue;
        }

        let x = solve_pell_minimal_x(d);
        if x > largest_x {
            largest_x = x;
            result_d = d;
        }
    }

    println!("The value of D for which the largest minimal x is obtained is {}", result_d);
}
