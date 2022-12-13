const DAY: &str = "12";
const START_CHAR: i16 = 83; // -> S
const END_CHAR: i16 = 69; // -> E
const DIRECTION: [[i16; 2]; 4] = [
    [0, 1],
    [1, 0], // south = Y+1
    [0, -1],
    [-1, 0], // north = Y-1
];
use std::collections::HashSet;

use itertools::Itertools;

use crate::utils;

#[derive(Debug)]
struct PriorityQueueItem {
    key: Vec<i16>,
    weight: u32,
}
#[derive(Debug)]
struct PriorityQueue {
    items: Vec<PriorityQueueItem>,
}

impl PriorityQueue {
    pub fn new() -> PriorityQueue {
        return PriorityQueue { items: vec![] };
    }

    pub fn insert(&mut self, queue: PriorityQueueItem) {
        let index = self
            .items
            .iter()
            .position(|f| queue.weight < f.weight)
            .unwrap_or(self.items.len());

        self.items.insert(index, queue);
    }

    pub fn is_empty(&self) -> bool {
        return self.items.len() == 0;
    }

    pub fn shift(&mut self) -> PriorityQueueItem {
        self.items.rotate_left(1);
        return self.items.pop().unwrap();
    }
}

fn traverse(pq: &mut PriorityQueue, map: &Vec<Vec<i16>>, end_pos: &Vec<i16>, part_1: bool) -> u32 {
    let mut visited_coordinates: HashSet<Vec<i16>> = HashSet::new();
    visited_coordinates.insert((&pq.items[0].key).to_owned()); // mark initial coordinate as visited

    while !pq.is_empty() {
        let current = pq.shift(); // take first item in the queue

        for i in 0..DIRECTION.len() {
            // iterate through direction
            let new_coordinate = vec![
                // calculate new coordinate
                current.key[0] + DIRECTION[i][0],
                current.key[1] + DIRECTION[i][1],
            ];

            if visited_coordinates.contains(&new_coordinate) {
                // if we already visit the coordinate, skip
                continue;
            }

            let reachable = is_reachable(&current.key, &DIRECTION[i], &map, &part_1);

            if !reachable {
                // if new coordinate is unreachable, skip
                continue;
            }

            // check end condition based on part 1 or part 2
            let new_map_value = map[new_coordinate[0] as usize][new_coordinate[1] as usize];
            let new_weight = &current.weight + 1;

            if new_coordinate == *end_pos && part_1 {
                // part 1 - need to find E
                return new_weight;
            } else if !part_1 && new_map_value == 97 {
                // part 2 - need to find a
                return new_weight;
            }

            // insert new priority queue item
            let new_pq_item = PriorityQueueItem {
                key: new_coordinate.clone(),
                weight: new_weight,
            };
            pq.insert(new_pq_item);

            // mark new coordinate as visited
            visited_coordinates.insert(new_coordinate);
        }
    }

    return 0;
}

fn is_reachable(
    current_coordinate: &Vec<i16>,
    move_direction: &[i16; 2],
    map: &Vec<Vec<i16>>,
    part_1: &bool,
) -> bool {
    let new_coordinate = vec![
        current_coordinate[0] + move_direction[0],
        current_coordinate[1] + move_direction[1],
    ];

    // check whether new coordinate is within map
    let is_inbound = new_coordinate[0] >= 0
        && new_coordinate[0] < map.len() as i16
        && new_coordinate[1] >= 0
        && new_coordinate[1] < map[0].len() as i16;

    if !is_inbound {
        return false;
    }

    // in part 1, we travel from a --> z
    // in part 2, we travel from z --> a
    let height_difference = match part_1 {
        true => {
            map[new_coordinate[0] as usize][new_coordinate[1] as usize]
                - map[current_coordinate[0] as usize][current_coordinate[1] as usize]
        }
        false => {
            map[current_coordinate[0] as usize][current_coordinate[1] as usize]
                - map[new_coordinate[0] as usize][new_coordinate[1] as usize]
        }
    };

    // we need to ensure height difference from current and new coordinate is at most 1
    return height_difference <= 1;
}

pub fn test_results() -> (String, String) {
    let part1 = String::from("31");
    let part2 = String::from("29");
    return (part1, part2);
}

pub fn solve(target_input: &str) -> (String, String) {
    let contents = utils::helper::read_file(DAY, target_input);

    // Y axis is reversed and start from top to bottom
    // e.g.
    // top left = 0,0
    // bottom right = 9,9
    let mut start_pos: Vec<i16> = vec![];
    let mut end_pos: Vec<i16> = vec![];

    let mut map: Vec<Vec<i16>> = vec![];
    // parse map
    for line in contents.lines() {
        let line_arr = line.as_bytes().iter().map(|&f| f as i16).collect_vec();

        match line_arr.iter().position(|&r| r == START_CHAR) {
            Some(v) => {
                start_pos = vec![map.len() as i16, v as i16];
            }
            None => {}
        }

        match line_arr.iter().position(|&r| r == END_CHAR) {
            Some(v) => {
                end_pos = vec![map.len() as i16, v as i16];
            }
            None => {}
        }

        map.push(line_arr);
    }

    map[start_pos[0] as usize][start_pos[1] as usize] = 97; // replace S to a
    map[end_pos[0] as usize][end_pos[1] as usize] = 122; // replace E to z

    let mut pq = PriorityQueue::new();

    // part 1, we travel from a --> z
    pq.insert(PriorityQueueItem {
        key: start_pos,
        weight: 0,
    });

    let part1 = traverse(&mut pq, &map, &end_pos, true);

    // part 2, we travel from z --> a
    pq.items.clear();
    pq.insert(PriorityQueueItem {
        key: end_pos.clone(),
        weight: 0,
    });

    let part2 = traverse(&mut pq, &map, &end_pos, false);

    return (part1.to_string(), part2.to_string());
}
