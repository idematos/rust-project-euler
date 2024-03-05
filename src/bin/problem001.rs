use std::io::{self, BufRead};

fn sum_multiples_three_or_five(n: u32) -> u32 {
    let mut sum: u32 = 0;
    for i in 0..n {
        if i % 5 == 0 || i % 3 == 0 {
            sum += i;
        }
    }
    sum
}

fn main() {
    println!("How many test cases would you like to execute?");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        println!("\nChoose a natural number:");
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<u32>().unwrap();
        println!("The sum of all the multiples of 3 or 5 below {} is {}.", n, sum_multiples_three_or_five(n));
    }
}