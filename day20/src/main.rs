use std::collections::{HashMap, VecDeque};

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut modules = parse_input(input);
    let (low_count, high_count) = simulate(&mut modules);

    let part1_answer = low_count * high_count;
    let part2_answer = 0;
    (part1_answer, part2_answer)
}

fn simulate(modules: &mut HashMap<&'static str, Module>) -> (usize, usize) {
    let mut pulses = VecDeque::new();
    let (mut low_count, mut high_count) = (0, 0);

    for i in 1..=1000 {
        println!("Button press {}", i);
        println!("  Broadcasting from button");
        pulses.push_back(Pulse {
            high: false,
            from: "button",
            to: "broadcaster",
        });
        while let Some(pulse) = pulses.pop_front() {
            if pulse.high {
                high_count += 1;
            } else {
                low_count += 1;
            }
            println!("  Processing pulse: {:?}", pulse);
            println!("  hi: {}, lo: {}", high_count, low_count);
            let module = modules.get_mut(pulse.to);
            if module.is_none() {
                println!("    No module found for pulse: {:?}", pulse);
                continue;
            }
            let module = module.unwrap();
            let new_pulse = match module.kind {
                ModuleKind::Broadcaster => Some(pulse.high),
                ModuleKind::FlipFlop { ref mut state } => {
                    if !pulse.high {
                        println!("    Flipping flipflop state: {}", state);
                        *state = !*state;
                        //module.kind = ModuleKind::FlipFlop { state };
                        Some(*state)
                    } else {
                        None
                    }
                }
                ModuleKind::Conjunction { ref mut state } => {
                    if pulse.high {
                        state.insert(pulse.from, true);
                    } else {
                        state.insert(pulse.from, false);
                    }
                    println!("    Conjunction state: {:?}", state);
                    Some(!state.values().all(|&v| v))
                }
                ModuleKind::None => unreachable!(),
            };
            if let Some(new_pulse) = new_pulse {
                for output in &module.outputs {
                    let p = Pulse {
                        high: new_pulse,
                        from: module.name,
                        to: output,
                    };
                    println!("    Sending pulse: {:?}", p);
                    pulses.push_back(p);
                }
            }
            println!("    Saved state: {:?}", modules.get(pulse.to).unwrap().kind);

            // if high_count > 100 {
            //     panic!("too many pulses");
            // }
        }
    }

    println!("low_count: {}", low_count);
    println!("high_count: {}", high_count);

    (low_count, high_count)
}

// fn send_pulse(modules: &HashMap<&'static str, Module>) -> Vec<Pulse {
//
// }

fn parse_input(input: &'static str) -> HashMap<&'static str, Module> {
    let mut modules = HashMap::new();
    for line in input.lines() {
        let (src, dst) = line.split_once(" -> ").unwrap();
        let kind = match src.chars().next().unwrap() {
            '%' => ModuleKind::FlipFlop { state: false },
            '&' => ModuleKind::Conjunction {
                state: HashMap::new(),
            },
            'b' => ModuleKind::Broadcaster,
            _ => unreachable!("unknown module kind: {}", src),
        };
        let name = src
            .strip_prefix('%')
            .unwrap_or_else(|| src.strip_prefix('&').unwrap_or(src));
        let outputs = dst.split(", ").collect();
        let module = Module {
            kind,
            name,
            outputs,
        };
        modules.insert(name, module);
    }
    let mut module_inputs: HashMap<&str, Vec<&str>> = HashMap::new();
    for module in modules.values() {
        for output in &module.outputs {
            let inputs = module_inputs.entry(output).or_default();
            inputs.push(module.name);
        }
    }

    println!("module_inputs: {:?}", module_inputs);
    for module in modules.values_mut() {
        if let ModuleKind::Conjunction { ref mut state } = module.kind {
            for output in module_inputs.get(module.name).unwrap().iter() {
                state.insert(output, false);
            }
            println!("module: {:?}", module);
        }
    }
    modules
}

#[derive(Debug)]
struct Pulse {
    high: bool,
    from: &'static str,
    to: &'static str,
}

#[derive(Debug)]
struct Module {
    kind: ModuleKind,
    name: &'static str,
    outputs: Vec<&'static str>,
}

#[derive(Debug)]
enum ModuleKind {
    Broadcaster,
    FlipFlop { state: bool },
    Conjunction { state: HashMap<&'static str, bool> },
    None,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 825167435, "incorrect part 1 answer");
        // assert_eq!(part2_answer, 0, "incorrect part 2 answer");
    }

    #[test]
    fn test_input_example1() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example1"));
        assert_eq!(part1_answer, 32000000, "incorrect part 1 answer");
        // assert_eq!(part2_answer, 0, "incorrect part 2 answer");
    }

    #[test]
    fn test_input_example2() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example2"));
        assert_eq!(part1_answer, 11687500, "incorrect part 1 answer");
        // assert_eq!(part2_answer, 0, "incorrect part 2 answer");
    }
}
