use day_04::{count_valid_passports, read_input};

fn main() {
    let passports = read_input();
    println!("{} valid passports", count_valid_passports(&passports));
}
