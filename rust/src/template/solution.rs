const DAY: &str = "template";

use crate::utils;

pub fn test_results() -> (String, String) {
    let part1 = String::from("0");
    let part2 = String::from("0");
    return (part1, part2);
}

pub fn solve(target_input: &str) -> (String, String) {
    let contents = utils::helper::read_file(DAY, target_input);

    let mut part1 = 0;
    let mut part2 = 0;

    for line in contents.lines() {
    }

    return (part1.to_string(), part2.to_string());
}
