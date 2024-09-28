// Problem 23: Non-Abundant Sums
// https://projecteuler.net/problem=23

fn sum_of_proper_divisors(n: usize) -> usize {
    let mut sum = 1;
    let limit = (n as f64).sqrt() as usize;

    for i in 2..=limit {
        if n % i == 0 {
            sum += i;
            let other_divisor = n / i;
            if other_divisor != i {
                sum += other_divisor;
            }
        }
    }

    sum
}

fn main() {
    let limit = 28123;

    let abundant_numbers: Vec<usize> = (1..=limit)
        .filter(|&n| sum_of_proper_divisors(n) > n)
        .collect();

    let mut is_sum_of_two_abundants = vec![false; limit + 1];

    for &a in &abundant_numbers {
        for &b in &abundant_numbers {
            if a + b <= limit {
                is_sum_of_two_abundants[a + b] = true;
            } else {
                break;
            }
        }
    }

    let sum_of_non_abundant_sums: usize = (1..=limit)
        .filter(|&n| !is_sum_of_two_abundants[n])
        .sum();

    println!("The sum of all numbers that cannot be written as the sum of two abundant numbers is {}", sum_of_non_abundant_sums);
}
