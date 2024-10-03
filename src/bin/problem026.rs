// Problem #26: Reciprocal Cycles
// https://projecteuler.net/problem=26

fn recurring_cycle_length(denominator: usize) -> usize {
    let mut remainders = vec![0; denominator];
    let mut value = 1;
    let mut position = 0;

    while remainders[value] == 0 && value != 0 {
        remainders[value] = position;
        value *= 10;
        value %= denominator;
        position += 1;
    }

    if value == 0 {
        0 
    } else {
        position - remainders[value]
    }
}

fn main() {
    let mut max_length = 0;
    let mut d_with_max_length = 0;

    for d in 2..1000 {
        let cycle_length = recurring_cycle_length(d);
        if cycle_length > max_length {
            max_length = cycle_length;
            d_with_max_length = d;
        }
    }

    println!("The value of d < 1000 with the longest recurring cycle is {}", d_with_max_length);
}
