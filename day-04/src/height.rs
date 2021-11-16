use std::str::FromStr;

use regex::Regex;

pub struct Height {
    pub value: u32,
    pub unit: HeightUnit,
}

impl FromStr for Height {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new("^(\\d+)(cm|in)$").unwrap();
        }
        let captures = match RE.captures(s) {
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
