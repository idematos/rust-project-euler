// Problem #63: Powerful Digit Counts
// https://projecteuler.net/problem=63

fn main() {
    let mut count = 0;

    for a in 1..=9 { 
        let mut b = 1; 
        loop {
            let power = a.pow(b);
            let digit_count = power.to_string().len();

            if digit_count > b {
                break;
            }

            if digit_count == b {
                count += 1;
            }

            b += 1; 
        }
    }

    println!("The total count of powerful digit counts is {}", count);
}
