use std::collections::HashMap;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut part1_answer = 0;
    let mut part2_answer = 0;
    for line in input.lines() {
        let mut s = line.split_ascii_whitespace();
        let record = s.next().unwrap();
        let group = s
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        part1_answer += solve(record, &group);
        let record = format!("{}?{}?{}?{}?{}", record, record, record, record, record);
        let group = group.repeat(5);
        part2_answer += solve(&record, &group);
    }
    (part1_answer, part2_answer)
}

fn solve(record: &str, groups: &[usize]) -> usize {
    return solve(
        &record.chars().collect::<Vec<_>>(),
        groups,
        0,
        0,
        0,
        &mut HashMap::new(),
    );

    fn solve(
        record: &[char],
        groups: &[usize],
        ri: usize, // record index
        gi: usize, // groups index
        gl: usize, // length of group at gi
        cache: &mut HashMap<(usize, usize, usize), usize>,
    ) -> usize {
        if let Some(&p) = cache.get(&(ri, gi, gl)) {
            return p;
        }
        if ri == record.len() {
            // record done
            return if (gi == groups.len() && gl == 0)
                || (gi == groups.len() - 1 && groups[gi] == gl)
            {
                // completed without no unfinished groups, so possible
                1
            } else {
                // completed with unfinished groups, so impossible
                0
            };
        }

        let mut possibilities = 0;
        let chars = if record[ri] == '?' {
            vec!['#', '.']
        } else {
            vec![record[ri]]
        };
        for &c in chars.iter() {
            if c == '#' {
                // continue current group with #
                possibilities += solve(record, groups, ri + 1, gi, gl + 1, cache);
            } else if c == '.' && gl == 0 {
                // continue current empty group with .
                possibilities += solve(record, groups, ri + 1, gi, gl, cache);
            } else if c == '.' && gi < groups.len() && groups[gi] == gl {
                // current group done, start new group
                possibilities += solve(record, groups, ri + 1, gi + 1, 0, cache);
            }
        }
        cache.insert((ri, gi, gl), possibilities);
        possibilities
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 7653);
        assert_eq!(part2_answer, 60681419004564);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 21);
        assert_eq!(part2_answer, 525152);
    }

    #[test]
    fn test_simple() {
        let p = solve("#.#.###", &[1, 1, 3]);
        assert_eq!(p, 1);
        let p = solve("?.#.###", &[1, 1, 3]);
        assert_eq!(p, 1);
        let p = solve("??#.###", &[1, 1, 3]);
        assert_eq!(p, 1);
        let p = solve("???.###", &[1, 1, 3]);
        assert_eq!(p, 1);
    }

    #[test]
    fn test_multiple() {
        let p = solve(".??..??...?##.", &[1, 1, 3]);
        assert_eq!(p, 4);
    }
}
