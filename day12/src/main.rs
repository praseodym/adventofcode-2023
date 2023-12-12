fn main() {
    let (part1_answer, _part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut part1_answer = 0;
    for line in input.lines() {
        let mut s = line.split_ascii_whitespace();
        let record = s.next().unwrap();
        let group = s
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        part1_answer += brute_force(record, &group, 0);
    }

    let mut part2_answer = 0;
    (part1_answer, part2_answer)
}

fn brute_force(record: &str, group: &[usize], i: usize) -> usize {
    let mut possibilities = 0;
    if i == record.len() {
        let option = record
            .split('.')
            .filter(|r| !r.is_empty())
            .map(|r| r.len())
            .collect::<Vec<_>>();
        return if option == group { 1 } else { 0 };
    }
    if record.chars().nth(i).unwrap() == '?' {
        let mut record = record.to_string();
        record.replace_range(i..i + 1, ".");
        possibilities += brute_force(&record, group, i + 1);
        let mut record = record.to_string();
        record.replace_range(i..i + 1, "#");
        possibilities += brute_force(&record, group, i + 1);
    } else {
        possibilities += brute_force(record, group, i + 1);
    }
    possibilities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 7653);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 21);
        // assert_eq!(part2_answer, 0);
    }
}
