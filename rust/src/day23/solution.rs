const DAY: &str = "23";
use std::collections::{HashMap, HashSet};
use crate::utils;

#[derive(Copy, Clone, Debug)]
enum Directions {
    N,
    NE,
    NW,
    E,
    SE,
    S,
    SW,
    W,
}

impl Directions {
    fn value(&self) -> [i32; 2] {
        match *self {
            Directions::N => [0, -1],
            Directions::NE => [1, -1],
            Directions::NW => [-1, -1],
            Directions::E => [1, 0],
            Directions::SE => [1, 1],
            Directions::S => [0, 1],
            Directions::SW => [-1, 1],
            Directions::W => [-1, 0],
        }
    }
}

#[derive(Debug)]
struct Map {
    data: HashSet<[i32; 2]>,
    first_half_data: HashMap<[i32; 2], Vec<[i32; 2]>>, // new_pos, Vector<old_pos>
    coordinate_check_target: Vec<Vec<Directions>>,
    elf_previously_moves: bool,
    boundary: [[i32; 2]; 2],
}

impl Map {
    pub fn new() -> Map {
        return Map {
            data: HashSet::new(),
            first_half_data: HashMap::new(),
            coordinate_check_target: vec![
                vec![Directions::N, Directions::NE, Directions::NW],
                vec![Directions::S, Directions::SE, Directions::SW],
                vec![Directions::W, Directions::NW, Directions::SW],
                vec![Directions::E, Directions::NE, Directions::SE],
            ],
            elf_previously_moves: true,
            boundary: [[9999, -9999], [9999, -9999]],
        };
    }

    pub fn insert_data(&mut self, pos: [i32; 2]) {
        self.data.insert(pos);
        self.update_boundary(pos);
    }

    // update rectangle boundary
    fn update_boundary(&mut self, pos: [i32; 2]) {
        self.boundary[0][0] = self.boundary[0][0].min(pos[0]);
        self.boundary[0][1] = self.boundary[0][1].max(pos[0]);
        self.boundary[1][0] = self.boundary[1][0].min(pos[1]);
        self.boundary[1][1] = self.boundary[1][1].max(pos[1]);
    }

    // returns new position coordinate from given start position and direction
    fn add_directions(start: &[i32; 2], direction: &Directions) -> [i32; 2] {
        let add_value = direction.value();
        let new_value = [start[0] + add_value[0], start[1] + add_value[1]];

        return new_value;
    }

    // for debugging purpose
    pub fn print_grid(&self, size: i32) {
        for y in -size..=size {
            let mut line = String::from("");

            for x in -size..=size {
                let mut new_value = ".";

                if self.data.contains(&[x, y]) {
                    new_value = "#";
                }

                line.push_str(new_value);
            }
            println!("{}", line);
        }
    }

    pub fn get_empty_ground_in_rectangle(&self) -> u32 {
        let min_x = self.boundary[0][0];
        let max_x = self.boundary[0][1];
        let min_y = self.boundary[1][0];
        let max_y = self.boundary[1][1];

        let mut area = (min_x.abs_diff(max_x) + 1) * (min_y.abs_diff(max_y) + 1);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.data.contains(&[x, y]) {
                    area -= 1;
                }
            }
        }
        return area;
    }

    pub fn do_move(&mut self) {
        self.first_half();
        self.second_half();
    }

    // for given pos, check surrounding coordinates
    // return true if there is elf in surround coordinates, otherwise returns false
    fn elf_in_surrounding(&self, pos: &[i32; 2]) -> bool {
        for direction in [
            Directions::N,
            Directions::NE,
            Directions::NW,
            Directions::E,
            Directions::SE,
            Directions::S,
            Directions::SW,
            Directions::W,
        ] {
            let new_pos = Map::add_directions(pos, &direction);
            if self.data.contains(&new_pos) {
                return true;
            }
        }

        return false;
    }

    fn first_half(&mut self) {
        self.first_half_data.drain(); // empty the data

        for pos in self.data.iter() {
            // if there is no elf surrounding current elf, do nothing
            if !self.elf_in_surrounding(pos) {
                continue;
            }

            // otherwise, check based on directions priority
            for check_target in self.coordinate_check_target.iter() {
                let mut eligible_to_move = true;
                for direction in check_target.iter() {
                    let new_pos = Map::add_directions(pos, direction);

                    // check if there is elf in 3 new coordinates
                    if self.data.contains(&new_pos) {
                        eligible_to_move = false;
                        break;
                    }
                }

                if eligible_to_move {
                    // if there is no elf in 3 new coordinates
                    let new_pos = Map::add_directions(pos, &check_target[0]);

                    let mut new_vec = vec![];

                    if self.first_half_data.contains_key(&new_pos) {
                        new_vec = self.first_half_data.get(&new_pos).unwrap().to_vec();
                    }

                    new_vec.push(*pos);

                    self.first_half_data.insert(new_pos, new_vec);
                    break;
                }
            }
        }
    }

    fn second_half(&mut self) {
        let mut new_pos_arr: Vec<[i32; 2]> = Vec::new();
        self.elf_previously_moves = self.first_half_data.len() > 0; // check if any elves has moved in current round

        // moves all elves simultaneously
        for (new_pos, current_elves) in self.first_half_data.iter() {
            // only moves elves that new_pos doesn't conflict with other elves
            if current_elves.len() == 1 {
                let old_pos = current_elves.last().unwrap();
                self.data.remove(old_pos);
                self.data.insert(*new_pos);
                new_pos_arr.push(*new_pos);
            }
        }

        for new_pos in new_pos_arr {
            // update boundary, workaround for 'borrowing more than once' error
            self.update_boundary(new_pos);
        }

        self.coordinate_check_target.rotate_left(1); // rotate direction check priority
    }
}

pub fn test_results() -> (String, String) {
    let part1 = String::from("110");
    let part2 = String::from("20");
    return (part1, part2);
}

pub fn solve(target_input: &str) -> (String, String) {
    let contents = utils::helper::read_file(DAY, target_input);

    let mut part1 = 0;
    let part2;

    let mut map = Map::new();

    for (index_y, line) in contents.lines().enumerate() {
        for (index_x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert_data([index_x as i32, index_y as i32]);
            }
        }
    }

    let mut iter = 0;

    loop {
        if !map.elf_previously_moves {
            part2 = iter;
            break;
        }

        map.do_move();
        iter += 1;

        if iter == 10 {
            part1 = map.get_empty_ground_in_rectangle();
        }
    }

    return (part1.to_string(), part2.to_string());
}
