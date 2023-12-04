use std::collections::{HashSet, VecDeque};

struct Number {
    chars: Vec<char>,
    is_adjacent: bool,
}

trait ToInt {
    fn to_int(&self) -> i32;
}

impl ToInt for Number {
    fn to_int(&self) -> i32 {
        if self.chars.len() == 0 {
            return 0;
        }

        let str: String = self.chars.iter().collect();
        str.parse().expect("Invalid digits")
    }
}

fn has_adjacent_symbol(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    const ADJACENT_COORDINATES: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for (dx, dy) in ADJACENT_COORDINATES {
        let new_x = match x as i32 + dx {
            nx if nx >= 0 => nx as usize,
            _ => continue,
        };

        let new_y = match y as i32 + dy {
            ny if ny >= 0 => ny as usize,
            _ => continue,
        };

        if new_y >= grid.len() || new_x >= grid[new_y].len() {
            continue;
        }

        let value = grid[new_y][new_x];
        let digit = value.is_digit(10);
        let dot = value == '.';

        if digit == false && dot == false {
            return true;
        }
    }

    false
}

pub fn process_part_1(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut numbers: Vec<Number> = Vec::new();
    numbers.push(Number {
        chars: Vec::new(),
        is_adjacent: false,
    });

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let value = grid[y][x];
            let is_digit = value.is_digit(10);

            if is_digit == false {
                // No number, so begin a new number block.
                numbers.push(Number {
                    chars: Vec::new(),
                    is_adjacent: false,
                });
            } else {
                // A number, so add the number to the current number block.
                let number: &mut Number = numbers.last_mut().expect("No number block!");
                number.chars.push(value);

                // If the current digit is adjacent to a symbol, set it so.
                let has_adjacent_symbol = has_adjacent_symbol(&grid, x, y);
                if has_adjacent_symbol == true {
                    number.is_adjacent = true;
                }
            }
        }

        numbers.push(Number {
            chars: Vec::new(),
            is_adjacent: false,
        });
    }

    numbers
        .into_iter()
        .filter(|n| n.is_adjacent)
        .map(|n| n.to_int())
        .sum::<i32>()
        .to_string()
}

fn get_full_number(grid: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut digits: VecDeque<char> = [grid[y][x]].into();
    let row = &grid[y];

    // Go backwards
    for nx in (0..x).rev() {
        let value = row[nx];
        if value.is_digit(10) {
            digits.push_front(value);
        } else {
            break;
        }
    }

    // Go forwards
    for nx in (x + 1)..(row.len()) {
        let value = row[nx];
        if value.is_digit(10) {
            digits.push_back(value);
        } else {
            break;
        }
    }

    digits
        .into_iter()
        .collect::<String>()
        .parse::<i32>()
        .expect("Unable to parse number")
}

fn get_adjacent_numbers(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<i32> {
    const ADJACENT_COORDINATES: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut numbers: HashSet<i32> = HashSet::new();

    for (dx, dy) in ADJACENT_COORDINATES {
        let new_x = match x as i32 + dx {
            nx if nx >= 0 => nx as usize,
            _ => continue,
        };

        let new_y = match y as i32 + dy {
            ny if ny >= 0 => ny as usize,
            _ => continue,
        };

        if new_y >= grid.len() || new_x >= grid[new_y].len() {
            continue;
        }

        let value = grid[new_y][new_x];
        let digit = value.is_digit(10);

        if digit == true {
            numbers.insert(get_full_number(grid, new_x, new_y));
        }
    }

    numbers.into_iter().collect()
}

pub fn process_part_2(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut sum: i32 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != '*' {
                continue;
            }

            let adjacent_numbers = get_adjacent_numbers(&grid, x, y);
            if adjacent_numbers.len() != 2 {
                continue;
            }

            sum += adjacent_numbers[0] * adjacent_numbers[1];
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part_1() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "4361");
    }

    #[test]
    fn test_part_2() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "467835");
    }
}
