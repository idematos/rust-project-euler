// Problem #47: Distinct Primes Factors
// https://projecteuler.net/problem=47

fn get_prime_factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut x = n;
    let mut i = 2;
    while i  <= x {
        if x % i == 0 {
            if !factors.contains(&i) {
                factors.push(i);
            }

            x = x / i;
        }
        else {
            i += 1;
        }
    }

    factors
}

fn first_of_consecutives(n: u64) -> u64 {
    let mut i = 2;
    'outer: loop {
        for j in 0..n {
            if get_prime_factors(i+j).len() != n as usize {
                i += 1;
                continue 'outer;
            }
        }

        break;
    }

    i
}

fn main() {
    let n = 4;

    println!("\nThe first number of the first {} consecutive integers to have {} distinct prime factors each is {}.", n, n, first_of_consecutives(n));
}
