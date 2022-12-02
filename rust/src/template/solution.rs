const DAY: &str = "template";

use std::collections::HashMap;

use crate::utils;

pub fn test_results() -> (String, String) {
    let part1 = String::from("0");
    let part2 = String::from("0");
    return (part1, part2);
}

pub fn solve(target_input: &str) -> (String, String) {
    let contents = utils::helper::read_file(DAY, target_input);

    for line in contents.lines() {
    }

    return (String::from("0"), String::from("0"));
}
