fn main() {
    let (part1_answer, _part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    // println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut part1_answer = 0;
    let r = regex::Regex::new(r"Game (?<game_id>\d+): (?<games>.*)").unwrap();

    'games: for (_, line) in input.lines().enumerate() {
        let captures = r.captures(line).unwrap();
        let game_id = captures
            .name("game_id")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let games = captures.name("games").unwrap().as_str().split("; ");
        for game in games {
            let draws = game.split(", ");
            for draw in draws {
                let mut s = draw.split(' ');
                let count = s.next().unwrap().parse::<usize>().unwrap();
                let colour = s.next().unwrap();
                match colour {
                    "red" => {
                        if count > 12 {
                            continue 'games;
                        }
                    }
                    "green" => {
                        if count > 13 {
                            continue 'games;
                        }
                    }
                    "blue" => {
                        if count > 14 {
                            continue 'games;
                        }
                    }
                    _ => panic!("unknown colour: {}", colour),
                }
            }
        }
        part1_answer += game_id;
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
        assert_eq!(part1_answer, 2101);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 8);
        // assert_eq!(part2_answer, 0);
    }
}
