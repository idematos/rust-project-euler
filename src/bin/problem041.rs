// Project Euler #41: Pandigital Prime
// https://projecteuler.net/problem=41

use std::collections::HashSet;

fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
  
    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            return false;
        }
    }
  
    true
}

fn pandigital_numbers(n: usize) -> Vec<usize> {
    let digits: Vec<char> = (1..=n).map(|d| std::char::from_digit(d as u32, 10).unwrap()).collect();
    let mut results = HashSet::new(); 
    let mut permute = |current: Vec<char>, remaining: Vec<char>| {
        if remaining.is_empty() {
            let num: usize = current.iter().collect::<String>().parse().unwrap();
            results.insert(num);
        }
        for i in 0..remaining.len() {
            let mut next_current = current.clone();
            let mut next_remaining = remaining.clone();
            next_current.push(next_remaining.remove(i));
            permute(next_current, next_remaining);
        }
    };
  
    permute(vec![], digits);
  
    results.into_iter().collect()
}

fn main() {
    let mut largest_pandigital_prime = 0;

    for n in 1..=7 {
        let pandigitals = pandigital_numbers(n);
        for &number in pandigitals.iter() {
            if is_prime(number) && number > largest_pandigital_prime {
                largest_pandigital_prime = number;
            }
        }
    }

    println!("The largest pandigital prime is {}", largest_pandigital_prime);
}
