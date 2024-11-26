use crate::day_result::DayResult;
use crate::helpers::fetch_input;
use std::collections::HashSet;

/**
 * Day 3 of 2022:
 * Rucksack Reorganization
 */
pub struct Day3;

impl DayResult for Day3 {
    fn print_day_result() {
        let input = fetch_input(3, 2022);
        println!("Result 1: {}", get_result_1(&input));
        println!("Result 2: {}", get_result_2(&input));
    }
}

fn get_result_1(input: &[String]) -> u16 {
    let mut total_sum = 0;

    for line in input {
        let (first, second) = line.split_at(line.len() / 2);

        let set_1: HashSet<char> = first.chars().collect();
        let set_2: HashSet<char> = second.chars().collect();
        let matching_chars: HashSet<_> = get_matching_chars(set_1, set_2);

        let sum: u16 = matching_chars.iter().map(|&c| get_priority(c)).sum();
        total_sum += sum;
    }

    total_sum
}

fn get_result_2(input: &[String]) -> u16 {
    let mut total_sum = 0;

    for chunk in input.chunks(3) {
        if chunk.len() != 3 {
            break;
        }

        let sets: Vec<HashSet<char>> = chunk.iter().map(|line| line.chars().collect()).collect();
        let match_first_second = get_matching_chars(sets[0].clone(), sets[1].clone());
        let match_all: HashSet<_> = get_matching_chars(match_first_second, sets[2].clone());

        let sum: u16 = match_all.iter().map(|&c| get_priority(c)).sum();
        total_sum += sum;
    }

    total_sum
}

fn get_priority(c: char) -> u16 {
    match c {
        'a'..='z' => (c as u16) - ('a' as u16) + 1,
        'A'..='Z' => (c as u16) - ('A' as u16) + 27,
        _ => 0,
    }
}

fn get_matching_chars(set_1: HashSet<char>, set_2: HashSet<char>) -> HashSet<char> {
    set_1.intersection(&set_2).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &[&str] = &[
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];

    fn get_test_input() -> Vec<String> {
        TEST_INPUT.iter().map(|&s| s.to_string()).collect()
    }

    #[test]
    fn test_get_result_1() {
        let result = get_result_1(&get_test_input());
        assert_eq!(result, 157);
    }

    #[test]
    fn test_get_result_2() {
        let result = get_result_2(&get_test_input());
        assert_eq!(result, 70);
    }
}
