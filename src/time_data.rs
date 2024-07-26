use std::fmt;
use std::ops::Add;

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
}

impl Add for TimeData {
    type Output = TimeData;
    fn add(self, rhs: TimeData) -> TimeData {
        let tot_ms = self.milliseconds + rhs.milliseconds;
        let ms = tot_ms % 1000;

        let tot_sec = self.seconds + rhs.seconds + (tot_ms / 1000) as u8;
        let sec = tot_sec % 60;

        let tot_min = self.minutes + rhs.minutes + (tot_sec / 60);
        let min = tot_min % 60;

        let hrs = self.hours + rhs.hours + (tot_min / 60) as usize;
        
        TimeData { 
            hours: hrs,
            minutes: min,
            seconds: sec,
            milliseconds: ms
        }
    }
}
