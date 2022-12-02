use std::fs;

pub fn read_file(day: &str, target_input: &str) -> String {
    let file_path = String::from(format!("src/day{}/{}", day, target_input));

    println!("In file {}", file_path);

    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}
