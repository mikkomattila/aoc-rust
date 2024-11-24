use crate::day_result::DayResult;
use crate::helpers::fetch_input;

/**
 * Day 1 of 2023.
 */
pub struct Day1;

impl DayResult for Day1 {
    fn print_day_result() {
        let input = fetch_input(1, 2022);
        println!("Results for day 1 of 2023");
        println!("Result 1: {}", get_result_1(input));
    }
}

fn get_result_1(input: Vec<String>) -> i32 {
    let split_input = split_by_empty_string(input);
    for array in split_input.iter() {
        for item in array.iter() {
            println!("{}", item);
        }
    }

    0
}

fn split_by_empty_string(input: Vec<String>) -> Vec<Vec<String>> {
    input
        .split(|s| s.is_empty())
        .map(|slice| slice.to_vec())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result_1() {
        let input = vec![
            "1000".to_string(),
            "2000".to_string(),
            "3000".to_string(),
            "".to_string(),
            "1000".to_string(),
            "2000".to_string(),
        ];

        let result = get_result_1(input);
    }

    #[test]
    fn test_split_by_empty_string_with_valid_input() {
        let input = vec![
            "a".to_string(),
            "b".to_string(),
            "".to_string(),
            "c".to_string(),
            "".to_string(),
            "d".to_string(),
        ];
        let expected = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string()],
            vec!["d".to_string()],
        ];
        let result = split_by_empty_string(input);
        assert_eq!(result, expected);
    }
}
