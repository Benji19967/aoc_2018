use std::io::{self, Write, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    for line in input.lines() {
        writeln!(io::stdout(), "{}", line)?;
    }

    // part1(&input)?;
    // part2(&input)?;

    Ok(())
}
