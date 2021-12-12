use std::collections::{HashMap, HashSet, VecDeque};

type Input = Vec<(Cave, Cave)>;

pub fn main() {
    let input = parse_input(include_str!("../input/day12.txt"));
    println!("Ex1: {}", exercise_1(&input));
    println!("Ex2: {}", exercise_2(&input));
}

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut split = line.split('-');
            let left = split.next().unwrap();
            let right = split.next().unwrap();
            (Cave::from(left), Cave::from(right))
        })
        .collect()
}

fn exercise_1(input: &Input) -> usize {
    let connections = find_connections(input);
    let mut queue: VecDeque<(Cave, HashSet<String>)> = VecDeque::new();
    queue.push_front((
        Cave::Small(String::from("start")),
        HashSet::from([String::from("start")]),
    ));

    let mut reached = 0;

    while let Some((cave, visited)) = queue.pop_front() {
        for neighbour in &connections[&cave] {
            match neighbour {
                Cave::Big(_) => queue.push_front((neighbour.clone(), visited.clone())),
                Cave::Small(c) => {
                    if c == "end" {
                        reached += 1;
                    } else if !visited.contains(c) {
                        let mut visited = visited.clone();
                        visited.insert(c.clone());
                        queue.push_front((neighbour.clone(), visited));
                    }
                }
            }
        }
    }

    reached
}

fn exercise_2(input: &Input) -> usize {
    let connections = find_connections(input);
    let mut reached = 0;

    let mut queue = VecDeque::new();
    queue.push_front((
        Cave::Small(String::from("start")),
        HashSet::from([String::from("start")]),
        false,
    ));
    while let Some((cave, visited, visited_twice)) = queue.pop_front() {
        for neighbour in &*connections[&cave] {
            match neighbour {
                Cave::Big(_) => {
                    queue.push_front((neighbour.clone(), visited.clone(), visited_twice))
                }
                Cave::Small(c) => {
                    if c == "end" {
                        reached += 1;
                    } else if visited.contains(c) && !visited_twice && c != "start" {
                        queue.push_front((neighbour.clone(), visited.clone(), true));
                    } else if !visited.contains(c) {
                        let mut visited = visited.clone();
                        visited.insert(c.clone());
                        queue.push_front((neighbour.clone(), visited, visited_twice));
                    }
                }
            }
        }
    }

    reached
}

fn find_connections(input: &Vec<(Cave, Cave)>) -> HashMap<Cave, Vec<Cave>> {
    let mut connections: HashMap<Cave, Vec<Cave>> = HashMap::new();
    for (left, right) in input {
        connections
            .entry(left.clone())
            .or_insert(Vec::new())
            .push(right.clone());
        connections
            .entry(right.clone())
            .or_insert(Vec::new())
            .push(left.clone());
    }
    connections
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Cave {
    Big(String),
    Small(String),
}

impl From<&'_ str> for Cave {
    fn from(text: &'_ str) -> Self {
        if text.to_uppercase() == text {
            Cave::Big(text.to_string())
        } else {
            Cave::Small(text.to_string())
        }
    }
}
