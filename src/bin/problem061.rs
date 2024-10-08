// Problem #61: Cyclical Figurate Numbers
// https://projecteuler.net/problem=61

use std::collections::{HashMap, HashSet};

fn figurate_number(n: u64, shape: &str) -> u64 {
    match shape {
        "triangular" => n * (n + 1) / 2,
        "square" => n * n,
        "pentagonal" => n * (3 * n - 1) / 2,
        "hexagonal" => n * (2 * n - 1),
        "heptagonal" => n * (5 * n - 3) / 2,
        "octagonal" => n * (3 * n - 2),
        _ => 0,
    }
}

fn generate_figurate_numbers(shape: &str) -> Vec<u64> {
    let mut numbers = Vec::new();
    let mut n = 1;

    while let number = figurate_number(n, shape) {
        if number < 1000 {
            n += 1;
            continue;
        }
        if number >= 10000 {
            break;
        }
        numbers.push(number);
        n += 1;
    }

    numbers
}

fn find_cycles(numbers: &HashMap<u64, HashSet<u64>>, start: u64, current: u64, count: usize, sum: u64, used: &mut HashSet<u64>, cycles: &mut Vec<u64>) {
    if count == 6 && current % 100 == start / 100 {
        cycles.push(sum);
        return;
    }

    if count == 6 {
        return;
    }

    if let Some(next) = numbers.get(&current) {
        for &next_number in next {
            if !used.contains(&next_number) {
                used.insert(next_number);
                find_cycles(numbers, start, next_number, count + 1, sum + next_number, used, cycles);
                used.remove(&next_number);
            }
        }
    }
}

fn main() {
    let shapes = ["triangular", "square", "pentagonal", "hexagonal", "heptagonal", "octagonal"];
    let mut all_numbers = HashMap::new();

    for shape in &shapes {
        let numbers = generate_figurate_numbers(shape);
        for &num in &numbers {
            let last_two = num % 100;
            all_numbers.entry(last_two).or_insert_with(HashSet::new).insert(num);
        }
    }

    let mut cycles = Vec::new();

    for shape in &shapes {
        let numbers = generate_figurate_numbers(shape);
        for &start in &numbers {
            let mut used = HashSet::new();
            used.insert(start);
            find_cycles(&all_numbers, start, start, 1, start, &mut used, &mut cycles);
        }
    }

    if let Some(min_sum) = cycles.iter().min() {
        println!("The minimum sum of the cyclic figurate numbers is {}", min_sum);
    } else {
        println!("No cyclic figurate numbers found.");
    }
}
