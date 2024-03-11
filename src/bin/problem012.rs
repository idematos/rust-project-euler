// Problem #12: Highly divisible triangular number
// https://projecteuler.net/problem=12

fn get_num_divisors(num: u64) -> u64 {
    let mut num_divisors = 0;
    let mut i = 1;
    while i <= num {
        if num % i == 0 {
            num_divisors += 1;
        }
        i += 1;
    }

    num_divisors
}

fn first_triangle_number_with_n_divisors(n: u64) -> u64 {
    let mut triangle_num = 1;
    let mut i = triangle_num;
    while get_num_divisors(triangle_num) <= n {
        i += 1;
        triangle_num += i;
    }

    triangle_num
}

fn main() {
    let n = 500;

    println!("\nThe first triangle number to have over {} divisors is {}", n, first_triangle_number_with_n_divisors(n));
}