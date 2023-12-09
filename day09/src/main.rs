fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (isize, isize) {
    let mut part1_answer = 0;
    let mut part2_answer = 0;
    for line in input.lines() {
        let values: Vec<isize> = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        let mut extrapolation: Vec<Vec<isize>> = vec![values];
        loop {
            let mut differences: Vec<isize> = vec![];
            let mut sum = 0;
            let last = extrapolation.last().unwrap();
            for i in 1..last.len() {
                let d = last[i] - last[i - 1];
                differences.push(d);
                sum += d.abs();
            }
            if sum == 0 {
                break;
            }
            extrapolation.push(differences);
        }
        extrapolation.reverse();

        part1_answer += extrapolation
            .iter()
            .map(|x| *x.last().unwrap())
            .sum::<isize>();
        part2_answer += extrapolation
            .iter()
            .map(|x| *x.first().unwrap())
            .fold(0, |acc, x| x - acc);
    }
    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 1647269739);
        assert_eq!(part2_answer, 864);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 114);
        assert_eq!(part2_answer, 2);
    }
}
