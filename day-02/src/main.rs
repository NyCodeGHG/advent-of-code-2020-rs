use day_02::{PasswordPolicy, count_valid_passwords, read_input};

fn main() {
    let passwords = read_input();
    let valid_passwords = count_valid_passwords(&passwords, &PasswordPolicy::RANGE);
    println!("There are {} valid passwords.", valid_passwords);

    let valid_passwords = count_valid_passwords(&passwords,&PasswordPolicy::POSITION);
    println!("There are {} valid passwords.", valid_passwords);
}
