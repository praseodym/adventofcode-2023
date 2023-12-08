use std::collections::HashMap;

use regex::Regex;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut s = input.split("\n\n");
    let instructions: Vec<char> = s.next().unwrap().trim().chars().collect();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    let re = Regex::new(r"(?<cur>[A-Z0-9]{3}) = \((?<left>[A-Z0-9]{3}), (?<right>[A-Z0-9]{3})\)")
        .unwrap();
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

    let part1_answer = if nodes.contains_key("AAA") {
        calculate_steps(&instructions, &nodes, "AAA")
    } else {
        // input-example3 doesn't have a node named AAA
        0
    };

    let steps: Vec<usize> = nodes
        .iter()
        .filter_map(|(k, _)| if k.ends_with('A') { Some(*k) } else { None })
        .map(|k| calculate_steps(&instructions, &nodes, k))
        .collect();
    let part2_answer = lcm(&steps);

    (part1_answer, part2_answer)
}

fn calculate_steps(
    instructions: &[char],
    nodes: &HashMap<&str, (&str, &str)>,
    start: &str,
) -> usize {
    let mut steps = 0;
    let mut cur = start;
    'outer: loop {
        for instruction in instructions {
            let node = nodes.get(cur).unwrap();
            steps += 1;
            cur = match instruction {
                'L' => node.0,
                'R' => node.1,
                _ => panic!("invalid instruction: {}", instruction),
            };
            if cur.ends_with('Z') {
                break 'outer;
            }
        }
    }
    steps
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(v: &[usize]) -> usize {
    let mut result = v[0];
    for &i in &v[1..] {
        result = result * i / gcd(result, i);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 13207);
        assert_eq!(part2_answer, 12324145107121);
    }

    #[test]
    fn test_input_example1() {
        let (part1_answer, _) = run(include_str!("../input-example1"));
        assert_eq!(part1_answer, 2);
    }

    #[test]
    fn test_input_example2() {
        let (part1_answer, _) = run(include_str!("../input-example2"));
        assert_eq!(part1_answer, 6);
    }

    #[test]
    fn test_input_example3() {
        let (_, part2_answer) = run(include_str!("../input-example3"));
        assert_eq!(part2_answer, 6);
    }
}
