pub fn process_part_1(input: &str) -> String {
    let digitset: Vec<char> = "0123456789".chars().collect();

    let lines: Vec<&str> = input.lines().collect();
    let digits: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let digits: Vec<char> = chars.into_iter().filter(|c| digitset.contains(c)).collect();
            digits
        })
        .collect();

    let numbers: Vec<i32> = digits
        .into_iter()
        .map(|digits| {
            let first_digit: char = digits[0];

            let last_index = digits.len() - 1;
            let last_digit: char = digits[last_index];

            let concat = first_digit.to_string() + &last_digit.to_string();
            let number: i32 = concat.parse().unwrap();

            number
        })
        .collect();

    let sum: i32 = numbers.into_iter().sum();
    sum.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let digitset: Vec<char> = "0123456789".chars().collect();

    let spelled_out: Vec<(&str, &str)> = [
        ("three", "3"),
        ("seven", "7"),
        ("eight", "8"),
        ("zero", "0"),
        ("nine", "9"),
        ("four", "4"),
        ("five", "5"),
        ("one", "1"),
        ("two", "2"),
        ("six", "6"),
    ]
    .to_vec();

    let lines: Vec<&str> = input.lines().collect();
    let new_lines: Vec<_> = lines
        .into_iter()
        .map(|line| {
            let mut line: String = line.to_string();

            for i in 0..line.len() {
                for number in spelled_out.iter() {
                    let part: Vec<char> = line
                        .chars()
                        .skip(i)
                        .take(number.0.len())
                        .into_iter()
                        .collect();

                    let part_str: String = part.into_iter().collect();

                    if part_str == number.0 {
                        let mut chars: Vec<char> = line.chars().into_iter().collect();
                        chars[i] = number.1.chars().nth(0).unwrap();
                        line = chars.into_iter().collect();
                    }
                }
            }

            line
        })
        .collect();

    let digits: Vec<Vec<char>> = new_lines
        .into_iter()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let digits: Vec<char> = chars.into_iter().filter(|c| digitset.contains(c)).collect();
            digits
        })
        .collect();

    let numbers: Vec<i32> = digits
        .into_iter()
        .map(|digits| {
            let first_digit: char = digits[0];

            let last_index = digits.len() - 1;
            let last_digit: char = digits[last_index];

            let concat = first_digit.to_string() + &last_digit.to_string();
            let number: i32 = concat.parse().unwrap();

            number
        })
        .collect();

    let sum: i32 = numbers.into_iter().sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_PART1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_part_1() {
        let result = process_part_1(INPUT_PART1);
        assert_eq!(result, "142");
    }

    const INPUT_PART2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part_2() {
        let result = process_part_2(INPUT_PART2);
        assert_eq!(result, "281");
    }
}
