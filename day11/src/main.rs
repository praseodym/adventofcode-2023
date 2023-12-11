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

    let mut galaxies = vec![];
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                let dx = empty_columns.iter().filter(|&&ex| ex < x).count();
                let dy = empty_rows.iter().filter(|&&ey| ey < y).count();
                galaxies.push(((x + dx) as isize, (y + dy) as isize));
            }
        }
    }

    let mut part1_answer = 0;
    let mut distances = vec![vec![0; galaxies.len()]; galaxies.len()];
    for (i, (x1, y1)) in galaxies.iter().enumerate() {
        for (j, (x2, y2)) in galaxies.iter().enumerate() {
            // manhattan distance between (x1, y1) and (x2, y2)
            distances[i][j] = (x1 - x2).abs() + (y1 - y2).abs();
            part1_answer += distances[i][j] as usize;
        }
    }
    part1_answer /= 2;

    let part2_answer = 0;
    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 9742154);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 374);
        // assert_eq!(part2_answer, 0);
    }
}
