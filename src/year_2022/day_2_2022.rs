use std::collections::HashMap;

use crate::day_result::DayResult;
use crate::helpers::fetch_input;

/**
 * Day 2 of 2022.
 */
pub struct Day2;

impl DayResult for Day2 {
    fn print_day_result() {
        let input = fetch_input(2, 2022);
        println!("Result 1: {}", get_result_1(&input));
        println!("Result 2: {}", get_result_2(&input));
    }
}

fn get_result_1(input: &[String]) -> u32 {
    let mut results: Vec<u32> = Vec::new();

    let char_beats = mapping();
    let equal = equal();

    for game in input {
        println!("Game: {}", game);

        let (player_1, player_2) = parse_game(game);

        if equal.get(&player_1) == Some(&player_2) {
            results.push(3);
        } else if char_beats.get(&player_2) == Some(&player_1) {
            results.push(6);
        }

        if player_2 == 'X' {
            results.push(1);
        } else if player_2 == 'Y' {
            results.push(2);
        } else if player_2 == 'Z' {
            results.push(3);
        }
    }

    results.iter().sum()
}

fn get_result_2(input: &[String]) -> u32 {
    0
}

fn parse_game(input: &str) -> (char, char) {
    let mut chars = input.chars().filter(|c| !c.is_whitespace());
    (chars.next().unwrap(), chars.next().unwrap())
}

fn equal() -> HashMap<char, char> {
    let mut map = HashMap::new();
    map.insert('A', 'X');
    map.insert('B', 'Y');
    map.insert('C', 'Z');
    map
}

fn mapping() -> HashMap<char, char> {
    let mut map = HashMap::new();
    map.insert('X', 'C');
    map.insert('Y', 'A');
    map.insert('Z', 'B');

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &[&str] = &["A Y", "B X", "C Z"];

    fn get_test_input() -> Vec<String> {
        TEST_INPUT.iter().map(|&s| s.to_string()).collect()
    }

    #[test]
    fn test_get_result_1() {
        let result = get_result_1(&get_test_input());
        assert_eq!(result, 15);
    }

    #[test]
    #[ignore]
    fn test_get_result_2() {
        let result = get_result_2(&get_test_input());
        assert_eq!(result, 0);
    }
}
