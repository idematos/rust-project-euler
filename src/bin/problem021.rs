// Problem #21: Amicable Numbers
// https://projecteuler.net/problem=21

fn proper_divisors_sum(n: usize) -> usize {
    let mut sum = 1; 
    let limit = (n as f64).sqrt() as usize;

    for i in 2..=limit {
        if n % i == 0 {
            sum += i;
            let other_divisor = n / i;
            if other_divisor != i && other_divisor != n {
                sum += other_divisor;
            }
        }
    }

    sum
}

fn main() {
    let mut amicable_numbers = std::collections::HashSet::new();
    let limit = 10_000;

    for a in 2..limit {
        let b = proper_divisors_sum(a);
        if b != a && b < limit {
            let sum_of_divisors_b = proper_divisors_sum(b);
            if sum_of_divisors_b == a {
                amicable_numbers.insert(a);
                amicable_numbers.insert(b);
            }
        }
    }

    let sum_of_amicable_numbers: usize = amicable_numbers.iter().sum();
    println!("The sum of all amicable numbers under {} is {}", limit, sum_of_amicable_numbers);
}
