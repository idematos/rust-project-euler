use std::io::{self, BufRead};

fn sum_square_difference(n: u32) -> u32 {
    let mut sum: u32 = 0;
    let mut sum_of_square: u32 = 0;
    for i in 1..(n + 1) {
        sum += i;
        sum_of_square += i * i;
    }
    let square_of_sum = sum * sum;

    square_of_sum - sum_of_square
}

fn main() {
    println!("How many test cases would you like to execute?");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        println!("\nChoose a natural number:");
        let n = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap();

        println!("The absolute difference between the sum of the squares of the first {} natural numbers and the square of the sum of these numbers is {}.", n, sum_square_difference(n));
    }
}