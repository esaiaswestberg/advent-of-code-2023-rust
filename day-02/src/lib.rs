use std::str::FromStr;

#[derive(PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err("Unknown color: ".to_string() + s),
        }
    }
}

fn count_color(color: Color, colors: &Vec<(i32, Color)>) -> i32 {
    colors
        .into_iter()
        .filter(|(_, c)| *c == color)
        .map(|(count, _)| count)
        .sum()
}

struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colors: Vec<(i32, Color)> = s
            .split(',')
            .map(|part| part.trim())
            .map(|part| {
                let parts: Vec<&str> = part.split(' ').collect();
                (
                    parts[0]
                        .parse::<i32>()
                        .expect("Unable to parse color count"),
                    Color::from_str(parts[1]).expect("Unable to parse color name"),
                )
            })
            .collect();

        let red_count = count_color(Color::Red, &colors);
        let green_count = count_color(Color::Green, &colors);
        let blue_count = count_color(Color::Blue, &colors);

        Ok(Round {
            red: red_count,
            green: green_count,
            blue: blue_count,
        })
    }
}

struct Game {
    nr: i32,
    rounds: Vec<Round>,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nr: i32 = s.split(':').collect::<Vec<&str>>()[0]
            .split(' ')
            .collect::<Vec<&str>>()[1]
            .parse()
            .expect("Invalid number");

        let rounds: Vec<Round> = s.split(':').collect::<Vec<&str>>()[1]
            .split(';')
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|str| Round::from_str(str).expect(""))
            .collect();

        Ok(Game {
            nr: nr,
            rounds: rounds,
        })
    }
}

trait FindMinimum {
    fn find_mimimum(&self) -> Round;
}

impl FindMinimum for Game {
    fn find_mimimum(&self) -> Round {
        let highest_red: i32 = self.rounds.iter().map(|round| round.red).max().unwrap();
        let highest_green: i32 = self.rounds.iter().map(|round| round.green).max().unwrap();
        let highest_blue: i32 = self.rounds.iter().map(|round| round.blue).max().unwrap();

        Round {
            red: highest_red,
            green: highest_green,
            blue: highest_blue,
        }
    }
}

trait IsPossible {
    fn is_possible(&self, max_red: i32, max_green: i32, max_blue: i32) -> bool;
}

impl IsPossible for Game {
    fn is_possible(&self, max_red: i32, max_green: i32, max_blue: i32) -> bool {
        let min = self.find_mimimum();

        min.red <= max_red && min.green <= max_green && min.blue <= max_blue
    }
}

pub fn process_part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| Game::from_str(line).expect("Unable to parse line"))
        .filter(|game| game.is_possible(12, 13, 14))
        .map(|game| game.nr)
        .sum::<i32>()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    input
        .lines()
        .map(|line| Game::from_str(line).expect("Unable to parse line"))
        .map(|game| game.find_mimimum())
        .map(|min| min.red * min.green * min.blue)
        .sum::<i32>()
        .to_string()
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
    fn test_part_2() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "2286");
    }
}
