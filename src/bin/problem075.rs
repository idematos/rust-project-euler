// Problem #75: Singular Integer Right Triangles
// https://projecteuler.net/problem=75

fn main() {
    let limit = 1000;
    let mut perimeter_count = vec![0; limit + 1]; 

    for m in 2.. {
        let m_squared = m * m;
        if m_squared > limit {
            break; 
        }
        
        for n in 1..m {
            if (m - n) % 2 == 1 && gcd(m, n) == 1 { 
                let a = m_squared - n * n;
                let b = 2 * m * n;
                let c = m_squared + n * n;
                let perimeter = a + b + c;

                if perimeter <= limit {
                    let mut k = 1;
                    while k * perimeter <= limit {
                        perimeter_count[k * perimeter] += 1;
                        k += 1;
                    }
                }
            }
        }
    }

    let unique_triangle_count = perimeter_count.iter().filter(|&&count| count == 1).count();

    println!("The number of unique right triangles with integer sides and perimeter ≤ {} is {}", limit, unique_triangle_count);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
