// Problem #92: Square Digit Chains
// https://projecteuler.net/problem=92

use std::collections::HashMap;

fn sum_of_squares(n: usize) -> usize {
    let mut sum = 0;
    let mut number = n;

    while number > 0 {
        let digit = number % 10;
        sum += digit * digit;
        number /= 10;
    }

    sum
}

fn square_digit_chain(n: usize, memo: &mut HashMap<usize, bool>) -> bool {
    if let Some(&result) = memo.get(&n) {
        return result;
    }

    let mut current = n;

    while current != 1 && current != 89 {
        current = sum_of_squares(current);
    }

    let result = current == 89;
    memo.insert(n, result);
    result
}

fn main() {
    let limit = 10_000_000;
    let mut count = 0;
    let mut memo: HashMap<usize, bool> = HashMap::new();

    for i in 1..limit {
        if square_digit_chain(i, &mut memo) {
            count += 1;
        }
    }

    println!("The total number of chains that arrive at 89 is {}.", count);
}
