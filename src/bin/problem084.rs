// Problem #84: Monopoly Odds
// https://projecteuler.net/problem=84

use rand::Rng;

const BOARD_SIZE: usize = 40;
const GO_TO_JAIL: usize = 30;
const JAIL: usize = 10;
const CHANCE: [usize; 3] = [7, 22, 36];
const COMMUNITY_CHEST: [usize; 2] = [2, 17];

const CHANCE_CARDS: [i32; 16] = [
    -1, -1, -1, 0, 10, 11, 24, 39, 5, -3, -3, -3, -3, -3, -3, -3
];

const COMMUNITY_CHEST_CARDS: [i32; 16] = [0, 10, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];

fn roll_dice() -> (u32, u32) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(1..=6), rng.gen_range(1..=6))
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut position = 0;
    let mut visit_count = vec![0; BOARD_SIZE];

    let mut chance_cards = CHANCE_CARDS;
    let mut community_chest_cards = COMMUNITY_CHEST_CARDS;

    chance_cards.shuffle(&mut rng);
    community_chest_cards.shuffle(&mut rng);

    let mut consecutive_doubles = 0;

    for _ in 0..1_000_000 {
        let (d1, d2) = roll_dice();

        if d1 == d2 {
            consecutive_doubles += 1;
        } else {
            consecutive_doubles = 0;
        }

        if consecutive_doubles == 3 {
            position = JAIL;
            consecutive_doubles = 0;
        } else {
            position = (position + (d1 + d2) as usize) % BOARD_SIZE;

            if position == GO_TO_JAIL {
                position = JAIL;
            } else if CHANCE.contains(&position) {
                let card = chance_cards[0];
                chance_cards.rotate_left(1);
                if card >= 0 {
                    position = card as usize;
                } else if card == -3 {
                    position = (position + BOARD_SIZE - 3) % BOARD_SIZE;
                }
            } else if COMMUNITY_CHEST.contains(&position) {
                let card = community_chest_cards[0];
                community_chest_cards.rotate_left(1);
                if card >= 0 {
                    position = card as usize;
                }
            }
        }

        visit_count[position] += 1;
    }

    let mut most_visited: Vec<(usize, usize)> = visit_count
        .iter()
        .enumerate()
        .map(|(pos, &count)| (pos, count))
        .collect();
    most_visited.sort_by(|a, b| b.1.cmp(&a.1));

    let result = format!(
        "{:02}{:02}{:02}",
        most_visited[0].0, most_visited[1].0, most_visited[2].0
    );

    println!("The six-digit modal string is {}", result);
}
