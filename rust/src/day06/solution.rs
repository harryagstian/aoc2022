const DAY: &str = "06";

use std::collections::HashSet;

use crate::utils;

pub fn test_results() -> (String, String) {
    let part1 = String::from("11");
    let part2 = String::from("26");
    return (part1, part2);
}

pub fn solve(target_input: &str) -> (String, String) {
    let contents = utils::helper::read_file(DAY, target_input);

    let mut part1: usize = 0;
    let mut part2: usize = 0;
    for line in contents.lines() {
        for i in 4..line.len() {
            let mut current_string = line[i-4..i].as_bytes();
            let mut current_set: HashSet<_> = HashSet::from_iter(current_string);

            if current_set.len() == 4 && part1 == 0 {
                part1 = i;
            }

            if i >= 14 {
                current_string = line[i-14..i].as_bytes();
                current_set = HashSet::from_iter(current_string);

                if current_set.len() == 14 && part2 == 0 {
                    part2 = i;
                }
            }

            if part1 != 0 && part2 != 0 {
                break;
            }

            // dbg!(current_string, current_set.len());
        }
    }

    return (part1.to_string(), part2.to_string());
}
