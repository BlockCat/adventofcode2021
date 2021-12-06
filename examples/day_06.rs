use std::collections::HashMap;

type Input = Vec<usize>;

pub fn main() {
    let input = parse_input(include_str!("../input/day06.txt"));

    println!("Ex1: {}", exercise_n::<80>(&input));
    println!("Ex2: {}", exercise_n::<256>(&input));
}

fn parse_input(input: &str) -> Input {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

fn exercise_n<const N: usize>(lines: &Input) -> usize {
    let mut group = HashMap::with_capacity(9);

    for fish in lines {
        *group.entry(*fish).or_insert(0usize) += 1;
    }

    for _ in 0..N {
        let mut ng = HashMap::with_capacity(9);

        for (days, amount) in group {
            if days == 0 {
                *ng.entry(6).or_insert(0) += amount;
                *ng.entry(8).or_insert(0) += amount;
            } else {
                *ng.entry(days - 1).or_insert(0) += amount;
            }
        }
        group = ng;
    }

    group.values().sum()
}

// fn exercise_2(lines: &Input) -> usize {}
