use std::io::{self, Write, Read};
use std::collections::HashSet;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn part1(input: &String) -> Result<()> {
    let mut frequency = 0;
    for line in input.lines() {
        frequency += line.parse::<i32>().unwrap();
    }

    // Using `writeln` rather than `println` https://github.com/BurntSushi/advent-of-code/issues/17
    // In essense: `println` panics if there is an error, whereas `writeln` propagates the
    // error
    writeln!(io::stdout(), "Frequency is: {}", frequency)?;
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
                writeln!(io::stdout(), "First frequency seen twice: {}", frequency)?;
                return Ok(());
            }
            seen.insert(frequency);
        }
    }
}


fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}
