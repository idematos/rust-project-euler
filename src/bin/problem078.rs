// Problem #78: Coin Partitions
// https://projecteuler.net/problem=78

fn generalized_pentagonal(k: i64) -> i64 {
    k * (3 * k - 1) / 2
}

fn partition_divisible_by(limit: usize, modulus: usize) -> usize {
    let mut partitions = vec![0; limit + 1];
    partitions[0] = 1;  

    for n in 1..=limit {
        let mut k = 1;
        let mut sign = 1;
        partitions[n] = 0;

        while let Some(gp1) = generalized_pentagonal(k).checked_into() {
            if gp1 > n {
                break;
            }
            partitions[n] = (partitions[n] + sign * partitions[n - gp1]) % modulus;

            let gp2 = generalized_pentagonal(-k).checked_into();
            if gp2 <= n {
                partitions[n] = (partitions[n] + sign * partitions[n - gp2]) % modulus;
            }

            k += 1;
            sign = -sign;
        }

        if partitions[n] == 0 {
            return n;
        }
    }

    0 
}

fn main() {
    let modulus = 1000000;
    let limit = 100000;  
    let result = partition_divisible_by(limit, modulus);
    println!("The smallest value of n such that p(n) is divisible by {} is {}", modulus, result);
}
