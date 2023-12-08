use std::collections::HashMap;

use regex::Regex;

fn main() {
    let (part1_answer, _part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut s = input.split("\n\n");
    let instructions: Vec<char> = s.next().unwrap().trim().chars().collect();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    let re = Regex::new(r"(?<cur>[A-Z]{3}) = \((?<left>[A-Z]{3}), (?<right>[A-Z]{3})\)").unwrap();
    for line in s.next().unwrap().lines() {
        let captures = re.captures(line).unwrap();
        nodes.insert(
            captures.name("cur").unwrap().as_str(),
            (
                captures.name("left").unwrap().as_str(),
                captures.name("right").unwrap().as_str(),
            ),
        );
    }

    let mut part1_answer = 0;
    let mut cur = "AAA";
    'outer: loop {
        for instruction in &instructions {
            let node = nodes.get(cur).unwrap();
            part1_answer += 1;
            match instruction {
                'L' => cur = node.0,
                'R' => cur = node.1,
                _ => panic!("invalid instruction: {}", instruction),
            }
            if cur == "ZZZ" {
                break 'outer;
            }
        }
    }

    let mut part2_answer = 0;
    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 13207);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_example1() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example1"));
        assert_eq!(part1_answer, 2);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_example2() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example2"));
        assert_eq!(part1_answer, 6);
        // assert_eq!(part2_answer, 0);
    }
}
