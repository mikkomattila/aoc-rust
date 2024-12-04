use crate::day_result::DayResult;

#[allow(unused)]
use crate::helpers::fetch_input;

/**
 * Day 4 of 2024:
 * Ceres Search
 */
pub struct Day4_2024;

impl DayResult for Day4_2024 {
    fn print_day_result() {
        let input = fetch_input(4, 2024);
        println!("Result 1: {}", get_result_1(input.clone()));
        println!("Result 2: {}", get_result_2(input));
    }
}

#[allow(unused)]
fn get_result_1(input: Vec<String>) -> u32 {
    0
}

#[allow(unused)]
fn get_result_2(input: Vec<String>) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &[&str] = &["MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX"];

    fn get_test_input() -> Vec<String> {
        TEST_INPUT.iter().map(|&s| s.to_string()).collect()
    }

    #[test]
    fn test_get_result_1() {
        let result = get_result_1(get_test_input());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_result_2() {
        let result = get_result_2(get_test_input());
        assert_eq!(result, 0);
    }
}
