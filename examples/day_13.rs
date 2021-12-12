
type Input = Vec<usize>;
pub fn main() {
    let input = parse_input(include_str!("../input/day10.txt"));
    println!("Ex1: {}", exercise_1(&input));
    println!("Ex2: {}", exercise_2(&input));
}

fn parse_input(input: &str) -> Input {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<BlockEntry> {
    line.chars().map().collect()
}

fn exercise_1(input: &Input) -> usize {
  
}

fn exercise_2(input: &Input) -> usize {
    0
}