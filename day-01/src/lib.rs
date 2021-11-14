use std::collections::HashMap;
use std::fs;

pub fn first_solution(input: &[i64]) -> Option<i64> {
    let mut pairs: HashMap<i64, i64> = HashMap::new();
    for a in input {
        for b in input {
            if a == b {
                continue;
            }
            pairs.insert(a + b, a * b);
        }
    }
    pairs.get(&2020).copied()
}

pub fn second_solution(input: &[i64]) -> Option<i64> {
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
    pairs.get(&2020).copied()
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
                .unwrap_or_else(|_| panic!("Unable to parse {} into a number", &line))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<i64> {
        vec![1721, 979, 366, 299, 675, 1456]
    }

    #[test]
    fn test_first_solution() {
        assert_eq!(first_solution(&test_data()).unwrap(), 514579);
    }

    #[test]
    fn test_second_solution() {
        assert_eq!(second_solution(&test_data()).unwrap(), 241861950);
    }
}
