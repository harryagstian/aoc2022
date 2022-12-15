const DAY: &str = "14";

use std::collections::HashSet;

use crate::utils;

struct Simulation {
    map_data: HashSet<Vec<i32>>,
    max_depth: i32,
    sand_count: i32,
    answer: [i32; 2],
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation {
            map_data: HashSet::new(),
            max_depth: 0,
            sand_count: 0,
            answer: [0, 0],
        }
    }

    // set max depth / Y
    fn set_max_depth(&mut self, new_value: i32) {
        self.max_depth = self.max_depth.max(new_value);
    }

    pub fn init_map(&mut self, start: &Vec<i32>, end: &Vec<i32>) {
        self.set_max_depth(start[1]);
        self.set_max_depth(end[1]);

        let (direction_x, move_times_x) = self.determine_direction(start[0], end[0]);
        let (direction_y, move_times_y) = self.determine_direction(start[1], end[1]);

        // the line should be horizontal or vertical only - shouldn't go diagonally.
        assert!((move_times_x == 0 || move_times_y == 0) && move_times_x + move_times_y > 0);

        self.map_data.insert(start.clone());
        self.map_data.insert(end.clone());

        let mut i = 1;
        while i < move_times_x || i < move_times_y {
            // move
            // if horizontal, then only direction_x changes
            // if vertical, then only direction_y changes
            let new_x = start[0] + (direction_x * i);
            let new_y = start[1] + (direction_y * i);
            let new_coordinate = vec![new_x, new_y];

            self.map_data.insert(new_coordinate);

            self.set_max_depth(new_y);
            i += 1;
        }
    }

    pub fn simulate_falling_sand(&mut self) {
        loop {
            let mut current_sand_position_x = 500;
            let mut current_sand_position_y = 0;

            loop {
                if self.map_data.contains(&vec![500, 0]) {
                    // solve for part 2
                    self.answer[1] = self.sand_count;
                    return;
                }

                // check if we reach the abyss
                if current_sand_position_y == self.max_depth + 1 {
                    if self.answer[0] == 0 {
                        // if we reach this part, means part 1 is done
                        self.answer[0] = self.sand_count;
                    }

                    // settle immediately, since the abyss is infinite-long floor
                    let new_coordinate = vec![current_sand_position_x, current_sand_position_y];
                    self.map_data.insert(new_coordinate);
                    self.sand_count += 1;

                    break;
                }

                // check if down is a sand / rock
                if self
                    .map_data
                    .contains(&vec![current_sand_position_x, current_sand_position_y + 1])
                {
                    // check if down-left exists
                    if !self.map_data.contains(&vec![
                        current_sand_position_x - 1,
                        current_sand_position_y + 1,
                    ]) {
                        // go down-left
                        current_sand_position_x -= 1;
                        current_sand_position_y += 1;
                        continue;
                    }

                    // check if down-right exists
                    if !self.map_data.contains(&vec![
                        current_sand_position_x + 1,
                        current_sand_position_y + 1,
                    ]) {
                        // go down-right
                        current_sand_position_x += 1;
                        current_sand_position_y += 1;
                        continue;
                    }

                    // otherwise, settle on current position
                    let new_coordinate = vec![current_sand_position_x, current_sand_position_y];

                    self.map_data.insert(new_coordinate);
                    self.sand_count += 1;
                    break;
                }

                // go down once
                current_sand_position_y += 1;
            }
        }
    }

    fn determine_direction(&self, start: i32, end: i32) -> (i32, i32) {
        // for 2 given values, determine how we should move and how much
        // e.g.
        // start: 5
        // end: 0
        // result: we need to move with direction -1 for 5 times
        // direction = -1
        // times = 5
        let mut direction = end - start;
        let times = direction.abs();

        if direction < -1 {
            direction = -1;
        } else if direction > 1 {
            direction = 1;
        }

        return (direction, times);
    }
}

pub fn test_results() -> (String, String) {
    let part1 = String::from("24");
    let part2 = String::from("93");
    return (part1, part2);
}

pub fn solve(target_input: &str) -> (String, String) {
    let contents = utils::helper::read_file(DAY, target_input);

    let mut simulation = Simulation::new();

    for line in contents.lines() {
        // convert input into Array of 2D array
        // e.g:
        // 498,4 -> 498,6 -> 496,6
        // [[498,4],[498,6],[496,6]]
        let arr: Vec<Vec<i32>> = line
            .split("->")
            .map(|f| f.trim().split(",").map(|v| v.parse().unwrap()).collect())
            .collect();

        for index in 1..arr.len() {
            // get current and previous item in array
            let start = &arr[index - 1];
            let end = &arr[index];

            // add rock lines to the map
            simulation.init_map(start, end);
        }
    }

    simulation.simulate_falling_sand();

    let part1 = &simulation.answer[0];
    let part2 = &simulation.answer[1];

    // part 2 takes a while - 10-15 seconds!
    return (part1.to_string(), part2.to_string());
}
