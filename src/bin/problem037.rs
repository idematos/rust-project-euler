// Problem #37: Truncatable Primes
// https://projecteuler.net/problem=37

fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn left_truncations(mut n: usize) -> Vec<usize> {
    let mut truncations = Vec::new();
    let mut divisor = 10;
    
    while n >= divisor {
        truncations.push(n % divisor);
        divisor *= 10;
    }
    
    truncations
}

fn right_truncations(mut n: usize) -> Vec<usize> {
    let mut truncations = Vec::new();
    
    while n > 0 {
        truncations.push(n);
        n /= 10;
    }
    
    truncations
}

fn is_truncatable_prime(n: usize) -> bool {
    if !is_prime(n) {
        return false;
    }
    
    for trunc in left_truncations(n) {
        if !is_prime(trunc) {
            return false;
        }
    }
    
    for trunc in right_truncations(n) {
        if !is_prime(trunc) {
            return false;
        }
    }
    
    true
}

fn main() {
    let mut sum = 0;
    let mut count = 0;
    let mut num = 10;
    
    while count < 11 {
        if is_truncatable_prime(num) {
            sum += num;
            count += 1;
        }
        num += 1;
    }
    
    println!("The sum of the eleven truncatable primes is {}", sum);
}
