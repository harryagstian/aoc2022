const DAY: &str = "15";

use std::collections::HashSet;

use itertools::Itertools;

use crate::utils;

#[derive(Debug)]
struct Item {
    x: i64,
    y: i64,
}

impl Item {
    pub fn new(position_arr: &Vec<&str>) -> Item {
        let (x, y) = Item::parse_position(position_arr);
        return Item { x, y };
    }

    fn parse_position(position_arr: &Vec<&str>) -> (i64, i64) {
        // parse input
        let len = position_arr.len();
        let x = position_arr[len - 2]
            .strip_prefix("x=")
            .unwrap()
            .strip_suffix(",")
            .unwrap()
            .parse()
            .unwrap();
        let y = position_arr[len - 1]
            .strip_prefix("y=")
            .unwrap()
            .parse()
            .unwrap();

        return (x, y);
    }

    pub fn get_scanned_area_on(&self, nearest_beacon: &Item, map: &mut Vec<Vec<[i64; 2]>>) {
        let manhattan_distance = self.get_manhattan_distance_to_other_item(nearest_beacon);

        let min_y = self.y - manhattan_distance;
        let max_y = self.y + manhattan_distance;

        for y in min_y..=max_y {
            // loop through min_y - max_y
            // get scanned X ranges for given Y axis
            // example:
            //                1    1    2    2
            //      0    5    0    5    0    5
            // 10 ....B############...........
            //
            // from the diagram, for y=10, the x range=[2,14]

            let remaining_distance = manhattan_distance - self.y.abs_diff(y) as i64;

            let min_x = self.x - remaining_distance;
            let max_x = self.x + remaining_distance;

            if 0 <= y && y < map.len() as i64 {
                // we only care between range 0-4000000
                map[y as usize].push([min_x, max_x]);
            }
        }
    }

    pub fn get_manhattan_distance_to_other_item(&self, target: &Item) -> i64 {
        // https://en.wikipedia.org/wiki/Taxicab_geometry
        let distance = self.x.abs_diff(target.x) + self.y.abs_diff(target.y);

        return distance as i64;
    }
}

fn merge_ranges(arr: &mut Vec<[i64; 2]>) -> Vec<[i64; 2]> {
    // merge x ranges
    // example:
    // input: [-2, 2], [3, 6]
    // output: [-2, 6]
    let mut return_arr: Vec<[i64; 2]> = vec![];

    arr.sort();
    return_arr.push(arr[0]);

    for i in 1..arr.len() {
        let len = return_arr.len();
        let last_item = return_arr.last().unwrap();
        let next_item = arr[i];

        if last_item[1] + 1 >= next_item[0] {
            // if next item is only 1 diff, merge the item
            return_arr[len - 1][1] = last_item[1].max(next_item[1]);
        } else {
            // otherwise push the item
            return_arr.push(next_item);
        }
    }

    return return_arr;
}

pub fn test_results() -> (String, String) {
    let part1 = String::from("26");
    let part2 = String::from("56000011");
    return (part1, part2);
}

pub fn solve(target_input: &str, is_test: bool) -> (String, String) {
    let mut part_1_y = 2000000;
    let mut part_2_upper_bound: i64 = 4000000;

    if is_test {
        part_1_y = 10;
        part_2_upper_bound = 20;
    }

    let contents = utils::helper::read_file(DAY, target_input);

    let mut map: Vec<Vec<[i64; 2]>> = vec![vec![]; (part_2_upper_bound as usize) + 1]; // we only care between range 0-4000000

    for line in contents.lines() {
        let (sensor_str, beacon_str) = line
            .split(":")
            .map(|f| f.split_whitespace().collect_vec())
            .collect_tuple()
            .unwrap();

        let sensor = Item::new(&sensor_str);
        let beacon = Item::new(&beacon_str);

        sensor.get_scanned_area_on(&beacon, &mut map);
    }

    for i in 0..map.len() {
        if map[i].len() > 1 {
            map[i] = merge_ranges(&mut map[i]);
        }
    }

    // part 1
    // get diff between min_x and max_x for given y axis
    assert!(map[part_1_y as usize].len() == 1);
    let part1 = map[part_1_y as usize][0][0].abs_diff(map[part_1_y as usize][0][1]);

    // part 2
    // find array which is not contiguous
    let part_2_x_arr = map.iter().filter(|f| f.len() > 1).collect_vec();

    // there should be only 1 item that is not contiguous
    assert!(part_2_x_arr.len() == 1);
    assert!(part_2_x_arr[0][1][0] - part_2_x_arr[0][0][1] == 2);

    let part_2_x = part_2_x_arr[0][0][1] + 1;
    let part_2_y = map.iter().position(|f| f.len() > 1).unwrap() as i64;

    let part2 = (part_2_x * 4000000) + part_2_y;

    // part 2 takes about 1 mins
    return (part1.to_string(), part2.to_string());
}
