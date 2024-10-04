// Project Euler #39: Integer Right Triangles
// https://projecteuler.net/problem=39

fn main() {
    let limit = 1000;
    let mut max_count = 0;
    let mut best_perimeter = 0;

    for p in 1..=limit {
        let mut count = 0;

        for a in 1..(p / 2) {
            for b in a..(p / 2) { 
                let c = p - a - b; 

                if a * a + b * b == c * c {
                    count += 1; 
                }
            }
        }

        if count > max_count {
            max_count = count;
            best_perimeter = p;
        }
    }

    println!("The perimeter with the maximum number of right triangles is {}", best_perimeter);
    println!("The maximum number of right triangles for any perimeter p less than or equal 1000 is {}", max_count);
}
