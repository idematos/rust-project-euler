// Problem #54: Poker Hands
// https://projecteuler.net/problem=54

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

fn evaluate_hand(hand: &str) -> HandRank {
    let cards: Vec<&str> = hand.split_whitespace().collect();
    let mut values: Vec<u8> = cards.iter().map(|&card| card[0..1].parse::<u8>().unwrap()).collect();
    let suits: Vec<char> = cards.iter().map(|&card| card.chars().nth(1).unwrap()).collect();

    values.sort_unstable();
    let is_flush = suits.iter().all(|&s| s == suits[0]);
    let is_straight = values.windows(2).all(|w| w[1] == w[0] + 1) || values == vec![10, 11, 12, 13, 14];

    if is_flush && is_straight {
        return if values[0] == 10 {
            HandRank::RoyalFlush
        } else {
            HandRank::StraightFlush
        };
    }

    let mut counts = vec![0; 15]; 
    for &value in &values {
        counts[value as usize] += 1;
    }

    let four_of_a_kind = counts.iter().any(|&count| count == 4);
    let three_of_a_kind = counts.iter().any(|&count| count == 3);
    let pairs = counts.iter().filter(|&&count| count == 2).count();

    if four_of_a_kind {
        return HandRank::FourOfAKind;
    }
    if three_of_a_kind && pairs == 1 {
        return HandRank::FullHouse;
    }
    if is_flush {
        return HandRank::Flush;
    }
    if is_straight {
        return HandRank::Straight;
    }
    if three_of_a_kind {
        return HandRank::ThreeOfAKind;
    }
    if pairs == 2 {
        return HandRank::TwoPair;
    }
    if pairs == 1 {
        return HandRank::OnePair;
    }

    HandRank::HighCard
}

fn compare_hands(hand1: &str, hand2: &str) -> bool {
    let rank1 = evaluate_hand(hand1);
    let rank2 = evaluate_hand(hand2);
    
    rank1 > rank2
}

fn main() {
    let file = File::open("poker.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    
    let mut player1_wins = 0;

    for line in reader.lines() {
        let hand = line.expect("Unable to read line");
        let hands: Vec<&str> = hand.split(" ").collect();
        let player1_hand = hands[0..5].join(" ");
        let player2_hand = hands[5..10].join(" ");
        
        if compare_hands(&player1_hand, &player2_hand) {
            player1_wins += 1;
        }
    }

    println!("Player 1 wins {} hands.", player1_wins);
}
