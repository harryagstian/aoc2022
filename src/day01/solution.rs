use std::fs;

pub fn main() {
    let file_path = "src/day01/input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut counter = 0;
    let mut highest_value = 0;
    let mut current_value = 0;
    let mut value_stacks: Vec<i32> = Vec::new();

    for line in contents.lines() {
        if line == "" {
            if current_value > highest_value {
                highest_value = current_value;
            }
            value_stacks.push(current_value);
            current_value = 0;
            counter = counter + 1;
        } else {
            let value: i32 = line.parse().unwrap();
            current_value = current_value + value;
        }
    }

    if current_value > highest_value {
        highest_value = current_value;
    }
    value_stacks.push(current_value);
    counter = counter + 1;

    println!("{} {}", counter, highest_value);
    value_stacks.sort();
    value_stacks.reverse();
    println!("{:?}", value_stacks);
    println!("{}", value_stacks[0] + value_stacks[1] + value_stacks[2]);
}
