// Problem #4: Largest palindrome product
// https://projecteuler.net/problem=4

use std::io::{self, BufRead};

fn find_largest_palindrome_less_than(n: u64) -> u64 {
    'outer: for num in (10000..(n-1)).rev() {
        for factor in (100..999).rev() {
            if num % factor == 0 {
                if (num / factor).to_string().len() == 3 {
                    let str_num = num.to_string();
                    for k in 0..(str_num.len() / 2) {
                        if str_num.chars().nth(k) != str_num.chars().nth(str_num.len() - k - 1) {
                            continue 'outer;
                        }
                    }

                    return num;
                }
            }
        }
    }
    0
}

fn main() {
    println!("How many test cases would you like to execute?");
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        println!("\nChoose a natural number:");
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<u64>().unwrap();
        println!("The largest palindrome made from the product of two 3-digit numbers which is less than {} is {}", n, find_largest_palindrome_less_than(n));
    }
}
