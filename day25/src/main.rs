use std::collections::HashMap;

use rand::{prelude::IteratorRandom, thread_rng};

fn main() {
    let part1_answer = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Node {
    name: String,
    weight: usize,
}

impl Node {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            weight: 1,
        }
    }
}

type Graph = HashMap<Node, Vec<Node>>;

fn run(input: &'static str) -> usize {
    let mut connections: Graph = HashMap::new();
    for line in input.lines() {
        let (a, edges) = line.split_once(": ").unwrap();
        let edges = edges.split_ascii_whitespace().collect::<Vec<_>>();
        edges.iter().for_each(|b| {
            connections
                .entry(Node::new(a))
                .or_default()
                .push(Node::new(b));
            connections
                .entry(Node::new(b))
                .or_default()
                .push(Node::new(a));
        });
    }

    println!("{:?} components", connections.keys().len());
    min_cut(connections)
}

/// Implementation of Karger's algorithm to find the minimum cut
///
/// References:
/// * https://en.wikipedia.org/wiki/Karger%27s_algorithm
/// * 'Algorithm Design' by Jon Kleinberg and Éva Tardos, Chapter 13.2
pub fn min_cut(graph: Graph) -> usize {
    let mut rng = thread_rng();
    loop {
        let mut graph = graph.clone();
        while graph.len() > 2 {
            // choose a random edge (a, b)
            let (a, w) = graph.iter().choose(&mut rng).unwrap();
            let b = w.iter().choose(&mut rng).unwrap();
            let (a, b) = (a.clone(), b.clone());

            // contract a and b into new_node
            let new_node = Node {
                name: format!("{}-{}", a.name, b.name),
                weight: a.weight + b.weight,
            };
            let mut v = graph.remove(&a).unwrap();
            let w = graph.remove(&b).unwrap();
            v.extend(w);
            v.retain(|x| x != &a && x != &b);
            for node in &v {
                graph.get_mut(node).unwrap().iter_mut().for_each(|x| {
                    if x == &a || x == &b {
                        x.clone_from(&new_node);
                    }
                });
            }
            graph.insert(new_node.clone(), v);
        }

        let cuts = graph.values().next().unwrap().len();
        if cuts == 3 {
            return graph.keys().map(|s| s.weight).product();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_own() {
        let part1_answer = run(include_str!("../input"));
        assert_eq!(part1_answer, 559143, "incorrect part 1 answer");
    }

    #[test]
    fn test_input_example() {
        let part1_answer = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 54, "incorrect part 1 answer");
    }
}
