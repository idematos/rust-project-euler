use std::io::{self, BufRead};

fn sum_of_multiples_three_or_five(mut n: u32) -> u32 {
    n -= 1;
    let sum = (sum_divisible_by(n, 3) + sum_divisible_by(n, 5)) - sum_divisible_by(n, 15);
    sum
}

fn sum_divisible_by(n: u32, k: u32) -> u32 {
    let p = n / k;
    let sum = k * p * (p + 1) / 2;
    sum
}

fn main() {
    println!("How many test cases would you like to execute?");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<u32>().unwrap();

    for _ in 0..t {
        println!("\nChoose a natural number:");
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<u32>().unwrap();
        println!("The sum of all the multiples of 3 or 5 below {} is {}.", n, sum_of_multiples_three_or_five(n));
    }
}