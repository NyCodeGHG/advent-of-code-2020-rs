use std::collections::HashMap;
use std::fs;

fn main() {
    let inputs = read_input();
    let mut pairs: HashMap<i32, i32> = HashMap::new();
    for a in &inputs {
        for b in &inputs {
            pairs.insert(a + b, a * b);
        }
    }
    println!("Solution One: {}", match pairs.get(&2020) {
        Some(n) => n.to_string(),
        None => "No solution".to_string()
    })
}

fn read_input() -> Vec<i32> {
    fs::read_to_string("inputs/day01.txt")
        .expect("Unable to read day 1 input")
        .lines()
        .map(|line| line.parse().expect(&format!("Unable to parse {} into a number", &line)))
        .collect()
}
