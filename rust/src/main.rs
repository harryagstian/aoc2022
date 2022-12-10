use chrono::{Datelike, Local};
use clap::Parser;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day08;
mod day09;
mod day10;
mod utils;

#[derive(Parser, Debug)]
struct Args {
    /// Run against test.txt instead of input.txt
    #[arg(short, long, default_value_t = false)]
    test: bool,

    /// What day to solve
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

    let target_input = if args.test { "test.txt" } else { "input.txt" };

    match args.day {
        // TODO: how to make this dynamic?
        1 => {
            (part1, part2) = day01::solution::solve(target_input);
            (expected_result_part1, expected_result_part2) = if args.test {
                day01::solution::test_results()
            } else {
                (String::from(""), String::from(""))
            }
        }
        2 => {
            (part1, part2) = day02::solution::solve(target_input);
            (expected_result_part1, expected_result_part2) = if args.test {
                day02::solution::test_results()
            } else {
                (String::from(""), String::from(""))
            }
        }
        3 => {
            (part1, part2) = day03::solution::solve(target_input);
            (expected_result_part1, expected_result_part2) = if args.test {
                day03::solution::test_results()
            } else {
                (String::from(""), String::from(""))
            }
        }
        4 => {
            (part1, part2) = day04::solution::solve(target_input);
            (expected_result_part1, expected_result_part2) = if args.test {
                day04::solution::test_results()
            } else {
                (String::from(""), String::from(""))
            }
        }
        5 => {
            (part1, part2) = day05::solution::solve(target_input);
            (expected_result_part1, expected_result_part2) = if args.test {
                day05::solution::test_results()
            } else {
                (String::from(""), String::from(""))
            }
        }
        6 => {
            (part1, part2) = day06::solution::solve(target_input);
            (expected_result_part1, expected_result_part2) = if args.test {
                day06::solution::test_results()
            } else {
                (String::from(""), String::from(""))
            }
        }
        8 => {
            (part1, part2) = day08::solution::solve(target_input);
            (expected_result_part1, expected_result_part2) = if args.test {
                day08::solution::test_results()
            } else {
                (String::from(""), String::from(""))
            }
        }
        9 => {
            (part1, part2) = day09::solution::solve(target_input);
            (expected_result_part1, expected_result_part2) = if args.test {
                day09::solution::test_results()
            } else {
                (String::from(""), String::from(""))
            }
        }
        10 => {
            (part1, part2) = day10::solution::solve(target_input);
            (expected_result_part1, expected_result_part2) = if args.test {
                day10::solution::test_results()
            } else {
                (String::from(""), String::from(""))
            }
        }
        _ => todo!("No such day"),
    }
    utils::logger::print_answers(&part1, &part2);

    if args.test {
        assert_eq!(
            &part1, &expected_result_part1,
            "Unexpected test results on part 1"
        );
        assert_eq!(
            &part2, &expected_result_part2,
            "Unexpected test results on part 2"
        );
    }
}
