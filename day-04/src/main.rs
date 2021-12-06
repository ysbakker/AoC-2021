use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input"));
    let input = get_input::get_input_as_strings(INPUT);
    let (numbers, mut bingo_cards) = get_bingo_input(&input);
    println!(
        "Part 1: {}",
        mark_bingo_cards_and_calculate_winner(&numbers, &mut bingo_cards)
    );
}

/// Returns `(numbers, Vec<Vec<HashSet<u32>>>)`
/// - `Vec<Vec<HashSet<u32>>>`: All bingo cards
/// - `Vec<HashSet<u32>>`: Bingo card
/// - `HashSet<u32>`: Rows + columns of bingo card
fn get_bingo_input(lines: &[String]) -> (Vec<u32>, Vec<Vec<HashSet<u32>>>) {
    let numbers = lines[0]
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut bingo_cards = Vec::<Vec<HashSet<u32>>>::new();
    let mut bingo_card: Vec<HashSet<u32>> = Vec::new();
    let mut initialized = false;
    for line in lines.iter().skip(2) {
        if line.is_empty() {
            bingo_cards.push(bingo_card);
            bingo_card = Vec::new();
            initialized = false;
            continue;
        }
        let line_numbers: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        if !initialized {
            for _ in 0..line_numbers.len() {
                bingo_card.push(HashSet::new())
            }
            initialized = true;
        }

        for (i, number) in line_numbers.iter().enumerate() {
            bingo_card[i].insert(*number);
        }
        bingo_card.push(HashSet::from_iter(line_numbers));
    }
    bingo_cards.push(bingo_card);

    (numbers, bingo_cards)
}

fn mark_bingo_cards_and_calculate_winner(numbers: &[u32], cards: &mut [Vec<HashSet<u32>>]) -> u32 {
    for number in numbers {
        mark_bingo_cards(*number, cards);
        let winner = calculate_winner_score(cards, *number);
        if winner > 0 {
            return winner;
        }
    }
    0
}

fn mark_bingo_cards(number: u32, cards: &mut [Vec<HashSet<u32>>]) {
    for card in cards {
        for numbers in card {
            numbers.remove(&number);
        }
    }
}

fn calculate_winner_score(cards: &mut [Vec<HashSet<u32>>], number: u32) -> u32 {
    for card in cards {
        for numbers in card.clone() {
            if numbers.is_empty() {
                let mut sum = 0;
                for nums in card.iter().skip(5) {
                    sum += nums.iter().sum::<u32>()
                }
                return sum * number;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_bingo_input() {
        const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/test"));
        let input = get_input::get_input_as_strings(INPUT);
        let (numbers, bingo_cards) = get_bingo_input(&input);
        assert_eq!(27, numbers.len());
        assert_eq!(3, bingo_cards.len());
    }
    #[test]
    fn test_mark_bingo_cards_and_calculate_winner() {
        const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/test"));
        let input = get_input::get_input_as_strings(INPUT);
        let (numbers, mut bingo_cards) = get_bingo_input(&input);
        assert_eq!(
            4512,
            mark_bingo_cards_and_calculate_winner(&numbers, &mut bingo_cards)
        );
    }
}
