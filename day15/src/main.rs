fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut part1_answer = 0;
    for step in input.trim().split(',') {
        part1_answer += hash(step);
    }

    let mut map = vec![Vec::<(&str, usize)>::new(); 256];
    for step in input.trim().split(',') {
        let label = step.split_terminator(&['=', '-'][..]).next().unwrap();
        let operation = step.chars().nth(label.len()).unwrap();
        let index = map[hash(label)].iter().position(|x| x.0 == label);
        match operation {
            '-' => {
                if let Some(i) = index {
                    map[hash(label)].remove(i);
                }
            }
            '=' => {
                let focal_length = step.chars().last().unwrap().to_digit(10).unwrap() as usize;
                if let Some(i) = index {
                    map[hash(label)][i].1 = focal_length;
                } else {
                    map[hash(label)].push((label, focal_length));
                }
            }
            _ => panic!("unknown operation: {}", operation),
        };
    }

    let mut part2_answer = 0;
    for (i, list) in map.iter().enumerate() {
        for (j, lens) in list.iter().enumerate() {
            part2_answer += (1 + i) * (1 + j) * lens.1;
        }
    }

    (part1_answer, part2_answer)
}

fn hash(value: &str) -> usize {
    let mut cur = 0;
    for c in value.chars() {
        cur += c as usize;
        cur *= 17;
        cur %= 256;
    }
    cur
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 513158);
        assert_eq!(part2_answer, 200277);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 1320);
        assert_eq!(part2_answer, 145);
    }
}
