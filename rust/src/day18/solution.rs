const DAY: &str = "18";
const DIRECTIONS: [[i32; 3]; 6] = [
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
];

use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;

use crate::utils;
#[derive(Eq, Hash, PartialEq, Debug)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
struct Map {
    data: HashSet<[i32; 3]>,
    outer_side: i32,
    min_bound: [i32; 3],
    max_bound: [i32; 3],
}

impl Map {
    fn add_directions(start: &[i32; 3], end: &[i32; 3]) -> [i32; 3] {
        return [start[0] + end[0], start[1] + end[1], start[2] + end[2]];
    }

    pub fn new() -> Map {
        return Map {
            data: HashSet::new(),
            outer_side: 0,
            min_bound: [0, 0, 0],
            max_bound: [0, 0, 0],
        };
    }

    pub fn compare_boundary(&mut self, current_position: &[i32; 3]) {
        self.min_bound[0] = current_position[0].min(self.min_bound[0]);
        self.max_bound[0] = current_position[0].max(self.max_bound[0]);
        self.min_bound[1] = current_position[1].min(self.min_bound[1]);
        self.max_bound[1] = current_position[1].max(self.max_bound[1]);
        self.min_bound[2] = current_position[2].min(self.min_bound[2]);
        self.max_bound[2] = current_position[2].max(self.max_bound[2]);
    }

    pub fn add_cube(&mut self, x: i32, y: i32, z: i32) {
        self.data.insert([x, y, z]);
        let mut visible_side = 6;

        for direction in DIRECTIONS.iter() {
            let new_coordinate = Map::add_directions(&[x, y, z], direction);

            self.compare_boundary(&new_coordinate);

            if self.data.contains(&new_coordinate) {
                visible_side -= 2;
            }
        }

        self.outer_side += visible_side;
    }

    fn is_in_boundary(&self, current_position: &[i32; 3]) -> bool {
        let x = current_position[0];
        let y = current_position[1];
        let z = current_position[2];

        if x < self.min_bound[0] || x > self.max_bound[0] {
            return false;
        }

        if y < self.min_bound[1] || y > self.max_bound[1] {
            return false;
        }

        if z < self.min_bound[2] || z > self.max_bound[2] {
            return false;
        }

        return true;
    }

    pub fn flood_fill(
        &self,
        visited: &mut HashSet<[i32; 3]>,
        current_position: &[i32; 3],
    ) -> (bool, Vec<[i32; 3]>) {
        let mut air_gap_coordinates: Vec<[i32; 3]> = vec![];
        let in_boundary = self.is_in_boundary(current_position);
        let mut all_good = true;

        visited.insert(current_position.clone());

        if !in_boundary {
            return (false, air_gap_coordinates);
        }

        if self.data.contains(current_position) {
            return (true, air_gap_coordinates);
        }

        air_gap_coordinates.push(current_position.clone());

        // fill adjacent coordinate
        for direction in DIRECTIONS.iter() {
            let new_coordinate = Map::add_directions(current_position, direction);
            if visited.contains(&new_coordinate) {
                continue;
            }

            // recurse
            let (result, mut value) = self.flood_fill(visited, &new_coordinate);

            air_gap_coordinates.append(&mut value);


            if !result {
                // if recursive result is out of bound, current traversal is invalid
                all_good = false;
            }
        }

        return (all_good, air_gap_coordinates);
    }
}

pub fn test_results() -> (String, String) {
    let part1 = String::from("64");
    let part2 = String::from("58");
    return (part1, part2);
}

pub fn solve(target_input: &str) -> (String, String) {
    let contents = utils::helper::read_file(DAY, target_input);

    // part 1
    let mut map = Map::new();

    for line in contents.lines() {
        let (x, y, z) = line
            .split(",")
            .map(|f| f.parse().unwrap())
            .collect_tuple()
            .unwrap();

        map.add_cube(x, y, z);
    }

    let part1 = map.outer_side;

    // part 2
    let mut part2 = part1.clone();

    let mut visited: HashSet<[i32; 3]> = HashSet::new();

    // iterate through all cubes coordinate
    for position in map.data.iter() {
        for direction in DIRECTIONS.iter() {
            let new_coordinate = Map::add_directions(position, direction);
            if visited.contains(&new_coordinate) {
                continue;
            }

            // flood-fill adjacent coordinate to find air-gap coordinates
            let (result, value) = map.flood_fill(&mut visited, &new_coordinate);

            if result {
                let mut inner_map = Map::new();
                for v in value.iter() {
                    inner_map.add_cube(v[0], v[1], v[2]);
                }

                part2 -= inner_map.outer_side;
            }
        }
    }

    return (part1.to_string(), part2.to_string());
}
