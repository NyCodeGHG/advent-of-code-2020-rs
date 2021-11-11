use std::collections::HashMap;
use std::fs;

pub fn first_solution(input: &Vec<i64>) -> Option<i64> {
    let mut pairs: HashMap<i64, i64> = HashMap::new();
    for a in input {
        for b in input {
            if a == b {
                continue;
            }
            pairs.insert(a + b, a * b);
        }
    }
    match pairs.get(&2020) {
        Some(n) => Some(*n),
        None => None,
    }
}

pub fn second_solution(input: &Vec<i64>) -> Option<i64> {
    let mut pairs: HashMap<i64, i64> = HashMap::new();
    for a in input {
        for b in input {
            for c in input {
                if a == b || b == c || a == c {
                    continue;
                }
                pairs.insert(a + b + c, a * b * c);
            }
        }
    }
    match pairs.get(&2020) {
        Some(n) => Some(*n),
        None => None,
    }
}

pub fn solution_to_string(solution: Option<i64>) -> String {
    match solution {
        Some(n) => n.to_string(),
        None => String::from("No Solution"),
    }
}

pub fn read_input() -> Vec<i64> {
    fs::read_to_string("inputs/day01.txt")
        .expect("Unable to read day 1 input")
        .lines()
        .map(|line| {
            line.parse()
                .expect(&format!("Unable to parse {} into a number", &line))
        })
        .collect()
}
