// Problem #85: Counting Rectangles
// https://projecteuler.net/problem=85

fn count_rectangles(m: usize, n: usize) -> usize {
    (m * (m + 1) / 2) * (n * (n + 1) / 2)
}

fn main() {
    let target: usize = 2000000;
    let mut closest_diff = usize::MAX;
    let mut best_area = 0;

    for m in 1..100 {
        for n in 1..100 {
            let rects = count_rectangles(m, n);
            let diff = (target as isize - rects as isize).abs() as usize;

            if diff < closest_diff {
                closest_diff = diff;
                best_area = m * n;
            }
        }
    }

    println!("The area of the grid with rectangles closest to 2000000 is {}", best_area);
}
