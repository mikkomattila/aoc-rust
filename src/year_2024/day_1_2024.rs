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
    let mut result = 0;

    for i in 0..left.len() {
        let diff = if left[i] > right[i] {
            left[i] - right[i]
        } else {
            right[i] - left[i]
        };
        result += diff;
    }

    result
}

fn get_result_2(input: &[String]) -> i32 {
    let (left, right) = parse_locations(input);
    let mut result = 0;

    for i in 0..left.len() {
        let factor = right.iter().filter(|&&x| x == left[i]).count();
        let left_actual = left[i] * factor as i32;
        result += left_actual;
    }

    result
}

fn parse_locations(input: &[String]) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Failed to parse string to u32"))
            .collect();

        left.push(numbers[0]);
        right.push(numbers[1]);
    }

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
