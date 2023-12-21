use std::collections::VecDeque;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

const N: usize = 131;

fn run(input: &'static str) -> (usize, usize) {
    let mut grid = [[false; N]; N];
    let mut len = 0;
    let mut start = (0, 0);
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '.' || c == 'S' {
                grid[y][x] = true;
            }
            if c == 'S' {
                start = (x, y);
            }
        });
        len = y;
    });

    // use BFS to get steps to all points
    let mut steps = [[0; N]; N];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || nx >= len as isize || ny < 0 || ny >= len as isize {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if grid[ny][nx] && steps[ny][nx] == 0 {
                steps[ny][nx] = steps[y][x] + 1;
                queue.push_back((nx, ny));
            }
        }
    }

    // calculate how many points can be visited in 64 steps
    let mut part1_answer = 0;
    for y in 0..=len {
        for x in 0..=len {
            if steps[y][x] > 0 && steps[y][x] <= 64 && steps[y][x] % 2 == 0 {
                part1_answer += 1;
            }
        }
    }

    let part2_answer = 0;
    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 3814, "incorrect part 1 answer");
        // assert_eq!(part2_answer, 0, "incorrect part 2 answer");
    }

    #[test]
    fn test_input_example() {
        let (_part1_answer, _part2_answer) = run(include_str!("../input-example"));
        // assert_eq!(part1_answer, 16, "incorrect part 1 answer");
        // assert_eq!(part2_answer, 0, "incorrect part 2 answer");
    }
}
