// Problem #68: Magic 5-gon Ring
// https://projecteuler.net/problem=68

use itertools::permutations;

fn is_magic_5gon(permutation: &[u32]) -> bool {
    let o1 = permutation[0];
    let o2 = permutation[1];
    let o3 = permutation[2];
    let o4 = permutation[3];
    let o5 = permutation[4];

    let i1 = permutation[5];
    let i2 = permutation[6];
    let i3 = permutation[7];
    let i4 = permutation[8];
    let i5 = permutation[9];

    let sum1 = o1 + i1 + i2;
    let sum2 = o2 + i2 + i3;
    let sum3 = o3 + i3 + i4;
    let sum4 = o4 + i4 + i5;
    let sum5 = o5 + i5 + i1;

    sum1 == sum2 && sum2 == sum3 && sum3 == sum4 && sum4 == sum5
}

fn magic_5gon_to_string(permutation: &[u32]) -> String {
    let o1 = permutation[0];
    let o2 = permutation[1];
    let o3 = permutation[2];
    let o4 = permutation[3];
    let o5 = permutation[4];

    let i1 = permutation[5];
    let i2 = permutation[6];
    let i3 = permutation[7];
    let i4 = permutation[8];
    let i5 = permutation[9];

    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        o1, i1, i2, o2, i2, i3, o3, i3, i4, o4, i4, i5, o5, i5, i1
    )
}

fn main() {
    let mut largest_solution = String::new();
    
    let numbers: Vec<u32> = (1..=10).collect();
    let perms = permutations(numbers, 10);

    for perm in perms {
        if perm[0] == 10 { 
            if is_magic_5gon(&perm) {
                let solution = magic_5gon_to_string(&perm);
                if solution.len() == 16 && solution > largest_solution {
                    largest_solution = solution;
                }
            }
        }
    }

    println!("The largest 16-digit solution for the magic 5-gon ring is {}", largest_solution);
}
