use crate::day_result::DayResult;

#[allow(unused)]
use crate::helpers::fetch_input;

/**
 * Day 7 of 2024:
 * Bridge Repair
 */
pub struct Day7_2024;

impl DayResult for Day7_2024 {
    fn print_day_result() {
        let input = fetch_input(7, 2024);
        println!("Result 1: {}", get_result_1(input.clone()));
        println!("Result 2: {}", get_result_2(input));
    }
}

#[allow(unused)]
fn get_result_1(input: Vec<String>) -> i32 {
    0
}

#[allow(unused)]
fn get_result_2(input: Vec<String>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &[&str] = &[
        "190: 10 19",
        "3267: 81 40 27",
        "83: 17 5",
        "156: 15 6",
        "7290: 6 8 6 15",
        "161011: 16 10 13",
        "192: 17 8 14",
        "21037: 9 7 18 13",
        "292: 11 6 16 20",
    ];

    fn get_test_input() -> Vec<String> {
        TEST_INPUT.iter().map(|&s| s.to_string()).collect()
    }

    #[test]
    fn test_get_result_1() {
        let result = get_result_1(get_test_input());
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_get_result_2() {
        let result = get_result_2(get_test_input());
        assert_eq!(result, 0);
    }
}
