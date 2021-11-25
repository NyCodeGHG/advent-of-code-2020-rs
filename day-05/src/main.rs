use day_05::{get_highest_seat_id, get_missing_seat_id, read_input};

fn main() {
    let seats = read_input();
    let highest_seat: String =
        get_highest_seat_id(&seats).map_or("None".to_string(), |seat| seat.seat_id().to_string());
    println!("Highest seat id is {}", highest_seat);
    let missing_seat = get_missing_seat_id(&seats).unwrap_or(0);
    println!("Missing seat id is {}", missing_seat);
}
