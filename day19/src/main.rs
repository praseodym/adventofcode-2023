use std::collections::HashMap;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut s = input.split("\n\n");
    let mut workflows = HashMap::new();
    for line in s.next().unwrap().lines() {
        let (name, rules) = line.split_once('{').unwrap();
        let rules: Vec<&str> = rules.strip_suffix('}').unwrap().split(',').collect();
        workflows.insert(name, rules);
    }
    let parts = s
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.strip_prefix('{')
                .unwrap()
                .strip_suffix('}')
                .unwrap()
                .split(',')
                .map(|s| {
                    let (_, v) = s.split_once('=').unwrap();
                    v.parse::<usize>().unwrap()
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let mut part1_answer = 0;
    'parts: for part in &parts {
        let mut rules = workflows.get("in").unwrap();
        'workflow: loop {
            for rule in rules {
                let action = if let Some((check, action)) = rule.split_once(':') {
                    let var = check.chars().nth(0).unwrap();
                    let op = check.chars().nth(1).unwrap();
                    let num = check[2..].parse::<usize>().unwrap();
                    let var = match var {
                        'x' => part[0],
                        'm' => part[1],
                        'a' => part[2],
                        's' => part[3],
                        _ => unreachable!(),
                    };
                    let result = match op {
                        '>' => var > num,
                        '<' => var < num,
                        _ => unreachable!(),
                    };
                    if result {
                        Some(action)
                    } else {
                        None
                    }
                } else {
                    Some(*rule)
                };
                if let Some(action) = action {
                    if action == "A" {
                        part1_answer += part.iter().sum::<usize>();
                        continue 'parts;
                    } else if action == "R" {
                        continue 'parts;
                    } else {
                        rules = workflows.get(action).unwrap();
                        continue 'workflow;
                    }
                }
            }
        }
    }

    let part2_answer = 0;
    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 492702, "incorrect part 1 answer");
        // assert_eq!(part2_answer, 0, "incorrect part 2 answer");
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 19114, "incorrect part 1 answer");
        // assert_eq!(part2_answer, 0, "incorrect part 2 answer");
    }
}
