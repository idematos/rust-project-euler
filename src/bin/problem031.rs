// Problem #31: Coin Sums
// https://projecteuler.net/problem=31

use std::collections::HashMap;

fn count_different_ways() -> u32 {
    let coins = &[200, 100, 50, 20, 10, 5, 2, 1];
    let amount = 200;

    let mut ways = HashMap::new();
    ways.insert(0, 1);

    for i in 1..coins.len() {
        for j in coins[i]..=amount {
            if ways.contains_key(&j) {
                ways.insert(j, ways[&j] + ways[&(j-coins[i])]);
            } else {
                ways.insert(j, 1);
            }
        }
    }

    ways[&amount]
}

fn main() {
    println!("\n£2 can be made using any number of coins in {} different ways.", count_different_ways());
}
