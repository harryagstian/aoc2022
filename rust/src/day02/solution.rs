use std::collections::HashMap;

use crate::utils;

pub fn test_results() -> (String, String) {
    let part1 = String::from("15");
    let part2 = String::from("12");
    return (part1, part2);
}

pub fn determine_score_part_1(
    score_reference_table: &HashMap<&str, i32>,
    left: &str,
    right: &str,
) -> i32 {
    let result: &str;
    let left_hand = *score_reference_table.get(left).unwrap();
    let right_hand = *score_reference_table.get(right).unwrap();

    if left_hand == right_hand {
        result = "draw";
    } else if (right == "X" && left == "C")
        || (right == "Y" && left == "A")
        || (right == "Z" && left == "B")
    {
        result = "win";
    } else {
        result = "lose";
    }

    let result_score = *score_reference_table.get(result).unwrap();
    return right_hand + result_score;
}

pub fn determine_score_part_2(
    score_reference_table: &HashMap<&str, i32>,
    left: &str,
    right: &str,
) -> i32 {
    let hand_to_play: &str;
    let expected_result: &str;

    if right == "X" {
        // lose
        hand_to_play = match left {
            "A" => "Z",
            "B" => "X",
            "C" => "Y",
            _ => todo!("Not possible"),
        };
        expected_result = "lose";
    } else if right == "Y" {
        // draw
        expected_result = "draw";
        hand_to_play = left;
    } else {
        // win
        hand_to_play = match left {
            "A" => "Y",
            "B" => "Z",
            "C" => "X",
            _ => todo!("Not possible"),
        };
        expected_result = "win";
    }

    let result_score = *score_reference_table.get(expected_result).unwrap();
    let choice_score = *score_reference_table.get(hand_to_play).unwrap();

    return choice_score + result_score;
}

pub fn solve(target_input: &str) -> (String, String) {
    let base_path = "src/day02"; // TODO: how to make it reusable?

    let contents = utils::helper::read_file(base_path, target_input);

    let score_reference_table = HashMap::from([
        ("A", 1), // rock
        ("B", 2), // paper
        ("C", 3), // scissors
        ("X", 1), // rock
        ("Y", 2), // paper
        ("Z", 3), // scissors
        ("lose", 0),
        ("draw", 3),
        ("win", 6),
    ]);

    let mut part1 = 0;
    let mut part2 = 0;

    for line in contents.lines() {
        let value: Vec<&str> = line.split_whitespace().collect();
        part1 = part1 + determine_score_part_1(&score_reference_table, value[0], value[1]);
        part2 = part2 + determine_score_part_2(&score_reference_table, value[0], value[1]);
    }

    return (part1.to_string(), part2.to_string());
}
