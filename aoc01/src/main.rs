use std::fs;
use std::path;
use std::collections::HashSet;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn part1(input: &String) -> Result<()> {
    let mut frequency = 0;
    for line in input.lines() {
        frequency += line.parse::<i32>().unwrap();
    }

    println!("Frequency is: {}", frequency);
    Ok(())
}

fn part2(input: &String) -> Result<()> {
    let mut frequency = 0;
    let mut seen: HashSet<i32> = HashSet::new();
    seen.insert(0);

    loop {
        for line in input.lines() {
            frequency += line.parse::<i32>().unwrap();
            if seen.contains(&frequency) {
                println!("First frequency seen twice: {}", frequency);
                return Ok(());
            }
            seen.insert(frequency);
        }
    }
}


fn main() -> Result<()> {
    let input_path: path::PathBuf = "input/input.txt".into();
    let input = fs::read_to_string(input_path).unwrap();

    part1(&input)?;
    part2(&input)?;

    Ok(())
}
