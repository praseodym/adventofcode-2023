fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

type Coord = (isize, isize, isize);

fn run(input: &'static str) -> (usize, usize) {
    let mut bricks: Vec<(Coord, Coord)> = Vec::new();
    for line in input.lines() {
        let (start, end) = line.split_once('~').unwrap();
        let start = start
            .split(',')
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let start = (start[0], start[1], start[2]);
        let end = end
            .split(',')
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let end = (end[0], end[1], end[2]);
        bricks.push((start, end));
    }

    // sort bricks by z1, z2
    bricks.sort_by(|a, b| {
        let ((_, _, z1), (_, _, z2)) = a;
        let ((_, _, z3), (_, _, z4)) = b;
        z1.cmp(z3).then(z2.cmp(z4))
    });

    let _fallen_bricks = fall_bricks(&mut bricks);

    // for brick in bricks.iter().rev() {
    //     println!("{:?}", brick);
    // }

    let mut removable_bricks = 0;
    for i in 0..bricks.len() {
        let mut bricks_tmp = bricks.clone();
        bricks_tmp.remove(i);
        let fallen_bricks_tmp = fall_bricks(&mut bricks_tmp);
        if fallen_bricks_tmp.is_empty() {
            // println!("brick[{}] can be removed", i);
            removable_bricks += 1;
        } else {
            // println!("brick[{}] cannot be removed, bricks {:?} would fall", i, fallen_bricks_tmp);
        }
    }

    let part1_answer = removable_bricks;
    let part2_answer = 0;
    (part1_answer, part2_answer)
}

fn fall_bricks(bricks: &mut Vec<(Coord, Coord)>) -> Vec<usize> {
    let mut fallen_bricks = Vec::new();
    for i in 0..bricks.len() {
        let ((x1, y1, z1), (x2, y2, z2)) = bricks[i];

        let mut max_j = 0;
        'j: for j in 1..z1.min(z2) {
            let z1 = z1 - j;
            let z2 = z2 - j;
            for k in 0..i {
                let ((x3, y3, z3), (x4, y4, z4)) = bricks[k];
                if x1 <= x4 && x2 >= x3 && y1 <= y4 && y2 >= y3 && z1 <= z4 && z2 >= z3 {
                    break 'j;
                }
            }
            max_j = j;
        }
        if max_j > 0 {
            let diff = max_j;
            bricks[i] = ((x1, y1, z1 - diff), (x2, y2, z2 - diff));
            // println!("brick[{}] moved down by {} to {:?}", i, diff, bricks[i]);
            fallen_bricks.push(i);
        }
    }
    fallen_bricks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 421, "incorrect part 1 answer");
        // assert_eq!(part2_answer, 0, "incorrect part 2 answer");
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 5, "incorrect part 1 answer");
        // assert_eq!(part2_answer, 0, "incorrect part 2 answer");
    }
}
