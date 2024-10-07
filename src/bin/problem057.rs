// Problem #57: Square Root Convergents
// https://projecteuler.net/problem=57

fn digit_count(n: u64) -> usize {
    n.to_string().len()
}

fn main() {
    let mut count = 0;
    let mut a = 1; 
    let mut b = 1; 

    for _ in 1..=1000 {
        let new_a = a + 2 * b; 
        let new_b = a + b;     
        a = new_a;
        b = new_b;

        if digit_count(a) > digit_count(b) {
            count += 1;
        }
    }

    println!("The number of fractions with a > b with more digits in the numerator is {}", count);
}
