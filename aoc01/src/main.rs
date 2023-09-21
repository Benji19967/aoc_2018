use std::fs;
use std::path;

fn main() {
    let input_path: path::PathBuf = "input/input.txt".into();
    let input = fs::read_to_string(input_path).unwrap();

    let mut frequency = 0;
    for line in input.lines() {
        frequency += line.parse::<i32>().unwrap();
    }

    println!("Frequency is: {}", frequency);
}
