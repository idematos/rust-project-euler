// Problem #94: Almost Equilateral Triangles
// https://projecteuler.net/problem=94

fn is_perfect_square(n: f64) -> bool {
    let sq = n.sqrt();
    sq == sq.floor() 
}

fn main() {
    let mut total_perimeter = 0;
    let limit = 1_000_000_000;

    for a in 1.. {
        let b1 = a - 1;
        let perimeter1 = 2 * a + b1;
        
        if perimeter1 > limit {
            break; 
        }
        
        let s1 = (2 * a + b1) as f64 / 2.0; 
        let area1_squared = s1 * (s1 - a as f64) * (s1 - a as f64) * (s1 - b1 as f64);
        
        if area1_squared > 0.0 && is_perfect_square(area1_squared) {
            total_perimeter += perimeter1;
        }

        let b2 = a + 1;
        let perimeter2 = 2 * a + b2;

        if perimeter2 > limit {
            break; 
        }
        
        let s2 = (2 * a + b2) as f64 / 2.0; 
        let area2_squared = s2 * (s2 - a as f64) * (s2 - a as f64) * (s2 - b2 as f64);
        
        if area2_squared > 0.0 && is_perfect_square(area2_squared) {
            total_perimeter += perimeter2;
        }
    }

    println!("The sum of the perimeters is {}.", total_perimeter);
}
