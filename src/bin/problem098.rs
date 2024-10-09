// Problem #98: Anagramic Squares
// https://projecteuler.net/problem=98

use std::collections::HashMap;

fn sorted_digits(n: u64) -> String {
    let mut digits: Vec<char> = n.to_string().chars().collect();
    digits.sort(); 
    digits.iter().collect() 
}

fn main() {
    let limit = 10_000; 
    let mut squares: HashMap<String, Vec<u64>> = HashMap::new();

    for i in 1..limit {
        let square = i * i;
        let key = sorted_digits(square);
        
        squares.entry(key).or_insert_with(Vec::new).push(square);
    }

    let mut largest_anagramic_square = 0;

    for (_, group) in squares.iter() {
        if group.len() > 1 { 
            let max_square = *group.iter().max().unwrap(); 
            largest_anagramic_square = largest_anagramic_square.max(max_square); 
        }
    }

    println!("The largest anagramic square is {}.", largest_anagramic_square);
}
