use std::ops::Rem;

fn main() {
    let (part1_answer, _part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

const N: usize = 100;

fn run(input: &'static str) -> (usize, usize) {
    let mut platform = [[' '; N]; N];
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            platform[y][x] = c;
            max_x = max_x.max(x);
        }
        max_y = max_y.max(y);
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            if platform[y][x] == 'O' {
                for dy in 1..=y {
                    if platform[y - dy][x] != '.' {
                        break;
                    }
                    platform[y - dy][x] = 'O';
                    platform[y - dy + 1][x] = '.';
                }
            }
        }
    }

    println!("platform:");
    for y in 0..=max_y {
        for x in 0..=max_x {
            print!("{}", platform[y][x])
        }
        println!();
    }

    let mut part1_answer = 0;
    for y in 0..=max_y {
        for x in 0..=max_x {
            if platform[y][x] == 'O' {
                part1_answer += max_y - y + 1;
            }
        }
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
        assert_eq!(part1_answer, 111339);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 136);
        // assert_eq!(part2_answer, 0);
    }
}
