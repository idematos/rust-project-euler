// Problem #91: Right-angled Triangles
// https://projecteuler.net/problem=91

fn count_right_angled_triangles(n: usize) -> usize {
    let mut total_count = 0;

    for x in 0..=n {
        for y in 0..=n {
            let horizontal_count = x * (n - y); 
            let vertical_count = y * (n - x); 

            total_count += horizontal_count + vertical_count;
        }
    }

    total_count
}

fn main() {
    let n = 50;
    let total_triangles = count_right_angled_triangles(n);
    println!("The number of right-angled triangles is {}.", total_triangles);
}
