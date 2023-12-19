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

    let part1_answer = evaluate_parts(&workflows, &parts);

    let result = acceptable_ranges(&workflows, workflows.get("in").unwrap());
    let part2_answer: usize = result
        .iter()
        .map(|r| r.iter().fold(1, |acc, (min, max)| acc * (max - min + 1)))
        .sum();

    (part1_answer, part2_answer)
}

fn evaluate_parts(workflows: &HashMap<&str, Vec<&str>>, parts: &[Vec<usize>]) -> usize {
    let mut part1_answer = 0;
    'parts: for part in parts {
        let mut rules = workflows.get("in").unwrap();
        'workflow: loop {
            for rule in rules {
                let action = if let Some((check, action)) = rule.split_once(':') {
                    let var = check.chars().next().unwrap();
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
    part1_answer
}

fn acceptable_ranges(
    workflows: &HashMap<&str, Vec<&str>>,
    rules: &[&str],
) -> Vec<Vec<(usize, usize)>> {
    let rule = rules[0];
    let a = if let Some((check, action)) = rule.split_once(':') {
        let var = check.chars().next().unwrap();
        let op = check.chars().nth(1).unwrap();
        let num = check[2..].parse::<usize>().unwrap();
        let var = match var {
            'x' => 0usize,
            'm' => 1usize,
            'a' => 2usize,
            's' => 3usize,
            _ => unreachable!(),
        };
        let gt = match op {
            '>' => true,
            '<' => false,
            _ => unreachable!(),
        };
        let num_acceptable = match op {
            '>' => num + 1,
            '<' => num - 1,
            _ => unreachable!(),
        };
        let mut a = constrain_paths(var, gt, num, acceptable_ranges(workflows, &[action]));
        let b = constrain_paths(
            var,
            !gt,
            num_acceptable,
            acceptable_ranges(workflows, &rules[1..]),
        );
        a.extend(b);
        a
    } else if rule == "A" {
        vec![vec![(1, 4000usize); 4]]
    } else if rule == "R" {
        vec![]
    } else {
        acceptable_ranges(workflows, workflows.get(rule).unwrap())
    };
    a
}

fn constrain_paths(
    var: usize,
    gt: bool,
    num: usize,
    paths: Vec<Vec<(usize, usize)>>,
) -> Vec<Vec<(usize, usize)>> {
    let mut new_ranges = vec![];
    for r in &paths {
        let mut n = r.clone();
        let (mut min, mut max) = r[var];
        if gt {
            min = min.max(num + 1);
        } else {
            max = max.min(num - 1);
        }
        if min > max {
            continue;
        }
        n[var] = (min, max);
        new_ranges.push(n)
    }
    new_ranges
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
