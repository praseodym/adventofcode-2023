fn main() {
    let (part1_answer, _part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut part1_answer = 0;
    for (_, line) in input.lines().enumerate() {
        let mut first = None;
        let mut last = None;
        for char in line.chars() {
            if char.is_digit(10) {
                if first.is_none() {
                    first = Some(char);
                }
                last = Some(char);
            }
        }
        let num = format!("{}{}", first.unwrap(), last.unwrap()).parse::<i32>().unwrap();
        part1_answer += num as usize;
    }
    let part2_answer = 0;
    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (_part1_answer, _part2_answer) = run(include_str!("../input"));
        // assert_eq!(part1_answer, 0);
        // assert_eq!(part2_answer, 0);
    }
}
