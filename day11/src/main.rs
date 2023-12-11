use std::cmp;
use std::collections::HashSet;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let (grid, empty_rows, empty_columns) = parse_input(input);

    let part1_answer = sum_of_distances(&grid, &empty_rows, &empty_columns, 1);
    let part2_answer = sum_of_distances(&grid, &empty_rows, &empty_columns, 1000000 - 1);

    (part1_answer, part2_answer)
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<usize>, Vec<usize>) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let max_x = grid[0].len();
    let _max_y = grid.len();

    let empty_rows = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            for c in row.iter() {
                if *c == '#' {
                    return None;
                }
            }
            Some(y)
        })
        .collect::<Vec<_>>();
    let empty_columns = (0..max_x)
        .flat_map(|x| {
            for row in grid.iter() {
                if row[x] == '#' {
                    return None;
                }
            }
            Some(x)
        })
        .collect::<Vec<_>>();
    (grid, empty_rows, empty_columns)
}

fn sum_of_distances(
    grid: &[Vec<char>],
    empty_rows: &[usize],
    empty_columns: &[usize],
    expansion: usize,
) -> usize {
    let mut galaxies = vec![];
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                let dx = empty_columns.iter().filter(|&&ex| ex < x).count() * expansion;
                let dy = empty_rows.iter().filter(|&&ey| ey < y).count() * expansion;
                galaxies.push(((x + dx) as isize, (y + dy) as isize));
            }
        }
    }

    let mut sum = 0;
    let mut seen = HashSet::new();
    for (i, (x1, y1)) in galaxies.iter().enumerate() {
        for (j, (x2, y2)) in galaxies.iter().enumerate() {
            let a = cmp::min(i, j);
            let b = cmp::max(i, j);
            if seen.contains(&(a, b)) {
                continue;
            }
            seen.insert((a, b));

            // manhattan distance between (x1, y1) and (x2, y2)
            sum += ((x1 - x2).abs() + (y1 - y2).abs()) as usize;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 9742154);
        assert_eq!(part2_answer, 411142919886);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, _) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 374);
    }

    #[test]
    fn test_input_example_expanded() {
        let (grid, empty_rows, empty_columns) = parse_input(include_str!("../input-example"));

        let sum = sum_of_distances(&grid, &empty_rows, &empty_columns, 10 - 1);
        assert_eq!(sum, 1030);

        let sum = sum_of_distances(&grid, &empty_rows, &empty_columns, 100 - 1);
        assert_eq!(sum, 8410);
    }
}
