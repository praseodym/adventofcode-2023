use std::cmp;

fn main() {
    let (part1_answer, _part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

#[derive(Debug, Copy, Clone)]
struct Mapping {
    dest: usize,
    src: usize,
    length: usize,
}

fn run(input: &'static str) -> (usize, usize) {
    let mut seeds: Vec<usize> = vec![];
    let mut maps: Vec<Vec<Mapping>> = vec![];
    for block in input.split("\n\n") {
        let mut s = block.split(':');
        let label = s.next().unwrap();
        let content = s.next().unwrap().trim();
        if label == "seeds" {
            seeds = content
                .split_ascii_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
        } else {
            let mut map: Vec<Mapping> = vec![];
            for line in content.lines() {
                let mut s = line.split_ascii_whitespace();
                let dest = s.next().unwrap().parse::<usize>().unwrap();
                let src = s.next().unwrap().parse::<usize>().unwrap();
                let length = s.next().unwrap().parse::<usize>().unwrap();
                map.push(Mapping { dest, src, length });
            }
            maps.push(map);
        }
    }

    let mut part1_answer = usize::MAX;
    for seed in seeds.iter() {
        let mut x = *seed;
        for map in &maps {
            for mapping in map {
                if mapping.src <= x && mapping.src + mapping.length > x {
                    x = x - mapping.src + mapping.dest;
                    break;
                }
            }
        }
        part1_answer = cmp::min(part1_answer, x);
    }

    let part2_answer = 0;
    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 388071289);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 35);
        // assert_eq!(part2_answer, 0);
    }
}
