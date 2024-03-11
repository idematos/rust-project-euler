// Problem #15: Lattice Paths
// https://projecteuler.net/problem=15

fn factorial(k: u128) -> u128 {
    let mut factorial = 1;
    for i in 1..=k {
        factorial = factorial * i;
    }

    factorial
}
fn get_routes(n: u128) -> u128 {
    let mut routes = 1;
    for i in (n+1)..=2*n {
        routes = routes * i;
    }

    routes / factorial(n)
}

fn main() {
    let n = 20;

    println!("\nThere are {} routes through a {}x{} grid.", get_routes(n), n, n);
}
