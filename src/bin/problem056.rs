// Problem #56: Powerful Digit Sum
// https://projecteuler.net/problem=56

fn digit_sum(n: u64) -> u64 {
    n.to_string().chars().map(|d| d.to_digit(10).unwrap() as u64).sum()
}

fn main() {
    let mut max_digit_sum = 0;

    for a in 2..=100 {
        for b in 2..=100 {
            let power = a.pow(b);
            let sum = digit_sum(power);

            if sum > max_digit_sum {
                max_digit_sum = sum;
            }
        }
    }

    println!("The maximum digit sum of a^b for 2 ≤ a ≤ 100 and 2 ≤ b ≤ 100 is {}", max_digit_sum);
}
