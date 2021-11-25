use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Seat {
    pub row: u32,
    pub column: u32,
}

impl Seat {
    pub fn seat_id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

impl FromStr for Seat {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = s[..7].replace("F", "0").replace("B", "1");
        let column = s[7..10].replace("L", "0").replace("R", "1");
        let row = match u32::from_str_radix(&row, 2) {
            Ok(n) => n,
            Err(err) => return Err(err),
        };
        let column = match u32::from_str_radix(&column, 2) {
            Ok(n) => n,
            Err(err) => return Err(err),
        };
        Ok(Seat { row, column })
    }
}

pub fn get_highest_seat_id(seats: &[Seat]) -> Option<&Seat> {
    seats.iter().max_by_key(|seat| seat.seat_id())
}

pub fn get_missing_seat_id(seats: &[Seat]) -> Option<u32> {
    let seat_id_iterator = seats.iter().map(|seat| seat.seat_id());
    let min = match seat_id_iterator.clone().min() {
        Some(n) => n,
        None => return None,
    };
    let max = match seat_id_iterator.clone().max() {
        Some(n) => n,
        None => return None,
    };
    let seat_ids: Vec<u32> = seat_id_iterator.collect();
    for n in min..max {
        if !seat_ids.contains(&n) {
            return Some(n);
        }
    }
    None
}

pub fn read_input() -> Vec<Seat> {
    fs::read_to_string("inputs/day05.txt")
        .expect("Unable to read day 5 input.")
        .lines()
        .map(|line| Seat::from_str(line).unwrap())
        .collect()
}
