use regex::Regex;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

#[derive(Debug, Copy, Clone)]
struct Gear {
    count: usize,
    ratio: usize,
}

fn run(input: &'static str) -> (usize, usize) {
    let mut symbols = [[false; 140]; 140];
    let mut gears: [[Option<Gear>; 140]; 140] = [[None; 140]; 140];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            symbols[i][j] = !(c == '.' || c.is_ascii_digit());
            if c == '*' {
                gears[i][j] = Some(Gear { count: 0, ratio: 1 });
            }
        }
    }

    let mut part1_answer = 0;
    let re = Regex::new(r"(\d+)").unwrap();
    for (i, line) in input.lines().enumerate() {
        for cap in re.captures_iter(line) {
            let d = cap[0].parse::<usize>().unwrap();
            let start = cap.get(0).unwrap().start();
            let end = cap.get(0).unwrap().end();
            let is_adjacent = is_adjacent_to_symbol(&symbols, i, start, end);
            if is_adjacent {
                part1_answer += d;
            }
            check_gear(&mut gears, i, start, end, d);
        }
    }

    let mut part2_answer = 0;
    gears.iter().for_each(|row| {
        row.iter().for_each(|gear| {
            if let Some(gear) = gear {
                if gear.count == 2 {
                    part2_answer += gear.ratio;
                }
            }
        })
    });

    (part1_answer, part2_answer)
}

fn is_adjacent_to_symbol(symbols: &[[bool; 140]], i: usize, start: usize, end: usize) -> bool {
    let mut adjacent = false;
    for j in start..end {
        // check if i,j is adjacent to a symbol
        if i > 0 {
            adjacent |= symbols[i - 1][j];
        }
        if i < 139 {
            adjacent |= symbols[i + 1][j];
        }
        if j > 0 {
            adjacent |= symbols[i][j - 1];
        }
        if j < 139 {
            adjacent |= symbols[i][j + 1];
        }
        if i > 0 && j > 0 {
            adjacent |= symbols[i - 1][j - 1];
        }
        if j < 139 && i > 0 {
            adjacent |= symbols[i - 1][j + 1];
        }
        if i < 139 && j > 0 {
            adjacent |= symbols[i + 1][j - 1];
        }
        if i < 139 && j < 139 {
            adjacent |= symbols[i + 1][j + 1];
        }
    }
    adjacent
}

fn check_gear(
    gears: &mut [[Option<Gear>; 140]; 140],
    i: usize,
    start: usize,
    end: usize,
    d: usize,
) {
    for j in start..end {
        if i > 0 {
            if let Some(mut gear) = gears[i - 1][j] {
                gear.count += 1;
                gear.ratio *= d;
                gears[i - 1][j] = Some(gear);
                break;
            }
        }
        if i < 139 {
            if let Some(mut gear) = gears[i + 1][j] {
                gear.count += 1;
                gear.ratio *= d;
                gears[i + 1][j] = Some(gear);
                break;
            }
        }
        if j > 0 {
            if let Some(mut gear) = gears[i][j - 1] {
                gear.count += 1;
                gear.ratio *= d;
                gears[i][j - 1] = Some(gear);
                break;
            }
        }
        if j < 139 {
            if let Some(mut gear) = gears[i][j + 1] {
                gear.count += 1;
                gear.ratio *= d;
                gears[i][j + 1] = Some(gear);
                break;
            }
        }
        if i > 0 && j > 0 {
            if let Some(mut gear) = gears[i - 1][j - 1] {
                gear.count += 1;
                gear.ratio *= d;
                gears[i - 1][j - 1] = Some(gear);
                break;
            }
        }
        if j < 139 && i > 0 {
            if let Some(mut gear) = gears[i - 1][j + 1] {
                gear.count += 1;
                gear.ratio *= d;
                gears[i - 1][j + 1] = Some(gear);
                break;
            }
        }
        if i < 139 && j > 0 {
            if let Some(mut gear) = gears[i + 1][j - 1] {
                gear.count += 1;
                gear.ratio *= d;
                gears[i + 1][j - 1] = Some(gear);
                break;
            }
        }
        if i < 139 && j < 139 {
            if let Some(mut gear) = gears[i + 1][j + 1] {
                gear.count += 1;
                gear.ratio *= d;
                gears[i + 1][j + 1] = Some(gear);
                break;
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
        assert_eq!(part1_answer, 526404);
        assert_eq!(part2_answer, 84399773);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 4361);
        assert_eq!(part2_answer, 467835);
    }
}
