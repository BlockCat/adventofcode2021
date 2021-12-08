use std::collections::{HashMap, HashSet};

type Input<'a> = Vec<Line<'a>>;

pub fn main() {
    let input = parse_input(include_str!("../input/day08.txt"));
    println!("Ex1: {}", exercise_1(&input));
    println!("Ex2: {}", exercise_2(&input));
}

fn parse_input(input: &str) -> Input {
    input.lines().map(parse_line).collect()
}

fn parse_line<'a>(line: &'a str) -> Line<'a> {
    let mut split1 = line.split('|');
    let left = split1.next().unwrap();
    let right = split1.next().unwrap();

    let left = left.trim().split(' ').collect();
    let right = right.trim().split(' ').collect();

    Line { left, right }
}

fn exercise_1(input: &Input) -> usize {
    input
        .iter()
        .map(|x| {
            x.right
                .iter()
                .filter(|x| match x.len() {
                    2 => true,
                    3 => true,
                    4 => true,
                    7 => true,
                    _ => false,
                })
                .count()
        })
        .sum()
}

fn exercise_2(input: &Input) -> usize {
    input.iter().map(deduce_line).sum()
}

fn deduce_line(line: &Line) -> usize {
    let mut sieve = create_mapping();

    for left in &line.left {
        handle_unique_str(left, &mut sieve)
    }
    for right in &line.right {
        handle_unique_str(right, &mut sieve)
    }

    let mut known = sieve
        .iter()
        .filter(|x| x.1.len() == 1)
        .map(|x| (*x.0, *x.1.iter().next().unwrap()))
        .collect::<HashMap<char, char>>();

    let mut changed = true;
    while changed {
        changed = false;
        for (l, s) in sieve.iter_mut() {
            for (lane, cha) in &known {
                if lane != l {
                    if s.remove(cha) {
                        changed = true;
                    }
                }
            }
        }

        known = sieve
            .iter()
            .filter(|x| x.1.len() == 1)
            .map(|x| (*x.0, *x.1.iter().next().unwrap()))
            .collect::<HashMap<char, char>>();
    }

    let mut sum = str_to_number(&line.right[0], &known) * 1000;
    sum += str_to_number(&line.right[1], &known) * 100;
    sum += str_to_number(&line.right[2], &known) * 10;
    sum += str_to_number(&line.right[3], &known);

    sum
}

fn clean_char(char: &[char], set: &HashSet<char>, sieve: &mut HashMap<char, HashSet<char>>) {
    for lane in char {
        sieve
            .entry(*lane)
            .and_modify(|a| *a = a.intersection(set).cloned().collect());
    }
}

fn handle_unique_str(left: &str, sieve: &mut HashMap<char, HashSet<char>>) {
    let set = left.chars().collect::<HashSet<_>>();
    match left.len() {
        2 => {
            clean_char(&['F', 'C'], &set, sieve);
        }
        3 => {
            clean_char(&['F', 'C', 'A'], &set, sieve);
        }
        4 => {
            clean_char(&['F', 'C', 'B', 'D'], &set, sieve);
        }
        7 => {}
        5 => {
            clean_char(&['A', 'D', 'G'], &set, sieve);
        }
        6 => {
            clean_char(&['A', 'B', 'F', 'G'], &set, sieve);
        }
        _ => {}
    }
}

fn str_to_number(l: &str, known: &HashMap<char, char>) -> usize {
    match l.len() {
        2 => return 1,
        3 => return 7,
        4 => return 4,
        7 => return 8,
        _ => {}
    }

    let c = known.get(&'C').map(|d| l.contains(*d)).unwrap_or(false);
    let d = known.get(&'D').map(|d| l.contains(*d)).unwrap_or(false);
    let e = known.get(&'E').map(|d| l.contains(*d)).unwrap_or(false);
    let f = known.get(&'F').map(|d| l.contains(*d)).unwrap_or(false);

    match l.len() {
        5 => match (c, f) {
            (true, true) => return 3,
            (true, false) => return 2,
            (false, true) => return 5,
            (false, false) => unreachable!(),
        },
        6 => {
            // 0 , 6, 9
            if e && d {
                return 6;
            }
            if c && d {
                return 9;
            }
            if !d {
                return 0;
            }
        }
        _ => unreachable!(),
    }

    unreachable!("{}: {:?}", l, known)
}

fn create_mapping() -> HashMap<char, HashSet<char>> {
    let all_set: HashSet<char> = {
        let mut a = HashSet::new();
        for c in 'a'..='g' {
            a.insert(c);
        }
        a
    };

    let mut sieve = HashMap::new();
    for ch in 'A'..='G' {
        sieve.insert(ch, all_set.clone());
    }
    sieve
}

struct Line<'a> {
    left: Vec<&'a str>,
    right: Vec<&'a str>,
}
