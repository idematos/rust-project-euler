// Problem #36: Double-base Palindromes
// https://projecteuler.net/problem=36

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn is_double_base_palindrome(n: usize) -> bool {
    let decimal_str = n.to_string();
    let binary_str = format!("{:b}", n); 
    is_palindrome(&decimal_str) && is_palindrome(&binary_str)
}

fn main() {
    let limit = 1_000_000;
    let mut sum = 0;
    
    for num in 1..limit {
        if is_double_base_palindrome(num) {
            sum += num;
        }
    }
    
    println!("The sum of all double-base palindromes below one million is {}", sum);
}
