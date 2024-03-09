// Problem #1: Multiples of 3 or 5
// https://projecteuler.net/problem=1

fn sum_of_multiples_three_or_five(mut n: u64) -> u64 {
    n -= 1;

    (sum_divisible_by(n, 3) + sum_divisible_by(n, 5)) - sum_divisible_by(n, 15)
}

fn sum_divisible_by(n: u64, k: u64) -> u64 {
    let p = n / k;

    k * p * (p + 1) / 2
}

fn main() {
    let n = 1000;

    println!("\nThe sum of all the multiples of 3 or 5 below {} is {}.", n, sum_of_multiples_three_or_five(n));
}