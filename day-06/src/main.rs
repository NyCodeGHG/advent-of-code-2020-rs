use day_06::{get_questions_answered_with_yes, read_input};

fn main() {
    let groups = read_input();
    let count = get_questions_answered_with_yes(&groups);
    println!("{} questions answered with yes", count);
}
