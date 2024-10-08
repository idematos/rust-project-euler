// Problem #70: Totient Permutation
// https://projecteuler.net/problem=70

use std::collections::HashMap;

fn compute_totients(limit: usize) -> Vec<usize> {
    let mut phi = (0..=limit).collect::<Vec<usize>>();
    
    for p in 2..=limit {
        if phi[p] == p { 
            for k in (p..=limit).step_by(p) {
                phi[k] *= (p - 1);
                phi[k] /= p;
            }
        }
    }
    
    phi
}

fn are_permutations(a: usize, b: usize) -> bool {
    let mut a_digits = a.to_string().chars().collect::<Vec<char>>();
    let mut b_digits = b.to_string().chars().collect::<Vec<char>>();
    
    a_digits.sort();
    b_digits.sort();
    
    a_digits == b_digits
}

fn main() {
    let limit = 10_000_000;
    let phi = compute_totients(limit);
    
    let mut min_ratio = f64::MAX;
    let mut result_n = 0;

    for n in 2..=limit {
        if are_permutations(n, phi[n]) {
            let ratio = n as f64 / phi[n] as f64;
            if ratio < min_ratio {
                min_ratio = ratio;
                result_n = n;
            }
        }
    }

    println!("The value of n < 10000000 for which n and phi(n) are permutations and n/phi(n) is minimized is {}", result_n);
}
