use std::collections::HashMap;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

type Workflows<'a> = HashMap<&'a str, Vec<Rule>>;
type Part = [usize; 4];
type Parts = [[usize; 4]];
type Action = &'static str;
type Paths = Vec<Vec<(usize, usize)>>;

enum Rule {
    Comparison {
        idx: usize,
        gt: bool,
        num: usize,
        action: Action,
    },
    Action(Action),
}

fn run(input: &'static str) -> (usize, usize) {
    let (workflows, parts) = parse_input(input);
    let part1_answer = evaluate_parts(&workflows, &parts);
    let part2_answer = acceptable_ranges(&workflows, workflows.get("in").unwrap())
        .iter()
        .map(|r| r.iter().fold(1, |acc, (min, max)| acc * (max - min + 1)))
        .sum();
    (part1_answer, part2_answer)
}

fn parse_input(input: &'static str) -> (Workflows, Vec<Part>) {
    let (w, p) = input.split_once("\n\n").unwrap();

    let mut workflows: Workflows = HashMap::new();
    for line in w.lines() {
        let (name, rules) = line.split_once('{').unwrap();
        let rules: Vec<Rule> = rules
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(|s| {
                if let Some((c, action)) = s.split_once(':') {
                    let var = c.chars().next().unwrap();
                    let idx = "xmas".chars().position(|c| c == var).unwrap();
                    let op = c.chars().nth(1).unwrap();
                    let gt = op == '>';
                    let num = c[2..].parse::<usize>().unwrap();
                    Rule::Comparison {
                        idx,
                        gt,
                        num,
                        action,
                    }
                } else {
                    Rule::Action(s)
                }
            })
            .collect();
        workflows.insert(name, rules);
    }

    let parts = p
        .lines()
        .map(|line| {
            let mut res = [0; 4];
            for (n, v) in res.iter_mut().zip(line[1..line.len() - 1].split(',')) {
                let (_, v) = v.split_once('=').unwrap();
                *n = v.parse::<usize>().unwrap();
            }
            res
        })
        .collect::<Vec<_>>();

    (workflows, parts)
}

fn evaluate_parts(workflows: &Workflows, parts: &Parts) -> usize {
    let mut ratings_sum = 0;
    'parts: for part in parts {
        let mut rules = workflows.get("in").unwrap();
        'workflow: loop {
            for rule in rules {
                let action = match *rule {
                    Rule::Comparison {
                        idx,
                        gt,
                        num,
                        action,
                    } => {
                        let var = part[idx];
                        let result = if gt { var > num } else { var < num };
                        if result {
                            Some(action)
                        } else {
                            None
                        }
                    }
                    Rule::Action(a) => Some(a),
                };
                if let Some(action) = action {
                    if action == "A" {
                        ratings_sum += part.iter().sum::<usize>();
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
    ratings_sum
}

fn acceptable_ranges(workflows: &Workflows, rules: &[Rule]) -> Vec<Vec<(usize, usize)>> {
    let rule = &rules[0];
    match *rule {
        Rule::Action("A") => vec![vec![(1, 4000); 4]],
        Rule::Action("R") => vec![],
        Rule::Action(action) => acceptable_ranges(workflows, workflows.get(action).unwrap()),
        Rule::Comparison {
            idx,
            gt,
            num,
            action,
        } => {
            let mut a = constrain_paths(
                idx,
                gt,
                num,
                acceptable_ranges(workflows, &[Rule::Action(action)]),
            );
            let b = constrain_paths(
                idx,
                !gt,
                if gt { num + 1 } else { num - 1 },
                acceptable_ranges(workflows, &rules[1..]),
            );
            a.extend(b);
            a
        }
    }
}

fn constrain_paths(idx: usize, gt: bool, num: usize, paths: Paths) -> Paths {
    paths
        .iter()
        .flat_map(|r| {
            let mut r = r.clone();
            let (mut min, mut max) = r[idx];
            if gt {
                min = min.max(num + 1);
            } else {
                max = max.min(num - 1);
            }
            if min > max {
                None
            } else {
                r[idx] = (min, max);
                Some(r)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 492702, "incorrect part 1 answer");
        assert_eq!(part2_answer, 138616621185978, "incorrect part 2 answer");
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 19114, "incorrect part 1 answer");
        assert_eq!(part2_answer, 167409079868000, "incorrect part 2 answer");
    }
}
