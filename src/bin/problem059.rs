// Problem #59: XOR Decryption
// https://projecteuler.net/problem=59

use std::fs;

fn xor_decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    ciphertext.iter()
        .enumerate()
        .map(|(i, &c)| c ^ key[i % key.len()])
        .collect()
}

fn is_valid_english(text: &[u8]) -> bool {
    let valid_chars = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ";
    text.iter().all(|&c| valid_chars.contains(&c))
}

fn main() {
    let data = fs::read_to_string("cipher.txt").expect("Unable to read file");
    let ciphertext: Vec<u8> = data.split(',')
        .filter_map(|s| s.trim().parse::<u8>().ok())
        .collect();

    let mut max_sum = 0;
    let mut best_message = Vec::new();

    for a in 97..=122 { 
        for b in 97..=122 { 
            for c in 97..=122 { 
                let key = vec![a as u8, b as u8, c as u8];
                let decrypted_message = xor_decrypt(&ciphertext, &key);

                if is_valid_english(&decrypted_message) {
                    let ascii_sum: u32 = decrypted_message.iter().map(|&c| c as u32).sum();
                    if ascii_sum > max_sum {
                        max_sum = ascii_sum;
                        best_message = decrypted_message;
                    }
                }
            }
        }
    }

    println!("The maximum ASCII sum of the original message is {}", max_sum);
}
