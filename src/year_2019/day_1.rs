use crate::day_result::DayResult;
use crate::helpers::fetch_input;

/**
 * Day 1 of 2019:
 * The Tyranny of the Rocket Equation
 */
pub struct Day1_2019;

impl DayResult for Day1_2019 {
    fn print_day_result() {
        let input = fetch_input(1, 2019);
        let numbers_input: Vec<i32> = input.clone()
            .iter()
            .map(|s| s.parse::<i32>().expect("Failed to parse string to i32"))
            .collect();

        println!("Result 1: {}", get_result_1(numbers_input.clone()));
        println!("Result 2: {}", get_result_2(numbers_input));
    }
}

fn calculate_fuel(n: i32) -> i32 {
    n / 3 - 2
}

fn calculate_fuel_recursive(n: i32) -> i32 {
    let fuel = calculate_fuel(n);
    if fuel < 1 {
        0
    } else {
        fuel + calculate_fuel_recursive(fuel)
    }
}

fn get_result_1(input: Vec<i32>) -> i32 {
    input
        .into_iter()
        .map(|n| calculate_fuel(n))
        .sum()
}

fn get_result_2(input: Vec<i32>) -> i32 {
    input
        .into_iter()
        .map(|n| calculate_fuel_recursive(n))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Vec<i32> {
        vec![12, 14, 1969, 100756]
    }

    #[test]
    fn test_get_result_1() {
        let result = get_result_1(get_test_input());
        assert_eq!(result, 34241);
    }

    #[test]
    fn test_get_result_2() {
        let result = get_result_2(get_test_input());
        assert_eq!(result, 51316);
    }
}
