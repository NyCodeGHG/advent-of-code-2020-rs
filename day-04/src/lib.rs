use std::collections::HashMap;
use std::fs;

pub struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    pub fn is_valid(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.id.is_some()
        // country_id is allowed to be missing
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
        let birth_year: Option<String> = parse_string("byr", &map);
        let country_id: Option<String> = parse_string("cid", &map);
        let expiration_year: Option<String> = parse_string("eyr", &map);
        let eye_color: Option<String> = parse_string("ecl", &map);
        let hair_color: Option<String> = parse_string("hcl", &map);
        let height: Option<String> = parse_string("hgt", &map);
        let id: Option<String> = parse_string("pid", &map);
        let issue_year: Option<String> = parse_string("iyr", &map);
        Passport {
            birth_year,
            country_id,
            expiration_year,
            eye_color,
            hair_color,
            height,
            id,
            issue_year,
        }
    }
}

fn parse_string(key: &str, map: &HashMap<&str, &str>) -> Option<String> {
    map.get(key)
        .map(|value| value.to_string())
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
        .map(|value| Passport::from(value.replace('\n', " ").as_str()))
        .collect()
}
