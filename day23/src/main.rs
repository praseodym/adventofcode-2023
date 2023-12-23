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

    let part1_answer = dfs(&grid, start, end, 0, &mut [[false; N]; N], false);
    let part2_answer = dfs(&grid, start, end, 0, &mut [[false; N]; N], true);
    (part1_answer, part2_answer)
}

const NEIGHBOURS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn dfs(
    grid: &[[char; N]; N],
    (x, y): (usize, usize),
    end: (usize, usize),
    steps: usize,
    visited: &mut [[bool; N]; N],
    part2: bool,
) -> usize {
    if (x, y) == end {
        return steps;
    }

    let neighbours = match grid[y][x] {
        _ if part2 => &NEIGHBOURS,
        '.' => &NEIGHBOURS,
        '<' => &NEIGHBOURS[0..1],
        '>' => &NEIGHBOURS[1..2],
        '^' => &NEIGHBOURS[2..3],
        'v' => &NEIGHBOURS[3..4],
        _ => unreachable!(),
    };

    let mut max = 0;
    visited[y][x] = true;
    for &(dx, dy) in neighbours {
        let (nx, ny) = (x.saturating_add_signed(dx), y.saturating_add_signed(dy));
        if visited[ny][nx] || grid[ny][nx] == '#' {
            continue;
        }
        max = max.max(dfs(grid, (nx, ny), end, steps + 1, visited, part2));
    }
    visited[y][x] = false;
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 2386, "incorrect part 1 answer");
        assert_eq!(part2_answer, 6246, "incorrect part 2 answer");
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 94, "incorrect part 1 answer");
        assert_eq!(part2_answer, 154, "incorrect part 2 answer");
    }
}
