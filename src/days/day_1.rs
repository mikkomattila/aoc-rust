use crate::day_result::DayResult;
use crate::helpers::fetch_input;

/**
 * This struct will be for the result of day 1 of 2024.
 */
pub struct Day1;

impl DayResult for Day1 {
    fn print_day_result() {
        print_result_rows();
    }
}

fn print_result_rows() {
    let input = fetch_input(1, 2023);
    for row in input.iter() {
        println!("{}", row);
    }
}
