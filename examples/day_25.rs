use std::{str::FromStr, fmt::Display};

use aoc_2021::{grid::{StaticGrid, Grid}, vector::Vector2};

type Input = StaticGrid<Direction>;

pub fn main() {
    let input = parse_input(include_str!("../input/day25.txt"));
    println!("Ex1: {}", exercise_1(&input));
}

fn parse_input(input: &str) -> Input {
    StaticGrid::from_vec(input.lines().map(parse_line).collect())
}

fn parse_line(line: &str) -> Vec<Direction> {
    line.chars()
        .map(|c| match c {
            '>' => Direction::East,
            'v' => Direction::South,
            '.' => Direction::Empty,
            _ => unreachable!(),
        })
        .collect()
}

fn exercise_1(input: &Input) -> usize {
    let mut grid = input.clone();

    print(&grid);

    for i in 1.. {
        let mut sum = 0;

        let (a, moved) = step_east(grid);
        let (a, moved2) = step_south(a);
        grid = a;

        // print(&grid);

        sum += moved;
        sum += moved2;
        // panic!();
        if sum == 0 {
            return i;
        }
    }

    unreachable!()
}

fn step_east(static_grid: StaticGrid<Direction>) -> (StaticGrid<Direction>, usize) {
    let mut new_grid = static_grid.clone();
    let mut moved = 0;
    // Handle east
    for (pos, _) in static_grid.iter().filter(|a| a.1 == &Direction::East) {
        let npos = if pos[0] as usize == static_grid.width - 1 {
            Vector2::new([0, pos[1]])
        } else {
            pos + Vector2::new([1, 0])
        };
        if static_grid.get_vec(&npos) == Some(&Direction::Empty) {
            new_grid.set_vec(&npos, Direction::East);
            new_grid.set_vec(&pos, Direction::Empty);
            moved += 1;
        }
    }

    (new_grid, moved)
}

fn step_south(old_grid: StaticGrid<Direction>) -> (StaticGrid<Direction>, usize) {
    let mut new_grid = old_grid.clone();
    let mut moved = 0;
    // Handle east
    for (opos, _) in old_grid.iter().filter(|a| a.1 == &Direction::South) {
        let npos = if opos[1] as usize == old_grid.height - 1 {
            Vector2::new([opos[0], 0])
        } else {
            opos + Vector2::new([0, 1])
        };
        if old_grid.get_vec(&npos) == Some(&Direction::Empty) {
            new_grid.set_vec(&npos, Direction::South);
            new_grid.set_vec(&opos, Direction::Empty);
            moved += 1;
            
        }
    }

    (new_grid, moved)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Empty,
    East,
    South,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Empty
    }
}

fn print(grid: &StaticGrid<Direction>) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let c = match grid.get(x as isize, y as isize).unwrap() {
                Direction::Empty => '.',
                Direction::East => '>',
                Direction::South => 'v',
            };
            print!("{}", c);
        }
        println!();
    }
    println!()
}