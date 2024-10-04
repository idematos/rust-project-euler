// Project Euler #42: Coded Triangle Numbers
// https://projecteuler.net/problem=42

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn triangle_number(n: usize) -> usize {
    n * (n + 1) / 2
}

fn load_words_from_file(filename: &str) -> Vec<String> {
    let mut words = Vec::new();
    if let Ok(file) = File::open(filename) {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                for word in line.split(',') {
                    let trimmed_word = word.trim_matches('"');
                    words.push(trimmed_word.to_string());
                }
            }
        }
    }
  
    words
}

fn word_value(word: &str) -> usize {
    word.chars()
        .map(|c| (c as usize) - ('A' as usize) + 1)
        .sum()
}

fn main() {
    let words = load_words_from_file("words.txt");

    let mut triangle_numbers = HashSet::new();
    let mut n = 1;
    while triangle_number(n) <= 1000 { 
        triangle_numbers.insert(triangle_number(n));
        n += 1;
    }

    let triangle_word_count = words.iter()
        .filter(|&word| triangle_numbers.contains(&word_value(word)))
        .count();

    println!("The number of triangle words is {}", triangle_word_count);
}
