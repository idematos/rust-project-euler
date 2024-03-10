// Problem #9: Special Pythagorean triplet
// https://projecteuler.net/problem=9

fn get_max_product_pithagorean_triplet(n: i64) -> i64 {
    let mut product = -1;
    for c in (1..(n-1)).rev() {
        for b in (1..(c-1)).rev() {
            let a: f64 = ((c*c - b*b) as f64).sqrt();
            if a == (n - b - c) as f64 {
                let new_product = (a as i64)*b*c;
                if new_product > product {
                    product = new_product;
                }
            }
        }
    }

    product
}

fn main() {
    let n = 1000;

    println!("\nThe product abc for the largest Pythagorean triplet (a,b,c) such that a+b+c = {} is {}.", n, get_max_product_pithagorean_triplet(n));
}