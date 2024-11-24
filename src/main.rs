use aoc_rust::helpers::print_result_for_day;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: u8,
    #[arg(short, long)]
    year: Option<u32>,
}

fn main() {
    let args = Args::parse();
    let year = args.year.unwrap_or(2024);
    print_result_for_day(args.day, year);
}
