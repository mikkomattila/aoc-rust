use std::collections::HashMap;
use std::hash::Hash;

use crate::day_result::DayResult;
use crate::helpers::fetch_input;

/**
 * Day 2 of 2022:
 * Rock Paper Scissors
 */
pub struct Day2;

impl DayResult for Day2 {
    fn print_day_result() {
        let input = fetch_input(2, 2022);
        println!("Result 1: {}", get_result_1(&input));
        println!("Result 2: {}", get_result_2(&input));
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum RpsResult {
    Lose,
    Draw,
    Win,
}

fn get_result_1(input: &[String]) -> u32 {
    parse_rps_games(input)
        .iter()
        .map(|(player_1, player_2)| {
            get_result_score(*player_1, *player_2) + get_rps_score(*player_2)
        })
        .sum()
}

fn get_result_2(input: &[String]) -> u32 {
    let round_end_map = get_round_end_map();
    let rps_wins_map = get_rps_wins_map();
    let rps_loses_map = get_rps_loses_map();

    parse_rps_games(input)
        .iter()
        .map(|(player_1, player_2)| {
            let round_end_result = round_end_map
                .get(player_2)
                .expect("Player 2 not found in round_end_map");

            let player_2_round_end = match round_end_result {
                RpsResult::Win => *rps_loses_map
                    .get(player_1)
                    .expect("Player 1 not found in rps_loses_map"),
                RpsResult::Draw => *player_1,
                RpsResult::Lose => *rps_wins_map
                    .get(player_1)
                    .expect("Player 1 not found in rps_wins_map"),
            };

            get_result_score(*player_1, player_2_round_end) + get_rps_score(player_2_round_end)
        })
        .sum()
}

fn get_round_end_map() -> HashMap<Rps, RpsResult> {
    HashMap::from([
        (Rps::Rock, RpsResult::Lose),
        (Rps::Paper, RpsResult::Draw),
        (Rps::Scissors, RpsResult::Win),
    ])
}

fn get_char_rps_map() -> HashMap<char, Rps> {
    HashMap::from([
        ('A', Rps::Rock),
        ('B', Rps::Paper),
        ('C', Rps::Scissors),
        ('X', Rps::Rock),
        ('Y', Rps::Paper),
        ('Z', Rps::Scissors),
    ])
}

fn get_rps_wins_map() -> HashMap<Rps, Rps> {
    HashMap::from([
        (Rps::Rock, Rps::Scissors),
        (Rps::Paper, Rps::Rock),
        (Rps::Scissors, Rps::Paper),
    ])
}

fn get_rps_loses_map() -> HashMap<Rps, Rps> {
    get_rps_wins_map().iter().map(|(k, v)| (*v, *k)).collect()
}

fn parse_rps_games(input: &[String]) -> Vec<(Rps, Rps)> {
    let char_rps_map = get_char_rps_map();
    let mut result: Vec<(Rps, Rps)> = Vec::new();

    for row in input {
        let mut chars = row.chars().filter(|c: &char| !c.is_whitespace());
        let player_1_char = chars.next().expect("Player 1 char not found");
        let player_2_char = chars.next().expect("Player 2 char not found");

        let player_1_rps = *char_rps_map
            .get(&player_1_char)
            .expect("Player 1 RPS not found");

        let player_2_rps = *char_rps_map
            .get(&player_2_char)
            .expect("Player 2 RPS not found");

        result.push((player_1_rps, player_2_rps));
    }
    result
}

fn get_result_score(player_1: Rps, player_2: Rps) -> u32 {
    match player_1 {
        _ if player_1 == player_2 => 3,
        _ if get_rps_wins_map().get(&player_2) == Some(&player_1) => 6,
        _ => 0,
    }
}

fn get_rps_score(value: Rps) -> u32 {
    match value {
        Rps::Rock => 1,
        Rps::Paper => 2,
        Rps::Scissors => 3,
    }
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
    fn test_get_result_2() {
        let result = get_result_2(&get_test_input());
        assert_eq!(result, 12);
    }
}
