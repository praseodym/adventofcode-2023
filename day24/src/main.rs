fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let hailstones = parse_input(input);
    let part1_answer = xy_intersections(&hailstones, 200000000000000i128, 400000000000000i128);
    let part2_answer = 0;
    (part1_answer, part2_answer)
}

type Coordinate = (i128, i128, i128);
type Hailstone = (Coordinate, Coordinate);

fn parse_input(input: &str) -> Vec<Hailstone> {
    fn str_to_tuple(s: &str) -> Coordinate {
        let mut s = s.splitn(3, ',');
        (
            s.next().unwrap().trim_start().parse::<i128>().unwrap(),
            s.next().unwrap().trim_start().parse::<i128>().unwrap(),
            s.next().unwrap().trim_start().parse::<i128>().unwrap(),
        )
    }
    let mut hailstones = Vec::new();
    for line in input.lines() {
        let (p, v) = line.split_once(" @ ").unwrap();
        let (px, py, pz) = str_to_tuple(p);
        let (vx, vy, vz) = str_to_tuple(v);
        hailstones.push(((px, py, pz), (vx, vy, vz)));
    }
    hailstones
}

fn xy_intersections(hailstones: &[Hailstone], area_start: i128, area_end: i128) -> usize {
    let mut count = 0;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            let ((px1, py1, _), (vx1, vy1, _)) = hailstones[i];
            let ((px2, py2, _), (vx2, vy2, _)) = hailstones[j];
            let i = line_intersection(
                (px1, py1),
                (px1 + vx1, py1 + vy1),
                (px2, py2),
                (px2 + vx2, py2 + vy2),
            );
            if let Some((ix, iy)) = i {
                if (ix > px1) == vx1.is_positive()
                    && (ix > px2) == vx2.is_positive()
                    && (area_start..=area_end).contains(&ix)
                    && (area_start..=area_end).contains(&iy)
                {
                    count += 1;
                }
            }
        }
    }
    count
}

fn line_intersection(
    (x1, y1): (i128, i128),
    (x2, y2): (i128, i128),
    (x3, y3): (i128, i128),
    (x4, y4): (i128, i128),
) -> Option<(i128, i128)> {
    // https://en.wikipedia.org/wiki/Line–line_intersection#Given_two_points_on_each_line
    let d = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if d != 0 {
        let px = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)) / d;
        let py = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)) / d;
        Some((px, py))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 31921, "incorrect part 1 answer");
        // assert_eq!(part2_answer, 0, "incorrect part 2 answer");
    }

    #[test]
    fn test_input_example() {
        let hailstones = parse_input(include_str!("../input-example"));
        let part1_answer = xy_intersections(&hailstones, 7i128, 27i128);
        assert_eq!(part1_answer, 2, "incorrect part 1 answer");
        // assert_eq!(part2_answer, 0, "incorrect part 2 answer");
    }
}
