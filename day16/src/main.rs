use std::collections::HashSet;

fn main() {
    let (part1_answer, _part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

const N: usize = 111;

fn run(input: &'static str) -> (usize, usize) {
    let mut contraption = [[' '; N]; N];
    let mut len = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            contraption[y + 1][x + 1] = c;
        }
        len = y + 1;
    }

    let mut queue: Vec<((usize, usize), (isize, isize))> = Vec::new();
    let mut seen: HashSet<((usize, usize), (isize, isize))> = HashSet::new();
    let mut energized: HashSet<(usize, usize)> = HashSet::new();
    queue.push(((1, 1), (1, 0)));
    while let Some(((x, y), (dx, dy))) = queue.pop() {
        if x == 0 || y == 0 || x > len || y > len {
            continue;
        }
        if !seen.insert(((x, y), (dx, dy))) {
            continue;
        }
        match contraption[y][x] {
            ' ' => {
                println!("hit empty tile at ({}, {})", x, y);
                continue;
            }
            '.' => {
                queue.push((
                    (x.saturating_add_signed(dx), y.saturating_add_signed(dy)),
                    (dx, dy),
                ));
            }
            '-' => {
                if dy == 0 {
                    queue.push(((x.saturating_add_signed(dx), y), (dx, dy)));
                } else {
                    queue.push(((x - 1, y), (-1, 0)));
                    queue.push(((x + 1, y), (1, 0)));
                }
            }
            '|' => {
                if dx == 0 {
                    queue.push(((x, y.saturating_add_signed(dy)), (dx, dy)));
                } else {
                    queue.push(((x, y - 1), (0, -1)));
                    queue.push(((x, y + 1), (0, 1)));
                }
            }
            '/' => match (dx, dy) {
                (0, 1) => queue.push(((x.saturating_sub(1), y), (-1, 0))),
                (0, -1) => queue.push(((x + 1, y), (1, 0))),
                (1, 0) => queue.push(((x, y.saturating_sub(1)), (0, -1))),
                (-1, 0) => queue.push(((x, y + 1), (0, 1))),
                _ => unreachable!(),
            },
            '\\' => match (dx, dy) {
                (0, 1) => queue.push(((x + 1, y), (1, 0))),
                (0, -1) => queue.push(((x.saturating_sub(1), y), (-1, 0))),
                (1, 0) => queue.push(((x, y + 1), (0, 1))),
                (-1, 0) => queue.push(((x, y.saturating_sub(1)), (0, -1))),
                _ => unreachable!(),
            },
            _ => panic!("unknown char: {}", contraption[y][x]),
        }
        energized.insert((x, y));
    }

    let part1_answer = energized.len();
    let mut part2_answer = 0;
    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 7482);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 46);
        // assert_eq!(part2_answer, 0);
    }
}
