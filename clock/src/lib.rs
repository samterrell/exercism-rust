use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    time: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:02}:{:02}", self.time / 60, self.time % 60)
    }
}

const MINUTES_IN_DAY: i32 = 60 * 24;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let time = (minutes + 60 * hours).rem_euclid(MINUTES_IN_DAY);
        Self { time }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.time + minutes)
    }
}
