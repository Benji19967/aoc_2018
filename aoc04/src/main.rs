#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::{
    collections::HashMap,
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

    let mut events: Vec<Event> = Vec::new();
    for line in input.lines() {
        let event: Event = line.parse()?;
        events.push(event);
        // writeln!(io::stdout(), "{:?}", event)?;
    }

    events.sort_by_key(|e| {
        (
            e.datetime.year,
            e.datetime.month,
            e.datetime.day,
            e.datetime.hour,
            e.datetime.minute,
        )
    });

    let mut current_guard_id: u32;
    let mut current_state: GuardState;
    let mut current_start: u32;
    let mut time_intervals: Vec<TimeInterval> = Vec::new();

    if let EventKind::START { guard_id } = events[0].kind {
        current_guard_id = guard_id;
        current_state = GuardState::AWAKE;
        current_start = events[0].datetime.minute;
    } else {
        return err!("Expected first event to be of start shift kind.");
    }

    for event in &events[1..] {
        let interval: TimeInterval;
        match event.kind {
            EventKind::START { guard_id } => {
                interval = TimeInterval {
                    guard_id: current_guard_id,
                    start: current_start,
                    end: event.datetime.minute,
                    guard_state: current_state,
                };
                current_guard_id = guard_id;
                current_state = GuardState::AWAKE;
            }
            EventKind::SLEEP => {
                interval = TimeInterval {
                    guard_id: current_guard_id,
                    start: current_start,
                    end: event.datetime.minute,
                    guard_state: current_state,
                };
                current_state = GuardState::ASLEEP;
            }
            EventKind::WAKEUP => {
                interval = TimeInterval {
                    guard_id: current_guard_id,
                    start: current_start,
                    end: event.datetime.minute,
                    guard_state: current_state,
                };
                current_state = GuardState::AWAKE;
            }
        }
        time_intervals.push(interval);
        current_start = event.datetime.minute;
    }
    // for event in &events {
    //     println!("{:?}", event);
    // }
    // for interval in &time_intervals {
    //     println!("Interval: {:?}", interval);
    // }

    part1(&time_intervals);
    // part2(&input)?;

    Ok(())
}

fn part1(intervals: &Vec<TimeInterval>) {
    // Find guard that is asleep the most amount of time and during which hour
    // he is most often asleep.
    let mut sleeping_minutes: HashMap<u32, [u32; 60]> = HashMap::new();
    for interval in intervals {
        match interval.guard_state {
            GuardState::ASLEEP => {
                let freq = sleeping_minutes.entry(interval.guard_id).or_insert([0; 60]);
                for minute in interval.start..interval.end {
                    freq[minute as usize] += 1;
                }
            }
            GuardState::AWAKE => ()
        }
    }
    let (sleepiest_guard_id, _) = sleeping_minutes
        .iter()
        .max_by_key(|(_, freq)| -> u32 { freq.iter().sum() })
        .unwrap();
    let (minute_most_asleep, _) = sleeping_minutes[sleepiest_guard_id]
        .iter()
        .enumerate()
        .max_by_key(|(_, freq)| -> u32 { **freq })
        .unwrap();
    println!("Freq: {:?}", sleeping_minutes[sleepiest_guard_id]);
    println!("Sleepiest: {:?}", sleepiest_guard_id);
    println!("Minute most asleep: {:?}", minute_most_asleep);
}

#[derive(Debug)]
struct Event {
    datetime: Datetime,
    kind: EventKind,
}

#[derive(Debug)]
struct TimeInterval {
    guard_id: u32,
    start: u32,
    end: u32, // exclusive
    guard_state: GuardState,
}

#[derive(Debug)]
struct Datetime {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

#[derive(Debug)]
enum EventKind {
    START { guard_id: u32 },
    SLEEP,
    WAKEUP,
}

#[derive(Debug)]
enum GuardState {
    ASLEEP,
    AWAKE,
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
                    (?<hour>[0-9]+):(?<minute>[0-9]+)
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
            hour: caps["hour"].parse()?,
            minute: caps["minute"].parse()?,
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
