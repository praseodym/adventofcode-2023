use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::Iterator;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Hand {
    card: Vec<char>,
    bid: usize,
}

const CARD_ORDER: &[char] = &[
    'X', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = calculate_hand_type(&self.card).cmp(&calculate_hand_type(&other.card));
        if a != Ordering::Equal {
            a
        } else {
            for i in 0..self.card.len() {
                let a = self.card[i];
                let b = other.card[i];
                if a != b {
                    return CARD_ORDER
                        .iter()
                        .position(|x| *x == a)
                        .unwrap()
                        .cmp(&CARD_ORDER.iter().position(|x| *x == b).unwrap());
                }
            }
            Ordering::Equal
        }
    }
}

fn calculate_hand_type(cards: &[char]) -> Vec<usize> {
    let mut card_count = cards.iter().fold(HashMap::new(), |mut counts, card| {
        *counts.entry(*card).or_insert(0) += 1;
        counts
    });
    let joker_count = card_count.remove(&'X').unwrap_or(0);
    if joker_count == 5 {
        return vec![5];
    }
    let mut card_count: Vec<usize> = card_count.into_iter().map(|x| x.1).collect();
    card_count.sort_by(|a, b| b.cmp(a));
    card_count[0] += joker_count;
    card_count
}

fn run(input: &'static str) -> (usize, usize) {
    let part1_answer = calculate_winnings(input, false);
    let part2_answer = calculate_winnings(input, true);
    (part1_answer, part2_answer)
}

fn calculate_winnings(input: &str, joker: bool) -> usize {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let mut s = line.split_ascii_whitespace();
            let card = s
                .next()
                .unwrap()
                .chars()
                .map(|c| if joker && c == 'J' { 'X' } else { c })
                .collect::<Vec<char>>();
            let bid = s.next().unwrap().parse::<usize>().unwrap();
            Hand { card, bid }
        })
        .collect();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 249748283);
        assert_eq!(part2_answer, 248029057);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 6440);
        assert_eq!(part2_answer, 5905);
    }
}
