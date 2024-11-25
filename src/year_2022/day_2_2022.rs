use std::collections::HashMap;
use std::hash::Hash;

use crate::day_result::DayResult;
use crate::helpers::fetch_input;

/**
 * Day 2 of 2022.
 */
pub struct Day2;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum RpsResult {
    Lose,
    Draw,
    Win,
}

impl DayResult for Day2 {
    fn print_day_result() {
        let input = fetch_input(2, 2022);
        println!("Result 1: {}", get_result_1(&input));
        println!("Result 2: {}", get_result_2(&input));
    }
}

fn get_result_1(input: &[String]) -> u32 {
    let mut score: Vec<u32> = Vec::new();
    let games = parse_rps_games(input);
    for (p1, p2) in games {
        score.push(get_result_score(p1, p2) + get_rps_score(p2));
    }
    score.iter().sum()
}

fn get_result_2(input: &[String]) -> u32 {
    let mut score: Vec<u32> = Vec::new();
    let games = parse_rps_games(input);

    let round_end = get_round_end_map();
    let rps_wins = get_rps_wins_map();
    let rps_loses = get_rps_loses_map();

    for (p1, p2) in games {
        let end = round_end.get(&p2).unwrap();

        let p2_actual = match end {
            RpsResult::Win => *rps_loses.get(&p1).unwrap(),
            RpsResult::Draw => p1,
            RpsResult::Lose => *rps_wins.get(&p1).unwrap(),
        };
        score.push(get_result_score(p1, p2_actual) + get_rps_score(p2_actual));
    }

    score.iter().sum()
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
    let rps_mapping = get_char_rps_map();
    let mut games = Vec::new();

    for game in input {
        let mut chars = game.chars().filter(|c: &char| !c.is_whitespace());
        let p1_char = chars.next().unwrap();
        let p2_char = chars.next().unwrap();

        let p_1 = *rps_mapping.get(&p1_char).unwrap();
        let p_2 = *rps_mapping.get(&p2_char).unwrap();

        games.push((p_1, p_2));
    }
    games
}

fn get_result_score(p1: Rps, p2: Rps) -> u32 {
    match p1 {
        _ if p1 == p2 => 3,
        _ if get_rps_wins_map().get(&p2) == Some(&p1) => 6,
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
