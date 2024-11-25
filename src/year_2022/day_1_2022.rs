use crate::day_result::DayResult;
use crate::helpers::fetch_input;

/**
 * Day 1 of 2022:
 * Calorie counting
 */
pub struct Day1;

impl DayResult for Day1 {
    fn print_day_result() {
        let input = fetch_input(1, 2022);
        println!("Result 1: {}", get_result_1(&input));
        println!("Result 2: {}", get_result_2(&input));
    }
}

fn get_result_1(input: &[String]) -> u32 {
    calculate_sums(input).into_iter().max().unwrap_or(0) as u32
}

fn get_result_2(input: &[String]) -> u32 {
    let mut results = calculate_sums(input);
    results.sort_unstable_by(|a, b| b.cmp(a));
    results.iter().take(3).sum()
}

fn calculate_sums(input: &[String]) -> Vec<u32> {
    input
        .split(|s| s.is_empty())
        .map(|slice| {
            slice
                .iter()
                .map(|s| s.parse::<u32>().expect("Failed to parse string to u32"))
                .collect()
        })
        .collect::<Vec<Vec<u32>>>()
        .into_iter()
        .map(|array| array.iter().sum())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &[&str] = &[
        "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
        "10000",
    ];

    fn get_test_input() -> Vec<String> {
        TEST_INPUT.iter().map(|&s| s.to_string()).collect()
    }

    #[test]
    fn test_get_result_1() {
        let result = get_result_1(&get_test_input());
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_get_result_2() {
        let result = get_result_2(&get_test_input());
        assert_eq!(result, 45000);
    }

    #[test]
    fn test_calculate_sums() {
        let result = calculate_sums(&get_test_input());
        let expected = vec![6000, 4000, 11000, 24000, 10000];
        assert_eq!(result, expected);
    }
}
