use core::num;

use crate::day_result::DayResult;
use crate::helpers::fetch_input;

/**
 * Day 7 of 2024:
 * Bridge Repair
 */
pub struct Day7_2024;

impl DayResult for Day7_2024 {
    fn print_day_result() {
        let input = fetch_input(7, 2024);
        println!("Result 1: {}", get_result_1(input));
    }
}

#[derive(Debug)]
pub struct Line {
    pub result: i64,
    pub numbers: Vec<i64>,
}

fn parse_input(input: Vec<String>) -> Vec<Line> {
    input
        .iter()
        .map(|line| {
            let mut parts = line.split(": ");
            let result = parts
                .next()
                .expect("No result found")
                .parse::<i64>()
                .expect("Invalid test value");
            let numbers = parts
                .next()
                .expect("No numbers found")
                .split_whitespace()
                .map(|s| s.parse::<i64>().expect("Invalid number"))
                .collect::<Vec<i64>>();
            Line { result, numbers }
        })
        .collect()
}

fn get_result_1(input: Vec<String>) -> i64 {
    parse_input(input)
        .into_iter()
        .filter(should_add_result)
        .map(|line| line.result)
        .sum()
}

fn should_add_result(line: &Line) -> bool {
    let numbers_count = line.numbers.len();
    let operators_count = numbers_count - 1;
    let total_combinations = 2i64.pow(operators_count as u32);
    let operators = ['+', '*'];

    let operator_combinations = (0..total_combinations).map(|num| {
        (0..operators_count)
            .map(|i| operators[(num >> i) as usize & 1])
            .collect::<Vec<char>>()
    });

    let expressions: Vec<String> = operator_combinations
        .map(|num| {
            let mut expression = String::new();
            for (i, n) in line.numbers.iter().enumerate() {
                expression.push_str(&n.to_string());
                if i < operators_count {
                    expression.push_str(&num[i].to_string());
                }
            }
            expression
        })
        .collect();

    for expression in expressions {
        if evaluate_expression(&expression) == line.result {
            return true;
        }
    }

    false
}

fn evaluate_expression(expression: &str) -> i64 {
    let mut result = 0;
    let mut num = 0;
    let mut sign = '+';

    for (i, ch) in expression.chars().enumerate() {
        if ch.is_ascii_digit() {
            num = num * 10 + ch.to_digit(10).expect("Invalid digit") as i64;
        }
        if !ch.is_ascii_digit() || i == expression.len() - 1 {
            match sign {
                '+' => result += num,
                '*' => result *= num,
                _ => panic!("Invalid operator"),
            }
            sign = ch;
            num = 0;
        }
    }

    result
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
}
