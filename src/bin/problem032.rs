// Problem #32: Pandigital Products
// https://projecteuler.net/problem=32

use std::collections::HashSet;

fn is_pandigital(a: usize, b: usize, c: usize) -> bool {
    let mut digits = Vec::new();
    digits.extend(a.to_string().chars());
    digits.extend(b.to_string().chars());
    digits.extend(c.to_string().chars());
    
    if digits.len() != 9 {
        return false;
    }
    
    digits.sort();
    digits == vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']
}

fn main() {
    let mut products = HashSet::new();
    
    for a in 1..100; 
    {
        for b in 100..10000; 
        {
            let product = a * b;
            if is_pandigital(a, b, product) {
                products.insert(product);
            }
        }
    }

    let sum: usize = products.iter().sum();
    println!("The sum of all pandigital products is {}", sum);
}
