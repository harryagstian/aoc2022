use std::fs;

pub fn test_results() -> (String, String) {
    let part1 = String::from("22000");
    let part2 = String::from("45000");
    return (part1, part2)
}

pub fn solve(target_input: &str) -> (String, String){
    let base_path = "src/day01"; // TODO: how to make it reusable?

    let file_path = String::from(format!("{}/{}", base_path, target_input));

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut current_value = 0;
    let mut value_stacks: Vec<i32> = Vec::new();

    for line in contents.lines() {
        if line == "" {
            value_stacks.push(current_value);
            current_value = 0;
        } else {
            let value: i32 = line.parse().unwrap();
            current_value = current_value + value;
        }
    }

    value_stacks.push(current_value);
    value_stacks.sort();
    value_stacks.reverse();

    let part1 = value_stacks[0];
    let part2 = value_stacks[0] + value_stacks[1] + value_stacks[2];

    return (part1.to_string(), part2.to_string());
}
