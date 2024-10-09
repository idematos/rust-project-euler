// Problem #100: Arranged Probability
// https://projecteuler.net/problem=100

fn is_perfect_square(x: u64) -> bool {
    let s = (x as f64).sqrt() as u64;
    s * s == x
}

fn main() {
    let mut n = 1; 
    let mut k = 1; 
    let mut solutions = 0;

    while solutions < 1_000_000 {
        n += 1;

        if n % 2 == 0 {
            let half_n = n / 2;

            let k_test = (half_n * half_n) / (n - half_n);

            if k_test * (n - half_n) == half_n * half_n {
                k = k_test;
                if is_perfect_square(k) {
                    solutions += 1; 
                }
            }
        }
    }

    println!("The first n with a perfect square k is {}.", n);
}
