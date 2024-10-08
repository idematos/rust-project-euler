// Problem #72: Counting Fractions
// https://projecteuler.net/problem=72

fn compute_totients(limit: usize) -> Vec<usize> {
    let mut phi = (0..=limit).collect::<Vec<usize>>();
    
    for p in 2..=limit {
        if phi[p] == p { 
            for k in (p..=limit).step_by(p) {
                phi[k] *= (p - 1);
                phi[k] /= p;
            }
        }
    }
    
    phi
}

fn main() {
    let n = 1000000; 
    let phi = compute_totients(n);

    let total_fractions: usize = phi.iter().skip(2).sum(); 
    
    println!("The number of distinct fractions a/b such that 1 ≤ a < b ≤ {} is {}", n, total_fractions);
}

