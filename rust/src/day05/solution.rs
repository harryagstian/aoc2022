const DAY: &str = "05";

use crate::utils;

pub fn test_results() -> (String, String) {
    let part1 = String::from("CMZ");
    let part2 = String::from("MCD");
    return (part1, part2);
}

pub fn solve(target_input: &str) -> (String, String) {
    let contents = utils::helper::read_file(DAY, target_input);

    let mut stacks_part_1: Vec<Vec<String>> = Vec::new();
    let mut stacks_part_2: Vec<Vec<String>> = Vec::new();
    let mut mode_parse_input = true;

    for l in contents.lines() {
        if l == "" {
            mode_parse_input = false;
            stacks_part_2 = stacks_part_1.clone();
            continue;
        }

        if mode_parse_input {
            parse_input(&mut stacks_part_1, l);
        } else {
            let value = l.to_string();
            let value_array = value.split_whitespace().collect::<Vec<&str>>(); // parse moves

            let move_count: usize = value_array[1].parse().unwrap();
            let source_stack: usize = value_array[3].parse::<usize>().unwrap() - 1;
            let target_stack: usize = value_array[5].parse::<usize>().unwrap() - 1;

            // part 1
            let source_stack_len_part_1 = stacks_part_1[source_stack].len();
            // pick up items
            let mut moved_stacks_part_1: Vec<String> = stacks_part_1[source_stack]
                .drain(source_stack_len_part_1 - move_count..)
                .collect();
            // drain picks up item in order, while the requirement is LIFO, so we reverse it
            moved_stacks_part_1.reverse();
            stacks_part_1[target_stack].append(&mut moved_stacks_part_1);

            // part 2
            let source_stack_len_part_2 = stacks_part_2[source_stack].len();
            // drain picks up item in order, so we just put it as is
            let mut moved_stacks_part_2 = stacks_part_2[source_stack]
                .drain(source_stack_len_part_2 - move_count..)
                .collect();
            stacks_part_2[target_stack].append(&mut moved_stacks_part_2);
        }
    }

    let mut part1 = String::from("");
    let mut part2 = String::from("");

    for stack in stacks_part_1.iter() {
        part1.push_str(&stack.last().unwrap());
    }

    for stack in stacks_part_2.iter() {
        part2.push_str(&stack.last().unwrap());
    }

    return (part1, part2);
}

fn parse_input(stacks: &mut Vec<Vec<String>>, l: &str) {
    let mut line = l.chars().collect::<Vec<_>>();
    let mut counter = 0;

    // go through each line until its empty
    while line.len() > 0 {
        if stacks.len() <= counter {
            stacks.push(Vec::new());
        }

        let value: Vec<char>;
        if line.len() == 3 {
            // handle last 3 chars in a line
            value = line.drain(0..).collect();
        } else {
            // take next 3 chars, delete the next char - next char is a separator / whitespace
            value = line.drain(0..=2).collect();
            line.remove(0);
        }

        if value[1].is_numeric() {
            // break loop on bottom of the stacks (indicated by numbers)
            return;
        }

        if value.iter().collect::<String>().trim().len() != 0 {
            // if it is not only whitespaces, insert the middle value
            stacks[counter].insert(0, String::from(value[1]))
        }

        counter = counter + 1;
    }
}
