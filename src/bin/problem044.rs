// Project Euler #44: Pentagon Numbers
// https://projecteuler.net/problem=44

use std::collections::HashSet;

fn pentagonal(n: usize) -> usize {
    n * (3 * n - 1) / 2
}

fn is_pentagonal(x: usize) -> bool {
    let n = (1.0 + (1.0 + 24.0 * x as f64).sqrt()) / 6.0;
    n == n.floor() 
}

fn main() {
    let mut pentagonals = Vec::new();
    let mut pentagonal_set = HashSet::new();
    
    for n in 1..=10000 {
        let p = pentagonal(n);
        pentagonals.push(p);
        pentagonal_set.insert(p);
    }

    let mut min_difference = usize::MAX;

    for j in 1..pentagonals.len() {
        for k in 0..j {
            let pj = pentagonals[j];
            let pk = pentagonals[k];
            let sum = pj + pk;
            let difference = pj - pk;

            if pentagonal_set.contains(&sum) && pentagonal_set.contains(&difference) {
                if difference < min_difference {
                    min_difference = difference;
                }
            }
        }
    }

    println!("The smallest value is {}", min_difference);
}
