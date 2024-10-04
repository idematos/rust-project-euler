// Project Euler #38: Pandigital Multiples
// https://projecteuler.net/problem=38

fn is_pandigital(s: &str) -> bool {
    if s.len() != 9 {
        return false;
    }
    
    let mut digits = [false; 10];
    
    for ch in s.chars() {
        if ch == '0' {
            return false;
        }
        let digit = ch.to_digit(10).unwrap() as usize;
        if digits[digit] {
            return false;
        }
        digits[digit] = true;
    }
    
    true
}

fn concatenated_product(x: usize) -> String {
    let mut result = String::new();
    let mut n = 1;
    
    while result.len() < 9 {
        result.push_str(&(x * n).to_string());
        n += 1;
    }
    
    result
}

fn main() {
    let mut largest_pandigital = 0;
    
    for x in 1..10000 {
        let concatenated = concatenated_product(x);
        
        if is_pandigital(&concatenated) {
            let pandigital_num = concatenated.parse::<usize>().unwrap();
            if pandigital_num > largest_pandigital {
                largest_pandigital = pandigital_num;
            }
        }
    }
    
    println!("The largest 1 to 9 pandigital number formed as a concatenated product is {}", largest_pandigital);
}
