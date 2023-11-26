use aoc_22::*;
use clap::Parser;
use std::io;

/// AOC 2022
#[derive(Parser, Debug)]
#[command(author = "Roberto Sero", version, about, long_about = None)]
struct Args {
  /// Problem to solve
  day: usize,
}

fn main() {
  let days = [d1, d1_2, d2];
  let max = days.len();
  let args = Args::parse();
  let day = args.day;
  assert!(day >= 1 && day <= max, "Input number between 1 and {max}");
  let stdin = io::stdin();
  // let input = fs::read_to_string("/dev/stdin").expect("No stdin");
  // let stdin = fs::File::open("/dev/stdin").expect("No stdin");
  let problem = days[day - 1];
  problem(stdin);
}
