use std::fs;

pub fn read_file(file_path: String) -> String{
    println!("In file {}", file_path);

    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}
