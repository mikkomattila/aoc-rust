use crate::day_result::DayResult;
use crate::helpers::fetch_input;

/**
 * Day 1 of 2023.
 */
pub struct Day1;

impl DayResult for Day1 {
    fn print_day_result() {
        let input = fetch_input(1, 2022);

        println!("Result 1: {}", get_result_1(&input));
        println!("Result 2: {}", get_result_2(&input));
    }
}

fn get_result_1(input: &[String]) -> i32 {
    let split_input = split_and_parse_u32(input);
    let mut results: Vec<u32> = Vec::new();
    for array in split_input.iter() {
        let mut sum = 0;
        for item in array.iter() {
            sum += item;
        }
        results.push(sum);
    }
    match results.iter().max() {
        Some(max) => *max as i32,
        None => 0,
    }
}

fn get_result_2(input: &[String]) -> i32 {
    0
}

fn split_and_parse_u32(input: &[String]) -> Vec<Vec<u32>> {
    input
        .split(|s| s.is_empty())
        .map(|slice| {
            slice
                .iter()
                .map(|s| s.parse::<u32>().expect("Failed to parse string to u32"))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result_1_correct_answer() {
        let input = vec![
            "1000".to_string(),
            "2000".to_string(),
            "3000".to_string(),
            "".to_string(),
            "4000".to_string(),
            "".to_string(),
            "5000".to_string(),
            "6000".to_string(),
            "".to_string(),
            "7000".to_string(),
            "8000".to_string(),
            "9000".to_string(),
            "".to_string(),
            "10000".to_string(),
        ];

        let result = get_result_1(&input);
        let expected = 6000;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_result_2_correct_answer() {
        let input = vec![
            "1000".to_string(),
            "2000".to_string(),
            "3000".to_string(),
            "".to_string(),
            "4000".to_string(),
            "".to_string(),
            "5000".to_string(),
            "6000".to_string(),
            "".to_string(),
            "7000".to_string(),
            "8000".to_string(),
            "9000".to_string(),
            "".to_string(),
            "10000".to_string(),
        ];

        let result = get_result_2(&input);
        let expected = 45000;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_and_parse_u32_with_valid_input() {
        let input = vec![
            "1".to_string(),
            "2".to_string(),
            "".to_string(),
            "3".to_string(),
            "".to_string(),
            "4".to_string(),
        ];
        let expected = vec![vec![1, 2], vec![3], vec![4]];
        let result = split_and_parse_u32(&input);
        assert_eq!(result, expected);
    }
}
