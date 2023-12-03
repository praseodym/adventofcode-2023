use regex::Regex;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

const N: usize = 141;
#[derive(Debug, Copy, Clone)]
struct Gear {
    count: usize,
    ratio: usize,
}

fn run(input: &'static str) -> (usize, usize) {
    let mut symbols = [[false; N]; N];
    let mut gears: [[Option<Gear>; N]; N] = [[None; N]; N];
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
            if is_adjacent_to_symbol(&symbols, i, start, end) {
                part1_answer += d;
            }
            update_gears(&mut gears, i, start, end, d);
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

fn is_adjacent_to_symbol(symbols: &[[bool; N]], i: usize, start: usize, end: usize) -> bool {
    let mut adjacent = false;
    for j in start..end {
        // check if i,j is adjacent to a symbol
        for a in -1isize..=1 {
            for b in -1isize..=1 {
                adjacent |= symbols[i.saturating_add_signed(a)][j.saturating_add_signed(b)];
            }
        }
    }
    adjacent
}

fn update_gears(gears: &mut [[Option<Gear>; N]], i: usize, start: usize, end: usize, d: usize) {
    for j in start..end {
        for a in -1isize..=1 {
            for b in -1isize..=1 {
                let x = i.saturating_add_signed(a);
                let y = j.saturating_add_signed(b);
                if let Some(mut gear) = gears[x][y] {
                    gear.count += 1;
                    gear.ratio *= d;
                    gears[x][y] = Some(gear);
                    return;
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
