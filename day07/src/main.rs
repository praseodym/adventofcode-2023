use std::cmp::Ordering;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Hand {
    card: Vec<Card>,
    bid: usize,
    hand_type: HandType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum HandType {
    None,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Card {
    fn from_char(c: char, joker: bool) -> Card {
        if joker && c == 'J' {
            return Card::Joker;
        }
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("invalid card"),
        }
    }

    fn from_str(s: &str, joker: bool) -> Vec<Card> {
        s.chars().map(|c| Card::from_char(c, joker)).collect()
    }
}

impl HandType {
    fn get_type(card: &[Card]) -> HandType {
        let mut card_count = std::collections::HashMap::new();
        for c in card {
            let count = card_count.entry(c.clone()).or_insert(0);
            *count += 1;
        }
        let mut card_count: Vec<(Card, usize)> = card_count.into_iter().collect();
        card_count.sort_by(|a, b| b.1.cmp(&a.1));
        let mut card_count: Vec<usize> = card_count.into_iter().map(|x| x.1).collect();
        card_count.sort_by(|a, b| b.cmp(&a));
        let jokers = card.iter().filter(|x| **x == Card::Joker).count();
        match (jokers, card_count.as_slice()) {
            (0, [5]) => HandType::FiveOfAKind,
            (1, [4, 1]) => HandType::FiveOfAKind,
            (2, [3, 2]) => HandType::FiveOfAKind,
            (3, [3, 2]) => HandType::FiveOfAKind,
            (4, [4, 1]) => HandType::FiveOfAKind,
            (5, [5]) => HandType::FiveOfAKind,
            (0, [4, 1]) => HandType::FourOfAKind,
            (1, [3, 1, 1]) => HandType::FourOfAKind,
            (2, [2, 2, 1]) => HandType::FourOfAKind,
            (3, [3, 1, 1]) => HandType::FourOfAKind,
            (0, [3, 2]) => HandType::FullHouse,
            (1, [2, 2, 1]) => HandType::FullHouse,
            (0, [3, 1, 1]) => HandType::ThreeOfAKind,
            (1, [2, 1, 1, 1]) => HandType::ThreeOfAKind,
            (2, [2, 1, 1, 1]) => HandType::ThreeOfAKind,
            (0, [2, 2, 1]) => HandType::TwoPair,
            (0, [2, 1, 1, 1]) => HandType::OnePair,
            (1, [1, 1, 1, 1, 1]) => HandType::OnePair,
            (0, [1, 1, 1, 1, 1]) => HandType::HighCard,
            _ => HandType::None,
        }
    }
}


impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type != other.hand_type {
            self.hand_type.cmp(&other.hand_type)
        } else {
            for i in 0..self.card.len() {
                if self.card[i] != other.card[i] {
                    return self.card[i].cmp(&other.card[i]);
                }
            }
            Ordering::Equal
        }
    }
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
            let card = s.next().unwrap().to_string();
            let card = Card::from_str(&card, joker);
            let bid = s.next().unwrap().parse::<usize>().unwrap();
            let hand_type = HandType::get_type(&card);
            let hand = Hand {
                card,
                bid,
                hand_type,
            };
            hand
        })
        .collect();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            (i + 1) * hand.bid
        })
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
