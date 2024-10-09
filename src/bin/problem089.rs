// Problem #89: Roman Numerals
// https://projecteuler.net/problem=89

use std::collections::HashMap;

fn roman_to_int(roman: &str) -> usize {
    let mut total = 0;
    let roman_map: HashMap<char, usize> = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]
    .iter()
    .cloned()
    .collect();

    let chars: Vec<char> = roman.chars().collect();
    let length = chars.len();

    for i in 0..length {
        let value = roman_map[&chars[i]];

        if i + 1 < length && value < roman_map[&chars[i + 1]] {
            total -= value;
        } else {
            total += value;
        }
    }
    total
}

fn int_to_shortest_roman(num: usize) -> String {
    let roman_numerals = vec![
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut result = String::new();
    let mut remaining = num;

    for &(value, symbol) in &roman_numerals {
        while remaining >= value {
            result.push_str(symbol);
            remaining -= value;
        }
    }
    result
}

fn main() {
    let roman_numerals = vec![
        "LXXXVIII",
        "XIII",
        "CDXLIV",
        "XC",
        "XCIV",
        "MMMCMXCIX",
    ];

    let mut total_original_length = 0;
    let mut total_shortest_length = 0;

    for &roman in &roman_numerals {
        let original_length = roman.len();
        let value = roman_to_int(roman);
        let shortest_roman = int_to_shortest_roman(value);
        let shortest_length = shortest_roman.len();

        total_original_length += original_length;
        total_shortest_length += shortest_length;
    }

    let saved_characters = total_original_length - total_shortest_length;
    println!("The total number of characters saved is {}.", saved_characters);
}
