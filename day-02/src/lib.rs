#[macro_use]
extern crate lazy_static;

use std::fs;
use std::ops::Range;

use regex::{Captures, Regex};

pub struct Password {
    policy: Range<usize>,
    character: String,
    password: String,
}

pub enum PasswordPolicy {
    RANGE,
    POSITION,
}

impl Password {
    pub fn is_valid(&self, policy: &PasswordPolicy) -> bool {
        match policy {
            PasswordPolicy::RANGE => {
                let occurrence = self.password.matches(&self.character).count();
                occurrence >= self.policy.start && occurrence <= self.policy.end
            }
            PasswordPolicy::POSITION => {
                let char = &self.character;
                let first_position = self.policy.start - 1;
                let second_position = self.policy.end - 1;
                let chars: Vec<char> = self.password.chars().collect();
                (&chars[first_position].to_string() == char)
                    ^ (&chars[second_position].to_string() == char)
            }
        }
    }

    pub fn new(input: &str) -> Result<Password, &str> {
        lazy_static! {
            // https://regex101.com/r/AHV4kK/1
            static ref RE: Regex = Regex::new("(\\d+)-(\\d+) ([a-z]): ([a-z]+)").unwrap();
        }
        let captures: Captures = match RE.captures(input) {
            Some(result) => result,
            None => return Err("Unable to parse input string"),
        };

        let mut captures = captures.iter().skip(1);

        let policy_start: usize = (captures.next().unwrap().unwrap())
            .as_str()
            .parse()
            .unwrap();
        let policy_end: usize = (captures.next().unwrap().unwrap())
            .as_str()
            .parse()
            .unwrap();
        let character = captures.next().unwrap().unwrap().as_str().to_string();
        let password = captures.next().unwrap().unwrap().as_str().to_string();

        Ok(Password {
            policy: policy_start..policy_end,
            character,
            password,
        })
    }
}

pub fn read_input() -> Vec<Password> {
    fs::read_to_string("inputs/day02.txt")
        .expect("Unable to read day 1 input")
        .lines()
        .map(|line| Password::new(line).expect(&*format!("Unable to parse password: {}", line)))
        .collect()
}

pub fn count_valid_passwords(passwords: &[Password], policy: &PasswordPolicy) -> usize {
    passwords
        .iter()
        .filter(|password| password.is_valid(policy))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<Password> {
        vec![
            Password::new("1-3 a: abcde").unwrap(),
            Password::new("1-3 b: cdefg").unwrap(),
            Password::new("2-9 c: ccccccccc").unwrap(),
        ]
    }

    #[test]
    fn test_first_solution() {
        assert_eq!(count_valid_passwords(&test_data(), &PasswordPolicy::RANGE), 2);
    }

    #[test]
    fn test_second_solution() {
        assert_eq!(count_valid_passwords(&test_data(), &PasswordPolicy::POSITION), 1);
    }
}
