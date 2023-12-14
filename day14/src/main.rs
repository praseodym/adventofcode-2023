use std::ops::Rem;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

const N: usize = 100;
const RUNS: usize = 1000000000;

fn run(input: &'static str) -> (usize, usize) {
    let mut platform = [[' '; N]; N];
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            platform[y][x] = c;
            max_x = max_x.max(x);
        }
        max_y = max_y.max(y);
    }
    assert_eq!(max_x, max_y);

    let part1_answer = {
        tilt_north(&mut platform, max_x, max_y);
        calculate_load(&platform, max_y)
    };

    let mut part2_answer = 0;
    let mut seen = Vec::new();
    for i in 0..RUNS {
        for _ in 0..4 {
            tilt_north(&mut platform, max_x, max_y);
            platform = rotate(&platform, max_x, max_y);
        }

        if seen.contains(&platform) {
            let cycle_start = seen.iter().position(|&p| p == platform).unwrap();
            let cycle_length = seen.len() - cycle_start;
            let cycle_index = (RUNS - i - 1).rem(cycle_length);
            part2_answer = calculate_load(&seen[cycle_start + cycle_index], max_y);
            break;
        }

        seen.push(platform);
    }

    (part1_answer, part2_answer)
}

fn tilt_north(platform: &mut [[char; N]; N], max_x: usize, max_y: usize) {
    for y in 0..=max_y {
        for x in 0..=max_x {
            if platform[y][x] == 'O' {
                for dy in 1..=y {
                    if platform[y - dy][x] != '.' {
                        break;
                    }
                    platform[y - dy][x] = 'O';
                    platform[y - dy + 1][x] = '.';
                }
            }
        }
    }
}

fn calculate_load(platform: &[[char; N]; N], max_y: usize) -> usize {
    platform.iter()
        .take(max_y + 1)
        .enumerate()
        .map(|(y, row)| row.iter().filter(|&&c| c == 'O').count() * (max_y - y + 1))
        .sum()
}

fn rotate(platform: &[[char; 100]; 100], max_x: usize, max_y: usize) -> [[char; N]; N] {
    // rotate platform clockwise, returning a new platform
    let mut new_platform = [[' '; N]; N];
    for (y, row) in platform.iter().enumerate().take(max_y + 1) {
        for (x, &cell) in row.iter().enumerate().take(max_x + 1) {
            new_platform[x][max_y - y] = cell;
        }
    }
    new_platform
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 111339);
        assert_eq!(part2_answer, 93736);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 136);
        assert_eq!(part2_answer, 64);
    }
}
