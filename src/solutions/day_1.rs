use crate::day_result::DayResult;
use crate::helpers::read_file_rows;

pub struct Day1;

impl DayResult for Day1 {
    fn print_day_result() {
        print_result_rows();
    }
}

fn print_result_rows() {
    let input = read_file_rows("day_1.txt");
    for row in input.iter() {
        println!("{}", row);
    }
}
