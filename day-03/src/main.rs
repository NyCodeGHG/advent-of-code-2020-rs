use day_02::{find_trees, read_input};

fn main() {
    let map = read_input();
    let trees = find_trees(&map, 3, 1);
    println!("{} Trees", trees);

    let trees: u32 = vec![
        find_trees(&map, 1, 1),
        find_trees(&map, 3, 1),
        find_trees(&map, 5, 1),
        find_trees(&map, 7, 1),
        find_trees(&map, 1, 2),
    ].iter().product();
    println!("{} Trees", trees);
}
