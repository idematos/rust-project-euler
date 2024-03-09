// Problem #6: Sum Square Difference
// https://projecteuler.net/problem=6

fn sum_square_difference(n: u64) -> u64 {
    let mut sum: u64 = 0;
    let mut sum_of_square: u64 = 0;
    for i in 1..(n + 1) {
        sum += i;
        sum_of_square += i * i;
    }
    let square_of_sum = sum * sum;

    square_of_sum - sum_of_square
}

fn main() {
    let n = 100;

    println!("\nThe difference between the sum of the squares of the first {} natural numbers and the square of the sum of these numbers is {}.", n, sum_square_difference(n));
}