// Problem #87: Prime Power Triples
// https://projecteuler.net/problem=87

fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    let mut primes = Vec::new();

    is_prime[0] = false; 
    is_prime[1] = false; 

    for num in 2..=limit {
        if is_prime[num] {
            primes.push(num);
            let mut multiple = num * num;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += num;
            }
        }
    }

    primes
}

fn main() {
    let limit = 50000;
    
    let primes = sieve_of_eratosthenes((limit as f64).sqrt() as usize + 1);
    
    let mut distinct_sums = std::collections::HashSet::new();

    for &p1 in &primes {
        let p1_sq = p1 * p1;
        if p1_sq >= limit {
            break;
        }

        for &p2 in &primes {
            let p2_cubed = p2 * p2 * p2;
            if p1_sq + p2_cubed >= limit {
                break;
            }

            for &p3 in &primes {
                let p3_fourth = p3 * p3 * p3 * p3;
                let sum = p1_sq + p2_cubed + p3_fourth;

                if sum < limit {
                    distinct_sums.insert(sum);
                } else {
                    break; 
                }
            }
        }
    }

    println!("The number of distinct sums is {}.", distinct_sums.len());
}
