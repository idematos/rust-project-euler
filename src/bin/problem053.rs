// Problem #53: Combinatoric Selections 
// https://projecteuler.net/problem=53

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn combinations(n: u64, r: u64) -> u64 {
    if r > n {
        return 0;
    }
    factorial(n) / (factorial(r) * factorial(n - r))
}

fn main() {
    let limit = 1_000_000;
    let mut count = 0;

    for n in 1..=100 {
        for r in 0..=n {
            let comb = combinations(n, r);
            if comb > limit {
                count += 1;
            }
        }
    }

    println!("The number of combinations greater than one million is {}", count);
}
