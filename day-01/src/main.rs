use std::collections::HashMap;
use std::fs;

fn main() {
    let inputs = read_input();
    let first = match first_solution(&inputs) {
        Some(n) => n.to_string(),
        None => "No Solution".to_string(),
    };
    println!("First Solution: {}", first);
}

fn first_solution(input: &Vec<i32>) -> Option<i32> {
    let mut pairs: HashMap<i32, i32> = HashMap::new();
    for a in input {
        for b in input {
            pairs.insert(a + b, a * b);
        }
    }
    match pairs.get(&2020) {
        Some(n) => Some(*n),
        None => None,
    }
}

fn read_input() -> Vec<i32> {
    fs::read_to_string("inputs/day01.txt")
        .expect("Unable to read day 1 input")
        .lines()
        .map(|line| {
            line.parse()
                .expect(&format!("Unable to parse {} into a number", &line))
        })
        .collect()
}
