use chrono::{Datelike, Local};
use clap::Parser;

mod day01;
mod utils;

#[derive(Parser, Debug)]
struct Args {
    // Run against test.txt instead of input.txt
    #[arg(short, long, default_value_t = false)]
    test: bool,

    // What day to solve
    #[arg(short, long, default_value_t = get_current_day())]
    day: u32,
}

fn get_current_day() -> u32 {
    let date = Local::now();
    return date.day();
}
fn main() {
    let args = Args::parse();
    let part1: String;
    let part2: String;
    let expected_result_part1: String;
    let expected_result_part2: String;

    let target_input = match args.test {
        true => "test.txt",
        false => "input.txt",
    };

    match args.day {
        // TODO: how to make this dynamic?
        1 => {
            (part1, part2) = day01::solution::solve(target_input);

            if args.test {
                (expected_result_part1, expected_result_part2) = day01::solution::test_results();
            } else {
                expected_result_part1 = String::from("");
                expected_result_part2 = String::from("");
            }
        }

        _ => todo!(),
    }
    utils::logger::print_answers(&part1, &part2);

    if args.test {
        assert_eq!(
            &part1, &expected_result_part1,
            "Unexpected test results on part 1"
        );
        assert_eq!(
            &part2, &expected_result_part2,
            "Unexpected test results on part 1"
        );
    }
}
