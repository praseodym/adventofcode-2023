use std::cmp;

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut part1_answer = 0;
    let mut part2_answer = 0;

    let r = regex::Regex::new(r"Game (?<game_id>\d+): (?<games>.*)").unwrap();

    for (_, line) in input.lines().enumerate() {
        let captures = r.captures(line).unwrap();
        let game_id = captures
            .name("game_id")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let games = captures.name("games").unwrap().as_str().split("; ");
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for game in games {
            let draws = game.split(", ");
            for draw in draws {
                let mut s = draw.split(' ');
                let count = s.next().unwrap().parse::<usize>().unwrap();
                let colour = s.next().unwrap();
                match colour {
                    "red" => min_red = cmp::max(min_red, count),
                    "green" => min_green = cmp::max(min_green, count),
                    "blue" => min_blue = cmp::max(min_blue, count),
                    _ => panic!("unknown colour: {}", colour),
                }
            }
        }
        if min_red <= 12 && min_green <= 13 && min_blue <= 14 {
            part1_answer += game_id;
        }
        let power = min_red * min_blue * min_green;
        part2_answer += power;
    }

    (part1_answer, part2_answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 2101);
        assert_eq!(part2_answer, 58269);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 8);
        assert_eq!(part2_answer, 2286);
    }
}
