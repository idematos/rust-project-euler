// Problem #88: Product-sum Numbers
// https://projecteuler.net/problem=88

use std::collections::HashSet;

fn find_product_sum_numbers(max_sum: usize) -> HashSet<usize> {
    let mut product_sum_numbers = HashSet::new();

    for n in 2..=max_sum {
        for k in 2..=n {
            let mut current_sum = 0;
            let mut current_product = 1;
            let mut parts = vec![0; k];

            while current_sum < n {
                for i in 0..k {
                    current_sum += parts[i];
                    current_product *= parts[i];

                    if current_sum == n && current_product == n {
                        product_sum_numbers.insert(n);
                    }
                }

                let mut index = k - 1;
                while index >= 0 {
                    if parts[index] < n {
                        parts[index] += 1;
                        current_sum += 1; 
                        break;
                    } else {
                        current_sum -= parts[index];
                        current_product /= parts[index];
                        parts[index] = 0; 
                    }
                    if index == 0 { break; }
                    index -= 1;
                }
            }
        }
    }

    product_sum_numbers
}

fn main() {
    let max_sum = 120;
    let product_sum_numbers = find_product_sum_numbers(max_sum);
    let total_sum: usize = product_sum_numbers.iter().sum();

    println!("The sum of all distinct product-sum numbers is {}.", total_sum);
}
