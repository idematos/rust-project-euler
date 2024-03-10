// Problem #9: Special Pythagorean triplet
// https://projecteuler.net/problem=9

fn is_pithagorean_triplet(a: i64, b: i64, c: i64) -> bool {
    c*c == a*a + b*b
}

fn get_max_product_pithagorean_triplet(n: i64) -> i64 {
    let mut product = -1;
    for c in (1..(n-1)).rev() {
        for b in (1..(c-1)).rev() {
            let a = n - b - c;
            if is_pithagorean_triplet(a,b,c) {
                let new_product = a*b*c;
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

    println!("The product abc for the largest Pythagorean triplet (a,b,c) such that a+b+c = {} is {}.", n, get_max_product_pithagorean_triplet(n));
}