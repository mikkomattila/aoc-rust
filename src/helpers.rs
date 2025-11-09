use crate::day_result::DayResult;
use crate::year_2019::day_1::Day1_2019;
use crate::year_2022::{day_1::Day1_2022, day_2::Day2_2022, day_3::Day3_2022};
use crate::year_2024::{
    day_1::Day1_2024, day_2::Day2_2024, day_3::Day3_2024, day_4::Day4_2024, day_5::Day5_2024,
    day_6::Day6_2024, day_7::Day7_2024,
};
use dotenv::dotenv;
use reqwest::blocking::Client;
use std::env;

/**
 * Print result(s) for a specified day and year.
 * ## Arguments
 * `day_number` - The day number for which the result should be printed.
 */
pub fn print_result_for_day(day_number: u8, year: u32) {
    match year {
        2019 => match day_number{
            1 => Day1_2019::print_day_result(),
            _ => println!("Result for day {} in year {} not found", day_number, year),
        }
        2022 => match day_number {
            1 => Day1_2022::print_day_result(),
            2 => Day2_2022::print_day_result(),
            3 => Day3_2022::print_day_result(),
            _ => println!("Result for day {} in year {} not found", day_number, year),
        },
        2024 => match day_number {
            1 => Day1_2024::print_day_result(),
            2 => Day2_2024::print_day_result(),
            3 => Day3_2024::print_day_result(),
            4 => Day4_2024::print_day_result(),
            5 => Day5_2024::print_day_result(),
            6 => Day6_2024::print_day_result(),
            7 => Day7_2024::print_day_result(),
            _ => println!("Result for day {} in year {} not found", day_number, year),
        },
        _ => println!("Result for day {} in year {} not found", day_number, year),
    }
}

/**
 * Read input for a specified day and year.
 * ## Arguments
 * `day` - The day number for which the input should be fetched.
 * `year` - The year for which the input should be fetched.
 * ## Returns
 * A vector of strings where each string is a line from the input.
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
