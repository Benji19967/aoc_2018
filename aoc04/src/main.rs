#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::{
    error::Error,
    io::{self, Read, Write},
    str::FromStr,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

// [1518-11-01 00:00] Guard #10 begins shift
// [1518-11-01 00:05] falls asleep
// [1518-11-01 00:25] wakes up
// [1518-11-01 00:30] falls asleep
// [1518-11-01 00:55] wakes up
// [1518-11-01 23:58] Guard #99 begins shift
// [1518-11-02 00:40] falls asleep
// [1518-11-02 00:50] wakes up
// [1518-11-03 00:05] Guard #10 begins shift
// [1518-11-03 00:24] falls asleep
// [1518-11-03 00:29] wakes up
// [1518-11-04 00:02] Guard #99 begins shift
// [1518-11-04 00:36] falls asleep
// [1518-11-04 00:46] wakes up
// [1518-11-05 00:03] Guard #99 begins shift
// [1518-11-05 00:45] falls asleep
// [1518-11-05 00:55] wakes up

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    for line in input.lines() {
        let event: Event = line.parse()?;
        // writeln!(io::stdout(), "{:?}", event)?;
    }

    // part1(&input)?;
    // part2(&input)?;

    Ok(())
}

fn part1() {}

#[derive(Debug)]
struct Event {
    datetime: Datetime,
    kind: EventKind,
}

#[derive(Debug)]
struct Datetime {
    year: u32,
    month: u32,
    day: u32,
    minute: u32,
    second: u32,
}

#[derive(Debug)]
enum EventKind {
    START { guard_id: u32 },
    SLEEP,
    WAKEUP,
}

// [1518-11-01 00:00] Guard #10 begins shift
// [1518-11-01 00:05] falls asleep
// [1518-11-01 00:25] wakes up
impl FromStr for Event {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                \[
                    (?<year>[0-9]+)-(?<month>[0-9]+)-(?<day>[0-9]+)
                    \s+
                    (?<minute>[0-9]+):(?<second>[0-9]+)
                \] 
                \s+
                (?:Guard\ \#(?<id>[0-9]+)|(?<sleep>.+))
                "
            )
            .unwrap();
        }

        let caps = RE.captures(s).ok_or("Regex failed to capture groups")?;
        let datetime = Datetime {
            year: caps["year"].parse()?,
            month: caps["month"].parse()?,
            day: caps["day"].parse()?,
            minute: caps["minute"].parse()?,
            second: caps["second"].parse()?,
        };
        let kind = if let Some(m) = caps.name("id") {
            EventKind::START {
                guard_id: m.as_str().parse()?,
            }
        } else if &caps["sleep"] == "falls asleep" {
            EventKind::SLEEP
        } else if &caps["sleep"] == "wakes up" {
            EventKind::WAKEUP
        } else {
            return err!("could not determine event kind");
        };

        Ok(Event { datetime, kind })
    }
}
