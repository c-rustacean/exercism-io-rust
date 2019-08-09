#[derive(Debug, PartialEq)]
pub struct Clock {
    mins_in_day: i32,
}

const fn mins_in_day() -> i32 {
    24 * 60
}

const fn mins_within_day(overflowed_minutes: i32) -> i32 {
    ((overflowed_minutes % mins_in_day()) + mins_in_day()) % mins_in_day()
}

const fn in_minutes(hours: i32, minutes: i32) -> i32 {
    hours * 60 + minutes
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mins = mins_within_day(in_minutes(hours, minutes));

        Clock { mins_in_day: mins }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mins = mins_within_day(self.mins_in_day + minutes);

        Clock { mins_in_day: mins }
    }
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.mins_in_day / 60, self.mins_in_day % 60)
    }
}
