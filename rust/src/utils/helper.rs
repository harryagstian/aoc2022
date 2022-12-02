use std::fs;

pub fn read_file(base_path: &str, target_input: &str) -> String {
    let file_path = String::from(format!("{}/{}", base_path, target_input));

    println!("In file {}", file_path);

    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}
