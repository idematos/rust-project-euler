// Problem #17: Number Letter Counts
// https://projecteuler.net/problem=17

fn number_to_words(n: usize) -> String {
    let under_twenty = vec![
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
        "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
    ];
    let tens = vec![
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    if n < 20 {
        return under_twenty[n].to_string();
    } else if n < 100 {
        return format!("{}{}", tens[n / 10], under_twenty[n % 10]);
    } else if n < 1000 {
        let remainder = n % 100;
        let hundreds = format!("{}hundred", under_twenty[n / 100]);
        if remainder > 0 {
            return format!("{}and{}", hundreds, number_to_words(remainder));
        } else {
            return hundreds;
        }
    } else if n == 1000 {
        return "onethousand".to_string();
    }

    String::new()
}

fn main() {
    let mut letter_count = 0;
    
    for n in 1..=1000 {
        let word = number_to_words(n);
        letter_count += word.len();
    }

    println!("The total number of letters is {}", letter_count);
}
