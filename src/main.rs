mod day_result;
mod helpers;
mod solutions;

use clap::Parser;
use helpers::print_requested_day_result;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day_number: String,
}

fn main() {
    let args = Args::parse();
    print_requested_day_result(args.day_number.as_str());
}
