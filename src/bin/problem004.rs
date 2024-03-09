// Problem #4: Largest palindrome product
// https://projecteuler.net/problem=4

fn find_largest_palindrome() -> u64 {
    'outer: for num in (10000..998001).rev() {
        for factor in (100..999).rev() {
            if num % factor == 0 {
                if (num / factor).to_string().len() == 3 {
                    let str_num = num.to_string();
                    for k in 0..(str_num.len() / 2) {
                        if str_num.chars().nth(k) != str_num.chars().nth(str_num.len() - k - 1) {
                            continue 'outer;
                        }
                    }

                    return num;
                }
            }
        }
    }
    0
}

fn main() {
    println!("\nThe largest palindrome made from the product of two 3-digit numbers is {}.", find_largest_palindrome());
}
