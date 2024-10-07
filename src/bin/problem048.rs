// Problem #48: Self Powers
// https://projecteuler.net/problem=48

fn main() {
    let modulo = 10_u64.pow(10); 
    let mut sum = 0_u64;

    for n in 1..=1000 {
        let mut temp = 1_u64;
        
        for _ in 0..n {
            temp = (temp * n as u64) % modulo;
        }
        
        sum = (sum + temp) % modulo;
    }


    println!("The last 10 digits of the series are {:010}", sum);
}
