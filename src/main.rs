mod day_result;
mod solutions;

use clap::Parser;
use day_result::DayResult;
use solutions::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day_number: String,
}

fn main() {
    let args = Args::parse();
    match args.day_number.as_str() {
        "1" => day_1::Day1::print_day_result(),
        "2" => day_2::Day2::print_day_result(),
        _ => println!("Result for day {} not found", args.day_number),
    }
}
