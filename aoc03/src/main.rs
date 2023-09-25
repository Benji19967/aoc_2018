#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::{
    collections::HashMap,
    error::Error,
    io::{self, Read, Write},
    str::FromStr,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

type Grid = HashMap<(u32, u32), u32>;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

// input: #1 @ 604,100: 17x27
// input: #id @ left,top: wide x tall

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut grid = Grid::new();
    let claims = parse_lines_into_claims(&input)?;
    for claim in &claims {
        populate_grid(claim, &mut grid);
    }

    part1(&grid)?;
    // part2(&input)?;
    Ok(())
}

fn parse_lines_into_claims(input: &str) -> Result<Vec<Claim>> {
    let mut claims = Vec::new();
    for line in input.lines() {
        let claim: Claim = line
            .parse()
            .or_else(|err| err!("failed to parse '{:?}': {}", line, err))?;
        claims.push(claim);
    }
    Ok(claims)
}

fn populate_grid(claim: &Claim, grid: &mut Grid) {
    for x in claim.x..claim.x + claim.width {
        for y in claim.y..claim.y + claim.height {
            *grid.entry((x, y)).or_default() += 1;
        }
    }
}

fn part1(grid: &Grid) -> Result<()> {
    let count = grid.values().filter(|&&count| count >= 2).count();
    writeln!(io::stdout(), "Number of claims with count of 2 or more: {}", count)?;
    Ok(())
}

fn get_num_rows_and_cols(input: &str) -> (u32, u32) {
    // let (mut max_rows, mut max_cols) = (0, 0);
    // for line in input.lines() {
    //     id, dist_left, dist_top, width, height = parse_line(&line);
    // }
    (0, 0)
}

#[derive(Debug)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl FromStr for Claim {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Claim> {
        // lazy_static makes sure the compiler can optimize and only compiles the
        // regex once, even if this function is used in a loop.
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                \#
                (?P<id>[0-9]+)
                \s+@\s+
                (?P<x>[0-9]+),(?P<y>[0-9]+):
                \s+
                (?P<width>[0-9]+)x(?P<height>[0-9]+)"
            )
            .unwrap();
        }
        let caps = RE.captures(s).ok_or("Regex failed to capture groups")?;
        Ok(Claim {
            id: caps["id"].parse()?,
            x: caps["x"].parse()?,
            y: caps["y"].parse()?,
            width: caps["width"].parse()?,
            height: caps["height"].parse()?,
        })
    }
}

// #123 @ 3,2: 5x4
//...........
//...........
//...#####...
//...#####...
//...#####...
//...#####...
//...........
//...........
//...........
