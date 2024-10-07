// Problem #55: Lychrel Numbers
// https://projecteuler.net/problem=55

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

fn is_lychrel(n: u64) -> bool {
    let mut num = n;

    for _ in 0..50 {
        num += reverse(num);
        if is_palindrome(num) {
            return false; 
        }
    }

    true 
}

fn reverse(n: u64) -> u64 {
    n.to_string().chars().rev().collect::<String>().parse::<u64>().unwrap()
}

fn main() {
    let limit = 10_000;
    let mut lychrel_count = 0;

    for n in 1..limit {
        if is_lychrel(n) {
            lychrel_count += 1;
        }
    }

    println!("The number of Lychrel numbers below {} is {}", limit, lychrel_count);
}
