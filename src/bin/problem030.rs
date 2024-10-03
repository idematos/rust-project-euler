// Problem #30: Digit Fifth Powers
// https://projecteuler.net/problem=30

fn digit_fifth_powers_sum(n: i64) -> i64 {
    let mut sum = 0;
    let mut num = n;
    
    while num > 0 {
        let digit = num % 10;
        sum += digit.pow(5);
        num /= 10;
    }
    
    sum
}

fn main() {
    let upper_limit = 354294;
    let mut total_sum = 0;
    
    for i in 2..=upper_limit {
        if i == digit_fifth_powers_sum(i) {
            total_sum += i;
        }
    }
    
    println!("The sum of all numbers that can be written as the sum of the fifth powers of their digits is {}", total_sum);
}
