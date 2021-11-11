use day_02::{count_valid_passwords, read_input};

fn main() {
    let passwords = read_input();
    let valid_passwords = count_valid_passwords(&passwords);
    println!("There are {} valid passwords.", valid_passwords);
}
