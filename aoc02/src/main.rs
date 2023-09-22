use std::{io::{self, Write, Read, BufRead}, collections::HashMap};
use std::iter::zip;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

fn part1(input: &String) -> Result<()> {
    let mut doubles = 0;
    let mut triples = 0;
    for line in input.lines() {
        let (doubles_change, triples_change) = count_doubles_and_triples(&line.to_string());
        doubles += doubles_change;
        triples += triples_change;
    }
    writeln!(io::stdout(), "Checkum is: {}", doubles * triples)?;
    Ok(())
}

fn part2(input: &String) -> Result<()> {
    let mut words: Vec<String> = Vec::new();
    for word in input.lines() {
        words.push(word.to_string());
    }
    for (i, word) in words.iter().enumerate() {
        for other_word in &words[i..] {
            let mut diff_count = 0;
            for (c1, c2) in zip(word.chars(), other_word.chars()) {
                if c1 != c2 {
                    diff_count += 1;
                }
            }
            if diff_count == 1 {
                writeln!(io::stdout(), "Word: {} Other word: {}", word, other_word)?;
                return Ok(());
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}
