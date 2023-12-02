pub fn process_part_1(input: &str) -> String {
    input.to_string()
}

pub fn process_part_2(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part_1() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "8");
    }

    #[test]
    #[ignore = "Not implemented yet"]
    fn test_part_2() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "PLACEHOLDER_OUTPUT");
    }
}
