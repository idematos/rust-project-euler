// Problem #62: Cubic Permutations
// https://projecteuler.net/problem=62

use std::collections::HashMap;

fn main() {
    let mut cubes = HashMap::new();

    let mut n = 1;

    while n < 10000 { 
        let cube = n * n * n;
        let key: String = cube.to_string().chars().sorted().collect();

        cubes.entry(key).or_insert(Vec::new()).push(cube);
        
        n += 1;
    }

    let mut min_cube = u64::MAX;

    for group in cubes.values() {
        if group.len() == 3 {
            let min_in_group = *group.iter().min().unwrap();
            if min_in_group < min_cube {
                min_cube = min_in_group;
            }
        }
    }

    if min_cube != u64::MAX {
        println!("The smallest cube with exactly three permutations is {}", min_cube);
    } else {
        println!("No cube with exactly three permutations found.");
    }
}
