use crate::day_result::DayResult;
use crate::helpers::fetch_input;

/**
 * Day 1 of 2024:
 * Historian Hysteria
 */
pub struct Day1;

impl DayResult for Day1 {
    fn print_day_result() {
        let input = fetch_input(1, 2024);
        println!("Result 1: {}", get_result_1(&input));
        println!("Result 2: {}", get_result_2(&input));
    }
}

fn get_result_1(input: &[String]) -> i32 {
    let (left, right) = parse_locations(input);
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn get_result_2(input: &[String]) -> i32 {
    let (left, right) = parse_locations(input);
    left.iter()
        .map(|&value| value * right.iter().filter(|&&x| x == value).count() as i32)
        .sum()
}

fn parse_locations(input: &[String]) -> (Vec<i32>, Vec<i32>) {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .iter()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().expect("Failed to parse string to i32"))
                .collect();
            (numbers[0], numbers[1])
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &[&str] = &["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"];

    fn get_test_input() -> Vec<String> {
        TEST_INPUT.iter().map(|&s| s.to_string()).collect()
    }

    #[test]
    fn test_get_result_1() {
        let result = get_result_1(&get_test_input());
        assert_eq!(result, 11);
    }

    #[test]
    fn test_get_result_2() {
        let result = get_result_2(&get_test_input());
        assert_eq!(result, 31);
    }
}
