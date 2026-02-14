use std::fmt::{Debug, Display};

pub struct Clock {
    hours: i32,
    minutes: i32,
}

fn mod_eucludien(x: i32, y: i32) -> i32 {
    ((x % y) + y) % y
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours_by_minutes = minutes / 60;
        let mut hours = hours + hours_by_minutes;
        if minutes < 0 && minutes % 60 != 0 {
            hours -= 1;
        }
        hours = mod_eucludien(hours, 24);
        let minutes = mod_eucludien(minutes, 60);
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Debug for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
