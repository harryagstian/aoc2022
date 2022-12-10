const DAY: &str = "09";

use std::collections::HashSet;

use crate::utils;

pub fn test_results() -> (String, String) {
    let part1 = String::from("13");
    let part2 = String::from("1"); // input for part 2 is using different test.txt
    return (part1, part2);
}

pub fn solve(target_input: &str) -> (String, String) {
    let contents = utils::helper::read_file(DAY, target_input);

    let mut position = vec![vec![0, 0], vec![0, 0]]; // head, tail
    let mut visited: HashSet<Vec<i32>> = HashSet::new();

    visited.insert(position[1].clone());

    for line in contents.lines() {
        do_move(&mut position, &mut visited, &line);
    }

    let part1 = &visited.len();

    // part 2
    position = vec![vec![0, 0]; 10];
    visited.clear();
    visited.insert(position[9].clone());

    for line in contents.lines() {
        do_move(&mut position, &mut visited, &line);
    }

    let part2 = &visited.len();

    return (part1.to_string(), part2.to_string());
}

fn do_move(position: &mut Vec<Vec<i32>>, visited: &mut HashSet<Vec<i32>>, line: &str) {
    let temp: Vec<&str> = line.split_whitespace().collect();
    let dir = temp[0].to_string();
    let step_count = temp[1].parse().unwrap();

    let direction_value = match dir.as_str() {
        "R" => vec![1, 0],
        "D" => vec![0, -1],
        "L" => vec![-1, 0],
        "U" => vec![0, 1],
        _ => todo!("not possible"),
    };

    for _ in 0..step_count {
        // move head
        position[0][0] += direction_value[0];
        position[0][1] += direction_value[1];

        for tail_index in 1..position.len() {
            let tail_move = determine_tail_move(&position[tail_index - 1], &position[tail_index]);

            // move tail
            position[tail_index][0] += tail_move[0];
            position[tail_index][1] += tail_move[1];

            visited.insert(position[position.len()-1].clone());
        }
    }
}

fn determine_tail_move(head: &Vec<i32>, tail: &Vec<i32>) -> Vec<i32> {
    let diff_x = head[0].abs_diff(tail[0]);
    let diff_y = head[1].abs_diff(tail[1]);

    let mut tail_direction_value = vec![0, 0];

    if diff_x == 1 && diff_y == 1 { // 1 tile away diagonally, no need to do anything
    } else if diff_x + diff_y < 2 { // 1 tile away horizontal / vertical, or overlapping, no need to do anything
    } else if diff_x + diff_y == 2 {
        // 2 tiles away horizontal / vertical
        let direction_value_x = match diff_x {
            0 => 0,
            _ => {
                // if t is positive, subtract 1
                // if t is negative, add 1
                let mut t = head[0] - tail[0];
                if t > 0 {
                    t += -1;
                } else {
                    t += 1;
                }
                t
            }
        };
        let direction_value_y = match diff_y {
            0 => 0,
            _ => {
                let mut t = head[1] - tail[1];
                if t > 0 {
                    t += -1;
                } else {
                    t += 1;
                }
                t
            }
        };

        tail_direction_value = vec![direction_value_x, direction_value_y];
    } else {
        // diagonal move
        let diff_y = head[1] - tail[1];
        let diff_x = head[0] - tail[0];

        let direction_value_x;
        let direction_value_y;

        if diff_x.abs() == 1 {
            direction_value_x = diff_x;
        } else if diff_x == -2 {
            // if negative, add 1
            direction_value_x = -1;
        } else if diff_x == 2 {
            // if positive, subtract 1
            direction_value_x = 1;
        } else {
            todo!("not possible")
        }

        if diff_y.abs() == 1 {
            direction_value_y = diff_y;
        } else if diff_y == -2 {
            // if negative, add 1
            direction_value_y = -1;
        } else if diff_y == 2 {
            // if positive, subtract 1
            direction_value_y = 1;
        } else {
            todo!("not possible")
        }

        tail_direction_value = vec![direction_value_x, direction_value_y];
    }

    return tail_direction_value;
}
