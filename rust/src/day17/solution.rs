const DAY: &str = "17";

use std::collections::HashSet;

use crate::utils;


#[derive(Copy, Clone, Debug, PartialEq)]
enum Directions {
    Down,
    Left,
    Right,
}

impl Directions {
    fn value(&self) -> [i32; 2] {
        match *self {
            Directions::Down => [0, -1],
            Directions::Left => [-1, 0],
            Directions::Right => [1, 0],
        }
    }
}

// A
// ####

// B
// .#.
// ###
// .#.

// C
// ..#
// ..#
// ###

// D
// #
// #
// #
// #

// E
// ##
// ##

enum TetrisPieceType {
    A,
    B,
    C,
    D,
    E,
}

struct TetrisPiece {
    pos: Vec<[i32;2]>
}

impl TetrisPiece {
    fn new(piece: TetrisPieceType) -> TetrisPiece {
        let new_vec = match piece {
            TetrisPieceType::A => vec![
                [0,0],
                [0,1],
                [0,2],
                [0,3],
            ],
            TetrisPieceType::B => vec![
                [1,0],
                [1,1],
                [0,1],
                [2,1],
                [1,2],
            ],
            TetrisPieceType::C => vec![
                [0,0],
                [0,1],
                [0,2],
                [1,2],
                [2,2],
            ],
            TetrisPieceType::D => vec![
                [0,0],
                [1,0],
                [2,0],
                [3,0],
            ],
            TetrisPieceType::E => vec![
                [0,0],
                [0,1],
                [1,0],
                [1,1],
            ],
        };

        return TetrisPiece { pos: new_vec };
    }

    fn add_directions(start: [i32;2], direction: &Directions) -> [i32;2] {
        let dir_value = direction.value();
        let return_value = [start[0] + dir_value[0], start[1] + dir_value[1]];

        return return_value;
    }

    fn do_move(&mut self, tetris: &mut Tetris, direction: &Directions) {
        let mut new_pos_arr : Vec<[i32;2]> = vec![];

        let mut collide = false;
        for index in 0..self.pos.len() {
            let current_pos = self.pos[index];
            let new_pos = TetrisPiece::add_directions(current_pos, direction);

            new_pos_arr.push(new_pos);

            if tetris.field.contains(&new_pos) {
                collide = true;
            }
        }


        if *direction == Directions::Down && collide{
            // freeze piece

        }

        if !collide {
            self.pos = new_pos_arr;
        }

        // TODO: implement collission
    }
}

struct Tetris {
    field: HashSet<[i32;2]>,
}


pub fn test_results() -> (String, String) {
    let part1 = String::from("3068");
    let part2 = String::from("0");
    return (part1, part2);
}

pub fn solve(target_input: &str) -> (String, String) {
    let contents = utils::helper::read_file(DAY, target_input);

    let mut part1 = 0;
    let mut part2 = 0;

    for line in contents.lines() {
    }

    return (part1.to_string(), part2.to_string());
}
