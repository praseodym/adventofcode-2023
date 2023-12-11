use std::collections::HashSet;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let (galaxies, empty_x, empty_y) = parse_input(input);

    let part1_answer = sum_of_distances(&galaxies, &empty_x, &empty_y, 2);
    let part2_answer = sum_of_distances(&galaxies, &empty_x, &empty_y, 1000000);

    (part1_answer, part2_answer)
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>, Vec<usize>) {
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect::<Vec<_>>();
    let all_x = galaxies.iter().map(|&(x, _)| x).collect::<HashSet<_>>();
    let all_y = galaxies.iter().map(|&(_, y)| y).collect::<HashSet<_>>();
    let empty_x = (0..*all_x.iter().max().unwrap()).filter(|&x| !all_x.contains(&x)).collect();
    let empty_y = (0..*all_y.iter().max().unwrap()).filter(|&y| !all_y.contains(&y)).collect();
    (galaxies, empty_x, empty_y)
}

fn sum_of_distances(
    galaxies: &[(usize, usize)],
    empty_x: &[usize],
    empty_y: &[usize],
    expansion: usize,
) -> usize {
    let expansion = expansion - 1;
    galaxies.iter().flat_map(|g| std::iter::repeat(g).zip(galaxies)).map(|((x1, y1), (x2, y2))| {
        let x1 = x1 + empty_x.iter().filter(|&&ex| ex < *x1).count() * expansion;
        let x2 = x2 + empty_x.iter().filter(|&&ex| ex < *x2).count() * expansion;
        let y1 = y1 + empty_y.iter().filter(|&&ey| ey < *y1).count() * expansion;
        let y2 = y2 + empty_y.iter().filter(|&&ey| ey < *y2).count() * expansion;
        x1.abs_diff(x2) + y1.abs_diff(y2)
    }).sum::<usize>() / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 9742154);
        assert_eq!(part2_answer, 411142919886);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, _) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 374);
    }

    #[test]
    fn test_input_example_expanded() {
        let (galaxies, empty_x, empty_y) = parse_input(include_str!("../input-example"));

        let sum = sum_of_distances(&galaxies, &empty_x, &empty_y, 10);
        assert_eq!(sum, 1030);

        let sum = sum_of_distances(&galaxies, &empty_x, &empty_y, 100);
        assert_eq!(sum, 8410);
    }
}
