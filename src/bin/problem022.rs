// Problem 22: Names Scores
// https://projecteuler.net/problem=22

use std::fs;

fn letter_value(letter: char) -> u32 {
    (letter as u32 - 'A' as u32 + 1)
}

fn name_score(name: &str) -> u32 {
    name.chars().map(letter_value).sum()
}

fn main() {
    let names_data = r#""MARY","PATRICIA","LINDA","BARBARA","ELIZABETH","JAMES","JOHN","ROBERT","MICHAEL","WILLIAM","DAVID","RICHARD","CHARLES","JOSEPH","THOMAS","MARY","PATRICIA","LINDA","BARBARA","ELIZABETH","JAMES","JOHN","ROBERT","MICHAEL","WILLIAM","DAVID","RICHARD","CHARLES","JOSEPH","THOMAS""#;

    let mut names: Vec<String> = names_data
        .replace('"', "")
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    names.sort();

    let total_score: u32 = names.iter()
        .enumerate()
        .map(|(index, name)| (index as u32 + 1) * name_score(name))
        .sum();

    println!("The total name score is {}", total_score);
}
