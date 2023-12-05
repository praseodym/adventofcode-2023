use rayon::prelude::*;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

#[derive(Debug, Copy, Clone)]
struct Mapping {
    dest: usize,
    src: usize,
    length: usize,
}

fn run(input: &'static str) -> (usize, usize) {
    let mut seeds: Vec<usize> = vec![];
    let mut seeds_range: Vec<(usize, usize)> = vec![];
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
            for i in (0..seeds.len()).step_by(2) {
                seeds_range.push((seeds[i], seeds[i + 1]));
            }
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

    let part1_answer = seeds
        .par_iter()
        .map(|seed| map_seed_to_location(&maps, *seed))
        .min()
        .unwrap();
    let part2_answer = seeds_range
        .par_iter()
        .flat_map(|(a, r)| *a..*a + *r)
        .map(|seed| map_seed_to_location(&maps, seed))
        .min()
        .unwrap();

    (part1_answer, part2_answer)
}

fn map_seed_to_location(maps: &Vec<Vec<Mapping>>, seed: usize) -> usize {
    let mut x = seed;
    for map in maps {
        for mapping in map {
            if mapping.src <= x && mapping.src + mapping.length > x {
                x = x - mapping.src + mapping.dest;
                break;
            }
        }
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 388071289);
        assert_eq!(part2_answer, 84206669);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 35);
        assert_eq!(part2_answer, 46);
    }
}
