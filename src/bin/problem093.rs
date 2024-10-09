// Problem #93: Arithmetic Expressions
// https://projecteuler.net/problem=93

use itertools::Itertools;
fn main() {
    let digits = [1, 2, 3, 4];
    let mut max_value = 0;
    let mut best_expression = String::new();

    for perm in digits.iter().permutations(4) {
        let (a, b, c, d) = (perm[0], perm[1], perm[2], perm[3]);

        let expressions = [
            a * b + c * d,
            a + b * c + d,
            a * (b + c) + d,
            (a + b) * c + d,
            a * b + c + d,
            a + b + c * d,
            a + b * c * d,
        ];

        for &value in &expressions {
            if value > max_value {
                max_value = value;
                best_expression = format!("{} * {} + {} + {}", a, b, c, d);
            }
        }
    }

    println!("The maximum value is {}.", max_value);
}
