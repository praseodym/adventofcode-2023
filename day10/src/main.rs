use std::cmp;
use std::collections::VecDeque;

fn main() {
    let (part1_answer, _part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let max_x = grid[0].len();
    let max_y = grid.len();

    let mut start = (0, 0);
    'outer: for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                start = (x, y);
                break 'outer;
            }
        }
    }

    let mut part1_answer = 0;

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = VecDeque::from(vec![(start.0, start.1, 0)]);

    // find possible points reachable from start
    if start.0 != 0 && ['-', 'L', 'F'].contains(&grid[start.1][start.0 - 1]) {
        queue.push_back((start.0 - 1, start.1, 1));
    }
    if start.0 < max_x && ['-', '7', 'J'].contains(&grid[start.1][start.0 + 1]) {
        queue.push_back((start.0 + 1, start.1, 1));
    }
    if start.1 != 0 && ['|', '7', 'F'].contains(&grid[start.1 - 1][start.0]) {
        queue.push_back((start.0, start.1 - 1, 1));
    }
    if start.1 < max_y && ['|', 'L', 'J'].contains(&grid[start.1 + 1][start.0]) {
        queue.push_back((start.0, start.1 + 1, 1));
    }

    // find furthest point using BFS
    while let Some((x, y, steps)) = queue.pop_front() {
        if visited[y][x] {
            continue;
        }
        part1_answer = cmp::max(part1_answer, steps);
        visited[y][x] = true;

        let valid: Vec<(isize, isize)> = match grid[y][x] {
            '|' => vec![(0, -1), (0, 1)],
            '-' => vec![(-1, 0), (1, 0)],
            'L' => vec![(0, -1), (1, 0)],
            'J' => vec![(0, -1), (-1, 0)],
            '7' => vec![(-1, 0), (0, 1)],
            'F' => vec![(1, 0), (0, 1)],
            'S' | '.' => continue,
            _ => panic!("unknown char: {}", grid[y][x]),
        };
        for (dx, dy) in valid {
            let x = x.saturating_add_signed(dx);
            let y = y.saturating_add_signed(dy);
            if x >= max_x || y >= max_y || visited[y][x] {
                continue;
            }
            queue.push_back((x, y, steps + 1));
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
        assert_eq!(part1_answer, 6701);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_example1() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example1"));
        assert_eq!(part1_answer, 4);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_example2() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example2"));
        assert_eq!(part1_answer, 8);
        // assert_eq!(part2_answer, 0);
    }
}
