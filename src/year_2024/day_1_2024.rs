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
    let mut first: Vec<i32> = Vec::new();
    let mut second: Vec<i32> = Vec::new();

    for line in input {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Failed to parse string to u32"))
            .collect();

        first.push(numbers[0]);
        second.push(numbers[1]);
    }

    first.sort_unstable();
    second.sort_unstable();

    let mut result = 0;

    for i in 0..first.len() {
        println!("{}", first[i]);
        println!("{}", second[i]);

        let diff = if first[i] > second[i] {
            first[i] - second[i]
        } else {
            second[i] - first[i]
        };
        result += diff;
    }

    println!("{:?}", result);

    result
}

fn get_result_2(input: &[String]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &[&str] = &[];

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
        assert_eq!(result, 0);
    }
}
