use crate::day_result::DayResult;
use crate::solutions::*;
use std::env;
use std::fs;

pub fn print_requested_day_result(day_number: &str) {
    match day_number {
        "1" => day_1::Day1::print_day_result(),
        "2" => day_2::Day2::print_day_result(),
        _ => println!("Result for day {} not found", day_number),
    }
}

pub fn read_file_rows(file_name: &str) -> Vec<String> {
    let project_root = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    println!("Project root path: {}", project_root);

    let file_path = format!("{}/src/data/{}", project_root, file_name);
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    lines
}
