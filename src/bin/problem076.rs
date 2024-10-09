// Problem #76: Counting Summations
// https://projecteuler.net/problem=76

fn count_summations(target: usize) -> usize {
    let mut dp = vec![0; target + 1];
    dp[0] = 1;

    for i in 1..target {
        for j in i..=target {
            dp[j] += dp[j - i];
        }
    }

    dp[target]
}

fn main() {
    let target = 100;
    let result = count_summations(target);
    println!("The number of different ways to write {} as a sum of at least two positive integers is {}", target, result);
}
