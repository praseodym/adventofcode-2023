use std::str::Lines;

fn main() {
    let (part1_answer, _part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut l = input.lines();
    let time: Vec<usize> = parse_line(&mut l);
    let distance: Vec<usize> = parse_line(&mut l);

    let mut part1_answer = 1;
    for i in 0..time.len() {
        let t = time[i];
        let d = distance[i];
        let mut count = 0;
        for j in 0..t {
            let x = (t - j) * j;
            if x > d {
                count += 1;
            }
        }
        part1_answer *= count;
    }

    let mut part2_answer = 0;
    (part1_answer, part2_answer)
}

fn parse_line(l: &mut Lines) -> Vec<usize> {
    l.next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 625968);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 288);
        // assert_eq!(part2_answer, 0);
    }
}
