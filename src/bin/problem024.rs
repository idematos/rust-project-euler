// Problem 24: Lexicographic Permutations
// https://projecteuler.net/problem=24

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn main() {
    let mut digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut target_permutation = 1000000 - 1; 
    let mut result = Vec::new();

    for i in (0..digits.len()).rev() {
        let fact = factorial(i);
        let index = target_permutation / fact;
        result.push(digits.remove(index));
        target_permutation %= fact;
    }

    let result_string: String = result.iter().map(|&d| d.to_string()).collect();
    println!("The millionth lexicographic permutation is {}", result_string);
}
