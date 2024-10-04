// Problem #33: Digit Cancelling Fractions
// https://projecteuler.net/problem=33

use std::cmp::min;

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let mut num_product = 1; 
    let mut denom_product = 1; 
  
    for numerator in 10..100 {
        for denominator in (numerator + 1)..100 {
            if numerator % 10 == 0 && denominator % 10 == 0 {
                continue;
            }

            let n1 = numerator / 10;
            let n2 = numerator % 10;
            let d1 = denominator / 10;
            let d2 = denominator % 10;
            
            if n1 == d1 && n2 * denominator == numerator * d2 {
                num_product *= n2;
                denom_product *= d2;
            } else if n1 == d2 && n2 * denominator == numerator * d1 {
                num_product *= n2;
                denom_product *= d1;
            } else if n2 == d1 && n1 * denominator == numerator * d2 {
                num_product *= n1;
                denom_product *= d2;
            } else if n2 == d2 && n1 * denominator == numerator * d1 {
                num_product *= n1;
                denom_product *= d1;
            }
        }
    }

    let common_divisor = gcd(num_product, denom_product);
    let simplified_denom = denom_product / common_divisor;

    println!("The denominator of the product in its lowest terms is {}", simplified_denom);
}
