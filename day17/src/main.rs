use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

const N: usize = 141;

type Position = (usize, usize);
type Direction = (isize, isize);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    heat_loss: usize,
    pos: Position,
    direction: Direction,
}

fn run(input: &'static str) -> (usize, usize) {
    let mut grid = [[0; N]; N];
    let mut len = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            grid[y][x] = c.to_digit(10).unwrap() as u8;
        });
        len = y;
    });

    let part1_answer = shortest_path(&mut grid, len, 1, 3);
    let part2_answer = shortest_path(&mut grid, len, 4, 10);
    (part1_answer, part2_answer)
}

fn shortest_path(grid: &mut [[u8; N]; N], len: usize, min_steps: usize, max_steps: usize) -> usize {
    let mut queue = BinaryHeap::new();
    let mut cost: HashMap<(Position, Direction), usize> = HashMap::new();
    queue.push(State {
        heat_loss: 0,
        pos: (0, 0),
        direction: (0, 0),
    });
    cost.insert(((0, 0), (0, 0)), 0);

    while let Some(state) = queue.pop() {
        let (x, y) = state.pos;
        if x == len && y == len {
            return state.heat_loss;
        }
        if let Some(&c) = cost.get(&((x, y), state.direction)) {
            if c < state.heat_loss {
                continue;
            }
        }

        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            if (dx, dy) == state.direction || (-dx, -dy) == state.direction {
                continue;
            }
            let mut heat_loss = state.heat_loss;
            for i in 1..=max_steps as isize {
                let next_x = x.checked_add_signed(dx * i);
                let next_y = y.checked_add_signed(dy * i);
                if next_x.is_none() || next_y.is_none() {
                    continue;
                }
                let (next_x, next_y) = (next_x.unwrap(), next_y.unwrap());
                if next_x > len || next_y > len {
                    continue;
                }

                heat_loss += grid[next_y][next_x] as usize;

                if i < min_steps as isize {
                    continue;
                }

                let k = ((next_x, next_y), (dx, dy));
                if &heat_loss < cost.get(&k).unwrap_or(&usize::MAX) {
                    cost.insert(k, heat_loss);
                    let next_state = State {
                        heat_loss,
                        pos: (next_x, next_y),
                        direction: (dx, dy),
                    };
                    queue.push(next_state);
                }
            }
        }
    }
    usize::MAX
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 1195);
        assert_eq!(part2_answer, 1347);
    }

    #[test]
    fn test_input_example() {
        let (part1_answer, part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 102);
        assert_eq!(part2_answer, 94);
    }
}
