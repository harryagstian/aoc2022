const DAY: &str = "04";

use crate::utils;
use itertools::Itertools;
use std::cmp;

pub fn test_results() -> (String, String) {
    let part1 = String::from("2");
    let part2 = String::from("4");
    return (part1, part2);
}

pub fn solve(target_input: &str) -> (String, String) {
    let contents = utils::helper::read_file(DAY, target_input);
    let mut part1 = 0;
    let mut part2 = 0;

    for line in contents.lines() {
        let (mut left, mut right) = line
            .split(",")
            .map(|x| {
                x.split("-")
                    .map(|f| f.parse().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect_tuple()
            .unwrap();

        left.sort();
        right.sort();

        // is there a better way to check this?
        if (left[0] <= right[0] && right[1] <= left[1]) || (right[0] <= left[0] && left[1] <= right[1]) {
            part1 = part1 + 1;
        }

        if cmp::max(left[0], right[0]) <= cmp::min(left[1], right[1]) {
            part2 = part2 + 1;
        }
    }

    return (part1.to_string(), part2.to_string());
}
