// Project Euler #45: Triangular, Pentagonal, and Hexagonal
// https://projecteuler.net/problem=45

use std::collections::HashSet;

fn triangular(n: usize) -> usize {
    n * (n + 1) / 2
}

fn pentagonal(n: usize) -> usize {
    n * (3 * n - 1) / 2
}

fn hexagonal(n: usize) -> usize {
    n * (2 * n - 1)
}

fn is_pentagonal(x: usize, pentagonal_set: &HashSet<usize>) -> bool {
    pentagonal_set.contains(&x)
}

fn is_hexagonal(x: usize) -> bool {
    let n = (1.0 + (1.0 + 8.0 * x as f64).sqrt()) / 4.0;
    n == n.floor()
}

fn main() {
    let mut pentagonal_set = HashSet::new();
    let mut n = 1;

    while let Some(p) = pentagonal(n).checked_add(0) {
        pentagonal_set.insert(p);
        n += 1;
    }

    let mut triangle_number = 0;
    let mut index = 1;

    loop {
        triangle_number = triangular(index);
        if is_pentagonal(triangle_number, &pentagonal_set) && is_hexagonal(triangle_number) {
            if triangle_number > 40755 { 
                break;
            }
        }
        index += 1;
    }

    println!("The next triangle number that is also pentagonal and hexagonal is {}", triangle_number);
}
