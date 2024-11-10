use crate::day_result::DayResult;
use crate::solutions::*;
use dotenv::dotenv;
use reqwest::blocking::Client;
use std::env;

/**
 * This function will print the result for the day number passed as argument.
 * # Arguments
 * - `day_number` - The day number for which the result should be printed.
 */
pub fn print_result_for_day(day_number: &str) {
    match day_number {
        "1" => day_1::Day1::print_day_result(),
        "2" => day_2::Day2::print_day_result(),
        _ => println!("Result for day {} not found", day_number),
    }
}

/**
 * This function will read input for the specified day and year.
 * # Arguments
 * - `day` - The day number for which the input should be fetched.
 * - `year` - The year for which the input should be fetched.
 */
pub fn fetch_input(day: u32, year: u32) -> Vec<String> {
    dotenv().ok();
    let session = env::var("AOC_SESSION").expect("AOC_SESSION is not found in .env file");
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
