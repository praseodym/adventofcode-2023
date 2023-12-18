fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn run(input: &'static str) -> (usize, usize) {
    let mut plan1 = Vec::new();
    let mut plan2 = Vec::new();
    for line in input.lines() {
        let mut s = line.split_ascii_whitespace();
        let direction = s.next().unwrap().chars().next().unwrap();
        let distance = s.next().unwrap().parse::<isize>().unwrap();
        let direction = match direction {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("unknown direction: {}", direction),
        };
        plan1.push((direction, distance));

        let colour = s.next().unwrap();
        let distance = isize::from_str_radix(&colour[2..7], 16).unwrap();
        let direction = usize::from_str_radix(&colour[7..8], 16).unwrap();
        let direction = match direction {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => panic!("unknown direction: {}", direction),
        };
        plan2.push((direction, distance));
    }

    let part1_answer = lagoon_area(&plan1);
    let part2_answer = lagoon_area(&plan2);
    (part1_answer, part2_answer)
}

fn lagoon_area(plan: &[(Direction, isize)]) -> usize {
    // https://en.wikipedia.org/wiki/Shoelace_formula#Trapezoid_formula
    // https://en.wikipedia.org/wiki/Pick's_theorem#Formula
    let mut a = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    for (d, n) in plan {
        let x1 = x2;
        let y1 = y2;
        match d {
            Direction::Up => y2 -= n,
            Direction::Down => y2 += n,
            Direction::Left => x2 -= n,
            Direction::Right => x2 += n,
        }
        a += (y1 + y2) * (x1 - x2) + n;
    }
    (a / 2 + 1) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 56923, "incorrect part 1 answer");
        assert_eq!(part2_answer, 66296566363189, "incorrect part 2 answer");
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 62, "incorrect part 1 answer");
        assert_eq!(part2_answer, 952408144115, "incorrect part 2 answer");
    }
}
