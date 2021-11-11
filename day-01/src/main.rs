use day_01::*;

fn main() {
    let inputs = read_input();
    let first = solution_to_string(first_solution(&inputs));
    println!("First Solution: {}", first);
    let second = solution_to_string(second_solution(&inputs));
    println!("Second Solution: {}", second);
}
