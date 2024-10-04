// Problem #34: Digit Factorials
// https://projecteuler.net/problem=34

fn factorial(n: usize) -> usize {
    match n {
        0 | 1 => 1,
        _ => (2..=n).product(),
    }
}

fn sum_of_digit_factorials(n: usize, factorials: &Vec<usize>) -> usize {
    let mut sum = 0;
    let mut number = n;
    
    while number > 0 {
        let digit = number % 10;
        sum += factorials[digit];
        number /= 10;
    }
    
    sum
}

fn main() {
    let factorials: Vec<usize> = (0..=9).map(|n| factorial(n)).collect();
    
    let upper_limit = 7 * factorials[9];
    let mut result_sum = 0;
    
    for num in 10..=upper_limit {
        if num == sum_of_digit_factorials(num, &factorials) {
            result_sum += num;
        }
    }
    
    println!("The sum of all numbers equal to the sum of the factorial of their digits is {}", result_sum);
}
