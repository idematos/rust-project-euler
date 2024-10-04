// Project Euler #43: Sub-string Divisibility
// https://projecteuler.net/problem=43

use std::collections::HashSet;
use itertools::Itertools;

fn is_valuable(pandigital: &str) -> bool {
    let divisors = [2, 3, 5, 7, 11, 13];
    
    for i in 0..6 {
        let sub_num: usize = pandigital[i + 1..i + 4].parse().unwrap(); 
        if sub_num % divisors[i] != 0 {
            return false; 
        }
    }
    
    true 
}

fn main() {
    let mut sum = 0;
    
    let digits = "0123456789";
    let pandigital_numbers = digits.chars().permutations(10);
    
    for pandigital in pandigital_numbers {
        let pandigital_str: String = pandigital.iter().collect();
        
        if is_valuable(&pandigital_str) {
            let number: usize = pandigital_str.parse().unwrap();
            sum += number;
        }
    }
    
    println!("The sum of all valuable pandigital numbers is {}", sum);
}
