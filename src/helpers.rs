use crate::day_result::DayResult;
use crate::year_2022::*;
use crate::year_2024::*;
use dotenv::dotenv;
use reqwest::blocking::Client;
use std::env;

/**
 * Print the result for the day number passed as argument.
 * # Arguments
 * - `day_number` - The day number for which the result should be printed.
 */
pub fn print_result_for_day(day_number: u8, year: u32) {
    match (year, day_number) {
        (2022, 1) => day_1_2022::Day1::print_day_result(),
        (2022, 2) => day_2_2022::Day2::print_day_result(),
        (2022, 3) => day_3_2022::Day3::print_day_result(),
        (2024, 1) => day_1_2024::Day1::print_day_result(),
        _ => println!("Result for day {} in year {} not found", day_number, year),
    }
}

/**
 * Read input for the specified day and year.
 * # Arguments
 * - `day` - The day number for which the input should be fetched.
 * - `year` - The year for which the input should be fetched.
 */
pub fn fetch_input(day: u32, year: u32) -> Vec<String> {
    dotenv().ok();
    let session =
        env::var("AOC_SESSION_TOKEN").expect("AOC_SESSION_TOKEN is not found in .env file");
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let day_input = Client::new()
        .get(url)
        .header("Cookie", format!("session={}", session))
        .send()
        .expect("Failed to fetch input")
        .text()
        .expect("Failed to read response text");

    day_input.lines().map(|line| line.to_string()).collect()
}
