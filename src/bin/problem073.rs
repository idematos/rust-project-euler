// Problem #73: Counting Fractions in a Range
// https://projecteuler.net/problem=73

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let limit = 1000000; 
    let mut count = 0;

    for b in 1..=limit {
        let start = (b + 2) / 3; 
        let end = b / 2; 

        for a in start..end {
            if gcd(a, b) == 1 {
                count += 1;
            }
        }
    }

    println!("The number of fractions in the range 1/3 < a/b < 1/2 for 1 ≤ a < b ≤ {} is {}", limit, count);
}
