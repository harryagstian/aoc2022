const DAY: &str = "08";

use crate::utils;

pub fn test_results() -> (String, String) {
    let part1 = String::from("21");
    let part2 = String::from("8");
    return (part1, part2);
}

pub fn solve(target_input: &str) -> (String, String) {
    let contents = utils::helper::read_file(DAY, target_input);

    let mut left_to_right_array: Vec<Vec<u32>> = vec![];
    let mut top_to_bottom_array: Vec<Vec<u32>> = vec![];

    for line in contents.lines() {
        let mut left_to_right = vec![];
        for (char_index, c) in line.chars().enumerate() {
            let value = c.to_digit(10).unwrap();

            left_to_right.push(value); // fill in left to right to temporary array

            if top_to_bottom_array.len() <= char_index {
                top_to_bottom_array.push(vec![]);
            }
            top_to_bottom_array[char_index].push(value); // fill in right to bottom
        }
        left_to_right_array.push(left_to_right); // insert left to right from temporary array
    }

    assert!(left_to_right_array.len() == top_to_bottom_array.len()); // we assume the input is square

    let mut part1 = (left_to_right_array.len() * 4) - 4; // all trees in edges are counted as visible
    let mut part2 = 0;

    // 1..len() -1 is to skip edge
    for index_y in 1..left_to_right_array.len() - 1 {
        let line_left_to_right = &left_to_right_array[index_y];

        for index_x in 1..line_left_to_right.len() - 1 {
            let line_top_to_bottom = &top_to_bottom_array[index_x];

            let current_tree_height = &line_left_to_right[index_x];

            // part 1
            let visible_from_left =
                is_visible(&current_tree_height, &line_left_to_right, 0, index_x);
            let visible_from_right = is_visible(
                &current_tree_height,
                &line_left_to_right,
                index_x + 1,
                line_left_to_right.len(),
            );
            let visible_from_top =
                is_visible(&current_tree_height, &line_top_to_bottom, 0, index_y);
            let visible_from_bottom = is_visible(
                &current_tree_height,
                &line_top_to_bottom,
                index_y + 1,
                line_top_to_bottom.len(),
            );

            if visible_from_left || visible_from_right || visible_from_top || visible_from_bottom {
                part1 = part1 + 1;
            }

            // part 2
            let distance_left =
                get_distance(&current_tree_height, &line_left_to_right, 0, index_x, true);
            let distance_right = get_distance(
                &current_tree_height,
                &line_left_to_right,
                index_x + 1,
                line_left_to_right.len(),
                false,
            );
            let distance_top =
                get_distance(&current_tree_height, &line_top_to_bottom, 0, index_y, true);
            let distance_bottom = get_distance(
                &current_tree_height,
                &line_top_to_bottom,
                index_y + 1,
                line_top_to_bottom.len(),
                false,
            );

            let scenic_value = distance_left * distance_right * distance_top * distance_bottom;

            if scenic_value > part2 {
                part2 = scenic_value
            }
        }
    }

    return (part1.to_string(), part2.to_string());
}

fn is_visible(current_value: &u32, line: &Vec<u32>, start_index: usize, end_index: usize) -> bool {
    return current_value > line[start_index..end_index].iter().max().unwrap();
}

fn get_distance(
    current_value: &u32,
    line: &Vec<u32>,
    start_index: usize,
    end_index: usize,
    reverse: bool,
) -> u32 {
    let mut value = 0;

    let mut arr = line[start_index..end_index].to_owned().to_vec();

    if reverse {
        // if we are looking from right to left / bottom to top, array should be reversed
        arr.reverse()
    }

    for c in arr {
        if *current_value > c {
            value = value + 1;
        } else {
            // if we meet tree, count it and break
            value = value + 1;
            break;
        }
    }
    return value;
}
