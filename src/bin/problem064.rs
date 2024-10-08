// Problem #64: Odd Period Square Roots
// https://projecteuler.net/problem=64

fn continued_fraction_period(n: u64) -> usize {
    let sqrt_n = (n as f64).sqrt() as u64;

    if sqrt_n * sqrt_n == n {
        return 0;
    }

    let mut m = 0;
    let mut d = 1;
    let mut a = sqrt_n;
    let initial_a = a;

    let mut period = 0;

    while a != 2 * initial_a {
        m = d * a - m;
        d = (n - m * m) / d;
        a = (sqrt_n + m) / d;
        period += 1;
    }

    period
}

fn main() {
    let limit = 10000;
    let mut odd_period_count = 0;

    for n in 2..=limit {
        let period = continued_fraction_period(n);
        if period % 2 == 1 {
            odd_period_count += 1;
        }
    }

    println!("The number of continued fractions with odd periods is {}", odd_period_count);
}
