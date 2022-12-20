const DAY: &str = "20";

use crate::utils;

#[derive(Debug, Clone)]
struct Item {
    value: i64,
    initial_position: usize,
}

fn mix(data: &mut Vec<Item>) {
    let array_length = data.len() - 1;

    for index in 0..=array_length {
        let base_position = data
            .iter()
            .position(|f| f.initial_position == index)
            .unwrap();

        let current_item = data[base_position].clone();

        let mut new_position = base_position as i64 + (current_item.value % array_length as i64);

        while new_position < 0 {
            new_position += array_length as i64;
        }

        while new_position >= array_length as i64 {
            new_position -= array_length as i64;
        }

        data.remove(base_position as usize);
        data.insert(new_position as usize, current_item);
    }
}

pub fn test_results() -> (String, String) {
    let part1 = String::from("3");
    let part2 = String::from("1623178306");
    return (part1, part2);
}

pub fn solve(target_input: &str) -> (String, String) {
    let contents = utils::helper::read_file(DAY, target_input);

    // value, position
    let mut data: Vec<Item> = vec![];
    let initial_data: Vec<Item>;

    for (index, line) in contents.lines().enumerate() {
        let value: i64 = line.parse().unwrap();
        let item = Item {
            value: value,
            initial_position: index,
        };
        data.push(item);
    }

    initial_data = data.clone();

    mix(&mut data);

    let mut zero_position = data.iter().position(|f| f.value == 0).unwrap();
    let part1 = (&data[(zero_position + 1000) % &data.len()].value
        + &data[(zero_position + 2000) % &data.len()].value
        + &data[(zero_position + 3000) % &data.len()].value)
        .to_owned();

    data = initial_data.clone();

    for v in data.iter_mut() {
        v.value *= 811589153;
    }

    for _ in 0..10 {
        mix(&mut data);
    }

    zero_position = data.iter().position(|f| f.value == 0).unwrap();
    let part2 = &data[(zero_position + 1000) % &data.len()].value
        + &data[(zero_position + 2000) % &data.len()].value
        + &data[(zero_position + 3000) % &data.len()].value;

    // dbg!(&zero_position);
    // dbg!(&data[(zero_position + 1000) % &data.len()]);
    // dbg!(&data[(zero_position + 2000) % &data.len()]);
    // dbg!(&data[(zero_position + 3000) % &data.len()]);

    return (part1.to_string(), part2.to_string());
}
