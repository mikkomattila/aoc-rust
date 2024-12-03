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
        println!("Result 1: {}", get_multiplication_result(input));
    }
}

fn get_multiplication_result(input: String) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_get_multiplication_result() {
        let result = get_multiplication_result(TEST_INPUT.to_string());
        assert_eq!(result, 161);
    }
}
