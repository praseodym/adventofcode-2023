use std::collections::HashMap;

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

    let part1_answer = solve(&grid, len, false);
    let part2_answer = solve(&grid, len, true);
    (part1_answer, part2_answer)
}

const NEIGHBOURS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

type Point = (usize, usize);
type Neighbour = ((usize, usize), usize);

fn solve(grid: &[[char; N]; N], len: usize, part2: bool) -> usize {
    let start = (1, 0);
    let end = (len - 2, len - 1);
    assert_eq!(grid[start.1][start.0], '.', "start must be '.'");
    assert_eq!(grid[end.1][end.0], '.', "end must be '.'");

    // approach heavily inspired by https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/23.rs
    let mut graph = HashMap::<Point, Vec<Neighbour>>::new();
    for x in 0..len {
        for y in 0..len {
            let neighbours = match grid[y][x] {
                '#' => continue,
                _ if part2 => &NEIGHBOURS,
                '.' => &NEIGHBOURS,
                '<' => &NEIGHBOURS[0..1],
                '>' => &NEIGHBOURS[1..2],
                '^' => &NEIGHBOURS[2..3],
                'v' => &NEIGHBOURS[3..4],
                _ => unreachable!(),
            };
            let e = graph.entry((x, y)).or_default();
            for &(dx, dy) in neighbours {
                let (nx, ny) = (x as isize + dx, y as isize + dy);
                if nx < 0 || nx >= len as isize || ny < 0 || ny >= len as isize {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if grid[ny][nx] == '#' {
                    continue;
                }
                e.push(((nx, ny), 1));
            }
        }
    }

    let corridors: Vec<_> = graph
        .iter()
        .filter_map(|(k, v)| if v.len() == 2 { Some(*k) } else { None })
        .collect();

    for point in corridors {
        let neighbours = graph.remove(&point).unwrap();
        let [(p1, d1), (p2, d2)] = neighbours.as_slice().try_into().unwrap();
        let n1 = graph.get_mut(&p1).unwrap();
        if let Some(n) = n1.iter_mut().find(|(p, _)| *p == point) {
            *n = (p2, d1 + d2);
        }
        let n2 = graph.get_mut(&p2).unwrap();
        if let Some(n) = n2.iter_mut().find(|(p, _)| *p == point) {
            *n = (p1, d1 + d2);
        }
    }

    dfs(&graph, start, end, 0, &mut [[false; N]; N], part2)
}

fn dfs(
    graph: &HashMap<Point, Vec<Neighbour>>,
    (x, y): Point,
    end: Point,
    steps: usize,
    visited: &mut [[bool; N]; N],
    part2: bool,
) -> usize {
    if (x, y) == end {
        return steps;
    }

    let mut max = 0;
    visited[y][x] = true;
    for &((nx, ny), d) in graph[&(x, y)].iter() {
        if visited[ny][nx] {
            continue;
        }
        max = max.max(dfs(graph, (nx, ny), end, steps + d, visited, part2));
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
