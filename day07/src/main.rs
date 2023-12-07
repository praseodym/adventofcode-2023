use std::cmp::Ordering;

fn main() {
    let (part1_answer, _part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Hand {
    card: Vec<Card>,
    bid: usize,
    hand_type: HandType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
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
    fn from_char(c: char) -> Card {
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

    fn from_str(s: &str) -> Vec<Card> {
        s.chars().map(|c| Card::from_char(c)).collect()
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
        println!("{:?}", card_count);
        match card_count.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [2, 1, 1, 1] => HandType::OnePair,
            [2, 2, 1] => HandType::TwoPair,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [3, 2] => HandType::FullHouse,
            [4, 1] => HandType::FourOfAKind,
            [5] => HandType::FiveOfAKind,
            _ => HandType::None,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type != other.hand_type {
            Some(self.hand_type.cmp(&other.hand_type))
        } else {
            for i in 0..self.card.len() {
                if self.card[i] != other.card[i] {
                    return Some(self.card[i].cmp(&other.card[i]));
                }
            }
            Some(Ordering::Equal)
        }
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
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let mut s = line.split_ascii_whitespace();
            let card = s.next().unwrap().to_string();
            let card = Card::from_str(&card);
            let bid = s.next().unwrap().parse::<usize>().unwrap();
            let hand_type = HandType::get_type(&card);
            let hand = Hand {
                card,
                bid,
                hand_type,
            };
            println!("{:?}", hand);
            hand
        })
        .collect();

    hands.sort();

    let part1_answer = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            println!("i: {}, hand: {:?}", i, hand);
            println!("i: {}, bid: {}", i + 1, hand.bid);
            (i + 1) * hand.bid
        })
        .sum();

    let mut part2_answer = 0;
    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 249748283);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 6440);
        // assert_eq!(part2_answer, 0);
    }
}
