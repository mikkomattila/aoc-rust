use crate::day_result::DayResult;
use crate::helpers::fetch_input;

/**
 * Day 2 of 2024:
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

fn is_safe_report(report: Vec<i32>) -> i32 {
    let mut direction = 0;

    for i in 0..report.len() - 1 {
        let current = report[i];
        let next = report[i + 1];
        let diff = (next - current).abs();

        if (1..=3).contains(&diff) {
            let new_direction = (next - current).signum();
            if direction == 0 {
                direction = new_direction;
            } else if direction != new_direction {
                return 0;
            }
        } else {
            return 0;
        }
    }

    1
}

fn is_safe_with_removal(report: Vec<i32>) -> i32 {
    let safe_1 = is_safe_report(report.to_vec());
    if safe_1 == 1 {
        return 1;
    }

    for i in 0..report.len() {
        let mut modified_report = report.to_vec();
        modified_report.remove(i);

        if is_safe_report(modified_report) == 1 {
            return 1;
        }
    }

    0
}

fn get_result_1(input: &[String]) -> i32 {
    parse_reports(input)
        .iter()
        .map(|report| is_safe_report(report.to_vec()))
        .sum()
}

fn get_result_2(input: &[String]) -> i32 {
    parse_reports(input)
        .iter()
        .map(|report| is_safe_with_removal(report.to_vec()))
        .sum()
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
