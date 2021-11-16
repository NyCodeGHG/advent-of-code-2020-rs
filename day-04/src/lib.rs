#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

use regex::Regex;

pub struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    id: Option<String>,
}

pub struct Height {
    value: u32,
    unit: HeightUnit,
}

impl FromStr for Height {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new("^(\\d+)(cm|in)$").unwrap();
        }
        let captures = match RE.captures(&s) {
            Some(captures) => captures,
            None => return Err(format!("Invalid height: {}", s)),
        };

        let size = match captures.get(1) {
            Some(size) => size.as_str().parse::<u32>().unwrap(),
            None => return Err(format!("Invalid height: {}", s)),
        };

        let unit = match captures.get(2) {
            Some(unit) => match unit.as_str() {
                "cm" => HeightUnit::Centimeters,
                "in" => HeightUnit::Inches,
                _ => return Err(format!("Invalid unit in height: {}", s)),
            },
            None => return Err(format!("Invalid height: {}", s)),
        };
        Ok(Height { value: size, unit })
    }
}

#[derive(PartialEq)]
pub enum HeightUnit {
    Inches,
    Centimeters,
}

impl Passport {
    pub fn are_required_fields_present(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.id.is_some()
    }

    pub fn is_valid(&self) -> bool {
        lazy_static! {
            static ref HAIR_COLOR_REGEX: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
            static ref EYE_COLOR_REGEX: Regex =
                Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            static ref ID_REGEX: Regex = Regex::new("^\\d{9}$").unwrap();
        }
        self.are_required_fields_present()
            && HAIR_COLOR_REGEX.is_match(&self.hair_color.as_ref().unwrap())
            && EYE_COLOR_REGEX.is_match(&self.eye_color.as_ref().unwrap())
            && ID_REGEX.is_match(&self.id.as_ref().unwrap())
            && self
                .birth_year
                .as_ref()
                .unwrap()
                .parse::<u32>()
                .map_or(false, |value| value >= 1920 && value <= 2002)
            && self
                .issue_year
                .as_ref()
                .unwrap()
                .parse::<u32>()
                .map_or(false, |value| value >= 2010 && value <= 2020)
            && self
                .expiration_year
                .as_ref()
                .unwrap()
                .parse::<u32>()
                .map_or(false, |value| value >= 2020 && value <= 2030)
            && self
                .height
                .as_ref()
                .unwrap()
                .parse::<Height>()
                .map_or(false, |value| match value.unit {
                    HeightUnit::Inches => value.value >= 59 && value.value <= 76,
                    HeightUnit::Centimeters => value.value >= 150 && value.value <= 193,
                })
    }

    pub fn from(string: &str) -> Passport {
        let map: HashMap<&str, &str> = string
            .split(' ')
            .map(|value| value.split(':'))
            .map(|mut value| (value.next(), value.next()))
            .filter(|value| value.0.is_some() && value.1.is_some())
            .map(|value| (value.0.unwrap(), value.1.unwrap()))
            .into_iter()
            .collect();
        let birth_year: Option<String> = parse_value("byr", &map);
        let expiration_year: Option<String> = parse_value("eyr", &map);
        let eye_color: Option<String> = parse_value("ecl", &map);
        let hair_color: Option<String> = parse_value("hcl", &map);
        let height: Option<String> = parse_value("hgt", &map);
        let id: Option<String> = parse_value("pid", &map);
        let issue_year: Option<String> = parse_value("iyr", &map);
        let passport = Passport {
            birth_year,
            expiration_year,
            eye_color,
            hair_color,
            height,
            id,
            issue_year,
        };
        passport
    }
}

fn parse_value<T>(key: &str, map: &HashMap<&str, &str>) -> Option<T>
where
    T: std::str::FromStr,
{
    match map.get(key) {
        Some(value) => match value.parse::<T>() {
            Ok(value) => Some(value),
            Err(_) => panic!("{:?}", value),
        },
        None => None,
    }
}

pub fn count_filled_passports(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|passport| passport.are_required_fields_present())
        .count()
}

pub fn count_valid_passports(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|passport| passport.is_valid())
        .count()
}

pub fn read_input() -> Vec<Passport> {
    fs::read_to_string("inputs/day04.txt")
        .expect("Unable to read day 3 input")
        .split("\n\n")
        .map(|value| Passport::from(&value.replace('\n', " ")))
        .collect()
}
