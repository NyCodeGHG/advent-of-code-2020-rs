#[macro_use]
extern crate lazy_static;

use std::fmt::{Display, Formatter};
use std::ops::Range;
use std::fs;

use regex::{Captures, Regex};

pub struct Password {
    policy: Range<usize>,
    character: String,
    password: String,
}

impl Password {
    pub fn is_valid(&self) -> bool {
        let occurrence = self.password.matches(&self.character).count();
        occurrence >= self.policy.start && occurrence <= self.policy.end
    }

    pub fn new(input: &str) -> Result<Password, &str> {
        lazy_static! {
            // https://regex101.com/r/AHV4kK/1
            static ref RE: Regex = Regex::new("(\\d+)-(\\d+) ([a-z]): ([a-z]+)").unwrap();
        }
        let captures: Captures = match RE.captures(&input) {
            Some(result) => result,
            None => return Err("Unable to parse input string")
        };

        let mut captures = captures.iter().skip(1);

        let policy_start: usize = (captures.next().unwrap().unwrap()).as_str().parse().unwrap();
        let policy_end: usize = (captures.next().unwrap().unwrap()).as_str().parse().unwrap();
        let character = captures.next().unwrap().unwrap().as_str().to_string();
        let password = captures.next().unwrap().unwrap().as_str().to_string();

        Ok(Password {
            policy: policy_start..policy_end,
            character,
            password,
        })
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{} {}: {}", self.policy.start.to_string(), self.policy.end.to_string(), self.character, self.password)
    }
}

pub fn read_input() -> Vec<Password> {
    fs::read_to_string("inputs/day02.txt")
        .expect("Unable to read day 1 input")
        .lines()
        .map(|line| Password::new(line).expect(&*format!("Unable to parse password: {}", line)))
        .collect()
}

pub fn count_valid_passwords(passwords: &Vec<Password>) -> usize {
    passwords
        .iter()
        .filter(|password| password.is_valid())
        .count()
}
