use day_03::process_part_2;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let result = process_part_2(&input);
    println!("{}", result);
}
