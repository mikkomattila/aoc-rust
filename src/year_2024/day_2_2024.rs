use crate::day_result::DayResult;
use crate::helpers::fetch_input;

/**
 * Day 2 of 2022:
 *
 */
pub struct Day2;

impl DayResult for Day2 {
    fn print_day_result() {
        let input = fetch_input(2, 2024);
        println!("Result 1: {}", get_result_1(&input));
        println!("Result 2: {}", get_result_2(&input));
    }
}

fn get_result_1(input: &[String]) -> u32 {
    0
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
        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_result_2() {
        let result = get_result_2(&get_test_input());
        assert_eq!(result, 0);
    }
}
