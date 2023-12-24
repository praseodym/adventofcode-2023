use ast::{Int, Real};
use z3::ast::Ast;
use z3::{ast, Config, Context, Solver};

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let hailstones = parse_input(input);
    let part1_answer = xy_intersections(&hailstones, 200000000000000i128, 400000000000000i128);
    let part2_answer = solve(&hailstones);
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

fn solve(hailstones: &[Hailstone]) -> usize {
    // for hailstone i in 0, 1, 2 in hailstones, solve:
    // rpx + rvx * ti = hipx + hivx * ti
    // rpy + rvy * ti = hipy + hivy * ti
    // rpz + rvz * ti = hipz + hivz * ti
    // ti >= 0
    // where (rpx, rpy, rpz) is the initial position of the rock

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let zero = Real::from_int(&Int::from_i64(&ctx, 0));

    let rx = Real::new_const(&ctx, "rpx");
    let ry = Real::new_const(&ctx, "rpy");
    let rz = Real::new_const(&ctx, "rpz");
    let rvx = Real::new_const(&ctx, "rvx");
    let rvy = Real::new_const(&ctx, "rvy");
    let rvz = Real::new_const(&ctx, "rvz");

    for i in 0..3 {
        let ((hpx, hpy, hpz), (hvx, hvy, hvz)) = hailstones[i];

        let t = Real::new_const(&ctx, format!("t{}", i));
        solver.assert(&t.ge(&zero));

        for a in [
            (&rx, &rvx, &hpx, &hvx),
            (&ry, &rvy, &hpy, &hvy),
            (&rz, &rvz, &hpz, &hvz),
        ]
        .iter()
        {
            let lhs = Real::add(&ctx, &[&a.0.clone(), &Real::mul(&ctx, &[a.1, &t])]);
            let hp = Real::from_int(&Int::from_i64(&ctx, *a.2 as i64));
            let hv = Real::from_int(&Int::from_i64(&ctx, *a.3 as i64));
            let rhs = Real::add(&ctx, &[&hp, &Real::mul(&ctx, &[&hv, &t])]);
            solver.assert(&lhs._eq(&rhs));
        }
    }

    // solve and get the value of (rx, ry, rz)
    if solver.check() != z3::SatResult::Sat {
        panic!("no solution found");
    }
    let model = solver.get_model().unwrap();
    let rx = model.get_const_interp(&rx).unwrap().as_real().unwrap();
    let ry = model.get_const_interp(&ry).unwrap().as_real().unwrap();
    let rz = model.get_const_interp(&rz).unwrap().as_real().unwrap();
    (rx.0 + ry.0 + rz.0) as usize
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
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 31921, "incorrect part 1 answer");
        assert_eq!(part2_answer, 761691907059631, "incorrect part 2 answer");
    }

    #[test]
    fn test_input_example() {
        let hailstones = parse_input(include_str!("../input-example"));
        let part1_answer = xy_intersections(&hailstones, 7i128, 27i128);
        assert_eq!(part1_answer, 2, "incorrect part 1 answer");
        let part2_answer = solve(&hailstones);
        assert_eq!(part2_answer, 47, "incorrect part 2 answer");
    }
}
