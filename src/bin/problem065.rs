// Problem #65: Convergents of e
// https://projecteuler.net/problem=65

fn sum_of_digits(num: u64) -> u64 {
    num.to_string().chars().map(|d| d.to_digit(10).unwrap() as u64).sum()
}

fn main() {
    let mut terms = Vec::new();
    terms.push(2); 

    for k in 1..=33 {
        terms.push(1);
        terms.push(2 * k);
        terms.push(1);
    }

    let mut numerator = 1;
    let mut denominator = *terms.last().unwrap();

    for i in (0..terms.len() - 1).rev() {
        let new_numerator = denominator;
        denominator = numerator + terms[i] * denominator;
        numerator = new_numerator;
    }

    numerator = numerator + 2 * denominator;

    let result = sum_of_digits(numerator);
    println!("The sum of digits in the numerator of the 100th convergent is {}", result);
}
