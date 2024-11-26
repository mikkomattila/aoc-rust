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

fn get_result_1(input: &[String]) -> u32 {
    let mut total_sum = 0;
    for line in input {
        let (first_half, second_half) = line.split_at(line.len() / 2);
        let first_half_set: HashSet<char> = first_half.chars().collect();
        let second_half_set: HashSet<char> = second_half.chars().collect();
        let matching_chars: HashSet<_> = first_half_set.intersection(&second_half_set).collect();
        let sum: u32 = matching_chars.iter().map(|&&c| get_priority(c)).sum();
        total_sum += sum;
    }

    total_sum
}

fn get_result_2(input: &[String]) -> u32 {
    0
}

fn get_priority(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => 0,
    }
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
        assert_eq!(result, 0);
    }
}
