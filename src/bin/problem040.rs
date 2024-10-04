// Project Euler #40: Champernowne's Constant
// https://projecteuler.net/problem=40

fn main() {
    let limit = 1000000;
    let mut champernowne = String::new();
    
    let mut i = 1;
    while champernowne.len() < limit {
        champernowne.push_str(&i.to_string());
        i += 1;
    }

    let indices = [1, 10, 100, 1000, 10000, 100000, 1000000];
    let mut product = 1;

    for &index in &indices {
        product *= champernowne.chars().nth(index - 1).unwrap().to_digit(10).unwrap(); 
    }

    println!("The product of the digits is {}", product);
}
