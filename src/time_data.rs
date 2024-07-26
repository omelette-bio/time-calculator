use std::fmt;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeData {
    hours: usize, // size we want
    minutes: u8, // max. 59
    seconds: u8, // max 59
    milliseconds: u16 // max 999
}

// implement Display trait
impl fmt::Display for TimeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{:02}:{:02}.{:03}", self.hours, self.minutes, self.seconds, self.milliseconds)
    }
}

// implement Default trait, should not allow above 59 for min and sec, and over 999 for
// milliseconds
impl Default for TimeData {
    fn default() -> Self {
        TimeData {
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 0
        }
    }
}

impl TimeData {
    pub fn new(hours: usize, minutes: u8, seconds: u8, milliseconds: u16) -> Self {
        TimeData { hours, minutes, seconds, milliseconds }
    }

    pub fn from_ms(milliseconds : usize) -> Self {
        let mut rem = milliseconds;
        let hours = rem / 3_600_000;
        rem = rem % 3_600_000;

        let minutes = (rem / 60_000) as u8;
        rem = rem % 60_000;

        let seconds = (rem / 1_000) as u8;
        rem = rem % 1_000;

        TimeData {
            hours, minutes, seconds, milliseconds: rem as u16
        }
    }

    pub fn to_ms(&self) -> usize {
        self.milliseconds as usize + self.seconds as usize * 1_000 + self.minutes as usize * 60_000 + self.hours * 3_600_000
    }
}

impl Add for TimeData {
    type Output = TimeData;
    fn add(self, rhs: TimeData) -> TimeData {
        TimeData::from_ms(self.to_ms() + rhs.to_ms())
    }
}

impl Sub for TimeData {
    type Output = TimeData;
    fn sub(self, rhs: TimeData) -> TimeData {
        if self.to_ms() < rhs.to_ms() {
            panic!("");
        }
        TimeData::from_ms(self.to_ms() - rhs.to_ms())
    }
}
