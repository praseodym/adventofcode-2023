use std::collections::HashSet;

use regex::Regex;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let re = Regex::new(r"Card +(?<id>\d+): (?<winning>[\d ]+) \| (?<own>[\d ]+)").unwrap();
    let mut num_cards: Vec<usize> = vec![1; input.lines().count()];

    let mut part1_answer = 0;
    for (i, line) in input.lines().enumerate() {
        for cap in re.captures_iter(line) {
            let winning: HashSet<_> = cap["winning"]
                .split_ascii_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let own: HashSet<_> = cap["own"]
                .split_ascii_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let num_winning = winning.intersection(&own).count();
            if num_winning > 0 {
                part1_answer += 2usize.pow((num_winning - 1) as u32);
            }
            for j in 1..=num_winning {
                num_cards[i + j] += num_cards[i];
            }
        }
    }
    let part2_answer = num_cards.iter().sum();
    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 23750);
        assert_eq!(part2_answer, 13261850);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 13);
        assert_eq!(part2_answer, 30);
    }
}
