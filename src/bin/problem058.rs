// Problem #58: Spiral Primes
// https://projecteuler.net/problem=58

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut prime_count = 0;
    let mut total_count = 1; 
    let mut number = 1; 
    let mut layer = 1; 

    while true {
        for i in 0..4 {
            number += layer * 2; 
            total_count += 1;
            
            if is_prime(number) {
                prime_count += 1;
            }
        }

        let ratio = prime_count as f64 / total_count as f64;

        if ratio < 0.10 {
            println!("The side length of the square spiral is {}", layer + 1);
            break;
        }

        layer += 2; 
    }
}
