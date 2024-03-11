// Problem #14: Longest Collatz Sequence
// https://projecteuler.net/problem=14

fn get_collatz_sequence_size(start: u64) -> u64 {
    let mut number = start;
    let mut size = 1;
    while number != 1 {
        if number % 2 == 0 {
            number = number/2;
        } else {
            number = 3*number + 1;
        }
        size += 1
    }

    size
}

fn longest_chain(n: u64) -> u64 {
    let mut number = 1;
    let mut longest_chain = 1;
    let mut number_longest_chain = 1;
    while number < n {
        let chain = get_collatz_sequence_size(number);
        if chain > longest_chain {
            longest_chain = chain;
            number_longest_chain = number;
        }

        number += 1
    }

    return number_longest_chain;
}

fn main() {
    let n = 1000000;

    println!("\nThe starting number, under {}, that produces the longest chain is {}.", n, longest_chain(n));
}