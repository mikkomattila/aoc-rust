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

fn parse_reports(input: &[String]) -> Vec<Vec<u32>> {
    input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u32>().expect("Failed to parse string to u32"))
                .collect()
        })
        .collect()
}

fn is_safe(a: &u32, b: &u32) -> bool {
    a.abs_diff(*b) >= 1 && a.abs_diff(*b) <= 3
}

fn get_result_1(input: &[String]) -> u32 {
    let mut score = 0;
    let reports = parse_reports(input);

    for report in reports {
        let mut invalid = true;
        let mut asc = false;
        let mut desc = false;

        for window in report.windows(2) {
            if let [current, next] = window {
                if next > current && is_safe(next, current) {
                    asc = true;
                } else if next < current && is_safe(next, current) {
                    desc = true;
                } else {
                    invalid = false;
                    break;
                }
            }
        }

        if invalid && !(asc && desc) {
            score += 1;
        }
    }

    score
}

fn get_result_2(input: &[String]) -> u32 {
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
        assert_eq!(result, 0);
    }
}
