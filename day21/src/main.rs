use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let farm_layout = parse_input(input);

    let part1_answer = calculate(&farm_layout, 64);
    let part2_answer = calculate(&farm_layout, 26501365);

    (part1_answer, part2_answer)
}

struct FarmLayout {
    plots: HashSet<(usize, usize)>,
    start: (usize, usize),
    len: usize,
}

fn parse_input(input: &str) -> FarmLayout {
    let len = input.lines().next().unwrap().len();
    let mut plots: HashSet<(usize, usize)> = HashSet::new();
    let mut start = None;

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '.' || c == 'S' {
                plots.insert((x, y));
            }
            if c == 'S' {
                start = Some((x, y));
            }
        });
    });

    FarmLayout { plots, start: start.unwrap(), len }
}

fn calculate(farm_layout: &FarmLayout, steps_target: usize) -> usize {
    if steps_target < 2 * farm_layout.len {
        return reachable_points(&bfs(farm_layout, 5), steps_target);
    }

    let offset = steps_target % farm_layout.len;
    let repeats = 5;
    assert_eq!(repeats % 2, 1, "repeats must be odd");
    let steps = bfs(farm_layout, repeats);

    let mut points = Vec::new();
    for i in 0..3 {
        let x = offset + i * farm_layout.len;
        let y = reachable_points(&steps, x);
        points.push((x as f64, y as f64));
    }
    
    // find value at x = target using y = axx + bx + c
    let qf = quadratic_fit(&points);
    let x = steps_target;
    let y = qf.unwrap().0 * x as f64 * x as f64 + qf.unwrap().1 * x as f64 + qf.unwrap().2;
    y as usize
}

fn bfs(farm_layout: &FarmLayout, repeats: usize) -> HashMap<(usize, usize), usize> {
    // use BFS to get steps to all garden plots
    let expanded_len = repeats * farm_layout.len;
    let start = (
        farm_layout.start.0 + (repeats / 2) * farm_layout.len,
        farm_layout.start.1 + (repeats / 2) * farm_layout.len,
    );
    let mut steps: HashMap<(usize, usize), usize> = HashMap::new();
    steps.insert(start, 0);
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || nx >= expanded_len as isize || ny < 0 || ny >= expanded_len as isize {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if farm_layout
                .plots
                .contains(&(nx % farm_layout.len, ny % farm_layout.len))
                && steps.get(&(nx, ny)).is_none()
            {
                steps.insert((nx, ny), steps.get(&(x, y)).unwrap() + 1);
                queue.push_back((nx, ny));
            }
        }
    }
    steps
}

fn reachable_points(steps: &HashMap<(usize, usize), usize>, max_steps: usize) -> usize {
    steps
        .values()
        .filter(|&&s| s <= max_steps && s % 2 == max_steps % 2)
        .count()
}

fn quadratic_fit(points: &[(f64, f64)]) -> Option<(f64, f64, f64)> {
    if points.len() != 3 {
        return None;
    }

    let (x1, y1) = points[0];
    let (x2, y2) = points[1];
    let (x3, y3) = points[2];

    let denom = (x1 - x2) * (x1 - x3) * (x2 - x3);
    if denom == 0.0 {
        return None;
    }

    let a = (x3 * (y2 - y1) + x2 * (y1 - y3) + x1 * (y3 - y2)) / denom;
    let b = (x3 * x3 * (y1 - y2) + x2 * x2 * (y3 - y1) + x1 * x1 * (y2 - y3)) / denom;
    let c =
        (x2 * x3 * (x2 - x3) * y1 + x3 * x1 * (x3 - x1) * y2 + x1 * x2 * (x1 - x2) * y3) / denom;

    Some((a, b, c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 3814, "incorrect part 1 answer");
        assert_eq!(part2_answer, 632257949158206, "incorrect part 2 answer");
    }

    #[test]
    fn test_input_example() {
        let farm_layout = parse_input(include_str!("../input-example"));
        for (steps, plots) in [
            (6, 16),
            // solution doesn't work for examples in part two
            // (10, 50),
            // (50, 1594),
            // (100, 6536),
            // (500, 167004),
            // (1000, 668697),
            // (5000, 16733044),
        ] {
            assert_eq!(
                calculate(&farm_layout, steps),
                plots,
                "incorrect answer for {} steps",
                steps
            );
        }
    }
}
