mod day_result;
mod helpers;
mod solutions;

use clap::Parser;
use helpers::print_result_for_day;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day_number: String,
}

fn main() {
    let args = Args::parse();
    print_result_for_day(args.day_number.as_str());
}
