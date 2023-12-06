use std::str::Lines;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut l = input.lines();
    let time = parse_line1(&mut l);
    let distance = parse_line1(&mut l);

    let mut part1_answer = 1;
    for i in 0..time.len() {
        part1_answer *= calculate(time[i], distance[i]);
    }

    let mut l = input.lines();
    let time = parse_line2(&mut l);
    let distance = parse_line2(&mut l);
    let part2_answer = calculate(time, distance);

    (part1_answer, part2_answer)
}

fn calculate(time: usize, distance: usize) -> usize {
    let mut count = 0;
    for i in 0..time {
        let x = (time - i) * i;
        if x > distance {
            count += 1;
        }
    }
    count
}

fn parse_line1(l: &mut Lines) -> Vec<usize> {
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
fn parse_line2(l: &mut Lines) -> usize {
    let s = l
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .replace(' ', "");
    s.parse::<usize>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 625968);
        assert_eq!(part2_answer, 43663323);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 288);
        assert_eq!(part2_answer, 71503);
    }
}
