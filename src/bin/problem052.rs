// Problem #52: Permuted Multiples
// https://projecteuler.net/problem=52

fn is_permutation(a: usize, b: usize) -> bool {
    let mut a_digits: Vec<char> = a.to_string().chars().collect();
    let mut b_digits: Vec<char> = b.to_string().chars().collect();
    
    a_digits.sort();
    b_digits.sort();
    a_digits == b_digits
}

fn main() {
    let mut n = 1;

    loop {
        let mut all_permuted = true;

        for i in 2..=6 {
            if !is_permutation(n, n * i) {
                all_permuted = false;
                break;
            }
        }

        if all_permuted {
            println!("The smallest positive integer such that 2n, 3n, 4n, 5n, and 6n are all permutations of n is {}", n);
            break;
        }

        n += 1; 
    }
}
