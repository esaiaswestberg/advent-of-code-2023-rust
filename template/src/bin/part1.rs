use day_xx::process_part_1;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let result = process_part_1(&input);
    println!("{}", result);
}
