use crate::day_result::DayResult;
use crate::helpers::fetch_input;

use regex::Regex;

/**
 * Day 3 of 2024:
 * Mull It Over
 */
pub struct Day3_2024;

impl DayResult for Day3_2024 {
    fn print_day_result() {
        let input = fetch_input(3, 2024).join("\n");
        println!("Result 1: {}", get_result_1(input.clone()));
        println!("Result 2: {}", get_result_2(input));
    }
}

fn get_result_1(input: String) -> i32 {
    Regex::new(r"mul\((\d+),(\d+)\)")
        .expect("Failed to create regex")
        .captures_iter(&input)
        .map(|captures: regex::Captures<'_>| {
            let a: i32 = captures[1].parse().expect("Failed to parse string to i32");
            let b: i32 = captures[2].parse().expect("Failed to parse string to i32");
            a * b
        })
        .sum()
}

fn get_result_2(input: String) -> i32 {
    let mut enabled = true;
    Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))")
        .expect("Failed to create regex")
        .captures_iter(&input)
        .filter_map(|captures| {
            if captures.get(3).is_some() {
                enabled = true;
                None
            } else if captures.get(4).is_some() {
                enabled = false;
                None
            } else if enabled && captures.get(1).is_some() && captures.get(2).is_some() {
                let a: i32 = captures[1].parse().expect("Failed to parse string to i32");
                let b: i32 = captures[2].parse().expect("Failed to parse string to i32");
                Some(a * b)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    static TEST_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_get_result_1() {
        let result = get_result_1(TEST_INPUT_1.to_string());
        assert_eq!(result, 161);
    }

    #[test]
    fn test_get_result_2() {
        let result = get_result_2(TEST_INPUT_2.to_string());
        assert_eq!(result, 48);
    }
}
