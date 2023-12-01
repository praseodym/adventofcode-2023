fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut part1_answer = 0;
    for (_, line) in input.lines().enumerate() {
        let mut first = None;
        let mut last = None;
        for char in line.chars() {
            if char.is_digit(10) {
                last = char.to_digit(10);
                if first.is_none() {
                    first = last.clone();
                }
            }
        }
        if let (Some(a), Some(b)) = (first, last) {
            part1_answer += (a * 10 + b) as usize;
        }
    }

    let mut part2_answer = 0;
    for (_, line) in input.lines().enumerate() {
        let mut first = None;
        let mut last = None;
        for i in 0..line.len() {
            let char = line.chars().nth(i).unwrap();
            let remaining = &line[i..];
            last = if char.is_digit(10) {
                char.to_digit(10)
            } else if remaining.starts_with("one") {
                Some(1)
            } else if remaining.starts_with("two") {
                Some(2)
            } else if remaining.starts_with("three") {
                Some(3)
            } else if remaining.starts_with("four") {
                Some(4)
            } else if remaining.starts_with("five") {
                Some(5)
            } else if remaining.starts_with("six") {
                Some(6)
            } else if remaining.starts_with("seven") {
                Some(7)
            } else if remaining.starts_with("eight") {
                Some(8)
            } else if remaining.starts_with("nine") {
                Some(9)
            } else {
                last
            };
            if first.is_none() && last.is_some() {
                first = last.clone();
            }
        }
        part2_answer += (first.unwrap() * 10 + last.unwrap()) as usize;
    }

    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 54953);
        assert_eq!(part2_answer, 53868);
    }

    #[test]
    fn test_input_example1() {
        let (part1_answer, _) = run(include_str!("../input-example1"));
        assert_eq!(part1_answer, 142);
    }

    #[test]
    fn test_input_example2() {
        let (_, part2_answer) = run(include_str!("../input-example2"));
        assert_eq!(part2_answer, 281);
    }
}
