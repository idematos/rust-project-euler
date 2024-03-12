// Problem #28: Number Spiral Diagonals
// https://projecteuler.net/problem=28

fn get_sum_diagonal(n: u64) -> u64 {
    let mut diag = 1;
    let mut sum = diag;
    let mut i = 2;
    'outer: loop {
        for _ in 1..=4 {
            diag += i;
            sum += diag;

            if diag == n * n {
                break 'outer;
            }
        }
        i += 2;
    }

    sum
}

fn main() {
    let n = 1001;

    println!("\nThe sum of the numbers on the diagonals in a {} by {} spiral is {}.", n, n, get_sum_diagonal(n));
}