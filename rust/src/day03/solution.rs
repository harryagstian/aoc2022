const DAY: &str = "03";

use std::collections::HashSet;

use crate::utils;

pub fn test_results() -> (String, String) {
    let part1 = String::from("157");
    let part2 = String::from("70");
    return (part1, part2);
}

pub fn solve(target_input: &str) -> (String, String) {
    let contents = utils::helper::read_file(DAY, target_input);

    let mut part1 = 0;
    let mut part2: u32 = 0;

    let mut part2_initial_set: HashSet<char> = HashSet::new();
    let mut part2_intersecting_set: HashSet<char> = HashSet::new();

    for (index, line) in contents.lines().enumerate() {
        // part 1
        let (left, right) = line.split_at(line.len() / 2);
        assert!(line.len() % 2 == 0);
        let mut left_set: HashSet<char> = HashSet::new();

        for c in left.chars() {
            left_set.insert(c);
        }

        for c in right.chars() {
            if left_set.contains(&c) {
                part1 = calculate_score(part1, c);
                break;
            }
        }

        // part 2
        if index % 3 == 0 {
            // first line
            // clear everything, add all chars to initial set
            part2_intersecting_set.clear();
            part2_initial_set.clear();
            part2_initial_set.extend(line.chars());
        } else if index % 3 == 1 {
            // second line
            // find match between second line and first line (initial set), and assign it to intersecting set
            for c in line.chars() {
                if part2_initial_set.contains(&c) {
                    part2_intersecting_set.insert(c);
                }
            }
        } else {
            // find match between intersecting set and third line
            for c in line.chars() {
                if part2_intersecting_set.contains(&c) {
                    part2 = calculate_score(part2, c);
                    break;
                }
            }
        }
    }

    return (part1.to_string(), part2.to_string());
}

fn calculate_score(score: u32, c: char) -> u32{
    // A = 65
    // Z = 90
    // a = 97
    // z = 122
    // Lowercase item types a through z have priorities 1 through 26. --> if c <= 90, c - 38
    // Uppercase item types A through Z have priorities 27 through 52. --> if c >= 97, c - 96

    let ascii_value = c as u32;
    if ascii_value <= 90 {
        return score + ascii_value - 38;
    } else if ascii_value >= 97 {
        return score + ascii_value - 96;
    } else {
        todo!("Not possible");
    }
}
