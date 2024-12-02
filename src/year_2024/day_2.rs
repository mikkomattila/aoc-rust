use crate::day_result::DayResult;
use crate::helpers::fetch_input;

/**
 * Day 2 of 2022:
 * Red-Nosed Reports
 */
pub struct Day2_2024;

impl DayResult for Day2_2024 {
    fn print_day_result() {
        let input = fetch_input(2, 2024);
        println!("Result 1: {}", get_result_1(&input));
        println!("Result 2: {}", get_result_2(&input));
    }
}

fn parse_reports(input: &[String]) -> Vec<Vec<i32>> {
    input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().expect("Failed to parse string to i32"))
                .collect()
        })
        .collect()
}

fn is_safe_1(report: Vec<i32>) -> i32 {
    let mut is_ascending: bool = false;
    let mut is_descending = false;

    for i in 0..report.len() - 1 {
        let current = report[i];
        let next = report[i + 1];
        let diff = (next - current).abs();

        if next > current && (1..=3).contains(&diff) {
            is_ascending = true;
        } else if next < current && (1..=3).contains(&diff) {
            is_descending = true;
        } else {
            return 0;
        }
    }

    if is_ascending != is_descending {
        1
    } else {
        0
    }
}

fn get_result_1(input: &[String]) -> i32 {
    parse_reports(input)
        .iter()
        .map(|report| is_safe_1(report.to_vec()))
        .sum()
}

fn get_result_2(input: &[String]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &[&str] = &[
        "7 6 4 2 1",
        "1 2 7 8 9",
        "9 7 6 2 1",
        "1 3 2 4 5",
        "8 6 4 4 1",
        "1 3 6 7 9",
    ];

    fn get_test_input() -> Vec<String> {
        TEST_INPUT.iter().map(|&s| s.to_string()).collect()
    }

    #[test]
    fn test_get_result_1() {
        let result = get_result_1(&get_test_input());
        assert_eq!(result, 2);
    }

    #[test]
    fn test_get_result_2() {
        let result = get_result_2(&get_test_input());
        assert_eq!(result, 4);
    }
}
