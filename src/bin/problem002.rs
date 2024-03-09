// Problem #2: Even Fibonacci Numbers
// https://projecteuler.net/problem=2

fn sum_of_even_fibonacci(n: u64) -> u64 {
    let mut sum = 0;

    let mut first_fib_num = 1;
    let mut second_fib_num = 2;
    let mut third_fib_num = 3;

    if n > 1 {
        sum += 2;
        while third_fib_num < n {
            if third_fib_num % 2 == 0 {
                sum += third_fib_num;
            }
            first_fib_num = second_fib_num;
            second_fib_num = third_fib_num;
            third_fib_num = first_fib_num + second_fib_num;
        }
    }

    sum
}

fn main() {
    let n = 4000000;

    println!("\nThe sum of the even-valued Fibonacci terms below {} is {}.", n, sum_of_even_fibonacci(n));
}