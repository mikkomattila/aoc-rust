use aoc_rust::helpers::print_result_for_day;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: String,
}

fn main() {
    let args = Args::parse();
    print_result_for_day(args.day.as_str());
}
