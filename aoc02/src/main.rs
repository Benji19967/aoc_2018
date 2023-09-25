use std::{io::{self, Write, Read}, collections::HashMap};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &String) -> Result<()> {
    let mut doubles = 0;
    let mut triples = 0;
    for line in input.lines() {
        let (doubles_change, triples_change) = count_doubles_and_triples(&line.to_string());
        doubles += doubles_change;
        triples += triples_change;
    }
    writeln!(io::stdout(), "Checksum is: {}", doubles * triples)?;
    Ok(())
}

fn count_doubles_and_triples(input: &String) -> (u32, u32) {
    let mut has_doubles: u32 = 0;
    let mut has_triples: u32 = 0;
    let mut char_counts: HashMap<char, u32> = HashMap::new();

    for c in input.chars() {
        *char_counts.entry(c).or_default() += 1;
    }
    for (_, count) in char_counts.iter() {
        match count {
            2 => has_doubles = 1,
            3 => has_triples = 1,
            _ => ()
        }
    }
    (has_doubles, has_triples)
}


fn part2(input: &str) -> Result<()> {
    let ids: Vec<&str> = input.lines().collect();

    for i in 0..ids.len() {
        for j in i+1..ids.len() {
            if let Some(common_letters) = common_letters_one_diff(ids[i], ids[j]) {
                writeln!(io::stdout(), "Common letters: {}", common_letters)?;
                return Ok(());
            }
        }
    }

    Err(From::from("could not find two correct box ids"))
}

fn common_letters_one_diff(id1: &str, id2: &str) -> Option<String> {
    if id1.len() != id2.len() {
        return None;
    }

    let mut diff_counter = 0;
    for (c1, c2) in id1.chars().zip(id2.chars()) {
        if c1 != c2 {
            diff_counter += 1;
            if diff_counter > 1 {
                return None;
            }
        }
    }

    Some(
        id1.chars().zip(id2.chars())
            .filter(|&(c1, c2)| c1 == c2)
            .map(|(c, _)| c)
            .collect()
    )
}

