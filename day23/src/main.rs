fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

const N: usize = 141;

fn run(input: &'static str) -> (usize, usize) {
    let len = input.lines().next().unwrap().len();
    let mut grid = [[' '; N]; N];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            grid[y][x] = c;
        });
    });
    let start = (1, 0);
    let end = (len - 2, len - 1);
    assert_eq!(grid[start.1][start.0], '.', "start must be '.'");
    assert_eq!(grid[end.1][end.0], '.', "end must be '.'");

    let mut max_steps = 0;
    let mut stack = Vec::new();
    stack.push((start, 0, [[false; N]; N]));
    while let Some(((x, y), steps, mut visited)) = stack.pop() {
        visited[y][x] = true;
        if (x, y) == end {
            if steps > max_steps {
                max_steps = steps;
            }
            continue;
        }
        let neighbours = match grid[y][x] {
            '#' => continue,
            '.' => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
            '>' => vec![(1, 0)],
            '<' => vec![(-1, 0)],
            '^' => vec![(0, -1)],
            'v' => vec![(0, 1)],
            _ => unreachable!(),
        };
        for (dx, dy) in neighbours {
            let (nx, ny) = (x.saturating_add_signed(dx), y.saturating_add_signed(dy));
            if visited[ny][nx] || grid[ny][nx] == '#' {
                continue;
            }
            stack.push(((nx, ny), steps + 1, visited));
        }
    }

    let part1_answer = max_steps;
    let part2_answer = 0;
    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 2386, "incorrect part 1 answer");
        // assert_eq!(part2_answer, 0, "incorrect part 2 answer");
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 94, "incorrect part 1 answer");
        // assert_eq!(part2_answer, 0, "incorrect part 2 answer");
    }
}
