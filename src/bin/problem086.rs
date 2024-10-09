// Problem #86: Cuboid Route
// https://projecteuler.net/problem=86

fn is_perfect_square(x: usize) -> bool {
    let s = (x as f64).sqrt() as usize;
    s * s == x
}

fn count_cuboids(n: usize) -> usize {
    let mut count = 0;

    for a in 1..=n {
        for b in a..=n {
            for c in b..=n {
                if is_perfect_square(a * a + b * b + c * c) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    let target = 1000; 
    let mut n = 0;

    loop {
        n += 1;
        let count = count_cuboids(n);
        
        if count > target {
            break;
        }
    }

    println!("The minimum n such that there are more than 1000 cuboids is {}", n);
}
