use day_04::{count_filled_passports, count_valid_passports, read_input};

fn main() {
    let passports = read_input();
    println!("{} passports", passports.len());
    println!("{} filled passports", count_filled_passports(&passports));
    println!("{} valid passports", count_valid_passports(&passports));
}
