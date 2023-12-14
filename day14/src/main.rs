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

    tilt_north(&mut platform, max_x, max_y);

    let mut part1_answer = 0;
    let mut part2_answer = 0;

    let mut seen = Vec::new();
    for i in 0..RUNS {
        tilt_north(&mut platform, max_x, max_y);
        if part1_answer == 0 {
            part1_answer = calculate_load(&platform, max_x, max_y);
        }
        tilt_west(&mut platform, max_x, max_y);
        tilt_south(&mut platform, max_x, max_y);
        tilt_east(&mut platform, max_x, max_y);

        if seen.contains(&platform) {
            let cycle_start = seen.iter().position(|&p| p == platform).unwrap();
            let cycle_length = seen.len() - cycle_start;
            let cycle_index = (RUNS - i - 1).rem(cycle_length);
            part2_answer = calculate_load(&seen[cycle_start + cycle_index], max_x, max_y);
            break;
        }

        seen.push(platform);
    }

    (part1_answer, part2_answer)
}

fn calculate_load(platform: &[[char; N]; N], max_x: usize, max_y: usize) -> usize {
    let mut load = 0;
    for y in 0..=max_y {
        for x in 0..=max_x {
            if platform[y][x] == 'O' {
                load += max_y - y + 1;
            }
        }
    }
    load
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

fn tilt_west(platform: &mut [[char; 100]; 100], max_x: usize, max_y: usize) {
    for x in 0..=max_x {
        for y in 0..=max_y {
            if platform[y][x] == 'O' {
                for dx in 1..=x {
                    if platform[y][x - dx] != '.' {
                        break;
                    }
                    platform[y][x - dx] = 'O';
                    platform[y][x - dx + 1] = '.';
                }
            }
        }
    }
}

fn tilt_south(platform: &mut [[char; N]; N], max_x: usize, max_y: usize) {
    for y in (0..=max_y).rev() {
        for x in 0..=max_x {
            if platform[y][x] == 'O' {
                for dy in 1..=(max_y - y) {
                    if platform[y + dy][x] != '.' {
                        break;
                    }
                    platform[y + dy][x] = 'O';
                    platform[y + dy - 1][x] = '.';
                }
            }
        }
    }
}

fn tilt_east(platform: &mut [[char; 100]; 100], max_x: usize, max_y: usize) {
    for x in (0..=max_x).rev() {
        for y in 0..=max_y {
            if platform[y][x] == 'O' {
                for dx in 1..=(max_x - x) {
                    if platform[y][x + dx] != '.' {
                        break;
                    }
                    platform[y][x + dx] = 'O';
                    platform[y][x + dx - 1] = '.';
                }
            }
        }
    }
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
