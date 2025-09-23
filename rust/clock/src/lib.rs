use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;

        let normalized_minutes = total_minutes.rem_euclid(24 * 60);

        let final_hours = normalized_minutes / 60;
        let final_minutes = normalized_minutes % 60;

        Clock {
            hours: final_hours,
            minutes: final_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let current_minutes = (self.hours as i32) * 60 + (self.minutes as i32);

        let total_minutes = current_minutes + minutes;

        let wrapped_minutes = total_minutes.rem_euclid(24 * 60);

        let new_hours = wrapped_minutes / 60;
        let new_minutes = wrapped_minutes % 60;

        Clock {
            hours: new_hours,
            minutes: new_minutes,
        }
    }
}
