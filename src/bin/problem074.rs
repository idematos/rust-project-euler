// Problem #74: Digit Factorial Chains
// https://projecteuler.net/problem=74

fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => (2..=n).product(),
    }
}

fn factorial_digit_sum(n: u32) -> u32 {
    let mut sum = 0;
    let mut num = n;

    while num > 0 {
        sum += factorial(num % 10);
        num /= 10;
    }

    sum
}

fn main() {
    let limit = 1_000_000;
    let mut count = 0;
    let mut cache = std::collections::HashMap::new();

    for i in 1..limit {
        let mut chain = Vec::new();
        let mut current = i;

        while !cache.contains_key(&current) {
            if chain.len() >= 60 {
                break;
            }
            chain.push(current);
            current = factorial_digit_sum(current);
        }

        let mut chain_length = chain.len();
        if let Some(&length) = cache.get(&current) {
            chain_length += length;
        }

        for &num in &chain {
            cache.insert(num, chain_length);
        }

        if chain_length == 60 {
            count += 1;
        }
    }

    println!("The number of starting numbers under {} that generate chains of exactly 60 non-repeating terms is {}", limit, count);
}
