use std::collections::{HashMap, VecDeque};

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut modules = parse_input(input);
    let (low_count, high_count, rx_iters) = simulate(&mut modules);

    let part1_answer = low_count * high_count;
    let part2_answer = rx_iters;
    (part1_answer, part2_answer)
}

fn simulate(modules: &mut HashMap<&'static str, Module>) -> (usize, usize, usize) {
    let mut pulses = VecDeque::new();
    let (mut low_count, mut high_count) = (0, 0);

    let rx_input = modules.values().find(|m| m.outputs.contains(&"rx"));
    let mut rx_state = if let Some(rx_input) = rx_input {
        let rx_input_name = rx_input.name;
        let rx_input_inputs_count = rx_input.inputs.len();
        let rx_periods: HashMap<&str, usize> = HashMap::with_capacity(rx_input_inputs_count);
        Some((rx_input_name, rx_input_inputs_count, rx_periods))
    } else {
        println!("no rx module found, no answer for part 2");
        None
    };

    let mut button_presses = 0;
    'button_presses: loop {
        button_presses += 1;
        pulses.push_back(Pulse {
            high: false,
            from: "button",
            to: "broadcaster",
        });
        while let Some(pulse) = pulses.pop_front() {
            if button_presses <= 1000 {
                if pulse.high {
                    high_count += 1;
                } else {
                    low_count += 1;
                }
            } else if rx_state.is_none() {
                break 'button_presses;
            }
            let module = modules.get_mut(pulse.to);
            if module.is_none() {
                continue;
            }
            let module = module.unwrap();
            let new_pulse = match module.kind {
                ModuleKind::Broadcaster => Some(pulse.high),
                ModuleKind::FlipFlop { ref mut state } => {
                    if !pulse.high {
                        *state = !*state;
                        Some(*state)
                    } else {
                        None
                    }
                }
                ModuleKind::Conjunction { ref mut state } => {
                    if let Some((rx_input_name, rx_input_inputs_count, ref mut rx_periods)) =
                        rx_state
                    {
                        if module.name == rx_input_name {
                            if pulse.high && !rx_periods.contains_key(pulse.from) {
                                rx_periods.insert(pulse.from, button_presses);
                            }
                        } else if rx_periods.len() == rx_input_inputs_count {
                            break 'button_presses;
                        }
                    }
                    if pulse.high {
                        state.insert(pulse.from, true);
                    } else {
                        state.insert(pulse.from, false);
                    }
                    Some(!state.values().all(|&v| v))
                }
            };
            if let Some(new_pulse) = new_pulse {
                for output in &module.outputs {
                    let p = Pulse {
                        high: new_pulse,
                        from: module.name,
                        to: output,
                    };
                    pulses.push_back(p);
                }
            }
        }
    }

    let rx_iters = if let Some((_, _, rx_periods)) = rx_state {
        lcm(&rx_periods.values().copied().collect::<Vec<_>>()[..])
    } else {
        0
    };
    (low_count, high_count, rx_iters)
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(v: &[usize]) -> usize {
    let mut result = v[0];
    for &i in &v[1..] {
        result = result * i / gcd(result, i);
    }
    result
}

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
            inputs: Vec::new(),
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

    for module in modules.values_mut() {
        if let Some(inputs) = module_inputs.remove(module.name) {
            module.inputs = inputs;
            if let ModuleKind::Conjunction { ref mut state } = module.kind {
                for output in &module.inputs {
                    state.insert(output, false);
                }
            }
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
    inputs: Vec<&'static str>,
    outputs: Vec<&'static str>,
}

#[derive(Debug)]
enum ModuleKind {
    Broadcaster,
    FlipFlop { state: bool },
    Conjunction { state: HashMap<&'static str, bool> },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 825167435, "incorrect part 1 answer");
        assert_eq!(part2_answer, 225514321828633, "incorrect part 2 answer");
    }

    #[test]
    fn test_input_example1() {
        let (part1_answer, _) = run(include_str!("../input-example1"));
        assert_eq!(part1_answer, 32000000, "incorrect part 1 answer");
    }

    #[test]
    fn test_input_example2() {
        let (part1_answer, _) = run(include_str!("../input-example2"));
        assert_eq!(part1_answer, 11687500, "incorrect part 1 answer");
    }
}
