fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut part1_answer = 0;
    let mut part2_answer = 0;

    for pattern_string in input.split("\n\n") {
        let p = pattern_string
            .lines()
            .map(|l| l.chars().map(|c| c == '#').collect::<Vec<bool>>())
            .collect::<Vec<Vec<bool>>>();

        // check for vertical reflections over each column
        'outer: for i in 1..p.get(0).unwrap().len() {
            for y in 0..p.len() {
                let line = p.get(y).unwrap();
                for x1 in 0..i {
                    let x2 = 2 * i - x1 - 1;
                    if x2 >= line.len() {
                        continue;
                    }
                    if line.get(x1).unwrap() != line.get(x2).unwrap() {
                        continue 'outer;
                    }
                }
            }
            part1_answer += i;
            break;
        }

        // check for horizontal reflections over each row
        'outer: for i in 1..p.len() {
            for y1 in 0..i {
                let y2 = 2 * i - y1 - 1;
                if y2 >= p.len() {
                    continue;
                }
                if p.get(y1).unwrap() != p.get(y2).unwrap() {
                    continue 'outer;
                }
            }
            part1_answer += i * 100;
            break;
        }

        // check for vertical reflections over each column, repairing a smudge
        'outer: for i in 1..p.get(0).unwrap().len() {
            let mut smudges = 0;
            for y in 0..p.len() {
                let line = p.get(y).unwrap();
                for x1 in 0..i {
                    let x2 = 2 * i - x1 - 1;
                    if x2 >= line.len() {
                        continue;
                    }
                    if line.get(x1).unwrap() != line.get(x2).unwrap() {
                        smudges += 1;
                        if smudges > 1 {
                            continue 'outer;
                        }
                    }
                }
            }
            if smudges == 1 {
                part2_answer += i;
                break;
            }
        }

        // check for horizontal reflections over each row, repairing a smudge
        'outer: for i in 1..p.len() {
            let mut smudges = 0;
            for y1 in 0..i {
                let y2 = 2 * i - y1 - 1;
                if y2 >= p.len() {
                    continue;
                }
                let line1 = p.get(y1).unwrap();
                let line2 = p.get(y2).unwrap();
                for x in 0..line1.len() {
                    if line1.get(x).unwrap() != line2.get(x).unwrap() {
                        smudges += 1;
                        if smudges > 1 {
                            continue 'outer;
                        }
                    }
                }
            }
            if smudges == 1 {
                part2_answer += i * 100;
                break;
            }
        }
    }

    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 31265);
        assert_eq!(part2_answer, 39359);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 405);
        assert_eq!(part2_answer, 400);
    }
}
