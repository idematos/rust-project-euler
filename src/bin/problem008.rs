// Problem #8: Largest product in a series
// https://projecteuler.net/problem=8

use std::io::{self, BufRead};

fn number_to_vec(n: u64) -> Vec<u64> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}

fn get_product(vec: &[u64]) -> u64 {
    let mut product = vec[0];
    for i in 1..vec.len() {
        product = product * vec[i];
    }
    product
}

fn greatest_product(n: u64, k: u64, num: u64) -> u64 {
    let vec_num = number_to_vec(num);

    let uk = k as usize;
    let mut product= 0;
    for i in 0..(n-k+1) {
        let ui = i as usize;

        let next_product = get_product(&vec_num[ui..(uk+ui)]);
        if next_product > product {
            product = next_product;
        }
    }
    product
}

fn main() {
    println!("How many test cases would you like to execute?");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        println!("\nChoose the number length and the amount of factor separated by space (e.g. '10 5'):");
        let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let n = first_multiple_input[0].trim().parse::<u64>().unwrap();

        let k = first_multiple_input[1].trim().parse::<u64>().unwrap();

        println!("\nWrite the number:");
        let num = stdin_iterator.next().unwrap().unwrap().trim().parse::<u64>().unwrap();

        println!("The greatest product of {} consecutive digits in {} is {}.", k, num, greatest_product(n,k,num));
    }
}
