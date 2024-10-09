// Problem #99: Largest Exponential
// https://projecteuler.net/problem=99

use std::fs;

fn main() {
    let content = fs::read_to_string("base_exp.txt").expect("Unable to read file");
    let mut max_value = f64::MIN; 
    let mut line_number = 0; 
    let mut max_line_number = 0; 

    for (index, line) in content.lines().enumerate() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 2 {
            continue; 
        }

        let base: f64 = parts[0].parse().expect("Invalid base");
        let exponent: f64 = parts[1].parse().expect("Invalid exponent");

        let value = exponent * base.log(10.0);

        if value > max_value {
            max_value = value;
            max_line_number = index + 1; 
        }
    }

    println!("The line number of the largest exponential is {}.", max_line_number);
}
