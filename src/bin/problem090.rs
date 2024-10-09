// Problem #90: Cube Digit Pairs
// https://projecteuler.net/problem=90

use std::collections::HashSet;

fn can_form_number(cube1: &HashSet<usize>, cube2: &HashSet<usize>, number: usize) -> bool {
    let tens = number / 10; 
    let units = number % 10; 
    (cube1.contains(&tens) || cube2.contains(&tens)) &&
    (cube1.contains(&units) || cube2.contains(&units))
}

fn main() {
    let digits = 0..10; 
    let mut count = 0;
    let mut unique_cubes = HashSet::new();

    for cube1 in (0..10).combinations(6) {
        for cube2 in (0..10).combinations(6) {
            let set1: HashSet<usize> = cube1.iter().copied().collect();
            let set2: HashSet<usize> = cube2.iter().copied().collect();

            if (0..100).all(|number| can_form_number(&set1, &set2, number)) {
                let cube_representation = (set1.clone(), set2.clone());
                if !unique_cubes.contains(&cube_representation) {
                    count += 1;
                    unique_cubes.insert(cube_representation);
                }
            }
        }
    }

    println!("The total number of distinct arrangements is {}.", count);
}
