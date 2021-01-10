use std::cmp::Ordering;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

#[derive(Debug, Clone)]
pub enum LogEntry {
    StartShift(TimeStamp, usize),
    FallAsleep(TimeStamp),
    WakesUp(TimeStamp),
}

impl LogEntry {
    pub fn get_timestamp(&self) -> TimeStamp {
        match self {
            LogEntry::StartShift(t, _) => t.clone(),
            LogEntry::FallAsleep(t) => t.clone(),
            LogEntry::WakesUp(t) => t.clone(),
        }
    }
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.get_timestamp())?;
        match self {
            LogEntry::StartShift(_, guard) => write!(f, "Guard #{} begins shift", guard),
            LogEntry::FallAsleep(_) => write!(f, "falls asleep"),
            LogEntry::WakesUp(_) => write!(f, "wakes up"),
        }
    }
}

impl FromStr for LogEntry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(start) = s.strip_suffix(" begins shift") {
            Ok(LogEntry::StartShift(
                start[0..18].parse()?,
                start[26..].parse()?,
            ))
        } else if let Some(start) = s.strip_suffix(" falls asleep") {
            Ok(LogEntry::FallAsleep(start.parse()?))
        } else if let Some(start) = s.strip_suffix(" wakes up") {
            Ok(LogEntry::WakesUp(start.parse()?))
        } else {
            Err(parse_error!("Invalid log entry: {}", s))
        }
    }
}

#[derive(Debug, Clone)]
pub struct TimeStamp {
    date: String,
    hour: u32,
    minute: u32,
}

impl TimeStamp {
    pub fn get_minute(&self) -> u32 {
        self.minute
    }

    fn get_time(&self) -> u32 {
        self.hour * 60 + self.minute
    }
}

impl PartialEq for TimeStamp {
    fn eq(&self, other: &Self) -> bool {
        self.date.eq(&other.date) && self.get_time().eq(&other.get_time())
    }
}

impl Eq for TimeStamp {}

impl PartialOrd for TimeStamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.date.partial_cmp(&other.date) {
            Some(Ordering::Equal) => self.get_time().partial_cmp(&other.get_time()),
            Some(o) => Some(o),
            None => None,
        }
    }
}

impl Ord for TimeStamp {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.date.cmp(&other.date) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.get_time().cmp(&other.get_time()),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {:02}:{:02}]", self.date, self.hour, self.minute)
    }
}

impl FromStr for TimeStamp {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TimeStamp {
            date: s[1..11].to_string(),
            hour: s[12..14].parse()?,
            minute: s[15..17].parse()?,
        })
    }
}
