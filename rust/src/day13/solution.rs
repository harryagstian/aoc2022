const DAY: &str = "13";

use std::cmp::Ordering;

use itertools::Itertools;

use crate::utils;

#[derive(Debug, Clone)]
enum Packet {
    Value(u32),
    Packet(Vec<Packet>),
}

impl Eq for Packet {}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        return self == other;
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // implement ordering
            // thankfully rust already covered the base case:
            // [2,3,4] > [1] == true
            // [2,3,4] > [3] == false

            // if both are vector, simply compare
            (Packet::Packet(left), Packet::Packet(right)) => left.cmp(right),
            // if vector vs value, convert value to vector, then compare
            (left @ Packet::Value(_), Packet::Packet(right)) => vec![left.clone()].cmp(right),
            (Packet::Packet(left), right @ Packet::Value(_)) => left.cmp(&(vec![right.clone()])),
            // if both are value, simply compare
            (Packet::Value(left), Packet::Value(right)) => left.cmp(right),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Packet {
    pub fn parse(input: &str) -> Packet {
        // parse string to Packet
        let mut new_packet: Vec<Packet> = vec![];
        let mut number_stacks = String::from("");
        let mut skip_counter = 0;

        for (i, c) in input.char_indices() {
            match c {
                '[' => {
                    if i == 0 {
                        // first character is always '['
                        // skip
                        continue;
                    }

                    // if we found another '[', we count it and recurse
                    skip_counter += 1;
                    let nested_packet = Packet::parse(&input[i..]);
                    // push recursive value to current stack
                    new_packet.push(nested_packet);
                }
                ']' => {
                    if skip_counter > 0 {
                        // skip until all brackets are matched. happens if we recurse
                        skip_counter -= 1;
                        continue;
                    }

                    if number_stacks.len() > 0 {
                        // push value if exists
                        let new_value: u32 = number_stacks.parse().unwrap();
                        new_packet.push(Packet::Value(new_value));
                    }

                    return Packet::Packet(new_packet);
                }
                ',' => {
                    if skip_counter > 0 {
                        // skip until all brackets are matched. happens if we recurse
                        continue;
                    }

                    if number_stacks.len() > 0 {
                        // push value if exists
                        let new_value: u32 = number_stacks.parse().unwrap();
                        new_packet.push(Packet::Value(new_value));
                        number_stacks = String::from("");
                    }
                }
                _ => {
                    if skip_counter > 0 {
                        // skip until all brackets are matched. happens if we recurse
                        continue;
                    }
                    number_stacks.push(c) // add digits to value stack
                }
            }
        }

        return Packet::Packet(new_packet);
    }
}

pub fn test_results() -> (String, String) {
    let part1 = String::from("13");
    let part2 = String::from("140");
    return (part1, part2);
}

pub fn solve(target_input: &str) -> (String, String) {
    let contents = utils::helper::read_file(DAY, target_input);

    let mut part1 = 0;

    let mut all_packets: Vec<Packet> = vec![];
    let divider_packet_1 = Packet::parse("[[2]]");
    let divider_packet_2 = Packet::parse("[[6]]");

    all_packets.push(divider_packet_1.clone());
    all_packets.push(divider_packet_2.clone());

    for (index, two_lines) in contents.split("\n\n").enumerate() {
        // split every blank lines
        let (left_line, right_line) = two_lines.lines().collect_tuple().unwrap(); // split every line

        let left = Packet::parse(left_line);
        let right = Packet::parse(right_line);
        let result = left.cmp(&right);

        all_packets.push(left);
        all_packets.push(right);

        if result == Ordering::Less {
            part1 += index + 1;
        }
    }

    // sort all packages
    all_packets.sort_by(|left, right| left.cmp(right));

    // find divider 1 and divider 2, then multiply
    let part2 = (all_packets
        .iter()
        .position(|f| f.cmp(&divider_packet_1) == Ordering::Equal)
        .unwrap()
        + 1)
        * (all_packets
            .iter()
            .position(|f| f.cmp(&divider_packet_2) == Ordering::Equal)
            .unwrap()
            + 1);

    return (part1.to_string(), part2.to_string());
}
