// Problem #71: Ordered Fractions
// https://projecteuler.net/problem=71

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let limit = 1_000_000;
    let target_num = 3;
    let target_den = 7;
    
    let mut best_a = 0;
    let mut best_b = 1; 
    let target_fraction = target_num as f64 / target_den as f64;

    for b in 1..=limit {
        let a = (target_num * b) / target_den; 
        if a > 0 && a < b { 
            if (a as f64) / (b as f64) < target_fraction && gcd(a, b) == 1 {
                if (best_a as f64) / (best_b as f64) < (a as f64) / (b as f64) {
                    best_a = a;
                    best_b = b;
                }
            }
        }
    }

    println!("The largest fraction a/b < 3/7 with a < b ≤ {} is {}/{}", limit, best_a, best_b);
}
