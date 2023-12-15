fn main() {
    let (part1_answer, _part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut part1_answer = 0;
    for step in input.trim().split(',') {
        let mut cur = 0;
        for c in step.chars() {
            let c = c as usize;
            cur += c;
            cur *= 17;
            cur %= 256;
        }
        part1_answer += cur;
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
        assert_eq!(part1_answer, 513158);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 1320);
        // assert_eq!(part2_answer, 0);
    }
}
