fn main() {
    let (part1_answer, _part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut plan = Vec::new();
    for line in input.lines() {
        let mut s = line.split_ascii_whitespace();
        let dir = s.next().unwrap().chars().next().unwrap();
        let num = s.next().unwrap().parse::<usize>().unwrap();
        let colour = s
            .next()
            .unwrap()
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap();
        plan.push((dir, num, colour));
    }

    let mut terrain = Vec::new();
    let mut pos = (0, 0);
    for instruction in plan.iter() {
        match instruction.0 {
            'U' => {
                for _ in 0..instruction.1 {
                    pos = (pos.0, pos.1 - 1);
                    terrain.push(pos);
                }
            }
            'D' => {
                for _ in 0..instruction.1 {
                    pos = (pos.0, pos.1 + 1);
                    terrain.push(pos);
                }
            }
            'L' => {
                for _ in 0..instruction.1 {
                    pos = (pos.0 - 1, pos.1);
                    terrain.push(pos);
                }
            }
            'R' => {
                for _ in 0..instruction.1 {
                    pos = (pos.0 + 1, pos.1);
                    terrain.push(pos);
                }
            }
            _ => panic!("unknown direction: {}", instruction.0),
        }
    }

    let min_x = terrain.iter().map(|p| p.0).min().unwrap();
    let max_x = terrain.iter().map(|p| p.0).max().unwrap();
    let min_y = terrain.iter().map(|p| p.1).min().unwrap();
    let max_y = terrain.iter().map(|p| p.1).max().unwrap();

    // flood fill exterior area of terrain
    let mut flood_fill = Vec::new();
    let mut frontier = Vec::new();
    frontier.push((min_x - 1, min_y - 1));
    while let Some(pos) = frontier.pop() {
        if flood_fill.contains(&pos) || terrain.contains(&pos) {
            continue;
        }
        if pos.0 < min_x - 1 || pos.1 < min_y - 1 || pos.0 > max_x + 1 || pos.1 > max_y + 1 {
            continue;
        }
        flood_fill.push(pos);
        frontier.push((pos.0 - 1, pos.1));
        frontier.push((pos.0 + 1, pos.1));
        frontier.push((pos.0, pos.1 - 1));
        frontier.push((pos.0, pos.1 + 1));
    }

    let part1_answer = ((max_x - min_x + 3) * (max_y - min_y + 3)) as usize - flood_fill.len();
    let part2_answer = 0;
    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 56923);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 62);
        // assert_eq!(part2_answer, 0);
    }
}
